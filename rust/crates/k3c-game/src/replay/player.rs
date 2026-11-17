//! Takt-Steuerung der Wiedergabe: ein Takt je 1/30 s, Takt 0 ist der Startzustand.

use k3c_sim::state::{Event, World};

use super::trace::{Head, Trace};

/// Aufzeichnung mit Leseposition.
pub struct Playback {
    pub head: Head,
    worlds: Vec<World>,
    events: Vec<Vec<Event>>,
    pos: usize,
}

impl Playback {
    pub fn new(t: Trace) -> Self {
        let (mut worlds, mut events) = (Vec::new(), Vec::new());
        for r in t.rows {
            worlds.push(r.zustand);
            events.push(r.ereignisse.unwrap_or_default());
        }
        Self {
            head: t.head,
            worlds,
            events,
            pos: 0,
        }
    }

    /// Anzahl der Zeilen (Startzustand plus Takte).
    pub fn len(&self) -> usize {
        self.worlds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.worlds.is_empty()
    }

    /// Index des aktuellen Takts.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Wahr, sobald der letzte Takt erreicht ist.
    pub fn finished(&self) -> bool {
        self.pos + 1 >= self.worlds.len()
    }

    /// Zustand des aktuellen Takts (leer: `None`).
    pub fn world(&self) -> Option<&World> {
        self.worlds.get(self.pos)
    }

    pub fn world_mut(&mut self) -> Option<&mut World> {
        self.worlds.get_mut(self.pos)
    }

    /// Ereignisse des aktuellen Takts.
    pub fn events(&self) -> &[Event] {
        self.events.get(self.pos).map_or(&[], Vec::as_slice)
    }

    /// Rückt einen Takt weiter; am Ende bleibt die Position. Liefert wahr, wenn sie sich bewegt hat.
    pub fn advance(&mut self) -> bool {
        if self.finished() {
            return false;
        }
        self.pos += 1;
        true
    }
}
