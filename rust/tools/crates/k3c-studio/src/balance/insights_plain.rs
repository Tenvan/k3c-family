//! Klartext des Berichts: Jede Zahl bekommt Einheit und Bezug (Läufe, Korridor, Prozentpunkte), die Richtung steht
//! als Wort („zu leicht“, „zu schwer“) und jede Stellschraube als Plan mit aktuellem Wert, Bereich und Startvorschlag.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::evaluate::{STATUS_INVALID, STATUS_NARROW, STATUS_VIOLATED};
use super::figures::round1;
use super::hints::Lever;
use super::insights::TargetInsight;
use super::insights_next::{TUNE_RANGE, signed};
use super::insights_text::step_text;
use super::summary::num;
use super::targets::KIND_SHARE;
use super::tuneparam::{TuneParam, parse_tune_param};

/// Zahlen zu einem Hebel: aktueller Wert in `data/`, Probierbereich und Startvorschlag. `known`: falsch, wenn der
/// Bereich nicht seriös abschätzbar ist (Wert 0 oder nicht lesbar); `text` sagt das dann.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LeverPlan {
    pub path: String,
    pub label: String,
    pub known: bool,
    pub current: String,
    pub min: String,
    pub max: String,
    pub start: String,
    /// Ohne Wirkungssatz: aktueller Wert, Bereich, Startvorschlag.
    pub short: String,
    pub text: String,
}

/// Was ein Wert über dem Korridor für das Spiel heißt; unbekannte Kennzahlen bleiben bei „zu hoch“.
fn high_means(measure: &str) -> Option<&'static str> {
    match measure {
        "castleHeld" | "firstWallBeforeDusk1" | "endbossDefeated" | "caveBeforeDay6" => Some("zu leicht"),
        "endbossDay" | "destroyedPerWave" => Some("zu schwer"),
        _ => None,
    }
}

/// Was bei einem Anteil „eingetreten“ ist (Satzteil nach „In 5 von 5 Läufen …“).
fn measure_clause(measure: &str) -> Option<&'static str> {
    match measure {
        "castleHeld" => Some("hielt die Burg Nacht 1 bis 5"),
        "firstWallBeforeDusk1" => Some("stand die erste Mauer vor der Dämmerung von Tag 1"),
        "endbossDefeated" => Some("wurde der Endboss besiegt"),
        "caveBeforeDay6" => Some("wurde die Höhle vor Tag 6 erreicht"),
        _ => None,
    }
}

/// Einheit eines Medians.
fn measure_unit(measure: &str) -> &'static str {
    match measure {
        "endbossDay" => "Tage",
        "destroyedPerWave" => "Gebäude",
        _ => "",
    }
}

impl TargetInsight {
    fn is_share(&self) -> bool {
        self.kind == KIND_SHARE
    }

    /// „Pkt.“ bei Anteilen (Prozentpunkte), sonst die Einheit des Medians.
    fn unit_short(&self) -> &'static str {
        if self.is_share() {
            "Pkt."
        } else {
            measure_unit(&self.measure)
        }
    }

    /// „Prozentpunkte“ bei Anteilen, sonst die Einheit des Medians.
    fn unit_long(&self) -> &'static str {
        if self.is_share() {
            "Prozentpunkte"
        } else {
            measure_unit(&self.measure)
        }
    }

    /// Wahr, wenn der Wert über der Obergrenze liegt.
    pub(crate) fn above(&self) -> bool {
        self.upper.is_some_and(|hi| self.value > hi)
    }

    /// Wahr, wenn der Wert unter der Untergrenze liegt.
    fn below(&self) -> bool {
        self.lower.is_some_and(|lo| self.value < lo)
    }

    /// Abstand des Werts zur verletzten Grenze in der Einheit des Werts.
    fn gap_value(&self) -> f64 {
        if let Some(hi) = self.upper.filter(|hi| self.value > *hi) {
            return self.value - hi;
        }
        if let Some(lo) = self.lower.filter(|lo| self.value < *lo) {
            return lo - self.value;
        }
        0.0
    }

    /// Abstand im Korridor zur nächsten wirksamen Grenze (nicht die natürlichen 0 und 100 % bei Anteilen) und deren
    /// Name.
    fn reserve(&self) -> Option<(f64, &'static str)> {
        let (lo, hi) = self.entry.verdict.target.bounds();
        let (mut best, mut side) = (f64::INFINITY, "");
        if let Some(lo) = lo.filter(|lo| self.value - lo < best) {
            (best, side) = (self.value - lo, "unteren");
        }
        if let Some(hi) = hi.filter(|hi| hi - self.value < best) {
            (best, side) = (hi - self.value, "oberen");
        }
        (!best.is_infinite()).then_some((best, side))
    }

    /// „zu leicht“ / „zu schwer“ bzw. „zu hoch“ / „zu niedrig“ für die Richtung der Abweichung.
    pub(crate) fn word(&self) -> &'static str {
        if self.above() {
            return high_means(&self.measure).unwrap_or("zu hoch");
        }
        if !self.below() {
            return "";
        }
        match high_means(&self.measure) {
            Some("zu leicht") => "zu schwer",
            Some("zu schwer") => "zu leicht",
            _ => "zu niedrig",
        }
    }

    /// Betrag mit Einheit („1 Prozentpunkt“, „10 Prozentpunkte“, „2 Gebäude“).
    fn amount(&self, v: f64) -> String {
        let mut unit = self.unit_long();
        if self.is_share() && round1(v) == 1.0 {
            unit = "Prozentpunkt";
        }
        format!("{} {unit}", num(round1(v))).trim().to_owned()
    }

    /// Spalte „Abstand“ der Tabelle: „10 Pkt. zu hoch“ außerhalb, „5 Pkt. Reserve“ im Korridor.
    pub(crate) fn gap_text(&self) -> String {
        if self.distance.is_none() {
            return "–".to_owned();
        }
        let with_unit = |v: f64| format!("{} {}", num(round1(v)), self.unit_short()).trim().to_owned();
        if self.status == STATUS_VIOLATED {
            let direction = if self.above() { "zu hoch" } else { "zu niedrig" };
            return format!("{} {direction}", with_unit(self.gap_value()));
        }
        match self.reserve() {
            Some((g, _)) => format!("{} Reserve", with_unit(g)),
            None => "keine Grenze nah".to_owned(),
        }
    }

    /// Die Messgröße als Satz-Anfang; bei Anteilen „In c von n Läufen …“.
    pub(crate) fn what(&self) -> String {
        if !self.is_share() {
            return format!("Median von „{}“: {}.", self.name, self.amount(self.value));
        }
        let clause = measure_clause(&self.measure).map_or_else(|| format!("traf „{}“ zu", self.name), str::to_owned);
        if self.runs <= 0 {
            return format!("Bei {} % der Läufe {clause}.", num(round1(self.value)));
        }
        let c = (self.value * self.runs as f64 / 100.0).round() as i64;
        format!(
            "In {c} von {} Läufen {clause} ({} %).",
            self.runs,
            num(round1(self.value))
        )
    }

    /// Der Sollwert als Satz.
    pub(crate) fn target(&self) -> String {
        let of = if self.is_share() { " der Läufe" } else { "" };
        let unit = self.entry.unit();
        match (self.lower, self.upper) {
            (Some(_), Some(_)) => format!("Sollwert: {}{of}.", self.bounds_text()),
            (Some(lo), None) => format!("Sollwert: mindestens {}{unit}{of}.", num(lo)),
            (None, None) => "Kein Sollwert hinterlegt.".to_owned(),
            (None, Some(hi)) => format!("Sollwert: höchstens {}{unit}{of}.", num(hi)),
        }
    }

    /// Klartext zu einem Ziel in drei Sätzen: Messwert, Sollwert, Lage.
    pub(crate) fn plain(&self) -> String {
        if self.status == STATUS_INVALID {
            return format!(
                "{} Kein belastbarer Messwert: {} Läufe abgebrochen oder ohne Ergebnis.",
                self.target(),
                self.invalid_seeds.len()
            );
        }
        let s = format!("{} {} ", self.what(), self.target());
        if self.above() || self.below() {
            let side = if self.above() { "über" } else { "unter" };
            let gap = self.amount(self.gap_value());
            return format!("{s}Damit {gap} {side} dem Korridor, also {}.", self.word());
        }
        match self.reserve() {
            None => format!("{s}Damit im Korridor."),
            Some((g, side)) if self.status == STATUS_NARROW => format!(
                "{s}Im Korridor, aber knapp: nur {} bis zur {side} Grenze.",
                self.amount(g)
            ),
            Some((g, side)) => format!("{s}Im Korridor, {} von der {side} Grenze entfernt.", self.amount(g)),
        }
    }

    /// Vergleich mit der Baseline in Worten; leer ohne Baseline.
    pub(crate) fn trend_plain(&self) -> String {
        let Some(trend) = &self.trend else {
            return String::new();
        };
        let unit = self.entry.unit();
        let before = num(trend.before);
        let d = trend.delta;
        if d.abs() < 1e-9 {
            return format!("Unverändert gegenüber der Baseline ({before}{unit}).");
        }
        let way = if trend.better {
            "näher am Korridor"
        } else {
            "weiter weg vom Korridor"
        };
        format!("Gegenüber der Baseline ({before}{unit}) {way}: {}{unit}.", signed(d))
    }

    /// Satz zur Wirkung auf das Ziel; das Ausmaß ist nicht berechenbar.
    fn effect(&self) -> String {
        let (name, value, bounds) = (&self.name, self.value_text(), self.bounds_text());
        if !self.above() && !self.below() {
            return format!(
                "Wirkung: „{name}“ ({value}) soll sich von der knappen Grenze entfernen, ohne den Korridor {bounds} zu \
                 verlassen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es."
            );
        }
        let way = if self.above() { "sinken" } else { "steigen" };
        format!(
            "Wirkung: „{name}“ soll von {value} Richtung Korridor {bounds} {way}. Wie weit, ist nicht berechenbar; erst \
             der volle Lauf zeigt es."
        )
    }

    /// Der Plan zu einem Hebel; `data` ist (Wert, ganzzahlig) aus `data/`, `None`: nicht lesbar.
    pub(crate) fn plan_for(&self, lever: &Lever, data: Option<(f64, bool)>) -> LeverPlan {
        let mut p = LeverPlan {
            path: lever.path.clone(),
            label: lever.label.clone(),
            ..LeverPlan::default()
        };
        let Some((base, int)) = data else {
            "Aktueller Wert in data/ nicht lesbar: Bereich nicht abschätzbar, Wert von Hand wählen."
                .clone_into(&mut p.short);
            p.text.clone_from(&p.short);
            return p;
        };
        let param = TuneParam {
            base,
            int,
            ..TuneParam::default()
        };
        p.current = param.format(base);
        let step = step_text(lever.step);
        let Some((start, lo, hi)) = lever_range(base, int, &lever.direction, lever.step) else {
            p.short = format!(
                "Aktuell {}. Bereich nicht seriös abschätzbar (Wert 0 oder ein Schritt von {step} % ändert ihn nicht): \
                 Wert von Hand wählen.",
                p.current
            );
            p.text = format!("{} {}", p.short, self.effect());
            return p;
        };
        p.known = true;
        (p.min, p.max, p.start) = (param.format(lo), param.format(hi), param.format(start));
        let way = if lever.direction == "-" { "runter" } else { "rauf" };
        p.short = format!(
            "Aktuell {}. Probierbereich {} … {} (1 bis {} Schritte à {step} {way}). Startvorschlag: {}.",
            p.current, p.min, p.max, TUNE_RANGE as i64, p.start
        );
        p.text = format!("{} {}", p.short, self.effect());
        p
    }

    /// Ein Plan je Hebel; der Wert kommt aus `data/` des Checkouts `root`.
    pub(crate) fn plans(&self, root: &Path) -> Vec<LeverPlan> {
        self.levers
            .iter()
            .map(|l| {
                let data = parse_tune_param(root, &format!("{}=0..1", l.path)).ok();
                self.plan_for(l, data.map(|p| (p.base, p.int)))
            })
            .collect()
    }
}

/// Go `math.Max(0, v)`: auch aus `-0` wird `0`.
fn at_least_zero(v: f64) -> f64 {
    if v > 0.0 { v } else { 0.0 }
}

/// (Startvorschlag, von, bis): ein Schritt und bis zu [`TUNE_RANGE`] Schritte ab dem Wert in `data/`. `None` bei Wert 0
/// oder wenn der Schritt (bei ganzen Zahlen gerundet) den Wert nicht ändert.
pub(crate) fn lever_range(base: f64, int: bool, direction: &str, step: f64) -> Option<(f64, f64, f64)> {
    let sign = if direction == "-" { -1.0 } else { 1.0 };
    if base == 0.0 || step <= 0.0 {
        return None;
    }
    let pick = |n: f64| {
        let mut v = base * (1.0 + sign * n * step);
        if sign < 0.0 {
            v = at_least_zero(v);
        }
        if !int {
            v
        } else if sign > 0.0 {
            v.ceil().max(base + 1.0)
        } else {
            at_least_zero(v.floor().min(base - 1.0))
        }
    };
    let (start, far) = (pick(1.0), pick(TUNE_RANGE));
    (start != base).then(|| (start, base.min(far), base.max(far)))
}
