#[derive(Debug, Clone)]
pub struct Template<'a> {
    name: &'a str,
    html: Vec<u8>,
    stylesheet: Vec<u8>,
}

impl<'a> Template<'a> {
    pub fn new(name: &'a str, html: Vec<u8>, stylesheet: Vec<u8>) -> Self {
        Self {
            name,
            html,
            stylesheet,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn html(&self) -> &[u8] {
        &self.html
    }

    pub fn stylesheet(&self) -> &[u8] {
        &self.stylesheet
    }
}
