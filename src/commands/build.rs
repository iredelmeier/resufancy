use clap::{App, ArgMatches};
use filesystem::FileSystem;
use resufancy::error::Result;
use resufancy::resume::RawResume;
use resufancy::{HtmlCompiler, PdfCompiler};

use super::args::{Arg, SKIP_HTML, SKIP_PDF};
use super::Command;
use crate::{CSS_PATH, FILE_SYSTEM, HTML_PATH, PDF_PATH, PUG_PATH, SCSS_PATH};

const NAME: &str = "build";

#[derive(Debug, Clone, Default)]
pub struct Build;

impl Build {
    pub fn new() -> Self {
        Self
    }
}

impl Command for Build {
    fn name(&self) -> &str {
        NAME
    }

    fn build(&self) -> App {
        App::new(NAME).args(&[SKIP_HTML.build(), SKIP_PDF.build()])
    }

    fn run(&self, matches: &ArgMatches) -> Result<Option<String>> {
        let pug = FILE_SYSTEM.read_file(PUG_PATH)?;
        let scss = FILE_SYSTEM.read_file(SCSS_PATH)?;
        let raw_resume = RawResume::new(&pug, &scss);
        let html_compiler = HtmlCompiler::default();
        let resume = html_compiler.compile(&raw_resume)?;

        if !SKIP_HTML.value_from(matches) {
            FILE_SYSTEM.write_file(HTML_PATH, resume.html())?;
            FILE_SYSTEM.write_file(CSS_PATH, resume.stylesheet())?;
        }

        if !SKIP_PDF.value_from(matches) {
            let pdf_compiler = PdfCompiler::default();
            let pdf = pdf_compiler.compile(&resume)?;

            FILE_SYSTEM.write_file(PDF_PATH, pdf)?;
        }

        Ok(None)
    }
}
