#![feature(test)]

extern crate test;

use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use sqlx::Connection;
use sqlx::{Database, PgConnection, Postgres};

const URL: &str = "postgresql://paul:pass123@127.0.0.1:8080/jetasap_dev";

pub async fn new() -> PgConnection {
    <Postgres as Database>::Connection::connect(URL)
        .await
        .unwrap()
}

#[macro_export]
macro_rules! time_query {
    ($x:expr) => {
        let mut conn = new().await;

        let performance = web_sys::window().unwrap().performance().unwrap();
        let start = performance.now();

        for _ in 0..3u8 {
            sqlx::query($x).fetch_all(&mut conn).await;
        }

        let end = performance.now();
        web_sys::console::log_1(&format!("Avg time is {}", (end - start) / 3f64).into());
    };
}
