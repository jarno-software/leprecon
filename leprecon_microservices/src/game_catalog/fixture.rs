use leprecon::template::Catalog;

pub async fn seed_db(client: &mongodb::Database) {
    let coll: mongodb::Collection<Catalog> = client.collection::<Catalog>("catalog");
    let catalog: Catalog = Catalog {
        name: "Blackjack".to_owned(),
        description: "This is blackjack!".to_owned(),
    };

    coll.insert_one(catalog, None).await.unwrap();

    let catalog: Catalog = Catalog {
        name: "Poker".to_owned(),
        description: "This is poker!".to_owned(),
    };

    coll.insert_one(catalog, None).await.unwrap();

    let catalog: Catalog = Catalog {
        name: "Slots".to_owned(),
        description: "This is slots!".to_owned(),
    };

    coll.insert_one(catalog, None).await.unwrap();
}
