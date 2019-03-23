use std::collections::HashMap;

use crate::template::Template;

lazy_static! {
    pub static ref BASIC: Template = {
        let html = include_bytes!("../templates/basic/resume.pug").to_vec();
        let stylesheet = include_bytes!("../templates/basic/style.scss").to_vec();

        Template::new(html, stylesheet)
    };
}

lazy_static! {
    pub static ref TEMPLATES: HashMap<&'static str, &'static Template> = {
        let mut templates = HashMap::new();

        templates.insert("basic", &*BASIC);

        templates
    };
}

lazy_static! {
    pub static ref NAMES: Vec<&'static str> = TEMPLATES.keys().cloned().collect();
}
