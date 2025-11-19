pub macro btmap {
    () => {
        std::collections::BTreeMap::new()
    },

    ( $($key:expr => $value:expr),* $(,)? ) => {{
        let mut map = std::collections::BTreeMap::new();
        $(
            map.insert($key, $value);
        )+
        map
    }}
}
