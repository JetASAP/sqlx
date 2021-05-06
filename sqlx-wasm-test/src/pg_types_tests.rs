use sqlx::postgres::Postgres;
use sqlx_test::{test_decode_type, test_prepared_type, test_type};

test_type!(null<Option<i16>>(Postgres,
    "NULL::int2" == None::<i16>
));
