use sqlx::postgres::PgRow;
use sqlx::Row;
use sqlx_wasm_test::new;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn select_query_count() {
    let mut conn = new().await;

    let count = sqlx::query(
        "SELECT count(*) from generate_series(1,100) AS id, md5(random()::text) AS descr",
    )
    .try_map(|row: PgRow| row.try_get::<i64, _>(0))
    .fetch_one(&mut conn)
    .await
    .unwrap();

    assert_eq!(count, 100i64);
}
