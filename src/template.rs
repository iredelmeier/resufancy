#[derive(Debug, Clone)]
pub struct Template<'a> {
    html: &'a [u8],
    stylesheet: &'a [u8],
}

impl<'a> Template<'a> {
    pub fn new(html: &'a [u8], stylesheet: &'a [u8]) -> Self {
        Self { html, stylesheet }
    }

    pub fn html(&self) -> &'a [u8] {
        self.html
    }

    pub fn stylesheet(&self) -> &'a [u8] {
        self.stylesheet
    }
}
