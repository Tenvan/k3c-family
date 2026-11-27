use std::io;
use std::path::Path;

use k3c_kit::logger::Logger;

use super::{ERROR_SHARING_VIOLATION, rename_replace, rename_replace_with};
use crate::test_helpers::tmp_dir;

#[test]
fn rename_replace_ersetzt() {
    let dir = tmp_dir();
    let (a, b) = (dir.path().join("a"), dir.path().join("b"));
    std::fs::write(&a, "neu").unwrap();
    std::fs::write(&b, "alt").unwrap();
    rename_replace(&a, &b, &Logger::discard()).unwrap();
    assert_eq!(std::fs::read_to_string(&b).unwrap(), "neu");
}

/// Unter Windows wird eine kurz gesperrte Datei wiederholt, andere Fehler nicht.
#[test]
fn rename_replace_wiederholt() {
    let (a, b) = (Path::new("a"), Path::new("b"));
    let log = Logger::discard();
    let mut n = 0;
    let result = rename_replace_with(a, b, &log, |_, _| {
        n += 1;
        if n < 3 {
            return Err(io::Error::from_raw_os_error(ERROR_SHARING_VIOLATION));
        }
        Ok(())
    });
    if cfg!(windows) {
        assert!(result.is_ok() && n == 3, "windows: {result:?}, {n} Versuche");
    } else {
        assert!(result.is_err() && n == 1, "sonst: {result:?}, {n} Versuche");
    }
    n = 0;
    let result = rename_replace_with(a, b, &log, |_, _| {
        n += 1;
        Err(io::Error::other("kaputt"))
    });
    assert_eq!(result.unwrap_err().to_string(), "kaputt");
    assert_eq!(n, 1, "anderer Fehler");
}
