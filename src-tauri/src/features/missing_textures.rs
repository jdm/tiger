use std::{collections::{HashMap, HashSet}, path::PathBuf, thread, time::Duration};

use crate::{app::TigerApp, dto::StateTrim, utils::texture_list::TextureList};

#[cfg(not(test))]
static PERIOD: Duration = Duration::from_millis(500);
#[cfg(test)]
static PERIOD: Duration = Duration::from_millis(100);

pub fn init<A: TigerApp + Send + Clone + 'static>(app: A) {
    thread::Builder::new()
        .name("missing-textures-thread".to_owned())
        .spawn(move || loop {
            thread::sleep(PERIOD);

            let (all_textures, old_missing_textures) = {
                let state_handle = app.state();
                let state = state_handle.lock();
                let mut all_textures = HashMap::new();
                let mut old_missing_textures = HashMap::new();
                for document in state.documents_iter() {
                    all_textures.insert(document.path().to_owned(), document.list_textures());
                    old_missing_textures.insert(
                        document.path().to_owned(),
                        document.missing_textures().clone(),
                    );
                }
                (all_textures, old_missing_textures)
            };

            let mut new_missing_textures: HashMap<PathBuf, HashSet<PathBuf>> = all_textures
                .into_iter()
                .map(|(p, t)| (p, t.into_iter().filter(|t| !t.exists()).collect()))
                .collect();

            if old_missing_textures != new_missing_textures {
                app.patch_state(StateTrim::Full, |state| {
                    for document in state.documents_iter_mut() {
                        if let Some(textures) = new_missing_textures.remove(document.path()) {
                            document.set_missing_textures(textures);
                        }
                    }
                });
            }
        })
        .unwrap();
}

#[cfg(test)]
mod tests {

    use retry::{delay::Fixed, retry};
    use std::{fs::File, path::PathBuf};

    use super::*;
    use crate::app::mock::TigerAppMock;

    #[test]
    fn detects_texture_addition_and_removal() {
        let filename = "test-output/detects_texture_addition_and_removal.png";
        std::fs::remove_file(filename).ok();

        let is_texture_missing =
            |app: &TigerAppMock| app.document().sheet.frames[0].missing_on_disk;

        let check_missing = |app: &TigerAppMock| {
            retry(Fixed::from(PERIOD).take(100), || {
                is_texture_missing(app).then_some(()).ok_or(())
            })
        };

        let check_present = |app: &TigerAppMock| {
            retry(Fixed::from(PERIOD).take(100), || {
                (!is_texture_missing(app)).then_some(()).ok_or(())
            })
        };

        let app = TigerAppMock::new();
        app.new_document("tmp.tiger");
        app.import_frames(vec![PathBuf::from(filename)]);
        assert!(check_missing(&app).is_ok());
        File::create(filename).unwrap();
        assert!(check_present(&app).is_ok());
        std::fs::remove_file(filename).ok();
        assert!(check_missing(&app).is_ok());
    }
}
