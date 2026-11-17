//! Schleifen-Kern: Uhr, Eingabe, Takte und Zeichnen. Treibt das Fenster und headless gleich; im Test läuft er
//! mit einer simulierten Uhr.

use crate::{App, Canvas, Clock, InputScript, InputState, STEP};

/// Gemeinsamer Kern von Fenster und headless.
pub struct Runner<'a> {
    app: &'a mut dyn App,
    script: Option<&'a InputScript>,
    clock: Clock,
    input: InputState,
    steps: u64,
}

impl<'a> Runner<'a> {
    pub fn new(app: &'a mut dyn App, script: Option<&'a InputScript>) -> Self {
        Self {
            app,
            script,
            clock: Clock::new(),
            input: InputState::new(),
            steps: 0,
        }
    }

    /// Eingabe, die das Backend zwischen zwei Bildern füllt.
    pub fn input_mut(&mut self) -> &mut InputState {
        &mut self.input
    }

    pub fn app(&self) -> &dyn App {
        self.app
    }

    /// Ausgeführte Takte seit dem Start.
    pub fn steps(&self) -> u64 {
        self.steps
    }

    /// Nimmt `elapsed` Sekunden auf, rechnet die fälligen Takte (je Takt Skript, `step`, Flanken löschen) und
    /// zeichnet ein Bild mit alpha. Liefert die Zahl der Takte.
    pub fn advance(&mut self, elapsed: f64, c: &mut dyn Canvas) -> u32 {
        let (n, alpha) = self.clock.advance(elapsed);
        let mut done = 0;
        for _ in 0..n {
            if self.app.done() {
                break;
            }
            if let Some(s) = self.script {
                s.apply(self.steps, &mut self.input);
            }
            self.app.step(&self.input, STEP);
            self.input.end_step();
            self.steps += 1;
            done += 1;
        }
        self.app.draw(c, alpha);
        done
    }
}

#[cfg(test)]
#[path = "runner_tests.rs"]
mod tests;
