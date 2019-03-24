use clap::{App, ArgMatches};

use resufancy::error::Result;

use self::build::Build;
use self::initialize::Initialize;

mod args;
mod build;
mod initialize;

pub struct Commands {
    commands: Vec<Box<dyn Command>>,
}

impl Commands {
    pub fn new() -> Self {
        Commands {
            commands: vec![Box::new(Initialize::new()), Box::new(Build::new())],
        }
    }

    #[allow(clippy::redundant_closure)] // Command::build expects an unboxed command
    pub fn build(&self) -> Vec<App> {
        self.commands.iter().map(|cmd| cmd.build()).collect()
    }

    pub fn run(&self, matches: &ArgMatches) -> Option<Result<Option<String>>> {
        if let (name, Some(matches)) = matches.subcommand() {
            self.commands
                .iter()
                .find(|cmd| cmd.name() == name)
                .map(|cmd| cmd.run(matches))
        } else {
            None
        }
    }
}

trait Command {
    fn name(&self) -> &str;
    fn build(&self) -> App;
    fn run(&self, matches: &ArgMatches) -> Result<Option<String>>;
}
