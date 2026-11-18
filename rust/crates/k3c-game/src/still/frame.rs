//! Frame des Standbilds für das HUD (RU20.7): Dateiformat `hud-<name>.frame.json` (geschrieben von
//! `native/k3c-game/hud/rustref_test.go`). Es trägt, was `Hud::listen` aus dem Frame liest und der Zustand nicht hat:
//! Sitze mit Gerät, Sprache, Eingabe-Profile, Takt, Meldungen und freie Geräte.

use serde::Deserialize;

use super::state::StillState;
use crate::view::MAX_PLAYERS;
use crate::view::frame::{Cue, CueKind, Seat};
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::settings::Settings;

#[derive(Deserialize)]
struct DeviceFile {
    /// 0 Tastatur, 1 Gamepad (`view.DeviceKind` in Go)
    kind: i64,
    index: i64,
}

impl DeviceFile {
    fn id(&self) -> DeviceID {
        let kind = if self.kind == 1 {
            DeviceKind::Gamepad
        } else {
            DeviceKind::Keyboard
        };
        DeviceID::new(kind, self.index)
    }
}

#[derive(Deserialize)]
struct SeatFile {
    player: i64,
    monarch: i64,
    stage: i64,
    device: DeviceFile,
}

#[derive(Deserialize)]
struct CueFile {
    /// nur `denied` (Meldung an den Spieler)
    kind: String,
    player: i64,
    text: String,
}

/// Inhalt der Frame-Datei.
#[derive(Deserialize)]
pub struct FrameFile {
    clock: f64,
    language: String,
    #[serde(rename = "guideOff", default)]
    guide_off: bool,
    #[serde(rename = "guideSeen", default)]
    guide_seen: i64,
    controls: Vec<String>,
    seats: Vec<SeatFile>,
    #[serde(default)]
    unseated: Vec<DeviceFile>,
    #[serde(default)]
    cues: Vec<CueFile>,
}

impl FrameFile {
    /// Liest die Frame-Datei aus JSON.
    pub fn parse(raw: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(raw).map_err(|e| format!("Frame: {e}"))
    }

    /// Liest die Datei `path`.
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let raw = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Self::parse(&raw).map_err(|e| format!("{}: {e}", path.display()))
    }
}

/// Was die Frame-Datei zum Zustand beiträgt (Einstellungen, freie Geräte, Meldungen).
#[derive(Clone, Default)]
pub struct FrameExtras {
    pub settings: Settings,
    pub unseated: Vec<DeviceID>,
    pub cues: Vec<Cue>,
}

impl StillState {
    /// Setzt Sitze (mit Gerät), Takt und die Zusätze des Frames aus `f`; die Sitze müssen zu den Kameras passen.
    pub fn with_frame(mut self, f: FrameFile) -> Result<Self, String> {
        if f.seats.len() != self.seats.len() {
            return Err(format!(
                "Frame: {} Sitze, der Zustand hat {}",
                f.seats.len(),
                self.seats.len()
            ));
        }
        let mut controls: [String; MAX_PLAYERS] = Default::default();
        for (c, v) in controls.iter_mut().zip(f.controls) {
            *c = v;
        }
        self.seats = f
            .seats
            .iter()
            .map(|s| Seat {
                player: s.player,
                device: s.device.id(),
                monarch: s.monarch,
                stage: s.stage,
                lost: false,
            })
            .collect();
        self.clock = f.clock;
        self.frame_extras = Some(FrameExtras {
            settings: Settings {
                language: f.language,
                guide_off: f.guide_off,
                guide_seen: f.guide_seen,
                controls,
                ..Settings::default()
            },
            unseated: f.unseated.iter().map(DeviceFile::id).collect(),
            cues: f
                .cues
                .into_iter()
                .filter(|c| c.kind == "denied")
                .map(|c| Cue {
                    kind: CueKind::Denied,
                    player: c.player,
                    text: c.text,
                    device: DeviceID::default(),
                })
                .collect(),
        });
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_lesen() {
        let f = FrameFile::parse(
            br#"{"clock":1.5,"language":"en","controls":["pfeile","gamepad"],"seats":[{"player":0,"monarch":0,"stage":0,
            "device":{"kind":1,"index":3}}],"unseated":[{"kind":0,"index":0}],"cues":[{"kind":"denied","player":-1,"text":"x"}]}"#,
        )
        .unwrap();
        assert_eq!(f.seats[0].device.id(), DeviceID::new(DeviceKind::Gamepad, 3));
        assert_eq!(f.unseated.len(), 1);
        assert!(FrameFile::parse(b"[]").is_err());
    }
}
