use log::error;
use squeak::Response;
use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::mpsc::channel,
    thread,
};

use crate::app::TigerApp;

pub fn init<A: TigerApp + Send>(app: A) {
    let state_handle = app.state();
    let mut state = state_handle.lock();
    let recent_documents_file = app.paths().lock().recent_documents_file.clone();

    match read_from_disk(&recent_documents_file) {
        Ok(mut documents) => {
            documents.retain(|d| d.exists());
            state.set_recent_documents(documents);
        }
        Err(e) => error!("Error while reading list of recently opened documents: {e}"),
    };

    let (tx, rx) = channel();
    state
        .recent_documents_delegate()
        .subscribe(move |recent_documents| {
            tx.send(recent_documents.clone()).ok();
            Response::StaySubscribed
        });

    thread::Builder::new()
        .name("recent-documents-thread".to_owned())
        .spawn(move || loop {
            let Ok(recent_documents) = rx.recv() else { break };
            if let Err(e) = write_to_disk(&recent_documents, &recent_documents_file) {
                error!("Error while saving list of recently opened documents: {e}");
            }
        })
        .unwrap();
}

fn write_to_disk(documents: &Vec<PathBuf>, destination: &Path) -> Result<(), std::io::Error> {
    let file = File::create(destination)?;
    serde_json::to_writer_pretty(file, documents)?;
    Ok(())
}

fn read_from_disk(source: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    if !source.exists() {
        return Ok(vec![]);
    }
    let file = File::open(source)?;
    let recent_files: Vec<PathBuf> = serde_json::from_reader(file)?;
    Ok(recent_files)
}

#[cfg(test)]
mod tests {

    use retry::{delay::Fixed, retry};
    use sugar_path::SugarPath;

    use super::*;
    use crate::{
        app::mock::{TigerAppMock, TigerAppMockBuilder},
        dto::RecentDocument,
    };

    #[test]
    fn reads_recent_documents_from_disk() {
        let samurai_file = PathBuf::from("test-data/samurai.tiger").resolve();
        let flame_file = PathBuf::from("test-data/flame.tiger").resolve();
        let recent_documents = vec![flame_file.clone(), samurai_file.clone()];
        let recent_documents_file = PathBuf::from("test-output/reads_recent_documents_from_disk");

        std::fs::write(
            &recent_documents_file,
            serde_json::to_string(&recent_documents).unwrap(),
        )
        .unwrap();

        let mut app_builder = TigerAppMockBuilder::new();
        app_builder.paths_mut().recent_documents_file = recent_documents_file;
        let app = app_builder.build();

        assert_eq!(
            app.client_state().recent_document_paths,
            vec![
                RecentDocument {
                    path: flame_file,
                    name: "flame.tiger".into(),
                },
                RecentDocument {
                    path: samurai_file,
                    name: "samurai.tiger".into(),
                }
            ]
        );
    }

    #[tokio::test]
    async fn writes_recent_documents_to_disk() {
        let samurai_file = PathBuf::from("test-data/samurai.tiger").resolve();
        let flame_file = PathBuf::from("test-data/flame.tiger").resolve();

        let app = TigerAppMock::new();
        let recent_documents_file = app.paths().lock().recent_documents_file.clone();
        app.open_documents(vec![&samurai_file, &flame_file]).await;

        let wrote_to_disk = retry(Fixed::from_millis(500).take(10), || {
            let Ok(recent_documents) = read_from_disk(&recent_documents_file) else {
                return Err("Read error");
            };
            match recent_documents == vec![flame_file.clone(), samurai_file.clone()] {
                true => Ok(()),
                false => Err("Content mismatch"),
            }
        });
        assert_eq!(Ok(()), wrote_to_disk);
    }
}
