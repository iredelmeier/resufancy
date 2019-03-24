use std::sync::mpsc;
use std::time::Duration;

use clap::{App, ArgMatches};
use filesystem::FileSystem;
use notify::{self, RecursiveMode, Watcher};
use resufancy::error::Result;
use resufancy::resume::RawResume;
use resufancy::{HtmlCompiler, PdfCompiler};

use super::args::{Arg, SKIP_HTML, SKIP_PDF, WATCH};
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
        App::new(NAME)
            .args(&[SKIP_HTML.build(), SKIP_PDF.build(), WATCH.build()])
            .about("Compile the resume into HTML and/or a PDF")
    }

    fn run(&self, matches: &ArgMatches) -> Result<Option<String>> {
        let skip_html = SKIP_HTML.value_from(matches);
        let skip_pdf = SKIP_PDF.value_from(matches);
        let pdf_compiler = PdfCompiler::default();

        if WATCH.value_from(matches) {
            let (tx, rx) = mpsc::channel();
            let mut watcher =
                notify::watcher(tx, Duration::from_millis(250)).expect("Failed to create watcher");

            watcher
                .watch(PUG_PATH, RecursiveMode::NonRecursive)
                .expect("Failed to watch file");
            watcher
                .watch(SCSS_PATH, RecursiveMode::NonRecursive)
                .expect("Failed to watch file");

            let recompile = || {
                match compile(skip_html, skip_pdf, &pdf_compiler) {
                    Ok(_) => println!("Successfully rebuilt"),
                    Err(err) => eprintln!("Failed to build: {}", err),
                };
            };

            recompile();

            loop {
                if rx.recv().is_ok() {
                    recompile();
                }
            }
        } else {
            compile(skip_html, skip_pdf, &pdf_compiler)
        }
    }
}

fn compile(skip_html: bool, skip_pdf: bool, pdf_compiler: &PdfCompiler) -> Result<Option<String>> {
    let pug = FILE_SYSTEM.read_file(PUG_PATH)?;
    let scss = FILE_SYSTEM.read_file(SCSS_PATH)?;
    let raw_resume = RawResume::new(&pug, &scss);
    let html_compiler = HtmlCompiler::default();
    let resume = html_compiler.compile(&raw_resume)?;

    if !skip_html {
        FILE_SYSTEM.write_file(HTML_PATH, resume.html())?;
        FILE_SYSTEM.write_file(CSS_PATH, resume.stylesheet())?;
    }

    if !skip_pdf {
        let pdf = pdf_compiler.compile(&resume)?;

        FILE_SYSTEM.write_file(PDF_PATH, pdf)?;
    }

    Ok(None)
}
