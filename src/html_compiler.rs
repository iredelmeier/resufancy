use pug;
use sass_rs::{self, Options};

use crate::error::sass_error::SassError;
use crate::error::Result;
use crate::resume::Resume;
use crate::template::Template;

#[derive(Debug, Clone)]
pub struct HtmlCompiler;

impl HtmlCompiler {
    pub fn new() -> Self {
        Self
    }
}

impl HtmlCompiler {
    pub fn compile(&self, template: &Template) -> Result<Resume> {
        let html = String::from_utf8(template.html().to_vec())?;
        let scss = String::from_utf8(template.stylesheet().to_vec())?;

        let html = pug::parse(html)?;
        let css = sass_rs::compile_string(&scss, Options::default()).map_err(SassError::from)?;

        Ok(Resume::new(html, css))
    }
}

impl Default for HtmlCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ErrorKind;
    use crate::templates::BASIC;

    use super::*;

    const INVALID_UTF8: &[u8] = &[0, 159, 146, 150];

    #[test]
    fn compile_creates_resume_from_valid_template() {
        let template = &BASIC;
        let compiler = HtmlCompiler::default();

        assert!(compiler.compile(template).is_ok());
    }

    #[test]
    fn compile_fails_if_template_has_non_utf8_html() {
        let invalid_html = INVALID_UTF8;
        let valid_stylesheet = &BASIC.stylesheet();
        let template = Template::new("test", invalid_html.to_vec(), valid_stylesheet.to_vec());
        let compiler = HtmlCompiler::default();

        let result = compiler.compile(&template);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), &ErrorKind::InvalidUtf8);
    }

    #[test]
    fn compile_fails_if_template_has_non_utf8_stylesheet() {
        let valid_html = &BASIC.html();
        let invalid_stylesheet = INVALID_UTF8;
        let template = Template::new("test", valid_html.to_vec(), invalid_stylesheet.to_vec());
        let compiler = HtmlCompiler::default();

        let result = compiler.compile(&template);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), &ErrorKind::InvalidUtf8);
    }

    #[test]
    fn compile_fails_if_template_html_is_not_valid_pug() {
        let invalid_html = b"<html></html>";
        let valid_stylesheet = &BASIC.stylesheet();
        let template = Template::new("test", invalid_html.to_vec(), valid_stylesheet.to_vec());
        let compiler = HtmlCompiler::default();

        let result = compiler.compile(&template);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), &ErrorKind::Pug);
    }

    #[test]
    fn compile_fails_if_template_stylesheet_is_not_valid_sass() {
        let valid_html = &BASIC.html();
        let invalid_stylesheet = b"invalid";
        let template = Template::new("test", valid_html.to_vec(), invalid_stylesheet.to_vec());
        let compiler = HtmlCompiler::default();

        let result = compiler.compile(&template);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), &ErrorKind::Sass);
    }
}
