use std::io::Read;

use wkhtmltopdf::PdfApplication;

use crate::error::Result;
use crate::resume::Resume;

#[derive(Debug, Clone, Default)]
pub struct PdfCompiler;

impl PdfCompiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(&self, resume: &Resume) -> Result<Vec<u8>> {
        let mut app = PdfApplication::new()?;
        let mut builder = app.builder();
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
