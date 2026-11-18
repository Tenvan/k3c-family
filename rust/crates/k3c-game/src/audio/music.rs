//! Port von `native/k3c-game/audio/music.go`: Musik-Zustände, Übergangstabelle und Überblendung.

/// Takt von Listen (game.TPS, 30 Hz; `game` ist nach der Importregel nicht erreichbar).
pub const TPS: i64 = 30;

/// Ein Musikzustand wie src/audio/music.ts › MUSIC_STATES; `None` = noch keiner bzw. Lücke. Die Reihenfolge folgt
/// den `iota`-Konstanten von Go und indiziert `MUSIC_SLOTS`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MusicState {
    #[default]
    None,
    Day,
    Dusk,
    Night,
    Battle,
    Depth,
    Boss,
    Lobby,
    Defeat,
    Ending,
}

/// Bildet jeden Zustand (Index = `MusicState as usize`) auf genau einen Musik-Slot ab (music.ts › MUSIC_SLOTS).
pub const MUSIC_SLOTS: [&str; 10] = [
    "",
    "music:tag",
    "music:abend",
    "music:nacht",
    "music:kampf",
    "music:tiefe",
    "music:boss",
    "music:lobby",
    "music:niederlage",
    "music:ende",
];

impl MusicState {
    /// Der Musik-Slot des Zustands ("" für `None`).
    pub fn slot(self) -> &'static str {
        MUSIC_SLOTS[self as usize]
    }
}

/// Auszug, den der Automat braucht (music.ts › MusicView). `boss` und `victory` setzt der Browser (`music_view_of`)
/// bisher nie; der Automat kennt sie, damit die Priorität vollständig bleibt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MusicView<'a> {
    pub lobby: bool,
    pub phase: &'a str,
    pub enemies: i64,
    pub depth: i64,
    pub boss: bool,
    pub victory: bool,
}

/// Zustands-Automat wie music.ts › nextMusicState. Priorität: Lobby > Niederlage > Sieg > Boss > Kampf > Tiefe >
/// Tageszeit; Niederlage (`castle_fallen`) und Sieg bleiben, bis die Lobby wieder läuft. Morgengrauen hat kein
/// eigenes Stück, die Nachtmusik läuft weiter. Eine unbekannte Phase ist eine Lücke (Stille).
pub fn next_music_state(prev: MusicState, v: &MusicView<'_>, castle_fallen: bool) -> MusicState {
    if v.lobby {
        return MusicState::Lobby;
    }
    if prev == MusicState::Defeat || castle_fallen {
        return MusicState::Defeat;
    }
    if prev == MusicState::Ending || v.victory {
        return MusicState::Ending;
    }
    if v.boss {
        return MusicState::Boss;
    }
    if v.enemies > 0 {
        return MusicState::Battle;
    }
    if v.depth > 0 {
        return MusicState::Depth;
    }
    match v.phase {
        "day" => MusicState::Day,
        "dusk" => MusicState::Dusk,
        "night" | "dawn" => MusicState::Night,
        _ => MusicState::None,
    }
}

/// Dauer eines Stückwechsels in Takten: 2 s (crossfade.ts › CROSSFADE_SECONDS) bei 30 Hz.
pub const CROSSFADE_TICKS: i64 = 2 * TPS;

/// Ein Musikstück mit seinem Pegel 0 … 1 (vor Bus-Lautstärke); Slot "" = keins.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MusicVoice {
    pub slot: &'static str,
    pub gain: f64,
}

/// Blendet die Musik über Takte über (crossfade.ts: gleiche Leistung, sin hinein, cos hinaus). Ein Wechsel mitten im
/// Übergang blendet das laufende Stück vom aktuellen Pegel aus (`out_from`); kehrt das ausblendende Stück zurück,
/// steigt es ohne Sprung vom aktuellen Pegel wieder an. Slots sind `'static` (aus `MUSIC_SLOTS`), so legt kein Takt
/// Zeichenketten an.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fader {
    pub in_slot: &'static str,
    pub out_slot: &'static str,
    in_step: i64,
    out_step: i64,
    out_from: f64,
}

#[allow(clippy::cast_precision_loss)]
fn ratio(step: i64) -> f64 {
    step.min(CROSSFADE_TICKS) as f64 / CROSSFADE_TICKS as f64
}

impl Fader {
    /// Pegel des einblendenden Stücks.
    pub fn in_level(&self) -> f64 {
        if self.in_slot.is_empty() {
            return 0.0;
        }
        (std::f64::consts::FRAC_PI_2 * ratio(self.in_step)).sin()
    }

    /// Pegel des ausblendenden Stücks.
    pub fn out_level(&self) -> f64 {
        if self.out_slot.is_empty() {
            return 0.0;
        }
        self.out_from * (std::f64::consts::FRAC_PI_2 * ratio(self.out_step)).cos()
    }

    /// Setzt das gewünschte Stück ("" = Stille). Dasselbe Stück läuft einfach weiter.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn target(&mut self, slot: &'static str) {
        if slot == self.in_slot {
            return;
        }
        let back = !slot.is_empty() && slot == self.out_slot;
        let back_level = self.out_level();
        if !self.in_slot.is_empty() {
            self.out_slot = self.in_slot;
            self.out_from = self.in_level();
            self.out_step = 0;
        } else if back {
            self.out_slot = "";
        }
        self.in_slot = slot;
        self.in_step = 0;
        if back {
            // Rückkehr: dort weitermachen, wo der Pegel gerade steht (asin, gleiche Kurve)
            self.in_step =
                (back_level.min(1.0).asin() * 2.0 / std::f64::consts::PI * CROSSFADE_TICKS as f64).round() as i64;
        }
    }

    /// Rückt den Übergang um einen Takt vor; ein ausgeblendetes Stück fällt weg.
    pub fn step(&mut self) {
        if !self.in_slot.is_empty() && self.in_step < CROSSFADE_TICKS {
            self.in_step += 1;
        }
        if !self.out_slot.is_empty() {
            self.out_step += 1;
            if self.out_step >= CROSSFADE_TICKS {
                self.out_slot = "";
                self.out_step = 0;
            }
        }
    }

    /// Das einblendende bzw. laufende Stück [0] und das ausblendende [1].
    pub fn voices(&self) -> [MusicVoice; 2] {
        [
            MusicVoice {
                slot: self.in_slot,
                gain: self.in_level(),
            },
            MusicVoice {
                slot: self.out_slot,
                gain: self.out_level(),
            },
        ]
    }
}
