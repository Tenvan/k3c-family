//! Port von `native/k3c-game/game/menu.go`: Menü der Szene im Aufnahme-Modus (`--menu`, `Scene::menu`): nach dem
//! Vorlauf das Hauptmenü, ein Untermenü (`MenuPages`) oder die Pause über dem stehenden Spiel; danach rechnet ein Takt
//! den Frame zum Menü.

use super::{Game, Mode, Scene, Stack};
use k3c_platform::{InputState, log};

impl Game {
    /// Legt das Menü der Szene auf. `main`, `saves` und `lobby` zeigen kein Spiel: Ein Spiel der Szene endet vorher
    /// (`Session::close` speichert, die Spielstand-Liste zeigt es). `pause` und `options` mit Szene liegen über dem
    /// pausierten Spiel.
    pub(super) fn open_menu(&mut self) -> Result<(), String> {
        let sc = self.args.scene.clone();
        if sc.menu.is_empty() {
            return Ok(());
        }
        if sc.in_game() {
            if !self.sim.active() {
                return Err(format!("💥 --menu {}: kein laufendes Spiel", sc.menu));
            }
            self.stack = Stack::new();
            self.sim.pause(true);
            let pause = self.parts.menus.pause();
            self.stack.push(pause);
        } else if self.sim.active() {
            self.sim.close().map_err(|e| e.to_string())?;
            self.stack = Stack::new();
            let main = self.parts.menus.main();
            self.stack.push(main);
        }
        self.mode = Mode::Menu;
        self.open_page(&sc)?;
        log::info(
            "🚦 Aufnahme: Menü offen",
            &[("menu", &sc.menu), ("spiel", &sc.in_game())],
        );
        self.advance(&InputState::new());
        Ok(())
    }

    /// Legt für `saves`, `options`, `lobby`, `keys` und `licenses` das Untermenü auf (`main` und `pause` haben keins).
    fn open_page(&mut self, sc: &Scene) -> Result<(), String> {
        if sc.menu == "main" || sc.menu == "pause" {
            return Ok(());
        }
        let by = sc.devices().first().copied().unwrap_or_default();
        let page = self
            .parts
            .menu_pages
            .as_mut()
            .and_then(|p| p.page(&sc.menu, sc.in_game(), by));
        let Some(page) = page else {
            return Err(format!("💥 --menu {}: das Menü kennt diese Seite nicht", sc.menu));
        };
        self.stack.push(page);
        Ok(())
    }
}
