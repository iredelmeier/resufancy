#[derive(Debug, Clone)]
pub struct Resume {
    html: String,
    stylesheet: String,
}

impl Resume {
    pub fn new(html: String, stylesheet: String) -> Self {
        Self { html, stylesheet }
    }

    pub fn html(&self) -> &str {
        &self.html
    }

    pub fn stylesheet(&self) -> &str {
        &self.stylesheet
    }
}
