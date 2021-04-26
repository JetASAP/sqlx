use sqlx::Executor;
use sqlx_wasm_test::time_update_query;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn updates_query_small() {
    time_update_query!("small", 100u32);
}
