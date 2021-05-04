use futures::TryStreamExt;
use sqlx::postgres::PgRow;
use sqlx::Executor;
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

#[wasm_bindgen_test]
async fn it_executes() {
    let mut conn = new().await;

    let _ = conn
        .execute(
            r#"
CREATE TEMPORARY TABLE users (id INTEGER PRIMARY KEY);
            "#,
        )
        .await;

    for index in 1..=10_i32 {
        let done = sqlx::query("INSERT INTO users (id) VALUES ($1)")
            .bind(index)
            .execute(&mut conn)
            .await
            .unwrap();

        assert_eq!(done.rows_affected(), 1);
    }

    let sum: i32 = sqlx::query("SELECT id FROM users")
        .try_map(|row: PgRow| row.try_get::<i32, _>(0))
        .fetch(&mut conn)
        .try_fold(0_i32, |acc, x| async move { Ok(acc + x) })
        .await
        .unwrap();

    assert_eq!(sum, 55);
}
