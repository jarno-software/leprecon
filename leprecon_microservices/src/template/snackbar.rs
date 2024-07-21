use askama::Template;

#[derive(Template)]
#[template(path = "snackbar.html")]
pub struct Snackbar<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub color: &'a str,
}

impl<'a> Default for Snackbar<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Snackbar<'a> {
    fn new() -> Snackbar<'a> {
        Snackbar {
            title: "Error",
            message: "Could not process request",
            color: "red",
        }
    }
}
