#[derive(Debug, Clone)]
pub struct Template {
    html: Vec<u8>,
    stylesheet: Vec<u8>,
}

impl Template {
    pub fn new(html: Vec<u8>, stylesheet: Vec<u8>) -> Self {
        Self { html, stylesheet }
    }

    pub fn html(&self) -> &[u8] {
        &self.html
    }

    pub fn stylesheet(&self) -> &[u8] {
        &self.stylesheet
    }
}
