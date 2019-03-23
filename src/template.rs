#[derive(Debug, Clone)]
pub struct Template<'a> {
    name: &'a str,
    html: &'a [u8],
    stylesheet: &'a [u8],
}

impl<'a> Template<'a> {
    pub fn new(name: &'a str, html: &'a [u8], stylesheet: &'a [u8]) -> Self {
        Self {
            name,
            html,
            stylesheet,
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn html(&self) -> &'a [u8] {
        self.html
    }

    pub fn stylesheet(&self) -> &'a [u8] {
        self.stylesheet
    }
}
