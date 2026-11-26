use std::collections::BTreeMap;

use serde_json::Value;

use super::basis::aufloesen;
use super::test_helpers::*;
use super::*;

/// Generisches JSON mit nach id sortierten Assets (so vergleichen die Tests).
fn generisch(e: &Ergebnis) -> Value {
    let mut e = e.clone();
    e.assets.sort_by(|a, b| a.id.cmp(&b.id));
    serde_json::to_value(e).expect("Ergebnis als JSON")
}

fn status(e: &Ergebnis) -> BTreeMap<&str, &str> {
    e.assets.iter().map(|a| (a.id.as_str(), a.status.as_str())).collect()
}

fn asset<'a>(e: &'a Ergebnis, id: &str) -> Option<&'a Asset> {
    e.assets.iter().find(|a| a.id == id)
}

/// Alle fünf Fälle gegen HEAD~1 exakt wie `tools/paper/fixtures/ui/AssetsReview.json` (AC-01).
#[test]
fn review_head1_wie_fixture() {
    let Some(repo) = test_repo() else { return };
    let got = vergleich(&repo.pfad, "HEAD~1", &fake_frames()).expect("Review");
    let roh = lies(fixture("ui/AssetsReview.json"));
    let mut want: Ergebnis = serde_json::from_slice(&roh).expect("Fixture");
    // Der Hash hängt von der echten Stil-Datei ab (B-738).
    want.basis_commit = git_in(&repo.pfad, &["rev-parse", "--short=7", "HEAD~1"]);
    assert_eq!(generisch(&got), generisch(&want));
    let reihenfolge: Vec<&str> = got.assets.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(reihenfolge, ["banner", "mauer", "monarch", "pferd", "fass"]);
}

/// B-705/AC-02: nur die Stil-Datei geändert; das Vorher-Bild entsteht mit dem Stil der Basis (`--stil` des
/// Schnappschusses), daher zeigt die echte CLI einen Pixel-Unterschied.
#[test]
fn review_stilwechsel_vorher_bild() {
    let Some(repo) = test_repo() else { return };
    git_in(&repo.pfad, &["add", "-A"]);
    git_in(&repo.pfad, &["commit", "-q", "--no-gpg-sign", "-m", "Arbeitsstand"]);
    let pfad = repo.pfad.join("public/grafik/stil.paper.json");
    let mut stil: Value = serde_json::from_slice(&lies(&pfad)).expect("Stil");
    stil["materialien"]["stein"]["basis"] = Value::from("#2050d0");
    schreibe(&pfad, serde_json::to_vec_pretty(&stil).expect("Stil"));
    let got = vergleich(&repo.pfad, "HEAD", &cli()).expect("Review");
    let mauer = asset(&got, "mauer").expect("mauer fehlt");
    assert!(
        mauer.status == STATUS_GEAENDERT && mauer.stil && mauer.pixel_anteil > 0.0,
        "{mauer:?}"
    );
}

/// Gegen HEAD nur monarch (geändert) und banner (ungültig).
#[test]
fn review_head() {
    let Some(repo) = test_repo() else { return };
    let got = vergleich(&repo.pfad, "HEAD", &fake_frames()).expect("Review");
    let want = BTreeMap::from([("monarch", STATUS_GEAENDERT), ("banner", STATUS_UNGUELTIG)]);
    assert_eq!(status(&got), want);
    assert_eq!(got.basis, "HEAD");
}

fn monarch_aus_head(repo: &TestRepo) -> Vec<u8> {
    show(&repo.pfad, "HEAD", MONARCH)
        .expect("show")
        .expect("monarch in HEAD")
}

/// Nur Zeilenenden geändert → kein Eintrag (AC-02).
#[test]
fn review_nur_crlf() {
    let Some(repo) = test_repo() else { return };
    let alt = String::from_utf8(monarch_aus_head(&repo)).expect("UTF-8");
    schreibe(repo.pfad.join(MONARCH), alt.replace('\n', "\r\n"));
    let got = vergleich(&repo.pfad, "HEAD", &fake_frames()).expect("Review");
    assert_eq!(asset(&got, "monarch"), None);
}

/// Nur das Manifest der Ausgabe anders → geändert ohne Felder.
#[test]
fn review_ausgabe_geaendert() {
    let Some(repo) = test_repo() else { return };
    schreibe(repo.pfad.join(MONARCH), monarch_aus_head(&repo));
    schreibe(
        repo.pfad.join("public/grafik/figuren/monarch/manifest.json"),
        r#"{"id":"monarch","renderKey":"neu"}"#,
    );
    let got = vergleich(&repo.pfad, "HEAD", &fake_frames()).expect("Review");
    let monarch = asset(&got, "monarch").expect("monarch fehlt trotz geänderter Ausgabe");
    assert!(
        monarch.status == STATUS_GEAENDERT && monarch.felder.is_empty() && !monarch.stil,
        "{monarch:?}"
    );
}

/// Ohne CLI; gegen HEAD sind nur monarch und banner geändert (Grundlage von AssetsList).
#[test]
fn geaendert_gegen_head() {
    let Some(repo) = test_repo() else { return };
    let got = geaendert(&repo.pfad, "HEAD").expect("geaendert");
    let want = BTreeMap::from([
        (MONARCH.to_owned(), true),
        ("public/grafik/requisiten/banner.paper.json".to_owned(), true),
        ("public/grafik/reittiere/pferd.paper.json".to_owned(), false),
        ("public/grafik/gebaeude/mauer.paper.json".to_owned(), false),
    ]);
    assert_eq!(got, want);
    assert_eq!(aufloesen(&repo.pfad, "").expect("HEAD").basis, "HEAD");
}
