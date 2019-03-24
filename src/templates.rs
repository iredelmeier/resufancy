use std::collections::HashMap;

use crate::template::Template;

lazy_static! {
    pub static ref BASIC: Template<'static> = {
        let html = include_bytes!("../templates/basic/resume.pug").to_vec();
        let stylesheet = include_bytes!("../templates/basic/style.scss").to_vec();

        Template::new("basic", html, stylesheet)
    };
}

lazy_static! {
    pub static ref TEMPLATES: HashMap<&'static str, &'static Template<'static>> = [&*BASIC]
        .iter()
        .map(|template| (template.name(), *template))
        .collect();
}

lazy_static! {
    pub static ref NAMES: Vec<&'static str> = TEMPLATES.keys().cloned().collect();
}
