//! Port von `native/k3c-game/hud/state.go`: Zustand je Spieler, einmal je Frame fortgeschrieben (`listen`): Texte (nur bei
//! geändertem Schlüssel neu), Meldungen, Hervorhebung des Goldtopfs, „gesichert“, Hinweise, Radar und Bürger-Labels.
//! `draw` liest nur.

use super::Hud;
use super::banner::{BannerQueue, event_text};
use super::glyphs::{JoinKey, join_from_key, new_join_key};
use super::hints::{GuideHint, GuideId, GuideSeen, action_hint, guide_keys, hint_text, step_guide};
use super::icons::{StockPart, stock_parts};
use super::kampf::{BossView, EventsKey, boss_bar, boss_title, event_banner, new_events_key};
use super::pending::purse_max_of;
use super::radar::{RadarMarker, radar_markers};
use super::seekarte::SeaView;
use super::skillmenu::{MenuEntry, MenuState, SlotView, slot_views};
use super::skilltext::{BarKey, MenuKey, MenuView, new_bar_key, slot_bar};
use super::story::StoryQueue;
use super::texts::tr;
use super::values::*;
use crate::props::pending::Economy;
use crate::props::simview::SimView;
use crate::view::MAX_PLAYERS;
use crate::view::frame::{CueKind, Frame, Seat, Stage};
use crate::view::input::DeviceID;
use crate::view::settings::Settings;
use k3c_sim::state::purse::purse_size;
use k3c_sim::state::{Player, World};

/// SAVED_SECONDS: so lange zeigt die Uhr nach dem Speichern „gesichert“ (B-186).
pub const SAVED_SECONDS: f64 = 2.0;

/// PLAYER_COLORS sind die Farben der Spieler (src/core/constants.ts › PLAYER_COLORS).
pub const PLAYER_COLORS: [u32; 4] = [0x3a86ff, 0xff006e, 0xfb5607, 0x8338ec];

/// Label ist ein Text in der Welt (Beruf über einem Bauern, Händler) mit Ort in Units.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Label {
    pub x: f64,
    /// Höhe über dem Boden in Units
    pub y: f64,
    pub text: String,
    /// Farbe des Berufssymbols, 0 = keins
    pub dot: u32,
}

/// SeatState ist alles, was das HUD für einen Spieler (`Seat::player`) zeichnet.
#[derive(Debug, Default)]
pub struct SeatState {
    pub active: bool,
    pub monarch: i64,
    pub stage: i64,
    pub device: DeviceID,
    /// Eingabe-Profil des Geräts (`Settings::profile_of`, B-894)
    pub profile: String,
    pub x: f64,
    pub down: bool,
    pub flash: bool,
    /// Größe des Geldsacks 1 bis 3 (`purse_size`, B-922)
    pub purse: i64,
    pub color: u32,
    pub width_units: f64,
    pub gold_s: String,
    pub clock_s: String,
    pub fight_s: String,
    pub shared_s: String,
    pub place_s: String,
    pub cit_s: String,
    pub travel_s: String,
    pub ev_s: String,
    pub bar_s: String,
    pub hint_s: String,
    pub guide_s: String,
    pub boss: BossView,
    pub boss_s: String,
    pub guide: GuideHint,
    pub banner: BannerQueue,
    /// Geschichtszeile (story.go, B-974)
    pub story: StoryQueue,
    pub saved_until: f64,
    pub gold: GoldFlash,
    pub radar: Vec<RadarMarker>,
    pub labels: Vec<Label>,

    pub m_gold: Memo<GoldKey>,
    pub m_clock: Memo<ClockKey>,
    pub m_fight: Memo<WaveKey>,
    pub m_shared: Memo<StockKey>,
    /// Vorrats-Anzeige mit Icons (icons.rs), gebaut aus `stock_k`
    pub parts: Vec<StockPart>,
    pub stock_k: StockKey,
    pub parts_set: bool,
    pub m_place: Memo<PlaceKey>,
    pub m_cit: Memo<HubKey>,
    pub m_travel: Memo<TravelKey>,
    pub m_events: Memo<EventsKey>,
    pub m_bar: Memo<BarKey>,
    pub m_hint: Memo<super::hints::HintKey>,
    pub m_boss: Memo<BossView>,
    pub m_merchant: Memo<PlaceKey>,

    pub menu: MenuState,
    pub entries: Vec<MenuEntry>,
    /// Schlüssel des gebauten Menü-Textes (Go: `mMenu`)
    pub m_menu: Option<MenuKey>,
    pub menu_v: MenuView,
    pub slots: [SlotView; 4],
    /// Seekarte am Hafen (seekarte.go)
    pub sea: SeaView,
}

/// put schreibt s nach dst, ohne zu allozieren, wenn der Text gleich bleibt.
fn put(dst: &mut String, s: &str) {
    if dst != s {
        dst.clear();
        dst.push_str(s);
    }
}

/// guide_bits liest die Bitmaske der Einstellungen (Go: `guideSeen(int)` schneidet auf 8 Bit).
fn guide_bits(v: i64) -> GuideSeen {
    GuideSeen(v as u8)
}

impl Hud {
    /// Go: `(*hud).Listen`: schreibt den Zustand aller Spieler des Frames fort (`Listener`).
    pub fn listen(&mut self, f: &Frame) {
        self.paint.outline.sweep();
        let lang_only = Settings {
            language: f.settings.language.clone(),
            ..Settings::default()
        };
        self.lang = lang_only.normalized().language;
        // Pause zeigt das Pause-Menü (menu), kein eigenes Band
        self.cells = f.seats.len().min(MAX_PLAYERS);
        let jk: JoinKey = new_join_key(&self.lang, &f.unseated);
        put(&mut self.join_s, self.m_join.get(jk, join_from_key));
        self.guide = !f.settings.guide_off;
        self.seen = GuideSeen(guide_bits(f.settings.guide_seen).0 | self.pending.0);
        self.cues(f);
        let mut seen = [false; MAX_PLAYERS];
        for s in &f.seats {
            if s.player < 0 || s.player as usize >= MAX_PLAYERS {
                continue;
            }
            let p = s.player as usize;
            seen[p] = true;
            self.seats[p].update(&self.lang, self.guide, &mut self.seen, self.view.as_ref(), f, s);
        }
        for (p, st) in self.seats.iter_mut().enumerate() {
            if !seen[p] {
                st.active = false;
                st.menu = MenuState::default();
                st.sea = SeaView::default();
            }
        }
        self.story(f);
        for st in &mut self.seats {
            st.banner.step(f.clock);
        }
        self.pending = GuideSeen(self.seen.0 & !guide_bits(f.settings.guide_seen).0);
    }

    /// cues: gespeichert setzt „gesichert“, Fehler und Ablehnungen kommen als Meldung (Spieler -1: an alle).
    fn cues(&mut self, f: &Frame) {
        for c in &f.cues {
            for (p, st) in self.seats.iter_mut().enumerate() {
                let mine = c.player < 0 || c.player == p as i64;
                match c.kind {
                    CueKind::Saved => st.saved_until = f.clock + SAVED_SECONDS,
                    k if is_report(k) && mine && !c.text.is_empty() => st.banner.add(&c.text),
                    _ => {}
                }
            }
        }
    }
}

/// is_report: Ablehnung, fehlgeschlagenes Speichern und Fehler kommen als Meldung.
fn is_report(k: CueKind) -> bool {
    matches!(k, CueKind::Denied | CueKind::SaveFailed | CueKind::Error)
}

impl SeatState {
    /// update liest den Platz s aus dem Frame.
    pub fn update(&mut self, lang: &str, guide: bool, seen: &mut GuideSeen, sv: &dyn SimView, f: &Frame, s: &Seat) {
        let found = f
            .stage(s.stage)
            .and_then(|stage| stage.world.as_deref().map(|w| (stage, w)))
            .zip(f.monarch(s.player));
        let Some(((stage, w), m)) = found else {
            self.active = false;
            self.menu = MenuState::default();
            self.sea = SeaView::default();
            return;
        };
        self.active = true;
        self.monarch = m.index;
        self.stage = s.stage;
        self.device = s.device;
        self.x = m.x;
        self.profile = f.settings.profile_of(s.device).to_string();
        self.color = PLAYER_COLORS[(m.index as usize) % PLAYER_COLORS.len()];
        let e = sv.economy_of(w);
        self.card(lang, w, m, f.clock);
        self.world(lang, w, &e, f.clock);
        self.events(lang, f, stage);
        self.hints(lang, w, m, guide, seen, sv);
        radar_markers(&mut self.radar, w, m.index);
        self.width_units = w.width_units;
        self.citizen_labels(lang, w);
        self.skills(lang, w, m);
        self.sea
            .refresh(lang, &self.profile, w.island_switch.as_ref(), &f.route);
    }

    /// card: Werte der Spielerkarte (Geldsack-Größe, Gold) und der Skill-Leiste. Die Münz-Obergrenze kommt vom Spieler
    /// (`purse_max_of`, B-922), nicht aus der Insel-Wirtschaft.
    fn card(&mut self, lang: &str, w: &World, m: &Player, now: f64) {
        self.down = m.crown.is_some();
        self.purse = purse_size(w, m);
        let mut purse = purse_max_of(w, m);
        if purse <= 0 {
            purse = super::data::purse_max_gold();
        }
        let k = GoldKey {
            lang: lang.to_string(),
            gold: m.gold,
            max: purse,
        };
        put(&mut self.gold_s, self.m_gold.get(k, gold_text));
        self.flash = self.gold.step(m.gold, now);
        self.slots = slot_views(&m.slots, &m.cooldowns);
        put(
            &mut self.bar_s,
            self.m_bar.get(new_bar_key(lang, &self.slots), slot_bar),
        );
    }

    /// world: Uhr, Kampf, Vorrat, Hub und Bürger, Ort, Reise, Boss und Events der Stufe.
    fn world(&mut self, lang: &str, w: &World, e: &Economy, now: f64) {
        let ck = new_clock_key(lang, &w.cycle, w.cycle_speed, w.aggression, now < self.saved_until);
        put(&mut self.clock_s, self.m_clock.get(ck, clock_text));
        let wk = WaveKey {
            lang: lang.to_string(),
            wave: w.wave,
            n: (w.enemies.len() + w.spawn_queue.len()) as i64,
        };
        put(&mut self.fight_s, self.m_fight.get(wk, wave_text));
        let sk = StockKey {
            lang: lang.to_string(),
            primary: w.biome.primary_resource.clone(),
            stock: stock_of(Some(&w.stock)),
            max: e.stock_max,
            points: w.skill_points,
        };
        put(&mut self.shared_s, self.m_shared.get(sk.clone(), shared_text));
        if !self.parts_set || self.stock_k != sk {
            stock_parts(&mut self.parts, &sk);
            self.stock_k = sk;
            self.parts_set = true;
        }
        put(
            &mut self.cit_s,
            self.m_cit.get(new_hub_key(lang, e, w.merchant.as_ref()), citizens_text),
        );
        let pk = PlaceKey {
            lang: lang.to_string(),
            id: w.biome.id.clone(),
            name: w.biome.name.clone(),
            player: self.monarch,
        };
        put(&mut self.place_s, self.m_place.get(pk, place_text));
        put(
            &mut self.travel_s,
            self.m_travel.get(new_travel_key(lang, w.travel.as_ref()), travel_text),
        );
        let ek = new_events_key(lang, &w.night_events, w.cycle_speed, w.merchant.as_ref());
        put(&mut self.ev_s, self.m_events.get(ek, event_banner));
        self.boss = boss_bar(lang, &w.enemies);
        put(&mut self.boss_s, self.m_boss.get(self.boss.clone(), boss_title));
    }

    /// events: Meldungen aus den Ereignissen der Stufe des Spielers. Fällt die Burg im Niederlage-Modus „lost“, meldet
    /// die Simulation im selben Takt gameOver (`sim.gameOver`); dann nennt die Meldung die Verluste dieses Modus (B-813).
    fn events(&mut self, lang: &str, f: &Frame, stage: &Stage) {
        let Some(w) = stage.world.as_deref() else {
            return;
        };
        let over = f.events_of(stage.index).any(|e| e.type_() == "gameOver");
        for e in f.events_of(stage.index) {
            let mut t = event_text(lang, e, &w.biome.id);
            if over && e.type_() == "castleFallen" {
                t = tr(lang, "ev.castleFallenLost").to_string();
            }
            if !t.is_empty() {
                self.banner.add(&t);
            }
        }
    }

    /// hints: Aktions-Hinweis über dem Monarchen und Hinweis der geführten ersten Nacht (nur mit Anleitung an, B-809).
    fn hints(&mut self, lang: &str, w: &World, m: &Player, guide: bool, seen: &mut GuideSeen, sv: &dyn SimView) {
        put(
            &mut self.hint_s,
            self.m_hint.get(action_hint(lang, w, m, sv), hint_text),
        );
        self.guide = if guide {
            step_guide(self.guide, w, m.x, m.crown.is_some(), seen)
        } else {
            GuideHint::default()
        };
        self.guide_s.clear();
        if self.guide.id != GuideId::None {
            self.guide_s.push_str(tr(lang, guide_keys(self.guide.id)));
        }
    }
}
