use moxy_token::{Lit, TokenStream};

#[test]
fn accept() {
    macro_rules! scan {
        ($($check:ident => $source:literal),+ $(,)?) => {
            $(
                let tokens: TokenStream = $source.parse().expect(concat!("literal ", $source, " could not be scanned"));
                let mut iter = tokens.into_iter();
                let token = iter.next().expect(concat!("no tokens scanned from ", $source));
                let token = token.as_literal().expect("expected literal");
                assert!(token.$check());
            )+
        };
    }

    scan! {
        is_int => "42u8",
        is_float => "3.5",
        is_float => "3.5f32",
        is_float => "3.5f64",
        is_str => r#""text""#,
        is_byte_str => r#"b"text""#,
        is_c_str => r#"c"text""#,
        is_char => "'x'",
        is_byte => "b'x'",
        is_bool => "true",
        is_bool => "false",
    }
}

#[test]
fn reject() {
    let cases = ["test", "dyn", ",", "|"];

    for case in cases {
        let tokens: TokenStream = case.parse().expect("invalid");
        let mut iter = tokens.into_iter();
        let token = iter.next().expect("no tokens scanned");
        assert!(!token.is_literal());
    }
}

#[test]
fn suffixed_integer_constructors() {
    for (lit, repr) in [
        (Lit::u8_suffixed(7), "7u8"),
        (Lit::u16_suffixed(7), "7u16"),
        (Lit::u32_suffixed(7), "7u32"),
        (Lit::u64_suffixed(7), "7u64"),
        (Lit::usize_suffixed(7), "7usize"),
        (Lit::i8_suffixed(7), "7i8"),
        (Lit::i16_suffixed(7), "7i16"),
        (Lit::i32_suffixed(7), "7i32"),
        (Lit::i64_suffixed(7), "7i64"),
        (Lit::isize_suffixed(7), "7isize"),
    ] {
        assert_eq!(lit.to_string(), repr);
    }
}
