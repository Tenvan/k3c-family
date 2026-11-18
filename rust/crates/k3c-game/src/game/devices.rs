//! Port von `native/k3c-game/game/devices.go`: Geräte der Schleife (B-798): welche Geräte verbunden sind, welche keinen
//! Platz haben, und die Zuordnungs-Regel für verlorene Pads. Ein Pad bekommt nach Wiederverbinden meist eine neue
//! Pad-ID; der Platz hängt deshalb nie an einem festen Index, sondern geht an ein freies Gerät, das neu verbunden wird
//! oder Bestätigen drückt.
//!
//! Abweichung: Go ruft `Input.Of` (kopiert die Aktionen), Rust prüft nur mit `connected`, ob das Gerät verbunden ist;
//! Go-Test `TestDevicesObserveDecorate` zählt Allokationen mit `AllocsPerRun`, der Rust-Test prüft stattdessen, dass die
//! Puffer nach dem ersten Durchlauf nicht mehr wachsen. `Frame::unseated` ist selbst der Puffer (Go: `d.unseat`).

use crate::view::MAX_PLAYERS;
use crate::view::frame::{Cue, CueKind, Frame, Seat};
use crate::view::input::{Action, DeviceID, DeviceInput, Input};

/// Rebind ist eine Neuzuordnung: Spieler `player` übernimmt Gerät `device`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rebind {
    pub player: i64,
    pub device: DeviceID,
}

/// Zuordnungs-Regel: Verwaiste Plätze (Gerät nicht in `input`), nach Spieler aufsteigend, bekommen der Reihe nach je ein
/// freies Gerät (in `input`, ohne Platz), das in diesem Update neu verbunden ist (`fresh`) oder Bestätigen gedrückt hat;
/// Kandidaten in Input-Reihenfolge. Die Neuzuordnungen hängt `rebinds` an `dst`.
pub fn rebinds(dst: &mut Vec<Rebind>, seats: &[Seat], input: &Input, fresh: &[DeviceID]) {
    let mut used = 0; // Zahl der schon vergebenen Kandidaten
    for p in 0..MAX_PLAYERS as i64 {
        if !orphaned(seats, p, input) {
            continue;
        }
        let candidates = input.devices.iter().filter(|di| candidate(di, seats, fresh));
        if let Some(di) = candidates.into_iter().nth(used) {
            dst.push(Rebind {
                player: p,
                device: di.device,
            });
            used += 1;
        }
    }
}

/// connected meldet, ob das Gerät d in der Eingabe vorkommt (ohne die Aktionen zu kopieren).
fn connected(input: &Input, d: DeviceID) -> bool {
    input.devices.iter().any(|di| di.device == d)
}

/// orphaned meldet, ob Spieler p einen Platz hat, dessen Gerät nicht verbunden ist.
fn orphaned(seats: &[Seat], p: i64, input: &Input) -> bool {
    seats
        .iter()
        .find(|s| s.player == p)
        .is_some_and(|s| !connected(input, s.device))
}

/// candidate meldet, ob das Gerät frei ist und neu verbunden wurde oder bestätigt hat.
fn candidate(di: &DeviceInput, seats: &[Seat], fresh: &[DeviceID]) -> bool {
    if seat_of(seats, di.device) >= 0 {
        return false;
    }
    fresh.contains(&di.device) || di.actions.contains(&Action::Confirm)
}

/// seat_of liefert den Spieler mit Gerät d (-1 = keiner).
pub fn seat_of(seats: &[Seat], d: DeviceID) -> i64 {
    seats.iter().find(|s| s.device == d).map_or(-1, |s| s.player)
}

/// Devices merkt sich die verbundenen Geräte über die Updates (Puffer, nach dem ersten Bild ohne Allokation).
#[derive(Debug, Default)]
pub struct Devices {
    started: bool,
    /// verbunden im vorigen Update
    known: Vec<DeviceID>,
    /// in diesem Update neu verbunden
    pub fresh: Vec<DeviceID>,
    pub rebinds: Vec<Rebind>,
}

impl Devices {
    /// observe vergleicht die Geräte in `input` mit dem vorigen Update: `fresh` sind die neu verbundenen; für jedes neue
    /// und jedes getrennte Gerät hängt es einen Cue an `cues` (player = sein Platz in `seats`). Das erste Update meldet
    /// nichts.
    pub fn observe(&mut self, input: &Input, seats: &[Seat], cues: &mut Vec<Cue>) {
        self.fresh.clear();
        for di in &input.devices {
            if self.started && !self.known.contains(&di.device) {
                self.fresh.push(di.device);
                cues.push(device_cue(CueKind::DeviceConnected, seats, di.device));
            }
        }
        for &k in &self.known {
            if !connected(input, k) {
                cues.push(device_cue(CueKind::DeviceLost, seats, k));
            }
        }
        self.known.clear();
        self.known.extend(input.devices.iter().map(|di| di.device));
        self.started = true;
    }

    /// decorate trägt die Geräte in den Frame ein: `unseated` (verbunden ohne Platz) und `Seat::lost`.
    pub fn decorate(&mut self, f: &mut Frame, input: &Input) {
        f.unseated.clear();
        for di in &input.devices {
            if seat_of(&f.seats, di.device) < 0 {
                f.unseated.push(di.device);
            }
        }
        for s in &mut f.seats {
            s.lost = !connected(input, s.device);
        }
    }
}

fn device_cue(kind: CueKind, seats: &[Seat], d: DeviceID) -> Cue {
    Cue {
        kind,
        player: seat_of(seats, d),
        device: d,
        ..Cue::default()
    }
}

#[cfg(test)]
#[path = "devices_tests.rs"]
mod devices_tests;
