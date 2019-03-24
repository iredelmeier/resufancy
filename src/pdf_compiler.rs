use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

use wkhtmltopdf::PdfApplication;

use crate::error::Result;
use crate::resume::Resume;

#[derive(Default)]
pub struct PdfCompiler {
    compiler: Rc<RefCell<Option<PdfApplication>>>,
}

impl PdfCompiler {
    pub fn new() -> Self {
        Self {
            compiler: Rc::new(RefCell::new(None)),
        }
    }

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
        let mut pdf = builder.build_from_html(&resume.html())?;
        let mut bytes: Vec<u8> = vec![];

        pdf.read_to_end(&mut bytes)?;

        Ok(bytes)
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
