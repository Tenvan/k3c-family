//! Port von `winicon_windows_test.go`. Go erzeugt das PNG zur Laufzeit; `k3c-kit` hat kein PNG-Crate, darum liest der
//! Test das Icon der Workbench (RU71 zieht den Pfad nach, falls die Datei wandert).

use super::{handle, set};

const ICON: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../tools/k3c-workbench/build/appicon.png"
);

/// Ein PNG wird zu einem gültigen Icon-Handle; ohne PNG gibt es keins.
#[test]
fn handle_aus_png() {
    let png = std::fs::read(ICON).expect("Icon der Workbench lesen");
    for px in [16, 32] {
        assert_ne!(handle(&png, px), 0, "kein Icon der Größe {px} aus dem PNG");
    }
    assert_eq!(handle(&[], 32), 0, "Icon aus leerem PNG");
    assert_eq!(handle(b"kein PNG, aber lang genug fuer die Mindestgroesse", 32), 0);
    assert_eq!(handle(&png[..8], 32), 0, "Icon aus bloßer Signatur");
    // Ohne PNG kehrt `set` sofort zurück, ohne Fenster zu suchen.
    set(&[]);
}
