#![allow(unused_imports)]

#[test]
#[rustfmt::skip]
fn test_imports() {
    use valve_server_query;
    use valve_server_query::types;
    use valve_server_query::types::{
        Short,
        Long,
        Float,
        LongLong,
        // Temporarily removed due to namespacing concerns.
        //String as VString,
    };
}
