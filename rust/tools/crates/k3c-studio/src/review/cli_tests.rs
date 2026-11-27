use super::super::test_helpers::*;
use super::super::{Fehler, FeldAenderung, auswerten, show};
use super::{Error, Renderer};

/// banner mit fremdem Material wird abgelehnt.
#[test]
fn cli_validate_ungueltig() {
    let Some(repo) = test_repo() else { return };
    let banner = repo.pfad.join("public/grafik/requisiten/banner.paper.json");
    let want = Fehler {
        feld: "teile[0].lagen[0].material".to_owned(),
        regel: "material-unbekannt".to_owned(),
        text: "Material „scharlach“ fehlt in der Stil-Datei".to_owned(),
    };
    assert_eq!(cli().validate(&banner).expect("validate"), [want]);
    let monarch = repo.pfad.join(MONARCH);
    assert_eq!(cli().validate(&monarch).expect("monarch"), []);
}

/// Feld-Diff des Umhangs, kein Teil geändert.
#[test]
fn cli_diff_umhang() {
    let Some(repo) = test_repo() else { return };
    let alt = repo.pfad.join("alt/monarch.paper.json");
    let raw = show(&repo.pfad, "HEAD", MONARCH)
        .expect("show")
        .expect("monarch in HEAD");
    schreibe(&alt, raw);
    let d = cli().diff(&alt, &repo.pfad.join(MONARCH), None).expect("diff");
    let want = FeldAenderung {
        pfad: "umhang.material".to_owned(),
        alt: "koenigsrot".into(),
        neu: "purpur".into(),
    };
    assert_eq!(d.felder, [want]);
    assert!(d.teile.is_empty() && !d.stil, "{d:?}");
}

/// Exit 2 wird ein Fehler mit stderr.
#[test]
fn cli_systemfehler() {
    if !werkzeuge() {
        return;
    }
    let dir = crate::test_helpers::tmp_dir();
    let err = cli()
        .validate(&dir.path().join("fehlt.paper.json"))
        .expect_err("Fehler");
    assert!(matches!(&err, Error::Sonst(t) if t.contains("Spec fehlt")), "{err}");
}

/// Deutung der Exit-Codes ohne Prozess: Exit 1 ohne Antwort ist keine Ablehnung.
#[test]
fn auswerten_nach_exit_code() {
    assert_eq!(auswerten("x", 0, b"{}".to_vec(), "", "").expect("Exit 0"), b"{}");
    let ab = auswerten("x", 1, br#"{"fehler":[{"text":"a"},{"text":"b"}]}"#.to_vec(), "", "");
    assert_eq!(ab.expect_err("abgelehnt").to_string(), "a; b");
    let absturz = auswerten("diff", 1, Vec::new(), " kaputt\n", "exit status: 1");
    assert_eq!(
        absturz.expect_err("Absturz").to_string(),
        "Render-CLI diff (Exit 1): kaputt"
    );
}
