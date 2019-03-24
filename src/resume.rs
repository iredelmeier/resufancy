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

#[derive(Debug, Clone)]
pub struct RawResume<'a> {
    html: &'a [u8],
    stylesheet: &'a [u8],
}

impl<'a> RawResume<'a> {
    pub fn new(html: &'a [u8], stylesheet: &'a [u8]) -> Self {
        Self { html, stylesheet }
    }

    pub fn html(&self) -> &[u8] {
        self.html
    }

    pub fn stylesheet(&self) -> &[u8] {
        self.stylesheet
    }
}
