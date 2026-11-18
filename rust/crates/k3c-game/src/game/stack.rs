//! Port von `native/k3c-game/game/stack.go`: Zustand der Schleife und Menü-Stapel (reine Logik, ohne Zeichnen).

use crate::view::screen::{Screen, Step, StepKind};

/// Mode ist der Zustand der Schleife.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// ein Screen liegt oben, kein Spiel oder Spiel pausiert
    Menu,
    /// Spiel läuft, kein Screen
    Play,
    /// Programm beenden
    Quit,
}

/// Stack ist der Menü-Stapel; oben liegt das letzte Element.
#[derive(Default)]
pub struct Stack {
    screens: Vec<Box<dyn Screen>>,
}

impl Stack {
    pub fn new() -> Self {
        Self::default()
    }

    /// Top liefert den obersten Screen (`None` bei leerem Stapel).
    pub fn top(&self) -> Option<&dyn Screen> {
        self.screens.last().map(|s| s.as_ref())
    }

    /// Wie `top`, veränderbar (für `Screen::update`).
    pub fn top_mut(&mut self) -> Option<&mut Box<dyn Screen>> {
        self.screens.last_mut()
    }

    /// Zahl der Screens.
    pub fn len(&self) -> usize {
        self.screens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.screens.is_empty()
    }

    /// Legt `sc` oben auf.
    pub fn push(&mut self, sc: Box<dyn Screen>) {
        self.screens.push(sc);
    }

    /// Wendet das Ergebnis eines `Screen::update` an und liefert den neuen Zustand. `active`: es läuft ein Spiel.
    /// Ein leerer Stapel ohne Spiel öffnet `main` (Hauptmenü), damit nie ein leeres Bild ohne Menü bleibt.
    pub fn apply(&mut self, st: Step, active: bool, main: &mut dyn FnMut() -> Box<dyn Screen>) -> Mode {
        match st.kind {
            StepKind::Push => {
                if let Some(next) = st.next {
                    self.push(next);
                }
            }
            StepKind::Replace => {
                if let (Some(next), Some(top)) = (st.next, self.screens.last_mut()) {
                    *top = next;
                }
            }
            StepKind::Pop => {
                self.screens.pop();
            }
            StepKind::Play => self.screens.clear(),
            StepKind::Quit => return Mode::Quit,
            StepKind::Stay => {}
        }
        if !self.screens.is_empty() {
            return Mode::Menu;
        }
        if active {
            return Mode::Play;
        }
        self.push(main());
        Mode::Menu
    }
}

#[cfg(test)]
#[path = "stack_tests.rs"]
mod stack_tests;
