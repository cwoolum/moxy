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
fn unicode_escape_underscores() {
    for source in ["\"\\u{00_41}\"", "'\\u{00_41}'", "c\"\\u{00_41}\""] {
        let tokens: TokenStream = source.parse().unwrap_or_else(|_| panic!("{source:?} could not be scanned"));
        let token = tokens.into_iter().next().expect("no tokens scanned");

        let value = match token.as_literal().expect("expected literal") {
            Lit::Str(v) => v.value().to_string(),
            Lit::Char(v) => v.value().to_string(),
            Lit::CStr(v) => String::from_utf8_lossy(v.value()).into_owned(),
            other => panic!("{source:?} scanned as {other:?}"),
        };

        assert_eq!(value, "A", "{source:?}");
    }
}
