use std::{collections::BTreeMap, str::FromStr};

use bigdecimal::BigDecimal;
use surrealdb::{sql::*, opt::auth::Root};

#[tokio::main]
async fn main() {
    let surreal = surrealdb::engine::any::connect(format!("ws://localhost:22774"))
        .await
        .unwrap();


    surreal.signin(Root {
        username: "root",
        password: "root",
    }).await.unwrap();

    surreal.use_ns("test").use_db("test").await.unwrap();


    let fields = BTreeMap::from([(
        "f".to_owned(),
        Value::from(BigDecimal::from_str("0.1").unwrap()),
    )]);

    surreal
        .query("UPDATE type::thing($table, $id) set f = $fields.f")
        .bind(("table", "a"))
        .bind(("id", 1))
        .bind(("fields", &fields))
        .await
        .unwrap()
        .check()
        .unwrap();

    let res = surreal
        .query("select * from a:1")
        .await
        .unwrap()
        .check()
        .unwrap();

    println!("{res:?}",);

}
