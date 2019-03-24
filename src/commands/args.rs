use clap::{self, ArgMatches};

use resufancy::template;
use resufancy::templates::{BASIC, NAMES, TEMPLATES};

lazy_static! {
    pub static ref TEMPLATE: Template<'static> = { Template::new() };
}

lazy_static! {
    pub static ref SKIP_HTML: SkipHtml = { SkipHtml::default() };
}

lazy_static! {
    pub static ref SKIP_PDF: SkipPdf = { SkipPdf::default() };
}

lazy_static! {
    pub static ref WATCH: Watch = { Watch::default() };
}

pub trait Arg<'a> {
    type Value;

    fn name(&self) -> &'static str;
    fn build<'b: 'a>(&self) -> clap::Arg<'a, 'b>;
    fn value_from<'b>(&self, matches: &'b ArgMatches) -> Self::Value;
}

#[derive(Debug, Clone)]
pub struct Template<'a> {
    default: &'a template::Template<'a>,
}

impl<'a> Template<'a> {
    pub fn new() -> Self {
        Self { default: &BASIC }
    }
}

impl<'a> Arg<'a> for Template<'a> {
    type Value = &'a template::Template<'a>;

    fn name(&self) -> &'static str {
        "template"
    }

    fn build<'b: 'a>(&self) -> clap::Arg<'a, 'b> {
        clap::Arg::with_name(self.name())
            .long(self.name())
            .takes_value(true)
            .required(true)
            .possible_values(&NAMES)
            .default_value(self.default.name())
    }

    fn value_from<'b>(&self, matches: &'b ArgMatches) -> Self::Value {
        let name = matches
            .value_of(self.name())
            .unwrap_or_else(|| self.default.name());
        TEMPLATES.get(name).unwrap_or(&self.default)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SkipHtml;

impl<'a> Arg<'a> for SkipHtml {
    type Value = bool;

    fn name(&self) -> &'static str {
        "skip-html"
    }

    fn build<'b: 'a>(&self) -> clap::Arg<'a, 'b> {
        clap::Arg::with_name(self.name()).long(self.name())
    }

    fn value_from<'b>(&self, matches: &'b ArgMatches) -> Self::Value {
        matches.is_present(self.name())
    }
}

#[derive(Debug, Clone, Default)]
pub struct SkipPdf;

impl<'a> Arg<'a> for SkipPdf {
    type Value = bool;

    fn name(&self) -> &'static str {
        "skip-pdf"
    }

    fn build<'b: 'a>(&self) -> clap::Arg<'a, 'b> {
        clap::Arg::with_name(self.name()).long(self.name())
    }

    fn value_from<'b>(&self, matches: &'b ArgMatches) -> Self::Value {
        matches.is_present(self.name())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Watch;

impl<'a> Arg<'a> for Watch {
    type Value = bool;

    fn name(&self) -> &'static str {
        "watch"
    }

    fn build<'b: 'a>(&self) -> clap::Arg<'a, 'b> {
        clap::Arg::with_name(self.name()).long(self.name())
    }

    fn value_from<'b>(&self, matches: &'b ArgMatches) -> Self::Value {
        matches.is_present(self.name())
    }
}
