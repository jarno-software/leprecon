use barrel::{backend::Pg, types, Migration};

pub(crate) fn migration() -> String {
    let mut m: Migration = Migration::new();

    m.create_table_if_not_exists("currencies", |t| {
        t.add_column("id", types::primary());
        t.add_column("acronym", types::text().unique(true));
    });

    m.create_table_if_not_exists("users", |t| {
        t.add_column("id", types::primary());
        t.add_column("sub", types::text().unique(true));
        t.add_column("balance", types::double());
        t.add_column("currency_id", types::integer());

        t.add_foreign_key(&["currency_id"], "currencies", &["id"]);
    });

    m.create_table_if_not_exists("customer_details", |t| {
        t.add_column("id", types::primary());
        t.add_column("first_name", types::text().nullable(true));
        t.add_column("middle_name", types::text().nullable(true));
        t.add_column("last_name", types::text().nullable(true));
        t.add_column("postal_code", types::text().nullable(true));
        t.add_column("street_name", types::text().nullable(true));
        t.add_column("street_nr", types::text().nullable(true));
        t.add_column("premise", types::text().nullable(true));
        t.add_column("settlement", types::text().nullable(true));
        t.add_column("country", types::text().nullable(true));
        t.add_column("country_code", types::text().nullable(true));
        t.add_column("user_id", types::integer().unique(true));

        t.add_foreign_key(&["user_id"], "users", &["id"]);
    });

    m.create_table_if_not_exists("sessions", |t| {
        t.add_column("id", types::primary());
        t.add_column("expires", types::custom("timestamp with time zone"));
        t.add_column("type", types::text());
        t.add_column("user_id", types::integer());

        t.add_foreign_key(&["user_id"], "users", &["id"]);
    });

    m.make::<Pg>()
}
