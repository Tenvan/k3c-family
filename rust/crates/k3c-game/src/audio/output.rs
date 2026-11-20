//! Port von `native/k3c-game/audio/output.go`: die Wiedergabe eines Mix über `k3c_platform::AudioOut`.
//!
//! Abweichungen von Go: `device` hält `Box<dyn AudioOut>` statt der oto-Senke; der Lader läuft synchron beim ersten
//! hörbaren Mix (Vorentscheidung RU17.5, Ladezeit im Log `📂 Ton geladen … ms`), darum gibt es kein `loading` und
//! `poll` fragt nur `failed()` ab. Musikstücke lädt die Plattform ganz (je Slot einmal, danach aus `music`) und spielt
//! sie mit `play_loop`; einen Lesefehler des Stroms (Go: `player.Err`) gibt es nicht mehr, stumm bleibt ein Slot nur
//! bei einer nicht ladbaren Datei. Stücke und Schleifen, die nicht klingen (Stimmen-Grenze 32 der Plattform), starten
//! neu statt stumm zu bleiben (Review RU17.5). Warnungen landen zusätzlich in `notes`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::Arc;

use k3c_platform::{AudioOut, SoundId, VoiceId, log, open_audio};

use super::bank::{Bank, clip, load_bank, load_file, warn};
use super::mix::{Mix, Request};
use super::register::Register;

/// sampleRate des Ton-Kontexts: die Rate des Atlas (public/audio/README.md: 44,1 kHz).
pub const SAMPLE_RATE: i64 = 44100;

/// maxVoices begrenzt gleichzeitige Instanzen je Effekt-Slot (audioCore.ts › MAX_SAMPLE_VOICES).
pub const MAX_VOICES: usize = 4;

/// output setzt einen Mix um.
pub trait Output {
    fn apply(&mut self, m: &Mix);
    /// Wahr für die Wiedergabe ohne Ton (Go: Typprüfung auf `silent` im Test).
    fn is_silent(&self) -> bool {
        false
    }
}

/// silent ist die Wiedergabe ohne Ton (Tests, Aufnahme-Modus, kein Gerät).
#[derive(Clone, Copy, Debug, Default)]
pub struct Silent;

impl Output for Silent {
    fn apply(&mut self, _m: &Mix) {}

    fn is_silent(&self) -> bool {
        true
    }
}

struct Voice {
    slot: String,
    gain: f64,
    p: VoiceId,
}

/// Ein offenes Musikstück.
pub struct Track {
    pub slot: String,
    pub p: VoiceId,
}

struct LoopVoice {
    key: i64,
    p: VoiceId,
}

/// Öffnet das Ton-Gerät (Go: `openOto`, in Tests ein Fake).
pub type Opener = Box<dyn FnMut() -> Result<Box<dyn AudioOut>, String>>;

/// device ist die dünne Schicht über `AudioOut`: ein Gerät, erst beim ersten hörbaren Mix angelegt; der Ton lädt
/// danach sofort. Die Plattform mischt die Stimmen, jede mit der Lautstärke aus dem Mix (Pegel mal Bus). Fehler machen
/// stumm, nie einen Absturz (B-805). Alle Felder gehören dem Spiel-Thread (Listen).
pub struct Device {
    pub dir: PathBuf,
    pub reg: Arc<Register>,
    pub open: Opener,
    pub ctx: Option<Box<dyn AudioOut>>,
    pub off: bool,
    pub bank: Option<Bank>,
    voices: Vec<Voice>,
    pub tracks: Vec<Track>,
    loops: Vec<LoopVoice>,
    warned: BTreeSet<String>,
    /// Musik-Slot → geladenes Stück (die Plattform gibt Geladenes nicht frei: je Slot einmal laden)
    music: BTreeMap<String, SoundId>,
    /// zuletzt angewandte Effekt-Lautstärke
    bus: f64,
    /// Warnungen des Geräts (Log-Zeilen)
    pub notes: Vec<String>,
}

pub fn new_device(dir: impl Into<PathBuf>, reg: Arc<Register>) -> Device {
    Device {
        dir: dir.into(),
        reg,
        open: Box::new(|| Ok(open_audio())),
        ctx: None,
        off: false,
        bank: None,
        voices: Vec::new(),
        tracks: Vec::new(),
        loops: Vec::new(),
        warned: BTreeSet::new(),
        music: BTreeMap::new(),
        bus: -1.0,
        notes: Vec::new(),
    }
}

impl Output for Device {
    /// apply setzt den Mix um: Effekte starten, Musik und Schleifen auf Stand bringen, fertige Stimmen freigeben.
    fn apply(&mut self, m: &Mix) {
        if self.off || (self.ctx.is_none() && !audible(m)) {
            return;
        }
        if self.ctx.is_none() && !self.start() {
            return;
        }
        if !self.poll() {
            return;
        }
        self.reap(m);
        for r in &m.effects {
            self.play(r, m);
        }
        self.sync_music(m);
        self.sync_loops(m);
    }
}

/// audible meldet, ob der Mix etwas Hörbares verlangt (erst dann entsteht das Gerät).
pub fn audible(m: &Mix) -> bool {
    let music = m.music_bus > 0.0 && (m.music[0].gain > 0.0 || m.music[1].gain > 0.0);
    music || (m.effect_bus > 0.0 && (!m.effects.is_empty() || !m.loops.is_empty()))
}

/// Lautstärke für die Plattform (0 … 1 als `f32`).
#[allow(clippy::cast_possible_truncation)]
fn f32v(v: f64) -> f32 {
    v as f32
}

impl Device {
    /// start legt das Gerät an und lädt den Ton; ohne Gerät bleibt der Ton aus.
    fn start(&mut self) -> bool {
        let mut ctx = match (self.open)() {
            Ok(c) => c,
            Err(e) => {
                warn(
                    &mut self.notes,
                    "❌ Ton-Gerät nicht startbar, Spiel läuft stumm",
                    &[("err", &e)],
                );
                self.off = true;
                return false;
            }
        };
        log::info("🚀 Ton-Gerät angelegt (k3c-platform)", &[("rate", &SAMPLE_RATE)]);
        self.bank = Some(load_bank(&self.dir, &self.reg, ctx.as_mut()));
        self.ctx = Some(ctx);
        true
    }

    /// fail schaltet den Ton nach einem Treiberfehler ab (B-805): eine Log-Zeile, alle Stimmen enden, danach bleibt
    /// der Ton aus bis zum Neustart (kein Wiederverbinden). Der Fehler geht nicht nach außen.
    fn fail(&mut self, err: &str) {
        warn(
            &mut self.notes,
            "💥 Ton-Gerät ausgefallen, Spiel läuft stumm bis zum Neustart",
            &[("err", &err)],
        );
        if let Some(ctx) = self.ctx.as_deref_mut() {
            for v in &self.voices {
                ctx.stop(v.p);
            }
            for l in &self.loops {
                ctx.stop(l.p);
            }
            for t in &self.tracks {
                ctx.stop(t.p);
            }
        }
        self.voices.clear();
        self.loops.clear();
        self.tracks.clear();
        self.off = true;
    }

    /// poll prüft das Gerät (Go: `ctx.Err` und Übernahme der Bank); falsch nach einem Ausfall.
    fn poll(&mut self) -> bool {
        let failed = self.ctx.as_deref().and_then(AudioOut::failed);
        match failed {
            Some(err) => {
                self.fail(&err);
                false
            }
            None => self.ctx.is_some(),
        }
    }

    /// reap gibt fertige Effekt-Stimmen frei und stellt laufende auf eine geänderte Effekt-Lautstärke.
    #[allow(clippy::float_cmp)]
    fn reap(&mut self, m: &Mix) {
        let Some(ctx) = self.ctx.as_deref_mut() else {
            return;
        };
        let changed = m.effect_bus != self.bus;
        self.voices.retain(|v| {
            if !ctx.playing(v.p) {
                ctx.stop(v.p);
                return false;
            }
            if changed {
                ctx.set_volume(v.p, f32v(m.effect_volume(v.gain)));
            }
            true
        });
        self.bus = m.effect_bus;
    }

    /// play startet einen Effekt, wenn er hörbar ist, sein Ton geladen ist und weniger als maxVoices davon laufen.
    fn play(&mut self, r: &Request, m: &Mix) {
        let vol = m.effect_volume(r.gain);
        let Some(sound) = clip(self.bank.as_ref(), &r.slot) else {
            return;
        };
        if vol <= 0.0 || self.count(&r.slot) >= MAX_VOICES {
            return;
        }
        let Some(ctx) = self.ctx.as_deref_mut() else {
            return;
        };
        let p = ctx.play(sound, f32v(vol));
        self.voices.push(Voice {
            slot: r.slot.to_string(),
            gain: r.gain,
            p,
        });
    }

    fn count(&self, slot: &str) -> usize {
        self.voices.iter().filter(|v| v.slot == slot).count()
    }
}

impl Device {
    /// sync_music hält genau die Stücke des Mix offen und stellt ihre Lautstärke; andere enden. Ein Stück, das nicht
    /// mehr klingt (Stimmen-Grenze der Plattform, `NullAudio`), endet ohne Meldung und startet neu aus dem geladenen
    /// Klang; stumm bleibt ein Slot nur, wenn seine Datei nicht ladbar ist (Go: Lesefehler, `warn_once`).
    fn sync_music(&mut self, m: &Mix) {
        let Some(ctx) = self.ctx.as_deref_mut() else {
            return;
        };
        self.tracks.retain(|t| {
            if ctx.playing(t.p) && (t.slot == m.music[0].slot || t.slot == m.music[1].slot) {
                return true;
            }
            ctx.stop(t.p);
            false
        });
        for mv in &m.music {
            if mv.slot.is_empty() {
                continue;
            }
            let Some(i) = self.track(mv.slot) else {
                continue;
            };
            if let Some(ctx) = self.ctx.as_deref_mut() {
                ctx.set_volume(self.tracks[i].p, f32v(m.music_volume(mv.gain)));
            }
        }
    }

    /// track liefert das offene Stück eines Musik-Slots oder öffnet es als Schleife (Datei ganz geladen, je Slot
    /// einmal); `None` = stumm.
    fn track(&mut self, slot: &str) -> Option<usize> {
        if let Some(i) = self.tracks.iter().position(|t| t.slot == slot) {
            return Some(i);
        }
        if self.warned.contains(slot) {
            return None;
        }
        let ctx = self.ctx.as_deref_mut()?;
        let sound = match self.music.get(slot) {
            Some(s) => *s,
            None => {
                let file = self.reg.music.get(slot).map(String::as_str).unwrap_or_default();
                match load_file(&self.dir.join(file), ctx) {
                    Ok(s) => {
                        self.music.insert(slot.to_string(), s);
                        s
                    }
                    Err(e) => {
                        warn_once(&mut self.warned, &mut self.notes, slot, &e);
                        return None;
                    }
                }
            }
        };
        let p = ctx.play_loop(sound, 0.0);
        self.tracks.push(Track {
            slot: slot.to_string(),
            p,
        });
        Some(self.tracks.len() - 1)
    }

    /// sync_loops hält die Hufschlag-Schleifen auf dem gewünschten Stand: neue starten, fehlende enden, vorhandene
    /// stellen nur die Lautstärke nach (audioCore.ts › syncLoops). Abweichung: Eine Schleife, die nicht klingt
    /// (Stimmen-Grenze der Plattform), endet und startet im selben Takt neu.
    fn sync_loops(&mut self, m: &Mix) {
        let Some(ctx) = self.ctx.as_deref_mut() else {
            return;
        };
        self.loops.retain(|l| {
            if let Some(gain) = loop_gain(m, l.key).filter(|_| ctx.playing(l.p)) {
                ctx.set_volume(l.p, f32v(m.effect_volume(gain)));
                return true;
            }
            ctx.stop(l.p);
            false
        });
        for w in &m.loops {
            if self.has_loop(w.key) {
                continue;
            }
            let Some(sound) = clip(self.bank.as_ref(), w.slot) else {
                continue;
            };
            let Some(ctx) = self.ctx.as_deref_mut() else {
                return;
            };
            let p = ctx.play_loop(sound, f32v(m.effect_volume(w.gain)));
            self.loops.push(LoopVoice { key: w.key, p });
        }
    }

    fn has_loop(&self, key: i64) -> bool {
        self.loops.iter().any(|l| l.key == key)
    }
}

pub fn loop_gain(m: &Mix, key: i64) -> Option<f64> {
    m.loops.iter().find(|l| l.key == key).map(|l| l.gain)
}

/// warn_once meldet einen Ton-Fehler je Slot genau einmal; der Slot bleibt stumm.
fn warn_once(warned: &mut BTreeSet<String>, notes: &mut Vec<String>, slot: &str, err: &str) {
    if !warned.insert(slot.to_string()) {
        return;
    }
    warn(
        notes,
        "❌ Ton nicht abspielbar, Slot stumm",
        &[("slot", &slot), ("err", &err)],
    );
}
