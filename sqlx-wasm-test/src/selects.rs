// use sqlx::Row;
// use sqlx::{postgres::PgRow, Connection};
// use sqlx::{Database, PgConnection, Postgres};
// use wasm_bindgen_futures::futures_0_3::spawn_local as spawn;
// use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
// use wasm_bindgen::prelude::*;
use sqlx_tests::new;
use web_sys::console;
use instant::Instant;


#[wasm_bindgen_test]
async fn select_query_small() {
    let mut conn = new().await;

    let start = performance.now();

    for _ in 0..3u8 {
        sqlx::query("select * from airports")
            .fetch_all(&mut conn)
            .await;
    }

    let end = performance.now();
    web_sys::console::log_1(&format!("Avg time is {}", (end - start) / 3f64).into());
    // assert!(airports.len() == 396);
}

async fn select_query_medium() {
    let mut conn = new().await;

    let airports = sqlx::query("select * from medium")
        .fetch_all(&mut conn)
        .await.unwrap();
    assert!(airports.len() == 396);
}
