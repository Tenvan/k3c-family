use std::collections::BTreeMap;

use super::cache::gueltiger_name;
use super::git::ist_spec;
use super::test_helpers::*;
use super::*;

/// Animationsschlüssel wie `idle@spieler2` sind erlaubt, Pfadteile nicht (Review-Befund 1).
#[test]
fn bild_animation_mit_variante() {
    for ok in ["idle", "idle@spieler2", "stand.s2", "reiten-2"] {
        assert!(gueltiger_name(ok), "{ok:?} abgelehnt");
    }
    for bad in ["", "..", "a/b", r"a\b", "-x", "a..b", "C:x", "a\n"] {
        assert!(!gueltiger_name(bad), "{bad:?} angenommen");
    }
}

fn monarch_stand(repo: &TestRepo) -> Quelle {
    quelle_arbeitsstand(&repo.pfad, MONARCH)
        .expect("Quelle")
        .expect("monarch")
}

/// Gleichzeitige Aufrufe für denselben Frame scheitern nicht an einer gemeinsamen Temp-Datei (Befund 2).
#[test]
fn bild_parallel() {
    let Some(repo) = test_repo() else { return };
    let q = monarch_stand(&repo);
    std::thread::scope(|scope| {
        let laeufe: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| bild(&cli(), &repo.pfad, &q, "reiten", 0, "")))
            .collect();
        for lauf in laeufe {
            lauf.join().expect("Thread").expect("Bild");
        }
    });
}

/// Die CLI rendert die gehashten Bytes, nicht die spätere Datei auf der Platte (Befund 3).
#[test]
fn bild_aus_schnappschuss() {
    let Some(repo) = test_repo() else { return };
    let q = monarch_stand(&repo);
    let datei = q.spec_datei(&repo.pfad, &cli()).expect("Spec-Datei");
    assert_eq!(
        datei.ancestors().nth(5),
        Some(q.ordner(&repo.pfad, &cli()).as_path()),
        "Spec für die CLI liegt nicht im Cache: {datei:?}"
    );
}

/// Nicht lesbares JSON wird ein ungültiger Eintrag, der Rest des Reviews bleibt (Befund 4).
#[test]
fn review_kaputte_spec() {
    let Some(repo) = test_repo() else { return };
    schreibe(
        repo.pfad.join("public/grafik/requisiten/kaputt.paper.json"),
        "{nicht json",
    );
    let got = vergleich(&repo.pfad, "HEAD", &fake_frames()).expect("Review");
    let status: BTreeMap<&str, &str> = got.assets.iter().map(|a| (a.id.as_str(), a.status.as_str())).collect();
    assert_eq!(status["kaputt"], STATUS_UNGUELTIG);
    assert_eq!(status["monarch"], STATUS_GEAENDERT);
    assert_eq!(status["banner"], STATUS_UNGUELTIG);
    let kaputt = got.assets.iter().find(|a| a.id == "kaputt").expect("kaputt");
    assert_eq!(kaputt.fehler.len(), 1);
    assert_eq!(kaputt.fehler[0].regel, "schema");
}

/// Ein kaputter Commit oder Ordner ist ein Fehler, nicht „fehlt in der Basis“ (Befund 5).
#[test]
fn show_git_fehler() {
    let Some(repo) = test_repo() else { return };
    let fehlt = show(&repo.pfad, "HEAD", "public/grafik/fehlt.paper.json");
    assert!(matches!(fehlt, Ok(None)), "{fehlt:?}");
    let kein_repo = repo.pfad.join("kein-repo");
    assert!(
        show(&kein_repo, "HEAD", "public/grafik/stil.paper.json").is_err(),
        "Git-Fehler verschluckt"
    );
}

/// Nur echte Spec-Pfade, keine Pfadteile wie `..` oder Backslash (Befund 6).
#[test]
fn ist_spec_haertung() {
    for bad in [
        "public/grafik/../x.paper.json",
        r"public/grafik/a\..\x.paper.json",
        "public/grafik/a/..paper.json",
        "public/grafik/a/index.paper.json",
        "public/grafik/A:/x.paper.json",
        "public/grafik/a/-x.paper.json",
    ] {
        assert!(!ist_spec(bad), "{bad:?} gilt als Spec");
    }
    assert!(ist_spec(MONARCH));
}

/// Revisionen, die git als Schalter läse, erreichen git nicht; Pfade außerhalb der Ablage werden nicht gelesen.
#[test]
fn eingaben_bleiben_im_checkout() {
    let dir = crate::test_helpers::tmp_dir();
    let root = dir.path();
    for commit in ["", "-p", "--output=x"] {
        for err in [
            show(root, commit, MONARCH).err(),
            basis_dateien(root, commit).err(),
            specs_in_commit(root, commit).err(),
        ] {
            let text = err.expect("abgelehnt").to_string();
            assert!(text.ends_with("ungültig"), "{commit:?}: {text}");
        }
    }
    for basis in ["--all", "HEAD\n", "HEAD;x", "origin/main --upload-pack=x"] {
        let err = aufloesen(root, basis).expect_err(basis);
        assert_eq!(err.to_string(), fehler_basis(basis).to_string());
    }
    let pfad = "public/grafik/../../x.paper.json";
    for err in [
        quelle_arbeitsstand(root, pfad).err(),
        quelle_basis(root, "HEAD", pfad).err(),
    ] {
        let text = err.expect("abgelehnt").to_string();
        assert_eq!(text, format!("{pfad} ist keine Spec"));
    }
}

/// Die echte CLI (RD1) rendert unter `RENDERER_VERSION`, ein Renderer ohne eigene Kennung ebenso (Befund 7).
#[test]
fn cache_kennung() {
    assert_eq!(cli().kennung(), RENDERER_VERSION);
    struct Ohne;
    impl Renderer for Ohne {
        fn validate(&self, _: &std::path::Path) -> Result<Vec<Fehler>, Error> {
            Ok(Vec::new())
        }
        fn diff(
            &self,
            _: &std::path::Path,
            _: &std::path::Path,
            _: Option<(&std::path::Path, &std::path::Path)>,
        ) -> Result<DiffAntwort, Error> {
            Ok(DiffAntwort::default())
        }
        fn frame(
            &self,
            _: &std::path::Path,
            _: Option<&std::path::Path>,
            _: &str,
            _: i64,
            _: &str,
            _: &std::path::Path,
        ) -> Result<FrameAntwort, Error> {
            Ok(FrameAntwort::default())
        }
        fn pixel_diff(
            &self,
            _: &std::path::Path,
            _: &std::path::Path,
            _: &std::path::Path,
        ) -> Result<PixelAntwort, Error> {
            Ok(PixelAntwort::default())
        }
    }
    assert_eq!(Ohne.kennung(), RENDERER_VERSION);
}

/// Der Schnappschuss enthält das Reittier der Spec, damit die echte CLI (Regel `reittier-unbekannt`) es neben der Spec
/// findet; im Arbeitsstand aus dem Checkout, in der Basis aus dem Commit.
#[test]
fn schnappschuss_mit_reittier() {
    let Some(repo) = test_repo() else { return };
    let pferd = "public/grafik/reittiere/pferd.paper.json";
    let q = monarch_stand(&repo);
    assert!(q.beilagen.contains_key(pferd), "{:?}", q.beilagen.keys());
    let spec = q.spec_datei(&repo.pfad, &cli()).expect("Spec-Datei");
    let grafik = spec.ancestors().nth(2).expect("grafik");
    assert!(
        grafik.join("reittiere/pferd.paper.json").is_file(),
        "Reittier fehlt im Schnappschuss"
    );
    let b = quelle_basis(&repo.pfad, "HEAD", MONARCH)
        .expect("Basis")
        .expect("monarch in HEAD");
    assert!(b.beilagen.contains_key(pferd), "{:?}", b.beilagen.keys());
    assert_ne!(
        b.ordner(&repo.pfad, &cli()),
        q.ordner(&repo.pfad, &cli()),
        "Basis und Arbeitsstand teilen den Cache-Ordner"
    );
}

/// Die Animationen kommen in Datei-Reihenfolge (`erste_animation` bestimmt den Pixel-Anteil).
#[test]
fn animationen_in_datei_reihenfolge() {
    let spec = br#"{"animationen":{"z":{},"a":{"reittier":"pferd"},"m":{"reittier":"pferd"}}}"#;
    assert_eq!(animationen(spec).expect("Namen"), ["z", "a", "m"]);
    assert_eq!(erste_animation(spec).expect("erste"), "z");
    assert_eq!(reittiere(spec), ["pferd"]);
    assert_eq!(erste_animation(b"{}").expect("ohne"), "");
    assert_eq!(erste_animation(br#"{"animationen":null}"#).expect("null"), "");
    assert!(erste_animation(b"{kaputt").is_err());
}
