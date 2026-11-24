//! Port von `native/k3c-game/hud/story.go`: Geschichtszeile (B-974, docs/rules/welten.md § 6.2): An den Wendepunkten der
//! Kampagne blenden 1 bis 2 Zeilen der Geschichte in jeder Spielerhälfte ein und wieder aus, ohne Eingabe und ohne
//! Pause. Welche Zeilen gelten, sagt die Engine (`WorldDef::story_…`, RU19); hier steht nur, welches Ereignis welche
//! Funktion fragt, und der Ablauf der Einblendung. Die Zeit kommt nur aus `Frame::clock`; in der Pause, hinter der
//! Ladeanzeige und während der Schiffsfahrt (die Uhr steht) wartet die Einblendung. Menüs und Seekarte verdecken sie,
//! sie läuft dahinter ab.
//!
//! Abweichung von Go: `Campaign::world` (die Welt-Definition) gibt es erst mit RU19; `pending::campaign_world` liefert
//! bis dahin `None`, `story_points` läuft dann nie (wie Go ohne Welt).

use super::Hud;
use super::banner::str_of;
use super::data::end_boss_phases;
use super::pending::{WorldDef, campaign_world, story_done_lines};
use crate::view::frame::{Frame, Outcome};
use crate::view::story::EndHolder;

/// Ablauf einer Einblendung in Sekunden (B-974/R4: Startwerte, vorläufig bis zur Abstimmung am TV).
pub const STORY_FADE_IN: f64 = 0.5;
pub const STORY_HOLD: f64 = 4.0;
pub const STORY_FADE_OUT: f64 = 0.5;
pub const STORY_SECONDS: f64 = STORY_FADE_IN + STORY_HOLD + STORY_FADE_OUT;

/// StoryEntry ist ein Eintrag der Geschichte: 1 bis 2 Zeilen (`b` leer = eine Zeile).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StoryEntry {
    pub a: String,
    pub b: String,
}

/// StoryQueue ist die Geschichtszeile eines Spielers: der laufende Eintrag mit seinem Alter, dahinter die wartenden.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StoryQueue {
    pub showing: StoryEntry,
    /// Sekunden seit Beginn des laufenden Eintrags
    pub age: f64,
    pub queue: Vec<StoryEntry>,
}

impl StoryQueue {
    /// add reiht einen Eintrag ein; derselbe Eintrag steht nur einmal (laufend oder wartend), ein leerer gar nicht.
    pub fn add(&mut self, e: &StoryEntry) {
        if e.a.is_empty() || *e == self.showing || self.queue.contains(e) {
            return;
        }
        self.queue.push(e.clone());
    }

    /// step lässt `dt` Sekunden vergehen: Ist der laufende Eintrag abgelaufen, kommt der nächste (oder keiner).
    pub fn step(&mut self, dt: f64) {
        if !self.showing.a.is_empty() {
            self.age += dt;
            if self.age < STORY_SECONDS {
                return;
            }
            self.showing = StoryEntry::default();
        }
        if self.queue.is_empty() {
            return;
        }
        self.showing = self.queue.remove(0);
        self.age = 0.0;
    }

    /// alpha ist die Deckkraft des laufenden Eintrags: einblenden, stehen, ausblenden (0 ohne Eintrag).
    pub fn alpha(&self) -> f64 {
        if self.showing.a.is_empty() {
            0.0
        } else if self.age < STORY_FADE_IN {
            self.age / STORY_FADE_IN
        } else if self.age > STORY_FADE_IN + STORY_HOLD {
            ((STORY_SECONDS - self.age) / STORY_FADE_OUT).max(0.0)
        } else {
            1.0
        }
    }

    /// reset verwirft den laufenden und die wartenden Einträge (der Puffer bleibt).
    pub fn reset(&mut self) {
        self.showing = StoryEntry::default();
        self.age = 0.0;
        self.queue.clear();
    }
}

/// TaleState ist der Zustand der Geschichte über alle Spieler: zu welchem Spiel die Einträge gehören, die Uhr des
/// letzten Frames und ob der Sieg schon gemeldet ist.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TaleState {
    pub run: u64,
    pub clock: f64,
    pub won: bool,
}

impl Hud {
    /// story schreibt die Geschichtszeilen aller Spieler fort (nach den Plätzen in `listen`): anderes Spiel oder kein
    /// Spiel verwirft alles, Wendepunkte dieses Frames reihen ein, dann läuft die Zeit.
    pub fn story(&mut self, f: &Frame) {
        if f.campaign.run != self.tale.run || f.stages.is_empty() {
            for st in &mut self.seats {
                st.story.reset();
            }
            self.tale.run = f.campaign.run;
            self.tale.won = false;
        }
        let mut dt = f.clock - self.tale.clock;
        self.tale.clock = f.clock;
        if dt < 0.0 || f.paused || f.loading {
            dt = 0.0;
        }
        self.bottle_story(f);
        if let Some(w) = campaign_world(f) {
            self.story_points(f, w);
        }
        for st in &mut self.seats {
            if st.active {
                st.story.step(dt);
            } else {
                st.story.reset(); // wer später dazukommt, sieht erst den nächsten Wendepunkt
            }
        }
    }

    /// story_points fragt die Engine zu den Wendepunkten dieses Frames und reiht ihre Zeilen ein: neue Kampagne, Ankunft
    /// (`islandSwitch`, wenn der Raum die Insel getauscht hat), Endboss ausgelöst und besiegt, Sieg der Insel. Die
    /// Ereignisse aller Stufen zählen, nicht nur die der eigenen (der Wendepunkt gilt für die Insel). Miniboss: keine
    /// Zeile.
    fn story_points(&mut self, f: &Frame, w: &WorldDef) {
        let c = &f.campaign;
        if c.fresh {
            self.tell(&w.story_start(), w.story_start_len());
        }
        for e in &f.events {
            match e.type_() {
                "islandSwitch" => {
                    let to = str_of(&e.data, "target");
                    if !to.is_empty() && to == c.island {
                        self.tell(&w.story_arrival(&to), 0);
                    }
                }
                "bossSpawned" if is_end_boss(&str_of(&e.data, "boss")) => {
                    self.tell(&w.story_boss(&c.island), 0);
                }
                "bossDefeated" if is_end_boss(&str_of(&e.data, "boss")) => {
                    self.tell(
                        &w.story_defeated(&c.island, &c.completed),
                        story_done_lines(w, &c.island),
                    );
                }
                _ => {}
            }
        }
        // Sieg der Insel (auch ohne Endboss, z. B. Tage überleben): Das Ereignis victory liegt in Stufe 0 und erreicht das
        // Spiel nicht immer, der Ausgang schon. Nach einem Sieg über den Endboss nennt die Engine dieselben Zeilen wie
        // bossDefeated; add reiht sie nicht doppelt ein.
        let won = f.outcome == Outcome::Won;
        if won != self.tale.won {
            self.tale.won = won;
            if won {
                self.tell(
                    &w.story_victory(&c.island, &c.completed),
                    story_done_lines(w, &c.island),
                );
            }
        }
    }

    /// tell reiht die Zeilen eines Wendepunkts bei allen aktiven Spielern ein, in Einträgen zu 1 bis 2 Zeilen: zuerst
    /// die ersten `first` Zeilen (der erste Eintrag des Wendepunkts; 0 = keiner), dann der Rest, beides je höchstens
    /// zwei Zeilen (die Zeilenregel erlaubt je Eintrag 1 bis 2, welten.md § 2.2); über die Grenze `first` hinweg mischt
    /// kein Eintrag.
    pub fn tell(&mut self, lines: &[String], first: usize) {
        let mut first = first.min(lines.len());
        let mut lines = lines;
        while !lines.is_empty() {
            let mut n = lines.len().min(2);
            if first > 0 {
                n = n.min(first);
            }
            first -= first.min(n);
            let e = StoryEntry {
                a: lines[0].clone(),
                b: if n == 2 { lines[1].clone() } else { String::new() },
            };
            lines = &lines[n..];
            for st in &mut self.seats {
                if st.active {
                    st.story.add(&e);
                }
            }
        }
    }
}

impl EndHolder for Hud {
    /// Am Spielende läuft oder wartet noch ein Eintrag der Geschichte; game zeigt das Hauptmenü erst danach (die Zeilen
    /// zum Sieg wären sonst nach `END_DELAY` abgeschnitten).
    fn end_holding(&self) -> bool {
        self.seats
            .iter()
            .any(|st| st.active && (!st.story.showing.a.is_empty() || !st.story.queue.is_empty()))
    }
}

/// is_end_boss meldet, ob die Boss-ID ein Endboss ist (bosses.json › endBosses).
pub fn is_end_boss(id: &str) -> bool {
    end_boss_phases().contains_key(id)
}
