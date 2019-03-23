use std::collections::HashMap;

use crate::template::{Template, TemplateBuilder};

lazy_static! {
    pub static ref BASIC: Template<'static> = {
        let mut builder = TemplateBuilder::new("basic");

        builder.html(include_bytes!("../templates/basic/resume.pug"));
        builder.stylesheet(include_bytes!("../templates/basic/style.scss"));

        builder.build()
    };
}

lazy_static! {
    pub static ref TEMPLATES: HashMap<&'static str, &'static Template<'static>> = {
        let mut templates = HashMap::new();

        templates.insert(BASIC.name(), &*BASIC);

        templates
    };
}

lazy_static! {
    pub static ref NAMES: Vec<&'static str> = TEMPLATES.keys().cloned().collect();
}
