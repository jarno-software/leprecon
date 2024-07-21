use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "catalog.html")]
pub struct Catalogs {
    pub catalogs: Vec<Catalog>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Catalog {
    pub name: String,
    pub description: String,
}
