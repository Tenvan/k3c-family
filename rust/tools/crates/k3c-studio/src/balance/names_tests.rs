//! Go `names_test.go` (RS2.2, B-726): Die Namen des Studios folgen dem Crate `k3c-balance`, das `k3c-game sim balance`
//! rechnet. Der Test liest den Rust-Quelltext (`bot_names`, `min_players`, Konstanten der Hilfen) und vergleicht.

use std::collections::BTreeMap;

use regex::Regex;

use super::names::{
    HELPS_AIDS, HELPS_BOSS, HELPS_FULL, MIN_PLAYERS, MIX_SEP, bot_names, known_bot, load_world, min_players,
};
use super::test_helpers::repo_root;

fn read_rust(file: &str) -> String {
    let path = repo_root().join("rust/crates/k3c-balance/src").join(file);
    std::fs::read_to_string(path).expect(file)
}

/// Rumpf der Funktion `name` (bis zur ersten Zeile, die nur `}` enthält).
fn rust_body<'a>(src: &'a str, name: &str) -> &'a str {
    let (_, rest) = src.split_once(&format!("fn {name}(")).expect(name);
    rest.split_once("\n}").map_or(rest, |(body, _)| body)
}

fn regex(pattern: &str) -> Regex {
    Regex::new(pattern).expect(pattern)
}

#[test]
fn bot_namen_wie_engine() {
    let src = read_rust("bots.rs");
    let mut names: Vec<String> = regex(r#""([^"]+)""#)
        .captures_iter(rust_body(&src, "bot_names"))
        .map(|m| m[1].to_owned())
        .collect();
    names.sort();
    assert_eq!(names, bot_names(), "Profile k3c-balance gegen Studio");
    let arms: Vec<(String, String)> = regex(r#""([^"]+)"\s*=>\s*(\d+)"#)
        .captures_iter(rust_body(&src, "min_players"))
        .map(|m| (m[1].to_owned(), m[2].to_owned()))
        .collect();
    for (name, n) in &arms {
        assert_eq!(&min_players(name).to_string(), n, "minPlayers[{name}]");
    }
    assert_eq!(
        arms.len(),
        MIN_PLAYERS.len(),
        "minPlayers: andere Anzahl als k3c-balance"
    );
}

#[test]
fn hilfen_wie_engine() {
    let rust_const = regex(r#"pub const (\w+): &str = "([^"]*)";"#);
    let mut got = BTreeMap::new();
    for file in ["helps.rs", "bots.rs"] {
        for m in rust_const.captures_iter(&read_rust(file)) {
            got.insert(m[1].to_owned(), m[2].to_owned());
        }
    }
    for (k, v) in [
        ("HELPS_AIDS", HELPS_AIDS),
        ("HELPS_FULL", HELPS_FULL),
        ("HELPS_BOSS", HELPS_BOSS),
        ("MIX_SEP", MIX_SEP),
    ] {
        assert_eq!(got.get(k).map(String::as_str), Some(v), "{k}");
    }
}

#[test]
fn known_bot_und_welten() {
    for (name, ok) in [
        ("saver", true),
        ("coop2", true),
        ("saver+passive", true),
        ("coop2+saver", false),
        ("", false),
        ("raten", false),
    ] {
        assert_eq!(known_bot(name), ok, "known_bot({name:?})");
    }
    let root = repo_root();
    let w = load_world(&root, "standard").expect("standard");
    assert!(!w.islands.is_empty() && !w.islands[0].depths.is_empty(), "{w:?}");
    assert!(
        load_world(&root, "../targets").is_err(),
        "Pfad außerhalb von data/worlds angenommen"
    );
}
