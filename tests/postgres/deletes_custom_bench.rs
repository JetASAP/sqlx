#[macro_export]
macro_rules! time_delete_query {
    ($n:expr, $count:literal) => {
        let mut conn = new::<Postgres>().await.unwrap();

        conn.execute("create temp table bench_deletes (id integer, descr text, primary key(id))")
            .await;

        conn.execute("create bitmap index id_idx on bench_deletes (id)")
            .await;

        let _ = sqlx::query(&format!(
            "insert into bench_deletes (id, descr) select generate_series(1,{}) AS id, md5(random()::text) AS descr",
            $count
        ))

        .execute(&mut conn)
        .await;

        let start = Instant::now();
        for _ in 0..3u8 {
            for i in 1..$count {
                let _ = sqlx::query(&format!(
                    "delete from bench_deletes where id = {}",
                    i
                ))
                .execute(&mut conn)
                .await;
            }
        }

        let end = Instant::now();

        println!("{}: Avg time is {}", $n, end.duration_since(start).as_millis() / 3u128);
    };
}

use sqlx::postgres::Postgres;
use sqlx::Executor;
use sqlx_test::{new, setup_if_needed};
use std::time::Instant;

#[sqlx_macros::test]
async fn deletes_query_small() {
    time_delete_query!("small", 100u32);
}

#[sqlx_macros::test]
async fn deletes_query_medium() {
    time_delete_query!("medium", 1000u32);
}

#[sqlx_macros::test]
async fn deletes_query_large() {
    time_delete_query!("large", 10000u32);
}
