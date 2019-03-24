use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

use filesystem::{FileSystem, OsFileSystem, TempDir, TempFileSystem};
use wkhtmltopdf::PdfApplication;

use crate::error::Result;
use crate::resume::Resume;
use crate::FILE_SYSTEM;

pub struct PdfCompiler<'a, T = OsFileSystem> {
    fs: &'a T,
    compiler: Rc<RefCell<Option<PdfApplication>>>,
}

impl<'a> PdfCompiler<'a, OsFileSystem> {
    pub fn new() -> Self {
        Self {
            fs: &FILE_SYSTEM,
            compiler: Rc::new(RefCell::new(None)),
        }
    }
}

impl<'a, FS: FileSystem> PdfCompiler<'a, FS> {
    pub fn fs<'b: 'a, T: FileSystem>(self, fs: &'b T) -> PdfCompiler<'a, T> {
        PdfCompiler {
            fs,
            compiler: self.compiler,
        }
    }
}

impl<'a, FS: FileSystem + TempFileSystem> PdfCompiler<'a, FS> {
    pub fn compile(&self, resume: &Resume) -> Result<Vec<u8>> {
        let mut compiler = self.compiler.borrow_mut();
        let mut builder = match *compiler {
            Some(ref mut compiler) => compiler.builder(),
            None => {
                let mut app = PdfApplication::new()?;
                let b = app.builder();

                *compiler = Some(app);

                b
            }
        };
        let temp_dir = self.fs.temp_dir("pdf")?;
        let html_path = temp_dir.path().join("resume.html");
        let css_path = temp_dir.path().join("style.css");

        self.fs.create_file(&html_path, &resume.html())?;
        self.fs.create_file(&css_path, &resume.stylesheet())?;

        let mut pdf = builder.build_from_path(&html_path)?;
        let mut bytes: Vec<u8> = vec![];

        pdf.read_to_end(&mut bytes)?;

        Ok(bytes)
    }
}

impl<'a> Default for PdfCompiler<'a, OsFileSystem> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::resume::RawResume;
    use crate::templates::BASIC;
    use crate::HtmlCompiler;

    use super::*;

    #[test]
    fn compile_creates_pdf_from_valid_resume() {
        let raw_resume = RawResume::new(BASIC.html(), BASIC.stylesheet());
        let resume = HtmlCompiler::default().compile(&raw_resume).unwrap();
        let compiler = PdfCompiler::default();

        let pdf = compiler.compile(&resume);
        assert!(pdf.is_ok());
    }
}
