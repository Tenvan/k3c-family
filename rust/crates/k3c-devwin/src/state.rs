//! Zustand des Fensters (ohne Win32, überall testbar): Verbindungsstand, Katalog, letzter Spielzustand, Befehle bis
//! zum `ack`, Text der Statuszeile.

use std::collections::BTreeMap;

use k3c_devproto::{Catalog, GameInfo, Msg, State};

/// Höchstzahl offener Befehle (ältere fallen weg, falls das Spiel nie quittiert).
const MAX_OPEN: usize = 256;

/// Conn ist der Verbindungsstand zum Spiel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Conn {
    /// Noch nie verbunden.
    Waiting,
    /// Das Spiel hat `hello` gesendet.
    Connected,
    /// Das Spiel war verbunden und ist weg.
    Disconnected,
}

/// Change sagt, was sich durch eine Nachricht geändert hat (für das Neuzeichnen).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Change {
    pub catalog: bool,
    pub status: bool,
}

/// LastAck ist die letzte Antwort des Spiels auf einen Befehl.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LastAck {
    pub label: String,
    pub ok: bool,
    pub err: String,
}

/// Model hält den Zustand des Fensters.
#[derive(Debug)]
pub struct Model {
    pub conn: Conn,
    pub game: Option<GameInfo>,
    pub catalog: Catalog,
    pub state: Option<State>,
    pub last_ack: Option<LastAck>,
    /// Nicht behebbarer Fehler (z. B. Pipe-Name belegt), ersetzt die Statuszeile.
    pub fatal: Option<String>,
    next_id: u64,
    open: BTreeMap<u64, String>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            conn: Conn::Waiting,
            game: None,
            catalog: Catalog::default(),
            state: None,
            last_ack: None,
            fatal: None,
            next_id: 1,
            open: BTreeMap::new(),
        }
    }
}

impl Model {
    /// enabled ist wahr, solange das Spiel verbunden ist (sonst sind alle Controls ausgegraut).
    pub fn enabled(&self) -> bool {
        self.conn == Conn::Connected && self.fatal.is_none()
    }

    /// open_count ist die Zahl der Befehle ohne `ack`.
    pub fn open_count(&self) -> usize {
        self.open.len()
    }

    /// next_command vergibt die Kennung für einen neuen Befehl und merkt ihn bis zum `ack`.
    pub fn next_command(&mut self, label: &str) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        if self.open.len() >= MAX_OPEN {
            self.open.pop_first();
        }
        self.open.insert(id, label.to_owned());
        id
    }

    /// on_msg nimmt eine Nachricht des Spiels auf.
    pub fn on_msg(&mut self, msg: Msg) -> Change {
        match msg {
            Msg::Hello { game, .. } => {
                self.conn = Conn::Connected;
                self.game = game;
                self.open.clear();
                self.last_ack = None;
                Change {
                    catalog: false,
                    status: true,
                }
            }
            Msg::Catalog(c) => {
                self.catalog = c;
                Change {
                    catalog: true,
                    status: false,
                }
            }
            Msg::State(s) => {
                self.state = Some(s);
                Change {
                    catalog: false,
                    status: true,
                }
            }
            Msg::Ack { id, ok, err } => {
                let label = self.open.remove(&id).unwrap_or_default();
                self.last_ack = Some(LastAck {
                    label,
                    ok,
                    err: err.unwrap_or_default(),
                });
                Change {
                    catalog: false,
                    status: true,
                }
            }
            Msg::Bye => self.on_lost(),
            _ => Change::default(),
        }
    }

    /// on_lost verarbeitet das Ende der Verbindung.
    pub fn on_lost(&mut self) -> Change {
        self.conn = Conn::Disconnected;
        self.state = None;
        self.open.clear();
        Change {
            catalog: false,
            status: true,
        }
    }

    /// status_is_error ist wahr bei einem Fehler (nicht behebbar oder abgelehnter Befehl).
    pub fn status_is_error(&self) -> bool {
        self.fatal.is_some() || (self.enabled() && self.last_ack.as_ref().is_some_and(|a| !a.ok))
    }

    /// status_text ist der Text der Statuszeile.
    pub fn status_text(&self) -> String {
        if let Some(f) = &self.fatal {
            return f.clone();
        }
        match self.conn {
            Conn::Waiting => "Wartet auf das Spiel …".to_owned(),
            Conn::Disconnected => "Getrennt – wartet auf das Spiel …".to_owned(),
            Conn::Connected => self.connected_text(),
        }
    }

    fn connected_text(&self) -> String {
        let mut t = "Verbunden".to_owned();
        if let Some(g) = self.game.as_ref().filter(|g| !g.name.is_empty()) {
            t.push_str(&format!(" · {} {}", g.name, g.version));
        }
        if let Some(s) = &self.state {
            t.push_str(&format!(" · {} {}.{} {}", s.world, s.island, s.stage, s.phase));
            if s.timescale > 1 {
                t.push_str(&format!(" {}×", s.timescale));
            }
        }
        match &self.last_ack {
            Some(a) if a.ok => t.push_str(&format!(" · ✓ {}", a.label)),
            Some(a) => t.push_str(&format!(" · ✗ {}: {}", a.label, a.err)),
            None => {}
        }
        t
    }
}

/// set_id setzt die Befehlskennung einer Nachricht des Fensters (andere Nachrichten bleiben unverändert).
pub fn set_id(msg: &mut Msg, new_id: u64) {
    match msg {
        Msg::Dev { id, .. } | Msg::Ui { id, .. } | Msg::Goto { id, .. } | Msg::Focus { id } => *id = new_id,
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hello() -> Msg {
        Msg::Hello {
            v: 1,
            game: Some(GameInfo {
                name: "k3c".into(),
                version: "0.1".into(),
            }),
        }
    }

    #[test]
    fn verbindungsstand_und_sperre() {
        let mut m = Model::default();
        assert_eq!(m.conn, Conn::Waiting);
        assert!(!m.enabled());
        assert!(m.status_text().starts_with("Wartet"));
        assert!(m.on_msg(hello()).status);
        assert!(m.enabled());
        assert!(m.status_text().starts_with("Verbunden · k3c 0.1"));
        assert!(m.on_lost().status);
        assert_eq!(m.conn, Conn::Disconnected);
        assert!(!m.enabled());
        assert!(m.status_text().starts_with("Getrennt"));
        m.on_msg(hello());
        assert!(m.enabled(), "Wiederverbindung");
        m.on_msg(Msg::Bye);
        assert_eq!(m.conn, Conn::Disconnected);
    }

    #[test]
    fn befehls_id_und_ack() {
        let mut m = Model::default();
        m.on_msg(hello());
        let (a, b) = (m.next_command("Nacht"), m.next_command("Boss"));
        assert_eq!((a, b), (1, 2));
        assert_eq!(m.open_count(), 2);
        m.on_msg(Msg::Ack {
            id: a,
            ok: true,
            err: None,
        });
        assert_eq!(m.open_count(), 1);
        assert!(m.status_text().ends_with("✓ Nacht"));
        assert!(!m.status_is_error());
        m.on_msg(Msg::Ack {
            id: b,
            ok: false,
            err: Some("kein Boss".into()),
        });
        assert_eq!(m.open_count(), 0);
        assert!(m.status_text().ends_with("✗ Boss: kein Boss"));
        assert!(m.status_is_error());
        m.on_msg(Msg::Ack {
            id: 99,
            ok: true,
            err: None,
        });
    }

    #[test]
    fn katalog_und_zustand() {
        let mut m = Model::default();
        m.on_msg(hello());
        let c = Catalog {
            phases: vec!["night".into()],
            ..Catalog::default()
        };
        assert!(m.on_msg(Msg::Catalog(c.clone())).catalog);
        assert_eq!(m.catalog, c);
        let s = State {
            world: "w".into(),
            island: 1,
            stage: 2,
            phase: "night".into(),
            timescale: 4,
            ..State::default()
        };
        m.on_msg(Msg::State(s));
        assert!(m.status_text().contains("w 1.2 night 4×"));
        m.on_lost();
        assert!(m.state.is_none() && m.catalog == c, "Katalog bleibt, Zustand fällt weg");
    }

    #[test]
    fn fatal_ersetzt_status_und_sperrt() {
        let mut m = Model::default();
        m.on_msg(hello());
        m.fatal = Some("Pipe belegt".into());
        assert!(!m.enabled() && m.status_is_error());
        assert_eq!(m.status_text(), "Pipe belegt");
    }

    #[test]
    fn set_id_nur_fuer_befehle() {
        let mut d = Msg::Focus { id: 0 };
        set_id(&mut d, 7);
        assert_eq!(d, Msg::Focus { id: 7 });
        let mut b = Msg::Bye;
        set_id(&mut b, 7);
        assert_eq!(b, Msg::Bye);
    }
}
