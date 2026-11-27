//! Ein Frame des Arbeitsstands ohne die Lagen einzelner Teile (B-734: Teil in der Vorschau ausblenden).

use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use super::{Dienst, Frame, RenderPlatz, animation, data_url, pruefe_frame};

#[cfg(test)]
#[path = "frameohne_tests.rs"]
mod tests;

/// Teilname wie `^[a-z0-9][a-z0-9-]*$`: nie ein Schalter und nie ein Pfad für die CLI.
fn gueltiger_teil(name: &str) -> bool {
    let zeichen = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit();
    name.bytes().next().is_some_and(zeichen) && name.bytes().all(|b| zeichen(b) || b == b'-')
}

/// Eigener Ordner unter `<root>/.work/tmp` (Regel „Schreiben nur im Repo“); verschwindet beim Fallenlassen.
struct TempOrdner(PathBuf);

impl TempOrdner {
    fn neu(root: &Path) -> io::Result<TempOrdner> {
        static ZAEHLER: AtomicU64 = AtomicU64::new(0);
        let basis = root.join(".work").join("tmp");
        std::fs::create_dir_all(&basis)?;
        loop {
            let zeit = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |d| d.subsec_nanos());
            let nr = ZAEHLER.fetch_add(1, Ordering::Relaxed);
            let pfad = basis.join(format!("k3c-ohne-{}-{zeit}-{nr}", std::process::id()));
            match std::fs::create_dir(&pfad) {
                Ok(()) => return Ok(TempOrdner(pfad)),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(e),
            }
        }
    }
}

impl Drop for TempOrdner {
    fn drop(&mut self) {
        // Bleibt etwas liegen, räumt `task ops:aufraeumen` `.work` auf.
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

impl Dienst {
    /// Ein Frame des Arbeitsstands, in dem die Teile `ohne` ausgeblendet sind (Data-URL mit Teil-Boxen). Ohne Cache auf
    /// der Platte: die Oberfläche merkt sich die Bilder selbst.
    pub fn assets_frame_ohne(&self, id: &str, anim: &str, frame: i64, ohne: &[String]) -> Result<Frame, String> {
        if let Some(teil) = ohne.iter().find(|t| !gueltiger_teil(t)) {
            return Err(format!("Teil „{teil}“ ungültig"));
        }
        let q = self.quelle(id, "")?;
        let anim = animation(&q, anim)?;
        pruefe_frame(&q, &anim, frame)?;
        let spec = q.spec_datei(&self.root, self.renderer()).map_err(|e| e.to_string())?;
        let ordner = TempOrdner::neu(&self.root).map_err(|e| e.to_string())?;
        let out = ordner.0.join("frame.png");
        let platz = RenderPlatz::belegen();
        let antwort = self.r.frame_ohne(&spec, &anim, frame, ohne, &out);
        drop(platz);
        let antwort = antwort.map_err(|e| e.to_string())?;
        Ok(Frame {
            bild: data_url(&out)?,
            teile: antwort.teile,
        })
    }
}
