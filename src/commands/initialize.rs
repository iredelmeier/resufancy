use clap::{App, ArgMatches};
use filesystem::FileSystem;
use resufancy::error::Result;

use super::args::{Arg, TEMPLATE};
use super::Command;
use crate::{FILE_SYSTEM, PUG_PATH, SCSS_PATH};

const NAME: &str = "init";

#[derive(Debug, Clone, Default)]
pub struct Initialize;

impl Initialize {
    pub fn new() -> Self {
        Self
    }
}

impl Command for Initialize {
    fn name(&self) -> &str {
        NAME
    }

    fn build(&self) -> App {
        App::new(NAME)
            .args(&[TEMPLATE.build()])
            .about("Scaffold a new resume")
    }

    fn run(&self, matches: &ArgMatches) -> Result<Option<String>> {
        let template = TEMPLATE.value_from(matches);

        FILE_SYSTEM.create_file(PUG_PATH, template.html())?;
        FILE_SYSTEM.create_file(SCSS_PATH, template.stylesheet())?;

        Ok(None)
    }
}
