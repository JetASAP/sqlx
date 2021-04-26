#![feature(test)]

extern crate test;

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
    ($n:expr, $q:expr) => {
        let mut conn = sqlx_wasm_test::new().await;

        let performance = web_sys::window().unwrap().performance().unwrap();
        let start = performance.now();

        for _ in 0..3u8 {
            let _ = sqlx::query($q).fetch_all(&mut conn).await;
        }

        let end = performance.now();
        web_sys::console::log_1(&format!("{}: Avg time is {}", $n, (end - start) / 3f64).into());
    };
}

#[macro_export]
macro_rules! time_insert_query {
    ($n:expr, $count:literal) => {
        let mut conn = sqlx_wasm_test::new().await;
        conn.execute("create temp table bench_inserts (id integer, descr text)")
            .await;

        let performance = web_sys::window().unwrap().performance().unwrap();
        let start = performance.now();

        for _ in 0..3u8 {
            for i in 0..$count {
                let _ = sqlx::query(&format!(
                    "insert into bench_inserts (id, desc) values ({}, md5(random()::text))",
                    i
                ))
                .execute(&mut conn)
                .await;
            }
        }

        let end = performance.now();
        web_sys::console::log_1(&format!("{}: Avg time is {}", $n, (end - start) / 3f64).into());
    };
}

#[macro_export]
macro_rules! time_update_query {
    ($n:expr, $count:literal) => {
        let mut conn = sqlx_wasm_test::new().await;
        conn.execute("create temp table bench_updates as SELECT generate_series(1,100) AS id, md5(random()::text) AS descr ")
            .await;

        let performance = web_sys::window().unwrap().performance().unwrap();
        let start = performance.now();

        for _ in 0..3u8 {
            for i in 1..$count {
                let _ = sqlx::query(&format!(
                    "update bench_updates set descr = md5(random()::text) where id = {}",
                    i
                ))
                .execute(&mut conn)
                .await;
            }
        }

        let end = performance.now();
        web_sys::console::log_1(&format!("{}: Avg time is {}", $n, (end - start) / 3f64).into());
    };
}
