#![feature(test)]

extern crate test;

use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use sqlx::{Database, PgConnection, Postgres};
use sqlx::Connection;

const URL: &str = "postgresql://paul:pass123@127.0.0.1:8080/jetasap_dev";

pub async fn new() -> PgConnection {
    <Postgres as Database>::Connection::connect(URL) .await.unwrap()
}
