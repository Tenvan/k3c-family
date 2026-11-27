use super::inline;

// Go `edit_test.go` › TestInlineStyle, vorgezogen, weil `inline.rs` in RU70.3 entsteht.
#[test]
fn inline_stil() {
    for (input, want) in [
        (
            r#"{"a":1.50,"b":[1,2],"c":[{"x":-7}],"d":{},"e":[]}"#,
            r#"{ "a": 1.50, "b": [1, 2], "c": [ { "x": -7 } ], "d": {}, "e": [] }"#,
        ),
        (r#""<&>""#, r#""<&>""#),
    ] {
        assert_eq!(inline(input).as_deref(), Ok(want), "{input}");
        assert_eq!(inline(want).as_deref(), Ok(want), "nicht stabil");
    }
}

#[test]
fn inline_lehnt_daten_nach_dem_ende_ab() {
    assert!(inline("1 2").is_err());
    assert!(inline("{").is_err());
}
