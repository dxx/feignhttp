#[doc(hidden)]
#[macro_export]
macro_rules! map {
    () => ({
        std::collections::HashMap::new()
    });
    ($($k:expr => $v:expr),*) => ({
        let mut map = std::collections::HashMap::new();
        $(map.insert($k, $v);)*
        map
    });
}
