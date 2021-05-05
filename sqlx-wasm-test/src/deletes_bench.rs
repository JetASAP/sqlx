use sqlx::Executor;
use sqlx_wasm_test::time_delete_query;
use wasm_bindgen_test::*;

async fn deletes_query_small() {
    time_delete_query!("small", 100u32);
}

#[wasm_bindgen_test]
async fn deletes_query_medium() {
    time_delete_query!("medium", 1000u32);
}

#[wasm_bindgen_test]
async fn deletes_query_large() {
    time_delete_query!("large", 10000u32);
}