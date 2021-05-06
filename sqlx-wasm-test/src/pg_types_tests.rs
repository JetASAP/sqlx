use sqlx::postgres::Postgres;
use sqlx_wasm_test::test_type;

test_type!(null<Option<i16>>(Postgres,
    "NULL::int2" == None::<i16>
));
