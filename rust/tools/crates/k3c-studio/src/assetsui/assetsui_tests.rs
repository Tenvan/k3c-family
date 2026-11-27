//! Port von `assetsui_test.go`.

use std::sync::Arc;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use serde_json::json;

use super::test_helpers::*;
use super::*;

/// Katalog aus dem Checkout; geaendert gegen `HEAD`, daher pferd und mauer false (AC-07, B-701).
#[test]
fn assets_list_test_repo() {
    let Some(repo) = test_repo() else { return };
    let got = dienst(&repo.pfad).assets_list().unwrap();
    let mut want = fixture("AssetsList");
    for e in want.as_array_mut().unwrap() {
        if e["id"] == "pferd" || e["id"] == "mauer" {
            e["geaendert"] = json!(false);
        }
    }
    assert_eq!(nach_id(&roundtrip(got)), nach_id(&want));
}

/// Spec ohne `$schema`; im Test-Repo trägt der Umhang purpur (AC-07).
#[test]
fn assets_get() {
    let Some(repo) = test_repo() else { return };
    let got = dienst(&repo.pfad).assets_get("monarch").unwrap();
    let mut want = fixture("AssetsGet");
    want["umhang"]["material"] = json!("purpur");
    assert_eq!(got, want);
}

#[test]
fn assets_get_fehlt() {
    let Some(repo) = test_repo() else { return };
    let e = dienst(&repo.pfad).assets_get("niemand").unwrap_err();
    assert_eq!(e, "Asset „niemand“ fehlt");
}

/// Echte Datei der Repo-Wurzel = Fixture (AC-07).
#[test]
fn assets_style() {
    let got = dienst(&repo_root()).assets_style().unwrap();
    assert_eq!(got, fixture("AssetsStyle"));
}

#[test]
fn assets_prompts() {
    let got = dienst(&repo_root()).assets_prompts().unwrap();
    assert_eq!(roundtrip(got), fixture("AssetsPrompts"));
}

/// Gegen `HEAD~1` wie `AssetsReview.json`; banner ungültig mit Fehlertext, die übrigen vollständig (AC-05).
#[test]
fn assets_review_test_repo() {
    let Some(repo) = test_repo() else { return };
    let mut got = roundtrip(dienst_fake(&repo.pfad).assets_review("HEAD~1").unwrap());
    let mut want = fixture("AssetsReview");
    // Der Hash des Test-Repos hängt von der echten Stil-Datei ab (B-738).
    want["basisCommit"] = got["basisCommit"].clone();
    got["assets"] = nach_id(&got["assets"]).into();
    want["assets"] = nach_id(&want["assets"]).into();
    assert_eq!(got, want);
}

/// Leere Basis = `HEAD`.
#[test]
fn assets_review_leer() {
    let Some(repo) = test_repo() else { return };
    let got = dienst_fake(&repo.pfad).assets_review("").unwrap();
    assert_eq!((got.basis.as_str(), got.assets.len()), ("HEAD", 2), "{got:?}");
}

/// Vertragsfehlertext; beim Pixel-Diff ist leer keine Basis.
#[test]
fn assets_basis_ungueltig() {
    let Some(repo) = test_repo() else { return };
    let d = dienst_fake(&repo.pfad);
    let fehler = [
        d.assets_review("main").unwrap_err(),
        d.assets_frame("monarch", "reiten", 0, "", "main").unwrap_err(),
        d.assets_pixel_diff("monarch", "reiten", 0, "").unwrap_err(),
        d.assets_pixel_diff("monarch", "reiten", 0, "; rm -rf").unwrap_err(),
    ];
    for (i, e) in fehler.iter().enumerate() {
        assert!(
            e.ends_with("ungültig: HEAD, HEAD~1, Commit-SHA oder origin/main"),
            "Aufruf {i}: {e}"
        );
    }
}

fn dekodiere(url: &str) -> Vec<u8> {
    let daten = url.strip_prefix("data:image/png;base64,").expect("PNG-Data-URL");
    STANDARD.decode(daten).unwrap()
}

/// Frame des Arbeitsstands als Data-URL mit den Teil-Boxen von Reiter und Reittier (echte CLI).
#[test]
fn assets_frame() {
    let Some(repo) = test_repo() else { return };
    let got = dienst(&repo.pfad).assets_frame("monarch", "reiten", 0, "", "").unwrap();
    assert!(dekodiere(&got.bild).starts_with(&[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]));
    // Reittier-Teile mit Präfix (PT2, B-709)
    for teil in ["kopf", "krone", "reittier:rumpf-vorn", "reittier:sattel"] {
        assert_eq!(got.teile.get(teil).map(Vec::len), Some(4), "{teil}: {:?}", got.teile);
    }
}

/// Der Frame der Basis kommt beim zweiten Aufruf aus dem Cache; gelöschte Assets gibt es nur dort.
#[test]
fn assets_frame_basis() {
    let Some(repo) = test_repo() else { return };
    let mut d = dienst(&repo.pfad);
    // HEAD: dort liegt das Reittier pferd; in HEAD~1 fehlt es, die echte CLI lehnt den Reiter dort ab.
    let erst = d.assets_frame("monarch", "reiten", 0, "", "HEAD").unwrap();
    d.r = Arc::new(Probe::ohne_frame());
    let zweit = d.assets_frame("monarch", "reiten", 0, "", "HEAD").unwrap();
    assert_eq!(zweit.bild, erst.bild);
    // fass gibt es nur in der Basis und hat keine Animation: gefunden, aber ohne Frame.
    let e = dienst(&repo.pfad)
        .assets_frame("fass", "", 0, "", "HEAD~1")
        .unwrap_err();
    assert_eq!(e, "Asset „fass“ hat keine Animation");
}

/// Der Text nennt die vorhandenen Animationen.
#[test]
fn assets_frame_animation_unbekannt() {
    let Some(repo) = test_repo() else { return };
    let e = dienst(&repo.pfad)
        .assets_frame("monarch", "flug", 0, "", "")
        .unwrap_err();
    for teil in ["„flug“", "idle", "reiten"] {
        assert!(e.contains(teil), "{e}");
    }
}

/// Leer = erste Animation der Spec.
#[test]
fn assets_frame_leere_animation() {
    let Some(repo) = test_repo() else { return };
    let d = dienst(&repo.pfad);
    let leer = d.assets_frame("monarch", "", 0, "", "").unwrap();
    let idle = d.assets_frame("monarch", "idle", 0, "", "").unwrap();
    assert_eq!(leer.bild, idle.bild);
}

/// Monarch gegen `HEAD~1` wie `AssetsPixelDiff.json` (anteil, boxen, overlay).
#[test]
fn assets_pixel_diff() {
    let Some(repo) = test_repo() else { return };
    let got = dienst_fake(&repo.pfad)
        .assets_pixel_diff("monarch", "reiten", 0, "HEAD~1")
        .unwrap();
    assert_eq!(roundtrip(got), fixture("AssetsPixelDiff"));
}

/// Sauberer Checkout: `"assets": []` statt `null`.
#[test]
fn assets_review_ohne_aenderung() {
    let Some(repo) = test_repo() else { return };
    git_in(&repo.pfad, &["add", "-A"]);
    git_in(&repo.pfad, &["commit", "-q", "-m", "alles"]);
    let got = dienst_fake(&repo.pfad).assets_review("HEAD").unwrap();
    let raw = serde_json::to_string(&got).unwrap();
    assert!(raw.contains(r#""assets":[]"#), "{raw}");
}

/// Ein CLI-Absturz trifft nur den Eintrag (gueltig false, Regel cli), nicht den Katalog; ein schon geprüfter Stand kommt
/// aus dem Memo und braucht die CLI nicht.
#[test]
fn assets_list_cli_fehler() {
    let Some(repo) = test_repo() else { return };
    let neu = |id: &str, x: u8| {
        let pfad = repo.pfad.join(format!("public/grafik/requisiten/{id}.paper.json"));
        schreibe(pfad, format!(r#"{{"id":"{id}","rig":"requisite","x":{x}}}"#));
    };
    neu("neu1", 1);
    let mut d = dienst(&repo.pfad);
    d.assets_list().unwrap();
    neu("neu2", 2);
    d.r = Arc::new(Probe::validate_kaputt());
    let liste = d.assets_list().unwrap();
    let finde = |id: &str| liste.iter().find(|e| e.id == id).expect(id);
    let neu2 = finde("neu2");
    assert!(!neu2.gueltig, "{neu2:?}");
    assert_eq!(neu2.fehler.len(), 1, "{neu2:?}");
    assert_eq!(neu2.fehler[0].regel, "cli");
    assert!(finde("monarch").gueltig, "monarch nicht aus dem Memo");
}

/// Die id muss dem Dateinamen gleichen, kein Suffix-Treffer über Gruppen hinweg.
#[test]
fn pfad_zu_exakt() {
    let Some(repo) = test_repo() else { return };
    assert!(dienst(&repo.pfad).assets_get("figuren/monarch").is_err());
}

/// Ein Frame jenseits der Animation ist ein Fehler, kein beliebiges Bild.
#[test]
fn assets_frame_ausserhalb() {
    let Some(repo) = test_repo() else { return };
    let d = dienst(&repo.pfad);
    let e = d.assets_frame("monarch", "idle", 8, "", "").unwrap_err();
    assert_eq!(e, "Frame 8 fehlt: Animation „idle“ hat 8 Frames");
    assert!(d.assets_pixel_diff("monarch", "idle", 8, "HEAD~1").is_err());
}

/// Zahlen und Schlüssel einer Spec wie Go `json.Marshal` eines nach `any` gelesenen Objekts.
#[test]
fn objekt_wie_go() {
    let got = objekt(br#"{"b":100.0,"$schema":"x","a":[1.5,-2.0,7]}"#).unwrap();
    assert_eq!(serde_json::to_string(&got).unwrap(), r#"{"a":[1.5,-2,7],"b":100}"#);
    assert_eq!(objekt(b"null").unwrap(), Value::Null);
    assert!(objekt(b"[]").is_err());
}
