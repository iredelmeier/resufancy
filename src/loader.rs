use filesystem::{FileSystem, OsFileSystem};

use crate::{HTML_PATH, CSS_PATH, FILE_SYSTEM},
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct Loader<'a, T> {
    fs: &'a T,
}

impl<'a> Loader<'a, OsFileSystem> {
    pub fn new() -> Self {
        Self {
            fs: &FILE_SYSTEM,
        }
    }
}

impl<'a, FS: FileSystem> Loader<'a, FS> {
    pub fn fs<'b: 'a, T: FileSystem>(&self, fs: &'b T) -> Loader<'a, T> {
        Loader { fs }
    }

    pub fn load(&self) -> Result<Template> {
        let html = self.fs.read_file(HTML_PATH).unwrap();
        let stylesheet = self.fs.read_file(CSS_PATH).unwrap();

        Ok(Template::new())
    }
}
