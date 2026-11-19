//! Test-Hilfen für die Entwickler-Tasten: die Entwickler-Rolle der `FakeSim` (Go: `devSim` aus `devtable_test.go`). Der
//! Test sieht ihre Aufrufe über `Probe::dev`.

use std::cell::RefCell;
use std::rc::Rc;

use crate::view::devkeys::{DevAction, DevSession};

/// Was die Entwickler-Rolle aufgezeichnet hat.
#[derive(Default)]
pub struct DevLog {
    pub halt: bool,
    /// Einzelschritte im Halt
    pub steps: i64,
    pub reloads: i64,
    pub actions: Vec<DevAction>,
    pub players: Vec<i64>,
    /// Fehler, den der nächste `reload` meldet
    pub fail_next: Option<String>,
}

/// Die Entwickler-Rolle, die `FakeSim::with_dev` einhängt.
pub struct DevHandle(pub Rc<RefCell<DevLog>>);

impl DevSession for DevHandle {
    fn dev(&mut self, player: i64, a: &DevAction) -> Result<(), String> {
        let mut l = self.0.borrow_mut();
        l.players.push(player);
        l.actions.push(a.clone());
        Ok(())
    }

    fn halt(&mut self, on: bool) {
        self.0.borrow_mut().halt = on;
    }

    fn halted(&self) -> bool {
        self.0.borrow().halt
    }

    fn step_once(&mut self) {
        self.0.borrow_mut().steps += 1;
    }

    fn reload(&mut self) -> Result<(), String> {
        let mut l = self.0.borrow_mut();
        l.reloads += 1;
        l.fail_next.clone().map_or(Ok(()), Err)
    }
}
