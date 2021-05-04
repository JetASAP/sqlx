use futures::TryStreamExt;
use sqlx::postgres::{PgDatabaseError, PgErrorPosition, PgRow, PgSeverity};
use sqlx::{Column, Connection, Executor, Row, Statement, TypeInfo};
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

#[wasm_bindgen_test]
async fn it_can_inspect_errors() {
    let mut conn = new().await;

    let res: Result<_, sqlx::Error> = sqlx::query("select f").execute(&mut conn).await;
    let err = res.unwrap_err();

    // can also do [as_database_error] or use `match ..`
    let err = err.into_database_error().unwrap();

    assert_eq!(err.message(), "column \"f\" does not exist");
    assert_eq!(err.code().as_deref(), Some("42703"));

    // can also do [downcast_ref]
    let err: Box<PgDatabaseError> = err.downcast();

    assert_eq!(err.severity(), PgSeverity::Error);
    assert_eq!(err.message(), "column \"f\" does not exist");
    assert_eq!(err.code(), "42703");
    assert_eq!(err.position(), Some(PgErrorPosition::Original(8)));
    assert_eq!(err.routine(), Some("errorMissingColumn"));
    assert_eq!(err.constraint(), None);
}

#[wasm_bindgen_test]
async fn it_can_prepare_then_execute() {
    let mut conn = new().await;
    let _ = conn
        .execute(
            "create temp table tweet (id  BIGSERIAL PRIMARY KEY,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                text       TEXT        NOT NULL,
                owner_id   BIGINT
            )",
        )
        .await;
    let mut tx = conn.begin().await.unwrap();

    let tweet_id: i64 =
        sqlx::query_scalar("INSERT INTO tweet (text) VALUES ( 'Hello, World' ) RETURNING id")
            .fetch_one(&mut tx)
            .await
            .unwrap();

    let statement = tx
        .prepare("SELECT * FROM tweet WHERE id = $1")
        .await
        .unwrap();

    assert_eq!(statement.column(0).name(), "id");
    assert_eq!(statement.column(1).name(), "created_at");
    assert_eq!(statement.column(2).name(), "text");
    assert_eq!(statement.column(3).name(), "owner_id");

    assert_eq!(statement.column(0).type_info().name(), "INT8");
    assert_eq!(statement.column(1).type_info().name(), "TIMESTAMPTZ");
    assert_eq!(statement.column(2).type_info().name(), "TEXT");
    assert_eq!(statement.column(3).type_info().name(), "INT8");

    let row = statement
        .query()
        .bind(tweet_id)
        .fetch_one(&mut tx)
        .await
        .unwrap();
    let tweet_text: &str = row.try_get("text").unwrap();

    assert_eq!(tweet_text, "Hello, World");
}
