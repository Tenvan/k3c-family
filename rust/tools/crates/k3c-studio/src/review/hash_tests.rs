use std::collections::BTreeSet;

use serde_json::Value;

use super::super::test_helpers::*;
use super::{RENDERER_VERSION, kanonisch, render_key, render_key_mit, spec_hash};

fn wert(raw: &str) -> Value {
    serde_json::from_str(raw).expect("JSON")
}

/// Beispiel aus `tools/paper/schema/hash.test.ts`.
#[test]
fn kanonisch_wie_hash_ts() {
    let v = wert(r#"{"b":[2,1],"a":{"d":1,"c":"x"}}"#);
    assert_eq!(kanonisch(&v), r#"{"a":{"c":"x","d":1},"b":[2,1]}"#);
}

/// Zahlen wie JS `JSON.stringify`, Texte ohne HTML-Escaping, CRLF als LF.
#[test]
fn kanonisch_zahlen_und_texte() {
    let v = wert(r#"{"z":[1.50,-0,1e21,1.5e-7,0.000001,100,-2.25],"t":"a<b>&\u2028\r\nc\"\\\u0001"}"#);
    let want = concat!(
        r#"{"t":"a<b>&"#,
        "\u{2028}",
        r#"\nc\"\\\u0001","z":[1.5,0,1e+21,1.5e-7,0.000001,100,-2.25]}"#
    );
    assert_eq!(kanonisch(&v), want);
}

/// Rust-Hash = `hash.ts` für Fixture-Spec und Stil-Datei, auch bei Zahlen mit vielen Stellen und Schlüsseln, deren
/// UTF-16-Reihenfolge von der Byte-Reihenfolge abweicht.
#[test]
fn spec_hash_wie_bun() {
    if !werkzeuge() {
        return;
    }
    let dir = crate::test_helpers::tmp_dir();
    let heikel = dir.path().join("heikel.json");
    schreibe(
        &heikel,
        "{\"\u{ff01}\":0.1,\"\u{1f600}\":1.0000000000000002,\"z\":[2.2250738585072014e-308,123456789012345678]}",
    );
    let pfade = [
        fixture("specs/monarch.paper.json"),
        repo_root().join("public/grafik/stil.paper.json"),
        heikel,
    ];
    let mut args = vec![
        "-e",
        "const h=await import(process.cwd()+'/tools/paper/schema/hash.ts');const fs=require('fs');\
         for (const p of process.argv.slice(1)) console.log(h.specHash(fs.readFileSync(p,'utf8')))",
    ];
    args.extend(pfade.iter().map(|p| p.to_str().expect("Pfad als Text")));
    let want: Vec<String> = bun(&args).split_whitespace().map(str::to_owned).collect();
    let got: Vec<String> = pfade.iter().map(|p| spec_hash(&lies(p)).expect("Hash")).collect();
    assert_eq!(got, want);
}

/// Zeilenenden und Schlüsselreihenfolge ändern den Hash nicht, `$schema` zählt nicht (AC-02).
#[test]
fn spec_hash_crlf() {
    let raw = String::from_utf8(lies(fixture("specs/monarch.paper.json"))).expect("UTF-8");
    let a = spec_hash(raw.as_bytes()).expect("Hash");
    let crlf = raw.replace("\r\n", "\n").replace('\n', "\r\n");
    let mut m = wert(&raw);
    m.as_object_mut().expect("Objekt").remove("$schema");
    let umsortiert = serde_json::to_string_pretty(&m).expect("JSON");
    for (name, v) in [("crlf", crlf), ("umsortiert", umsortiert)] {
        assert_eq!(spec_hash(v.as_bytes()).expect(name), a, "{name}");
    }
    let anders = raw.replacen("koenigsrot", "purpur", 1);
    assert_ne!(
        spec_hash(anders.as_bytes()).expect("Hash"),
        a,
        "andere Spec, gleicher Hash"
    );
}

/// Spec-, Stil-Hash und Renderer-Version ändern den Schlüssel.
#[test]
fn render_key_unterscheidet() {
    let k = render_key("a", "b");
    assert_eq!(k, render_key_mit("a", "b", RENDERER_VERSION));
    assert_eq!(k.len(), 64);
    let seen = BTreeSet::from([
        k,
        render_key("a", "c"),
        render_key("c", "b"),
        render_key_mit("a", "b", "paper-1"),
    ]);
    assert_eq!(seen.len(), 4);
}

/// Die Renderer-Version steht gleich in `hash.ts` und im Go-Orakel: sonst trennen sich die Caches.
#[test]
fn renderer_version_wie_hash_ts() {
    let ts = String::from_utf8(lies(repo_root().join("tools/paper/schema/hash.ts"))).expect("ts");
    assert!(ts.contains(&format!("RENDERER_VERSION = '{RENDERER_VERSION}'")));
}
