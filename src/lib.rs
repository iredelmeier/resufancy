extern crate filesystem;
#[macro_use]
extern crate lazy_static;

pub mod error;
pub mod template;
pub mod templates;

mod initializer;

use filesystem::OsFileSystem;

pub use crate::initializer::Initializer;

const HTML_PATH: &str = "resume.pug";
const CSS_PATH: &str = "style.scss";

lazy_static! {
    static ref FILE_SYSTEM: OsFileSystem = OsFileSystem::new();
}
