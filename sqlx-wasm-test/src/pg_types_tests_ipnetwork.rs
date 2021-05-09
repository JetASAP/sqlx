use sqlx_wasm_test::test_type;

test_type!(ipnetwork<sqlx::types::ipnetwork::IpNetwork>(Postgres,
    "'127.0.0.1'::inet"
        == "127.0.0.1"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'8.8.8.8/24'::inet"
        == "8.8.8.8/24"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'::ffff:1.2.3.0'::inet"
        == "::ffff:1.2.3.0"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'2001:4f8:3:ba::/64'::inet"
        == "2001:4f8:3:ba::/64"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'192.168'::cidr"
        == "192.168.0.0/24"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
    "'::ffff:1.2.3.0/120'::cidr"
        == "::ffff:1.2.3.0/120"
            .parse::<sqlx::types::ipnetwork::IpNetwork>()
            .unwrap(),
));

test_type!(ipnetwork_vec<Vec<sqlx::types::ipnetwork::IpNetwork>>(Postgres,
    "'{127.0.0.1,8.8.8.8/24}'::inet[]"
        == vec![
           "127.0.0.1".parse::<sqlx::types::ipnetwork::IpNetwork>().unwrap(),
           "8.8.8.8/24".parse::<sqlx::types::ipnetwork::IpNetwork>().unwrap()
        ]
));
