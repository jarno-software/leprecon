mod db;

use askama::Template;
use axum::{extract::State, response::Html};
use leprecon::template::{self, Catalog, Snackbar};
use reqwest::StatusCode;
use tracing::debug;

use self::db::get_catalog_db;

pub(super) async fn get_catalog(
    State(state): State<mongodb::Database>,
) -> (StatusCode, Html<String>) {
    let snackbar: Snackbar<'_> = Snackbar::default();

    let catalogs: Vec<Catalog> = match get_catalog_db(state).await {
        Ok(v) => v,
        Err(e) => {
            debug!("Could not get catalog: {:?}", e);
            return (StatusCode::BAD_GATEWAY, Html(snackbar.render().unwrap()));
        }
    };

    let catalog_template: template::Catalogs = template::Catalogs { catalogs };
    (StatusCode::OK, Html(catalog_template.render().unwrap()))
}
