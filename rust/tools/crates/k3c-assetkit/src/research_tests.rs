use chrono::TimeZone;

use super::*;
use crate::testutil::{repo, write_file};

fn day() -> DateTime<FixedOffset> {
    FixedOffset::east_opt(0)
        .and_then(|utc| utc.with_ymd_and_hms(2026, 10, 14, 9, 0, 0).single())
        .expect("Testtag")
}

fn hit(title: &str, link: &str, source: &str, license: &str) -> Hit {
    Hit {
        title: title.to_string(),
        link: link.to_string(),
        source: source.to_string(),
        license: license.to_string(),
        ..Hit::default()
    }
}

/// B-298/AC-04: Treffer bleiben gespeichert (auch nach Neuladen), ohne Doppel; Verwerfen entfernt sie.
#[test]
fn treffer_ablage() {
    let dir = repo();
    let root = dir.path();
    let hits = [
        hit("Coin", "https://freesound.org/s/1/", "freesound", "CC0 1.0"),
        hit("Coin itch", "https://x.itch.io/coin", "Agent", ""),
    ];
    assert_eq!(add_hits(root, "sfx:coin", &hits, day()).expect("add_hits"), 2);
    let again = add_hits(root, "sfx:coin", &hits[..1], day()).expect("add_hits");
    assert_eq!(again, 0, "Doppel angelegt");
    let got = hits_of(root, "sfx:coin").expect("hits_of");
    assert!(
        got.len() == 2
            && got[0].license_status == LICENSE_KNOWN
            && got[1].license_status == LICENSE_CHECK
            && got[0].created == "2026-10-14"
            && !got[0].id.is_empty(),
        "gespeichert: {got:?}"
    );
    check_drop(root, &got[0].id);
}

fn check_drop(root: &Path, id: &str) {
    drop_hit(root, "sfx:coin", id).expect("drop_hit");
    let got = hits_of(root, "sfx:coin").expect("hits_of");
    assert!(got.len() == 1 && got[0].title == "Coin itch", "nach Verwerfen: {got:?}");
    assert!(
        drop_hit(root, "sfx:coin", "nichts").is_err(),
        "unbekannter Treffer ohne Fehler"
    );
    let err = add_hits(root, "gibts-nicht", &[hit("a", "https://a", "", "")], day()).expect_err("unbekannter Slot");
    assert!(err.to_string().contains("unbekannter Asset-Slot"), "{err}");
}

#[test]
fn treffer_kaputte_datei_bleibt() {
    let dir = repo();
    let path = dir.path().join(RESEARCH_PATH);
    write_file(&path, "[");
    let added = add_hits(dir.path(), "farm", &[hit("a", "https://a", "", "")], day());
    assert!(added.is_err(), "kaputte Datei ohne Fehler");
    assert_eq!(fs::read_to_string(&path).expect("lesen"), "[", "überschrieben");
}

/// Q70: übernehmbar, NC markiert, ND und Quellen ohne Weitergabe ausgeschlossen, Unklares prüfen.
#[test]
fn classify_license() {
    let cases = [
        ("CC0 1.0", "https://opengameart.org/x", LICENSE_KNOWN, ""),
        ("CC BY-SA 4.0", "https://commons.wikimedia.org/x", LICENSE_KNOWN, "SA"),
        ("CC BY-NC 3.0", "https://freesound.org/x", LICENSE_KNOWN, "NC"),
        ("OGA-BY 3.0", "", LICENSE_KNOWN, ""),
        ("CC BY-NC-ND 4.0", "", LICENSE_EXCLUDED, "ND"),
        ("CC0 1.0", "https://pixabay.com/sound-effects/x", LICENSE_EXCLUDED, ""),
        ("", "https://x.itch.io/coin", LICENSE_CHECK, ""),
        ("Attribution required", "", LICENSE_CHECK, ""),
    ];
    for (license, link, status, flag) in cases {
        assert_eq!(
            super::classify_license(license, link),
            (status, flag),
            "{license:?} {link:?}"
        );
    }
}

/// Eigener Test: ID und Dateiform wie Go (SHA-1 über den Link, Feldfolge, `omitempty`), Unterdomänen ausgeschlossen.
#[test]
fn dateiform_wie_go() {
    let dir = repo();
    let hits = [hit("A & <b>", "abc", "Agent", "")];
    add_hits(dir.path(), "farm", &hits, day()).expect("add_hits");
    let text = fs::read_to_string(dir.path().join(RESEARCH_PATH)).expect("lesen");
    let want = format!(
        "{{\n  \"_comment\": \"{RESEARCH_COMMENT}\",\n  \"treffer\": {{\n    \"farm\": [\n      {{\n        \
         \"id\": \"a9993e36\",\n        \"titel\": \"A & <b>\",\n        \"link\": \"abc\",\n        \
         \"quelle\": \"Agent\",\n        \"lizenzStatus\": \"pruefen\",\n        \"erstellt\": \"2026-10-14\"\n      \
         }}\n    ]\n  }}\n}}\n"
    );
    assert_eq!(text, want);
    assert!(excluded_host("https://cdn.WWW.Pixabay.com/x") && excluded_host("https://www.mixkit.co"));
    assert!(!excluded_host("https://notpixabay.com") && !excluded_host("pixabay.com/x"));
}

#[test]
fn suchbegriff_ohne_abschnitt() {
    let slot = Slot {
        description: "Gebäude: farm ".to_string(),
        ..Slot::default()
    };
    assert_eq!(search_term(&slot), "farm");
}

/// Eigener Test: eine Liste ist wie in Go ungültig und bleibt unangetastet; `null` und `null`-Listen sind leer.
#[test]
fn liste_statt_objekt_bleibt() {
    let dir = repo();
    let path = dir.path().join(RESEARCH_PATH);
    write_file(&path, " []");
    let added = add_hits(dir.path(), "farm", &[hit("a", "https://a", "", "")], day());
    assert!(
        added
            .expect_err("Liste")
            .to_string()
            .contains("ungültig, nichts geschrieben")
    );
    assert_eq!(fs::read_to_string(&path).expect("lesen"), " []");
    for empty in ["null", r#"{"treffer":null}"#, r#"{"treffer":{"farm":null}}"#] {
        write_file(&path, empty);
        assert!(hits_of(dir.path(), "farm").expect(empty).is_empty(), "{empty}");
    }
}
