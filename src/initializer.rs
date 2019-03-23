use filesystem::{FileSystem, OsFileSystem};

use crate::error::Result;
use crate::template::Template;
use crate::templates::BASIC;
use crate::{CSS_PATH, FILE_SYSTEM, HTML_PATH};

#[derive(Debug, Clone)]
pub struct Initializer<'a, T = OsFileSystem> {
    fs: &'a T,
    template: &'a Template<'a>,
}

impl<'a> Initializer<'a, OsFileSystem> {
    pub fn new() -> Self {
        Self {
            fs: &FILE_SYSTEM,
            template: &BASIC,
        }
    }
}

impl<'a, FS: FileSystem> Initializer<'a, FS> {
    pub fn fs<'b: 'a, T: FileSystem>(&self, fs: &'b T) -> Initializer<'a, T> {
        Initializer {
            fs,
            template: self.template,
        }
    }

    pub fn template<'b: 'a>(&mut self, template: &'b Template) -> &mut Self {
        self.template = template;
        self
    }

    pub fn initialize(&self) -> Result<()> {
        self.fs.create_file(HTML_PATH, self.template.html())?;
        self.fs.create_file(CSS_PATH, self.template.stylesheet())?;

        Ok(())
    }
}

impl<'a> Default for Initializer<'a, OsFileSystem> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use filesystem::FakeFileSystem;

    use crate::error::ErrorKind;

    use super::*;

    #[test]
    fn initialize_uses_default_html_and_stylesheet_templates() {
        let fs = FakeFileSystem::new();
        let initializer = Initializer::new().fs(&fs);

        assert!(initializer.initialize().is_ok());

        let html = fs.read_file("resume.pug");
        assert!(html.is_ok());
        assert_eq!(html.unwrap(), BASIC.html());

        let css = fs.read_file("style.scss");
        assert!(css.is_ok());
        assert_eq!(css.unwrap(), BASIC.stylesheet());
    }

    #[test]
    fn initialize_uses_specified_template() {
        let fs = FakeFileSystem::new();
        let template = Template::new("test", "foo".as_bytes(), "bar".as_bytes());
        let mut initializer = Initializer::new().fs(&fs);

        initializer.template(&template);

        assert!(initializer.initialize().is_ok());

        let html = fs.read_file("resume.pug");
        assert!(html.is_ok());
        assert_eq!(html.unwrap(), "foo".as_bytes());

        let css = fs.read_file("style.scss");
        assert!(css.is_ok());
        assert_eq!(css.unwrap(), "bar".as_bytes());
    }

    #[test]
    fn initialize_fails_if_html_file_exists() {
        let fs = FakeFileSystem::new();
        let initializer = Initializer::new().fs(&fs);

        assert!(fs.create_file("resume.pug", "").is_ok());

        let result = initializer.initialize();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            &ErrorKind::Io(io::ErrorKind::AlreadyExists)
        );
    }

    #[test]
    fn initialize_fails_if_stylesheet_file_exists() {
        let fs = FakeFileSystem::new();

        assert!(fs.create_file("style.scss", "").is_ok());

        let initializer = Initializer::new().fs(&fs);

        let result = initializer.initialize();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().kind(),
            &ErrorKind::Io(io::ErrorKind::AlreadyExists)
        );
    }
}
