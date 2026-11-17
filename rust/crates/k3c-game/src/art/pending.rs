//! Platzhalter nach Port-Leitfaden § 7: was `art` aus späteren Sessions braucht.

use std::path::Path;

use super::ArtError;

/// Lädt die Rig-Bibliothek (`paint.LoadAssets`) und liefert ihren Speicher in Byte; kommt mit RU7.6 (`k3c-rig`, RU5).
/// Bis dahin ein Fehler statt `unimplemented!`: `Library::new` mit gesetztem `rig_dir` darf nicht paniken.
pub fn load_rigs(dir: &Path) -> Result<i64, ArtError> {
    Err(ArtError::Missing(format!(
        "Rig-Bibliothek {} kommt erst mit RU7.6",
        dir.display()
    )))
}
