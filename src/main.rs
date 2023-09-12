use std::collections::BTreeMap;

use rust_decimal_macros::dec;
use surrealdb::sql::*;

#[tokio::main]
async fn main() {
    let surreal = surrealdb::engine::any::connect(format!("ws://localhost:22773"))
    // let surreal = surrealdb::engine::any::connect(format!("mem://"))
    // let surreal = surrealdb::engine::any::connect(format!("file:///tmp/data.db"))
        .await
        .unwrap();

    surreal.use_ns("test").use_db("test").await.unwrap();

    let fields = BTreeMap::from([("f".to_owned(), Value::from(dec![0.1]))]); // this worked in beta 9
    
    // let fields = BTreeMap::from([("f".to_owned(), dec![0.1])]); // this works but ends up with `f` as a string

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
