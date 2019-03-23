#[derive(Debug, Clone)]
pub struct TemplateBuilder<'a> {
    name: &'a str,
    html: &'a [u8],
    stylesheet: &'a [u8],
}

impl<'a> TemplateBuilder<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            html: &[],
            stylesheet: &[],
        }
    }

    pub fn html<'b: 'a>(&mut self, html: &'b [u8]) -> &mut Self {
        self.html = html;
        self
    }

    pub fn stylesheet<'b: 'a>(&mut self, stylesheet: &'b [u8]) -> &mut Self {
        self.stylesheet = stylesheet;
        self
    }

    pub fn build(&self) -> Template<'a> {
        Template {
            name: self.name,
            html: self.html,
            stylesheet: self.stylesheet,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Template<'a> {
    name: &'a str,
    html: &'a [u8],
    stylesheet: &'a [u8],
}

impl<'a> Template<'a> {
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
