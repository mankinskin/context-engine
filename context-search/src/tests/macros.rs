#[macro_export]
macro_rules! assert_not_indices {
    ($graph:ident, $($name:ident),*) => {
        $(
        let result = $graph.find_sequence(stringify!($name).chars());
        assert_matches!(
            result,
            Err(_) | Ok(_)
        );
        if let Ok(ref response) = result {
            assert!(!response.is_complete(),
                "Expected incomplete or error for {}, but got complete match",
                stringify!($name));
        }
        )*
    };
}
