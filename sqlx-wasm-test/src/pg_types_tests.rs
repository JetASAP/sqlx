use sqlx::postgres::Postgres;
use sqlx_wasm_test::{test_prepared_type, test_type};

test_type!(null<Option<i16>>(Postgres,
    "NULL::int2" == None::<i16>
));

test_type!(null_vec<Vec<Option<i16>>>(Postgres,
    "array[10,NULL,50]::int2[]" == vec![Some(10_i16), None, Some(50)],
));

test_type!(bool<bool>(Postgres,
    "false::boolean" == false,
    "true::boolean" == true
));

test_type!(bool_vec<Vec<bool>>(Postgres,
    "array[true,false,true]::bool[]" == vec![true, false, true],
));

test_type!(byte_vec<Vec<u8>>(Postgres,
    "E'\\\\xDEADBEEF'::bytea"
        == vec![0xDE_u8, 0xAD, 0xBE, 0xEF],
    "E'\\\\x'::bytea"
        == Vec::<u8>::new(),
    "E'\\\\x0000000052'::bytea"
        == vec![0_u8, 0, 0, 0, 0x52]
));

// BYTEA cannot be decoded by-reference from a simple query as postgres sends it as hex
test_prepared_type!(byte_slice<&[u8]>(Postgres,
    "E'\\\\xDEADBEEF'::bytea"
        == &[0xDE_u8, 0xAD, 0xBE, 0xEF][..],
    "E'\\\\x0000000052'::bytea"
        == &[0_u8, 0, 0, 0, 0x52][..]
));
test_type!(str<&str>(Postgres,
    "'this is foo'" == "this is foo",
    "''" == "",
    "'identifier'::name" == "identifier",
    "'five'::char(4)" == "five",
    "'more text'::varchar" == "more text",
));

test_type!(string<String>(Postgres,
    "'this is foo'" == format!("this is foo"),
));

/*
test_type!(string_vec<Vec<String>>(Postgres,
    "array['one','two','three']::text[]"
        == vec!["one","two","three"],

    "array['', '\"']::text[]"
        == vec!["", "\""],

    "array['Hello, World', '', 'Goodbye']::text[]"
        == vec!["Hello, World", "", "Goodbye"]
));

test_type!(i8(
    Postgres,
    "0::\"char\"" == 0_i8,
    "120::\"char\"" == 120_i8,
));
*/

test_type!(u32(Postgres, "325235::oid" == 325235_u32,));

test_type!(i16(
    Postgres,
    "-2144::smallint" == -2144_i16,
    "821::smallint" == 821_i16,
));

test_type!(i32(
    Postgres,
    "94101::int" == 94101_i32,
    "-5101::int" == -5101_i32
));

test_type!(i32_vec<Vec<i32>>(Postgres,
    "'{5,10,50,100}'::int[]" == vec![5_i32, 10, 50, 100],
    "'{1050}'::int[]" == vec![1050_i32],
    "'{}'::int[]" == Vec::<i32>::new(),
    "'{1,3,-5}'::int[]" == vec![1_i32, 3, -5]
));

test_type!(i64(Postgres, "9358295312::bigint" == 9358295312_i64));

test_type!(f32(Postgres, "9419.122::real" == 9419.122_f32));

test_type!(f64(
    Postgres,
    "939399419.1225182::double precision" == 939399419.1225182_f64
));

test_type!(f64_vec<Vec<f64>>(Postgres,
    "'{939399419.1225182,-12.0}'::float8[]" == vec![939399419.1225182_f64, -12.0]
));
