use std::collections::HashMap;

use crate::template::Template;

lazy_static! {
    pub static ref BASIC: Template<'static> = {
        let html = include_bytes!("../templates/basic/resume.pug");
        let stylesheet = include_bytes!("../templates/basic/style.scss");

        Template::new("basic", html, stylesheet)
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
