use std::{collections::HashSet, path::PathBuf};

use crate::{
    document::Document,
    sheet::{Paths, Sheet},
    state::State,
};

pub trait TextureList {
    fn list_textures(&self) -> HashSet<PathBuf>;
}

impl TextureList for State {
    fn list_textures(&self) -> HashSet<PathBuf> {
        self.documents_iter()
            .flat_map(|d| d.list_textures())
            .collect::<HashSet<_>>()
    }
}

impl TextureList for Document {
    fn list_textures(&self) -> HashSet<PathBuf> {
        self.sheet().list_textures()
    }
}

impl<P: Paths> TextureList for Sheet<P> {
    fn list_textures(&self) -> HashSet<PathBuf> {
        self.frames_iter()
            .map(|f| f.source().to_owned())
            .collect::<HashSet<_>>()
    }
}
