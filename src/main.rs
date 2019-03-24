extern crate clap;
extern crate filesystem;
#[macro_use]
extern crate lazy_static;

use clap::{App, AppSettings};
use filesystem::OsFileSystem;

use crate::commands::Commands;

mod commands;

const PUG_PATH: &str = "resume.pug";
const SCSS_PATH: &str = "style.scss";
const HTML_PATH: &str = "resume.html";
const CSS_PATH: &str = "style.css";
const PDF_PATH: &str = "resume.pdf";

lazy_static! {
    static ref FILE_SYSTEM: OsFileSystem = OsFileSystem::new();
}

fn main() {
    let subcommands = Commands::new();
    let global_settings = [AppSettings::ColoredHelp];
    let settings = [AppSettings::SubcommandRequiredElseHelp];
    let app = App::new(env!("CARGO_PKG_NAME"))
        .subcommands(subcommands.build())
        .global_settings(&global_settings)
        .settings(&settings);
    let matches = app.get_matches();

    if let Some(result) = subcommands.run(&matches) {
        match result {
            Ok(Some(msg)) => println!("{}", msg),
            Err(err) => eprintln!("{}", err),
            _ => {}
        }
    }
}
