use super::*;
use crate::testutil::tempdir;

// Review RU67.2: Eine Windows-1252-Datei bleibt Byte für Byte, statt Umlaute zu verlieren.
#[test]
fn ungueltiges_utf8_wird_nicht_ueberschrieben() {
    let dir = tempdir();
    let file = credit_path(dir.path(), KIND_SOUND);
    fs::create_dir_all(file.parent().expect("Ordner")).expect("anlegen");
    let before = b"| a.ogg | M\xfcller |\n".to_vec();
    fs::write(&file, &before).expect("schreiben");
    let err = set_credit_status(dir.path(), KIND_SOUND, "a.ogg", "ok").expect_err("abgelehnt");
    assert!(err.to_string().contains("UTF-8"), "{err}");
    assert_eq!(fs::read(&file).expect("lesen"), before);
}
