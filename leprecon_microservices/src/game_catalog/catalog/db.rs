use futures::TryStreamExt;
use leprecon::template::Catalog;
use mongodb::Cursor;

pub(super) async fn get_catalog_db(
    conn: mongodb::Database,
) -> Result<Vec<Catalog>, mongodb::error::Error> {
    let collection: mongodb::Collection<Catalog> = conn.collection::<Catalog>("catalog");
    let mut cursor: Cursor<Catalog> = collection.find(None, None).await?;
    let mut catalogs: Vec<Catalog> = vec![];

    while let Some(c) = cursor.try_next().await? {
        catalogs.push(c);
    }

    Ok(catalogs)
}
