use interprocess::{
    os::windows::named_pipe::{
        MsgReaderPipeStream, MsgWriterPipeStream, PipeListenerOptions, PipeMode,
    },
    ReliableReadMsg,
};
use log::error;
use named_lock::NamedLock;
use parking_lot::{Condvar, Mutex};
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, io::Write, sync::Arc};

use crate::{api::Api, app::TigerApp, utils::handle};

static PIPE_NAME: &str = "tiger-named-pipe";
static LOCK_NAME: &str = "tiger-startup-mutex";

pub struct StartupGuard {
    release: Arc<(Mutex<bool>, Condvar)>,
}

pub type StartupGuardHandle = handle::Handle<Option<StartupGuard>>;

#[derive(Debug, Deserialize, Serialize)]
enum Message {
    FocusWindow,
    OpenDocuments(Vec<String>),
}

pub fn acquire_startup_guard() -> StartupGuard {
    let acquisition = Arc::new((Mutex::new(false), Condvar::new()));
    let release = Arc::new((Mutex::new(false), Condvar::new()));

    std::thread::spawn({
        let acquisition = acquisition.clone();
        let release = release.clone();
        move || {
            let named_lock = NamedLock::create(LOCK_NAME).unwrap();
            let named_lock_guard = named_lock.lock().unwrap();

            {
                let &(ref lock, ref cvar) = &*acquisition;
                *lock.lock() = true;
                cvar.notify_one();
            }

            {
                let &(ref lock, ref cvar) = &*release;
                let mut can_be_released = lock.lock();
                while !*can_be_released {
                    cvar.wait(&mut can_be_released);
                }
            }

            drop(named_lock_guard);
        }
    });

    {
        let &(ref lock, ref cvar) = &*acquisition;
        let mut acquired = lock.lock();
        while !*acquired {
            cvar.wait(&mut acquired);
        }
    }

    StartupGuard { release }
}

impl Drop for StartupGuard {
    fn drop(&mut self) {
        let &(ref lock, ref cvar) = &*self.release;
        *lock.lock() = true;
        cvar.notify_one();
    }
}

pub fn attach_to_primary_instance(
    command_line_arguments: Vec<String>,
    _startup_guard: &StartupGuard,
) -> Result<bool, ()> {
    let Ok(mut write_pipe) = MsgWriterPipeStream::connect(OsStr::new(PIPE_NAME)) else {
        return Ok(false);
    };
    send_message(&mut write_pipe, &Message::FocusWindow)?;
    send_message(
        &mut write_pipe,
        &Message::OpenDocuments(command_line_arguments),
    )?;
    Ok(true)
}

pub fn become_primary_instance<A: TigerApp + Api + Clone + Send + Sync + 'static>(
    app: A,
    _startup_guard: &StartupGuard,
) {
    std::thread::spawn(move || {
        let read_pipe = PipeListenerOptions::new()
            .mode(PipeMode::Messages)
            .name(OsStr::new(PIPE_NAME))
            .create::<MsgReaderPipeStream>()
            .unwrap();

        for incoming in read_pipe.incoming() {
            match incoming {
                Ok(stream) => {
                    std::thread::spawn({
                        let app = app.clone();
                        move || handle_attached_process(app, stream)
                    });
                }
                Err(e) => {
                    error!("Error reading from single instance named pipe: {e}");
                    continue;
                }
            }
        }
    });
}

fn handle_attached_process<A: TigerApp + Api + Clone>(app: A, mut stream: MsgReaderPipeStream) {
    let mut buffer = Vec::<u8>::new();
    loop {
        match stream.try_read_msg(buffer.as_mut_slice()) {
            Ok(Ok(size)) => {
                if let Ok(message) = serde_json::from_slice::<Message>(&buffer[..size]) {
                    receive_message(app.clone(), message);
                }
            }
            Ok(Err(size)) => {
                if size < 10 * 1024 {
                    buffer.resize(size, 0);
                } else {
                    error!(
                        "Single instance named pipe received an oversized message ({size} bytes)"
                    );
                    return;
                }
            }
            Err(_) => {
                return;
            }
        }
    }
}

fn receive_message<A: TigerApp + Api>(app: A, message: Message) {
    match message {
        Message::FocusWindow => app.focus_window(),
        Message::OpenDocuments(d) => tauri::async_runtime::block_on(async {
            Api::open_documents(&app, d).await.ok();
            app.replace_state();
        }),
    }
}

fn send_message(write_pipe: &mut MsgWriterPipeStream, message: &Message) -> Result<(), ()> {
    let message = serde_json::to_string(&message).unwrap();
    match write_pipe.write_all(message.as_bytes()) {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("Error writing to single instance named pipe: {e}");
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {

    use retry::{delay::Fixed, retry};
    use std::time::Duration;

    use super::*;
    use crate::app::mock::{TigerAppMock, TigerAppMockBuilder};

    #[tokio::test]
    async fn can_release_startup_guard() {
        let app = TigerAppMockBuilder::new().with_startup_guard().build();
        assert!(!app.is_startup_complete());
        app.finalize_startup().await;
        assert!(app.is_startup_complete());
    }

    #[tokio::test]
    async fn startup_guard_blocks_other_instances() {
        let app = TigerAppMockBuilder::new().with_startup_guard().build();
        assert!(!app.is_startup_complete());

        let acquired_inner_guard = Arc::new(Mutex::new(false));
        std::thread::spawn({
            let acquired_inner_guard = acquired_inner_guard.clone();
            move || {
                let _app = TigerAppMockBuilder::new().with_startup_guard().build();
                *acquired_inner_guard.lock() = true;
            }
        });

        std::thread::sleep(Duration::from_millis(500));
        assert!(!*acquired_inner_guard.lock());

        app.finalize_startup().await;
        let acquired_inner_guard = retry(Fixed::from_millis(100).take(100), || {
            (*acquired_inner_guard.lock()).then_some(()).ok_or(())
        });
        assert!(acquired_inner_guard.is_ok());
    }

    #[tokio::test]
    async fn cross_instance_communication() {
        let guard = acquire_startup_guard();
        let app = TigerAppMock::new();

        assert_eq!(
            attach_to_primary_instance(vec!["test-data/samurai.tiger".into()], &guard),
            Ok(false)
        );
        assert!(!app.is_focused());

        become_primary_instance(app.clone(), &guard);

        let attached = retry(
            Fixed::from_millis(100).take(100),
            || match attach_to_primary_instance(vec!["test-data/samurai.tiger".into()], &guard) {
                Ok(true) => Ok(()),
                Ok(false) => Err("Nothing to attach to".into()),
                Err(e) => Err(format!("Failed to attach: {e:?}")),
            },
        );
        assert_eq!(attached, Ok(()));

        let focused_window = retry(Fixed::from_millis(100).take(100), || {
            app.is_focused().then_some(()).ok_or(())
        });
        assert!(focused_window.is_ok());

        let opened_document = retry(Fixed::from_millis(100).take(100), || {
            (!app.client_state().documents.is_empty())
                .then_some(())
                .ok_or(())
        });
        assert!(opened_document.is_ok());
    }
}
