use std::fs;

use super::*;
use crate::register::load;
use crate::stamp::stamp;
use crate::testutil::{checkout_root, day, suggestion, tempdir, vorrat_repo, write_file};
use crate::vorrat_adopt::vorrat_adopt_if;
use crate::vorrat_download::vorrat_download;

/// B-600/AC-02 (WZ29.7): Übernahme aus dem Chat nur auf dem Stand des Registers; veralteter Stand schreibt nichts.
#[tokio::test]
async fn vorrat_adopt_if_stand() {
    let repo = vorrat_repo().await;
    let root = repo.root();
    vorrat_download(root, &suggestion(root, "v1"), &repo.client, day(), None)
        .await
        .expect("laden");
    let err = vorrat_adopt_if(root, "v1", "alt", day()).expect_err("veralteter Stand");
    assert!(matches!(err, Error::Stale), "{err}");
    let choices = || {
        let reg = load(root).expect("Register");
        reg.find("sfx:coin").expect("Slot").choices().to_vec()
    };
    assert!(choices().is_empty(), "veralteter Stand hat zugeordnet");
    let stand = stamp(root).expect("Stand");
    let (rel, next) = vorrat_adopt_if(root, "v1", &stand, day()).expect("Übernahme");
    assert_ne!(next, stand);
    assert!(
        choices().len() == 1 && choices()[0].file == rel,
        "Zuordnung fehlt: {:?}",
        choices()
    );
    // Noch einmal: keine doppelte Zuordnung, der Stand bleibt.
    let again = vorrat_adopt_if(root, "v1", &next, day()).expect("zweite Übernahme");
    assert_eq!(again, (rel, next));
}

/// B-600/AC-02: Verwerfen setzt den Status in der Liste und schreibt atomar; fremder Slot oder unbekannte ID scheitern.
#[tokio::test]
async fn drop_suggestion() {
    let repo = vorrat_repo().await;
    let root = repo.root();
    assert!(
        super::drop_suggestion(root, "boss", "v1").is_err(),
        "fremder Slot soll scheitern"
    );
    assert!(
        super::drop_suggestion(root, "", "fehlt").is_err(),
        "unbekannte ID soll scheitern"
    );
    super::drop_suggestion(root, "sfx:coin", "v1").expect("verwerfen");
    assert_eq!(suggestion(root, "v1").status, VORRAT_DROPPED);
    let left: Vec<String> = fs::read_dir(root.join("data"))
        .expect("data")
        .map(|e| e.expect("Eintrag").file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(".assets-"))
        .collect();
    assert!(left.is_empty(), "Zwischendateien bleiben: {left:?}");
}

/// Die echte Liste übersteht Laden und Speichern unverändert (Verwerfen ändert nur den Status).
#[test]
fn vorschlagsliste_rundlauf() {
    let want = fs::read_to_string(checkout_root().join(SUGGESTIONS_PATH)).expect("echte Liste");
    let dir = tempdir();
    let root = dir.path();
    write_file(&root.join(SUGGESTIONS_PATH), &want);
    load_suggestions(root).expect("laden").save(root).expect("speichern");
    // Windows-Arbeitsbaum (core.autocrlf) hat CRLF, save schreibt LF wie das Repo; nur der Inhalt zählt.
    let got = fs::read_to_string(root.join(SUGGESTIONS_PATH)).expect("lesen");
    assert!(
        got.replace("\r\n", "\n") == want.replace("\r\n", "\n"),
        "Speichern ändert die Form der Vorschlagsliste"
    );
}

/// Nur Rust: Ein Feld, das beim Zurückschreiben verloren ginge, verhindert das Schreiben.
#[test]
fn speichern_verliert_keine_unbekannten_felder() {
    for text in [
        r#"{"vorschlaege":[{"id":"v1","slot":"farm","notiz":"von Hand"}]}"#,
        r#"{"extra":1,"vorschlaege":[{"id":"v1","slot":"farm"}]}"#,
        r#"{"vorschlaege":[{"id":"v1","slot":"farm","Titel":"groß"}]}"#,
    ] {
        let dir = tempdir();
        let path = dir.path().join(SUGGESTIONS_PATH);
        write_file(&path, text);
        let err = super::drop_suggestion(dir.path(), "", "v1").expect_err(text);
        assert!(err.to_string().contains("unbekanntes Feld"), "{err}");
        assert_eq!(fs::read_to_string(path).expect("lesen"), text);
    }
}
