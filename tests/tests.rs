#![warn(clippy::all)]

macro_rules! test_match {
    ( with $text:expr; $( $name:ident: $pattern:expr => $expect:expr, )* ) => {
    $(
        #[test]
        fn $name() -> rex::Result<()> {
            assert_eq!($expect, rex::matches($pattern, $text)?);
            Ok(())
        }
    )*
    };
}

test_match! {
    with "foo";
    empty: "" => true,
}
