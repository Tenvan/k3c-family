//! Port von `Game.advance`, `tick` und `step` aus `native/k3c-game/game/game.go`: ein Takt der Schleife.

use std::mem::take;

use k3c_platform::InputState;

use super::{Game, layout};
use crate::view::input::Input;
use crate::view::screen::Step;

impl Game {
    /// Stellt die Tastatur auf das Eingabe-Profil, liest die Eingabe, rechnet den Takt (`tick`: Vollbild, Menü bzw.
    /// Spiel, Simulation; während der Schiffsfahrt `sail`) und bereitet den Frame vor: Geräte, Grafik der Level,
    /// Kameras, Vollbild, dann die Listener. Die Reihenfolge ist die von Go.
    pub(super) fn advance(&mut self, raw: &InputState) {
        if self.fault.is_some() {
            return; // Fehler im Fenster: nichts mehr rechnen
        }
        if self.scripted.is_none()
            && let Some(k) = self.parts.keyboard.as_mut()
        {
            // Belegung der Tastatur nach Eingabe-Profil (B-893)
            k.set_keyboard(self.frame.settings.keyboard_profile());
            k.set_menu(self.stack.top().is_some());
        }
        let input = self.poll(raw);
        self.devs.observe(&input, &self.frame.seats, &mut self.cues);
        if self.voy.on {
            self.sail(&input); // Schiffsfahrt: keine Spiel-Eingabe, kein Takt der Simulation
        } else {
            self.tick(&input);
        }
        self.frame.dev = self.dev;
        self.devs.decorate(&mut self.frame, &input);
        if !self.cues.is_empty() {
            self.frame.cues.append(&mut self.cues);
        }
        if self.parts.art.is_some() {
            self.sync_levels();
        }
        self.frame.loading = self.loading; // Ladeanzeige statt Spiel: Einblendungen der Geschichte warten (B-974)
        self.place_cells();
        self.frame.recording = !self.args.shots.is_empty();
        self.apply_window();
        for l in &mut self.parts.listeners {
            l.listen(&self.frame);
        }
        self.arrive();
    }

    /// Baut die Zellen des Frames neu, setzt die Kameras und trägt die Zellen in den Frame ein. Der Puffer wandert
    /// zwischen `self.cells` und `frame.cells` hin und her (nach dem ersten Takt ohne Allokation).
    fn place_cells(&mut self) {
        let mut buf = take(&mut self.cells);
        if buf.capacity() == 0 {
            buf = take(&mut self.frame.cells); // Schiffsfahrt: kein Takt, die Zellen liegen noch im Frame
        }
        layout::cells(&mut buf, self.win, &self.frame);
        self.cams.follow(&mut buf, &self.frame);
        self.frame.cells = buf;
    }

    /// Bedient Vollbild, Entwickler-Tasten und Menü bzw. Spiel, taktet die Simulation einmal und beginnt die
    /// Schiffsfahrt, wenn der Takt den Inselwechsel meldet.
    fn tick(&mut self, input: &Input) {
        self.toggle_window(input);
        self.menu_or_play(input);
        self.blend.capture(&self.frame, &self.frame.cells);
        self.voy.note(&self.frame);
        self.cells = take(&mut self.frame.cells); // der Puffer überlebt den neuen Frame
        self.frame = self.sim.step();
        if self.stress.is_some() {
            self.stress_step();
        }
        self.dev_values();
        self.embark();
    }

    /// Der oberste Screen bekommt die Eingabe, sonst läuft das Spiel (`play`).
    fn menu_or_play(&mut self, input: &Input) {
        let Some(mut top) = self.stack.take_top() else {
            self.play(input);
            return;
        };
        // Bis RU30.5 ist die Session des Menüs die Simulation selbst (Go: `menuSession{Sim, g}`).
        let st = top.update(input, self.sim.as_mut());
        self.stack.put_top(top);
        self.apply_step(st);
    }

    /// Wendet das Ergebnis des obersten Screens an.
    fn apply_step(&mut self, mut st: Step) {
        self.cues.append(&mut st.cues);
        let was = self.mode;
        let active = self.sim.active();
        let menus = &mut self.parts.menus;
        self.mode = self.stack.apply(st, active, &mut || menus.main());
        if was == super::Mode::Menu && self.mode == super::Mode::Play {
            self.sim.pause(false);
            if let Some(ds) = self.sim.dev_session() {
                ds.halt(false); // ein vergessener Halt (F7) ließ das Spiel nach dem Fortsetzen unsichtbar stehen (B-918)
            }
        }
    }
}
