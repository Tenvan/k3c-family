//! Port von `native/k3c-game/hud/kampf.go`: Kampf-Anzeige (Spiegel von src/scenes/kampfAnzeige.ts, K5.1, B-132):
//! Boss-Leiste und laufende Nacht-Events aus dem Zustand der Stufe. Der Warnkreis vor dem Flächenschlag liegt in der
//! Welt (Paket `fx`).

use k3c_sim::state::{ActiveEvent, Enemy, Merchant};

use super::data::{boss_names, end_boss_phases};
use super::texts::{name_of, tf, tr};
use super::values::{SEP, format_time};

/// BossView ist die Leiste eines Bosses: Anteil HP/Maximum, Name und Phase („Phase 2 von 3“, beim Miniboss leer).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BossView {
    pub ok: bool,
    pub id: i64,
    pub share: f64,
    pub name: String,
    pub phase: String,
}

/// boss_bar wählt den Endboss, sonst den ersten Boss; ohne Boss `ok` false.
pub fn boss_bar(lang: &str, enemies: &[Enemy]) -> BossView {
    let mut boss: Option<&Enemy> = None;
    for e in enemies.iter().filter(|e| e.boss) {
        if end_boss_phases().contains_key(&e.kind) {
            boss = Some(e);
            break;
        }
        if boss.is_none() {
            boss = Some(e);
        }
    }
    let Some(boss) = boss else {
        return BossView::default();
    };
    let share = if boss.max_hp > 0.0 {
        (boss.hp / boss.max_hp).clamp(0.0, 1.0)
    } else {
        0.0
    };
    BossView {
        ok: true,
        id: boss.id,
        share,
        name: name_of(
            lang,
            "boss",
            &boss.kind,
            boss_names().get(&boss.kind).map_or(boss.kind.as_str(), String::as_str),
        ),
        phase: phase_text(lang, &boss.kind, boss.phase),
    }
}

/// phase_text ist „Phase n von m“ eines Endbosses; Miniboss oder ohne Phase "".
pub fn phase_text(lang: &str, kind: &str, phase: i64) -> String {
    let Some(&of) = end_boss_phases().get(kind) else {
        return String::new();
    };
    if phase <= 0 {
        return String::new();
    }
    tf(lang, "boss.phase", &[("n", phase.into()), ("of", of.into())])
}

/// boss_title ist die Zeile über der Leiste: Name und Phase.
pub fn boss_title(b: &BossView) -> String {
    if b.phase.is_empty() {
        return b.name.clone();
    }
    format!("{}{SEP}{}", b.name, b.phase)
}

/// Zahl der Events, die der Schlüssel höchstens trägt.
pub const EVENTS_MAX: usize = 3;

/// EventsKey: laufende Events mit Restzeit in ganzen Sekunden (Zeitraffer eingerechnet) und Händler-Überfall.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EventsKey {
    pub lang: String,
    pub ids: [String; EVENTS_MAX],
    pub left: [i64; EVENTS_MAX],
    pub n: usize,
    pub raid: bool,
}

/// new_events_key liest höchstens drei laufende Events.
pub fn new_events_key(lang: &str, events: &[ActiveEvent], speed: f64, m: Option<&Merchant>) -> EventsKey {
    let speed = if speed <= 0.0 { 1.0 } else { speed };
    let mut k = EventsKey {
        lang: lang.to_string(),
        raid: m.is_some_and(|m| m.raid),
        ..EventsKey::default()
    };
    for e in events.iter().take(EVENTS_MAX) {
        k.ids[k.n].clone_from(&e.id);
        k.left[k.n] = (e.seconds_left / speed).ceil().max(0.0) as i64;
        k.n += 1;
    }
    k
}

/// event_banner ist eine Zeile je Event mit Restzeit, der Händler-Überfall ohne Restzeit; ohne Event "".
pub fn event_banner(k: &EventsKey) -> String {
    let mut lines: Vec<String> = (0..k.n)
        .map(|i| {
            tf(
                &k.lang,
                "event.timed",
                &[
                    ("name", name_of(&k.lang, "event", &k.ids[i], &k.ids[i]).into()),
                    ("left", format_time(k.left[i] as f64).into()),
                ],
            )
        })
        .collect();
    if k.raid {
        lines.push(tr(&k.lang, "event.merchantRaid").to_string());
    }
    lines.join("\n")
}
