extern crate filesystem;
#[macro_use]
extern crate lazy_static;
extern crate pug;
extern crate sass_rs;
extern crate wkhtmltopdf;

pub mod error;
pub mod resume;
pub mod template;
pub mod templates;

mod html_compiler;
mod initializer;
mod pdf_compiler;

use filesystem::OsFileSystem;

pub use crate::html_compiler::HtmlCompiler;
pub use crate::initializer::Initializer;
pub use crate::pdf_compiler::PdfCompiler;

const HTML_PATH: &str = "resume.pug";
const CSS_PATH: &str = "style.scss";

lazy_static! {
    static ref FILE_SYSTEM: OsFileSystem = OsFileSystem::new();
}
