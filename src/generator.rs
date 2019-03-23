use filesystem::{FileSystem, OsFileSystem};
use pug;

use crate::error::Result;
use crate::{CSS_PATH, FILE_SYSTEM, HTML_PATH};

#[derive(Debug, Clone)]
pub struct Generator<'a, T = OsFileSystem> {
    fs: &'a T,
}

impl<'a> Generator<'a, OsFileSystem> {
    pub fn new() -> Self {
        Self { fs: &FILE_SYSTEM }
    }
}

impl<'a, FS: FileSystem> Generator<'a, FS> {
    pub fn fs<'b: 'a, T: FileSystem>(&self, fs: &'b T) -> Generator<'a, T> {
        Generator { fs }
    }

    pub fn generate(&self) -> Result<()> {
        let pug = self.fs.read_file_to_string(HTML_PATH)?;
        let _css = self.fs.read_file_to_string(CSS_PATH)?;

        let html = pug::parse(pug).unwrap();
        println!("html: {}", html);

        Ok(())
    }
}

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        Self::new()
    }
}
