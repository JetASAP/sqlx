// use sqlx::Row;
// use sqlx::{postgres::PgRow, Connection};
// use sqlx::{Database, PgConnection, Postgres};
// use wasm_bindgen_futures::futures_0_3::spawn_local as spawn;
// use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
// use wasm_bindgen::prelude::*;
use sqlx_wasm_test::{new, time_query};
use web_sys::console;

#[wasm_bindgen_test]
async fn select_query_small() {
    time_query!("select * from airports");
}

async fn select_query_medium() {
    let mut conn = new().await;

    let airports = sqlx::query("select * from medium")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    assert!(airports.len() == 396);
}
