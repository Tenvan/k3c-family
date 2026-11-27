//! Port von `stil_test.go`.

use super::super::test_helpers::{hash_baum, lies, repo_root, schreibe, test_repo, unter, werkzeuge};
use super::*;
use crate::review::test_helpers::bun;

#[test]
fn stil_hash_wie_hash_ts() {
    if !werkzeuge() {
        return;
    }
    let schraeg = |rel: &str| unter(&repo_root(), rel).to_string_lossy().replace('\\', "/");
    let code = format!(
        "import {{ stilHash }} from '{}'; import {{ readFileSync }} from 'node:fs';\
         console.log(stilHash(readFileSync('{}', 'utf8')))",
        schraeg("tools/paper/schema/hash.ts"),
        schraeg(STIL_PFAD)
    );
    let want = bun(&["-e", &code]);
    let raw = std::fs::read(unter(&repo_root(), STIL_PFAD)).unwrap();
    assert_eq!(stil_hash(&raw).unwrap(), want);
}

#[test]
fn feld_diff_zeilen() {
    let felder = feld_diff(
        r##"{"m":{"stein":{"basis":"#1","licht":"#2"}},"a":1,"weg":true}"##,
        r##"{"m":{"stein":{"basis":"#3","licht":"#2"}},"a":1.0,"neu":[1]}"##,
    )
    .unwrap();
    let zeilen: Vec<String> = felder
        .iter()
        .map(|f| format!("{} {} {}", wert(&f.alt), f.pfad, wert(&f.neu)))
        .collect();
    assert_eq!(zeilen.join("|"), "#1 m.stein.basis #3|– neu [1]|true weg –");
}

#[test]
fn materialien_einer_spec() {
    let spec = std::fs::read(unter(&repo_root(), "tools/paper/fixtures/specs/monarch.paper.json")).unwrap();
    let m = materialien(&spec);
    // Lagen, Umhang, tausch
    for name in ["koenigsblau", "koenigsrot", "smaragd", "hermelin"] {
        assert!(m.contains(name), "{name} fehlt in {m:?}");
    }
}

#[test]
fn stil_set_schreibt() {
    let Some((w, cli)) = test_repo() else { return };
    let r = stil_set(&cli, r##"{"materialien":{"stein":{"basis":"#a8a294"}}}"##).unwrap();
    assert_eq!(r.felder.len(), 1, "{:?}", r.felder);
    assert_eq!(r.felder[0].pfad, "materialien.stein.basis");
    assert_eq!(wert(&r.felder[0].neu), "#a8a294");
    let ids: Vec<&str> = r.betroffen.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(ids, ["mauer"]);
    assert!(lies(&w.pfad, STIL_PFAD).contains("#a8a294"), "Stil nicht geschrieben");
    let r = stil_set(&cli, r#"{"papier":{"lage":{"dx":1}}}"#).unwrap();
    assert_eq!(r.betroffen.len(), 3, "papier betrifft alle");
}

#[test]
fn stil_set_abgelehnt() {
    let Some((w, cli)) = test_repo() else { return };
    let vorher = hash_baum(&w.pfad, &[STIL_PFAD]);
    for (patch, regel) in [
        (r##"{"farbe":"#fff"}"##, "schema"),
        (r#"{"materialien":{"stein":{"basis":"rot"}}}"#, "schema"),
        (r##"{"einzeltoene":{"stein":"#ffffff"}}"##, "material-doppelt"),
        (r#"{"materialien":{"koenigsrot":null}}"#, "material-unbekannt"),
    ] {
        let err = stil_set(&cli, patch).unwrap_err();
        let fehler = err.abgelehnt().unwrap_or_else(|| panic!("{patch}: {err}"));
        assert_eq!(fehler[0].regel, regel, "{patch}: {err}");
    }
    assert_eq!(hash_baum(&w.pfad, &[STIL_PFAD]), vorher, "Stil-Datei geändert");
}

/// Neu: Zeilenenden bleiben, keine Temp-Datei bleibt, kein UTF-8 wird nicht angefasst.
#[test]
fn stil_set_zeilenenden_und_utf8() {
    let Some((w, cli)) = test_repo() else { return };
    let root = &w.pfad;
    let lf = lies(root, STIL_PFAD).replace("\r\n", "\n");
    let patch = r##"{"materialien":{"stein":{"basis":"#a8a294"}}}"##;
    for crlf in [false, true] {
        let alt = if crlf { lf.replace('\n', "\r\n") } else { lf.clone() };
        schreibe(root, STIL_PFAD, &alt);
        stil_set(&cli, patch).unwrap();
        let neu = lies(root, STIL_PFAD);
        let zeilen = neu.matches('\n').count();
        let mit_cr = neu.matches("\r\n").count();
        assert_eq!(mit_cr, if crlf { zeilen } else { 0 });
    }
    assert!(!unter(root, "public/grafik/stil.paper.json.tmp").exists());
    schreibe(root, STIL_PFAD, [b'{', b'"', 0xff, b'"', b':', b'1', b'}']);
    let vorher = hash_baum(root, &[STIL_PFAD]);
    let err = stil_set(&cli, patch).unwrap_err().to_string();
    assert!(err.contains("kein UTF-8"), "{err}");
    assert_eq!(hash_baum(root, &[STIL_PFAD]), vorher);
}
