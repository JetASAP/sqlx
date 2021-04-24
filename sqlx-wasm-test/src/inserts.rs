#![feature(test)]

extern crate test;

use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod tests {
use sqlx::Row;
use sqlx::{postgres::PgRow, Connection};
use sqlx::{Database, PgConnection, Postgres};
use wasm_bindgen_futures::futures_0_3::spawn_local as spawn;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
}

