//! Lesende Teile des früheren Asset-Studios im Bereich Assets (Port von `app_studio.go`): Sound, Quellen und „Im Spiel
//! ansehen“ über `studioview`. Ändern bleibt Sache der MCP-Tools `assets_*`. `StudioWeltSicht` liegt in `assets.rs`.

use std::sync::Arc;

use super::{App, lock};
use crate::studioview::{self, Game, Quellen, SoundEintrag};

#[cfg(test)]
#[path = "studio_tests.rs"]
mod tests;

impl App {
    /// Die Sound-Slots mit Status und Dateien (Methode `StudioSounds`).
    pub fn studio_sounds(&self) -> Result<Vec<SoundEintrag>, String> {
        self.wait();
        studioview::sounds(&self.root)
    }

    /// Eine Audiodatei unter `public/` als Data-URL zum Anhören (Methode `StudioSoundData`).
    pub fn studio_sound_data(&self, path: &str) -> Result<String, String> {
        self.wait();
        studioview::sound_data(&self.root, path)
    }

    /// Katalog und Recherche-Treffer (Methode `StudioSources`).
    pub fn studio_sources(&self) -> Result<Quellen, String> {
        self.wait();
        studioview::sources(&self.root)
    }

    /// Legt den Testspielstand `voll-<seed>` an und startet das native Spiel mit ihm; die Antwort sagt, was gestartet ist
    /// (Methode `StudioGameOpen`, B-847). Blockiert.
    pub fn studio_game_open(&self, seed: &str) -> Result<String, String> {
        self.wait();
        self.studio_game().open(seed)
    }

    /// „Im Spiel ansehen“, beim ersten Aufruf angelegt.
    fn studio_game(&self) -> Arc<Game> {
        // Vor der Sperre: `logger` nimmt eine eigene.
        let log = self.logger();
        let mut game = lock(&self.game);
        Arc::clone(game.get_or_insert_with(|| Arc::new(Game::new(&self.root, None, log))))
    }

    /// Beendet „Im Spiel ansehen“: danach startet es nichts mehr, ein laufendes Spiel bleibt offen.
    pub(super) fn close_studio(&self) {
        self.studio_game().close();
    }
}
