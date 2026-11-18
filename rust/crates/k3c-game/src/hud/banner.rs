//! Port von `native/k3c-game/hud/banner.go`.
//!
//! Meldungen (Spiegel von HudScene.ts › eventText/showBanner und bannerQueue.ts, B-414): Ereignis → Text, Kürzen auf
//! eine Zeile, Warteschlange je Spieler. Die Zeit kommt nur aus Frame.Clock.
//!
//! Abweichung von Go: `numOf`/`strOf` lesen das Ereignis (`serde_json`-Map) wie `fx::effects`; sie sind hier nachgebaut,
//! nicht importiert (RU20.3).

use super::data::building_info;
use super::texts::{Arg, name_of, tf, tr};
use crate::view::frame::Event;
use k3c_sim::state::Event as SimEvent;

/// Dauer einer Meldung in Sekunden; bei mehr als zwei wartenden die Hälfte.
pub const BANNER_SECONDS: f64 = 2.8;

/// BANNER_MAX_CHARS: längste Meldung in Zeichen; längerer Text wird mit „…“ gekürzt. Die längste Meldung der Texte
/// (Burgfall, 81 Zeichen) passt ganz (B-813); was nicht in die Zelle passt, zeichnet draw kleiner (fitPx).
pub const BANNER_MAX_CHARS: usize = 90;

/// fit_banner macht eine Meldung einzeilig (Umbrüche und Leerraum zu einem Leerzeichen) und kürzt sie auf max Zeichen.
pub fn fit_banner(text: &str, max_chars: usize) -> String {
    let one = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if one.chars().count() <= max_chars {
        return one;
    }
    let head: String = one.chars().take(max_chars - 1).collect();
    format!("{}…", head.trim_end_matches(' '))
}

/// enqueue_banner hängt eine Meldung an die Warteschlange; dieselbe Meldung steht nur einmal (laufend oder wartend),
/// eine neue ersetzt die laufende nicht, eine leere wird nicht eingereiht.
pub fn enqueue_banner(mut queue: Vec<String>, showing: &str, text: &str) -> Vec<String> {
    let fitted = fit_banner(text, BANNER_MAX_CHARS);
    if fitted.is_empty() || fitted == showing || queue.contains(&fitted) {
        return queue;
    }
    queue.push(fitted);
    queue
}

/// BannerQueue ist die Meldung eines Spielers: die laufende bis until, dahinter die wartenden.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BannerQueue {
    pub showing: String,
    pub until: f64,
    pub queue: Vec<String>,
}

impl BannerQueue {
    /// add reiht eine Meldung ein.
    pub fn add(&mut self, text: &str) {
        let queue = std::mem::take(&mut self.queue);
        self.queue = enqueue_banner(queue, &self.showing, text);
    }

    /// step schaltet zur Zeit now weiter: Ist die laufende abgelaufen, kommt die nächste (oder keine).
    pub fn step(&mut self, now: f64) {
        if !self.showing.is_empty() && now < self.until {
            return;
        }
        self.showing.clear();
        if self.queue.is_empty() {
            return;
        }
        self.showing = self.queue.remove(0);
        let mut d = BANNER_SECONDS;
        if self.queue.len() > 2 {
            d /= 2.0;
        }
        self.until = now + d;
    }
}

/// event_text ist der Text der Meldung zu einem Ereignis der Simulation ("" = keine). biome_id ist das Biom der Stufe
/// des Ereignisses (für „arrived“).
pub fn event_text(lang: &str, e: &Event, biome_id: &str) -> String {
    let d = &e.data;
    let ty = e.type_();
    let key = format!("ev.{ty}");
    let p = || -> Arg { (int_of(d, "player") + 1).into() };
    match ty {
        "dusk" | "skillPoint" | "armed" | "castleFallen" | "stageSafe" | "shield" | "attackOrdered" => {
            tr(lang, &key).to_string()
        }
        "crownStolen" | "crownTaken" | "crownRestored" | "crownPickedUp" | "bannerTaken" | "crownFallen" => {
            tf(lang, &key, &[("p", p())])
        }
        "night" | "dawn" => tf(lang, &key, &[("day", int_of(d, "day").into())]),
        "wave" => {
            if int_of(d, "count") <= 0 {
                return String::new();
            }
            tf(lang, "ev.wave", &[("n", int_of(d, "count").into())])
        }
        "chest" => tf(lang, "ev.chest", &[("p", p()), ("gold", int_of(d, "gold").into())]),
        "built" | "destroyed" => tf(lang, &key, &[("name", site_name(lang, &str_of(d, "kind")).into())]),
        "goldStolen" => tf(
            lang,
            "ev.goldStolen",
            &[("p", p()), ("amount", int_of(d, "amount").into())],
        ),
        "arrived" => name_of(lang, "biome", biome_id, &str_of(d, "name")),
        _ => String::new(),
    }
}

/// site_name ist der Name einer Bauart aus den Texten, sonst aus den Daten, sonst die ID.
pub fn site_name(lang: &str, kind: &str) -> String {
    let mut name = building_info().get(kind).map(|b| b.name.clone()).unwrap_or_default();
    if name.is_empty() {
        name = kind.to_string();
    }
    name_of(lang, "site", kind, &name)
}

/// num_of liest eine Zahl aus einem Ereignis (ganze Zahl oder `f64`; fehlt: 0).
pub fn num_of(d: &SimEvent, key: &str) -> f64 {
    match d.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        _ => 0.0,
    }
}

/// int_of: `num_of`, gerundet wie `math.Round`.
#[allow(clippy::cast_possible_truncation)]
pub fn int_of(d: &SimEvent, key: &str) -> i64 {
    num_of(d, key).round() as i64
}

/// str_of liest einen Text aus einem Ereignis ("" ohne Text).
pub fn str_of(d: &SimEvent, key: &str) -> String {
    match d.get(key) {
        Some(serde_json::Value::String(s)) => s.clone(),
        _ => String::new(),
    }
}
