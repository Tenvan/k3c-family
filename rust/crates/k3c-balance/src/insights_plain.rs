//! Klartext des Berichts (Port von `engine/balance/insights_plain.go`): Jede Zahl bekommt Einheit und Bezug (Läufe,
//! Korridor, Prozentpunkte), die Richtung steht als Wort („zu leicht“, „zu schwer“) und jede Stellschraube als Plan mit
//! aktuellem Wert, Bereich und Startvorschlag.

use serde::{Deserialize, Serialize};

use crate::evaluate::{STATUS_INVALID, STATUS_NARROW, STATUS_VIOLATED, bounds};
use crate::figures::round1;
use crate::hints::Lever;
use crate::insights::TargetInsight;
use crate::insights_next::{TUNE_RANGE, signed};
use crate::insights_text::step_text;
use crate::summary_text::num;
use crate::targets::KIND_SHARE;
use crate::tuneparam::parse_tune_param;

/// Was ein Wert über dem Korridor für das Spiel heißt; unbekannte Kennzahlen bleiben bei „zu hoch“ (Go: `highMeans`).
fn high_means(measure: &str) -> Option<&'static str> {
    match measure {
        "castleHeld" | "firstWallBeforeDusk1" | "endbossDefeated" | "caveBeforeDay6" => Some("zu leicht"),
        "endbossDay" | "destroyedPerWave" => Some("zu schwer"),
        _ => None,
    }
}

/// Was bei einem Anteil „eingetreten“ ist (Satzteil nach „In 5 von 5 Läufen …“; Go: `measureClause`).
fn measure_clause(measure: &str) -> Option<&'static str> {
    match measure {
        "castleHeld" => Some("hielt die Burg Nacht 1 bis 5"),
        "firstWallBeforeDusk1" => Some("stand die erste Mauer vor der Dämmerung von Tag 1"),
        "endbossDefeated" => Some("wurde der Endboss besiegt"),
        "caveBeforeDay6" => Some("wurde die Höhle vor Tag 6 erreicht"),
        _ => None,
    }
}

/// Einheit eines Medians (Go: `measureUnit`).
fn measure_unit(measure: &str) -> &'static str {
    match measure {
        "endbossDay" => "Tage",
        "destroyedPerWave" => "Gebäude",
        _ => "",
    }
}

/// Die Stellschraube mit Zahlen zum Eintragen: aktueller Wert in data/, Probierbereich und Startvorschlag. `known` ist
/// falsch, wenn der Bereich nicht seriös abschätzbar ist (Wert 0 oder nicht lesbar); `text` sagt das dann.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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

impl TargetInsight {
    fn is_share(&self) -> bool {
        self.kind == KIND_SHARE
    }

    /// „Pkt.“ bei Anteilen (Prozentpunkte), sonst die Einheit des Medians (Go: `unitShort`).
    fn unit_short(&self) -> &'static str {
        if self.is_share() {
            "Pkt."
        } else {
            measure_unit(&self.measure)
        }
    }

    /// „Prozentpunkte“ bei Anteilen, sonst die Einheit des Medians (Go: `unitLong`).
    fn unit_long(&self) -> &'static str {
        if self.is_share() {
            "Prozentpunkte"
        } else {
            measure_unit(&self.measure)
        }
    }

    /// Wahr, wenn der Wert über der Obergrenze liegt (Go: `above`).
    pub fn above(&self) -> bool {
        self.upper.is_some_and(|u| self.value > u)
    }

    /// Wahr, wenn der Wert unter der Untergrenze liegt (Go: `below`).
    pub fn below(&self) -> bool {
        self.lower.is_some_and(|l| self.value < l)
    }

    /// Abstand des Werts zur verletzten Grenze in der Einheit des Werts (Go: `gapValue`).
    fn gap_value(&self) -> f64 {
        match (self.lower, self.upper) {
            (_, Some(u)) if self.above() => self.value - u,
            (Some(l), _) if self.below() => l - self.value,
            _ => 0.0,
        }
    }

    /// Abstand im Korridor zur nächsten wirksamen Grenze (nicht die natürlichen 0 und 100 % bei Anteilen) und deren
    /// Name (Go: `reserve`).
    fn reserve(&self) -> Option<(f64, &'static str)> {
        let (lo, hi) = bounds(&self.entry.verdict.target);
        let mut best: Option<(f64, &'static str)> = None;
        if let Some(lo) = lo
            && self.value - lo < best.map_or(f64::INFINITY, |b| b.0)
        {
            best = Some((self.value - lo, "unteren"));
        }
        if let Some(hi) = hi
            && hi - self.value < best.map_or(f64::INFINITY, |b| b.0)
        {
            best = Some((hi - self.value, "oberen"));
        }
        best
    }

    /// „zu leicht“ / „zu schwer“ bzw. „zu hoch“ / „zu niedrig“ für die Richtung der Abweichung (Go: `word`).
    pub fn word(&self) -> &'static str {
        if self.above() {
            return high_means(&self.measure).unwrap_or("zu hoch");
        }
        if self.below() {
            return match high_means(&self.measure) {
                Some("zu leicht") => "zu schwer",
                Some("zu schwer") => "zu leicht",
                _ => "zu niedrig",
            };
        }
        ""
    }

    /// Betrag mit Einheit („1 Prozentpunkt“, „10 Prozentpunkte“, „2 Gebäude“; Go: `amount`).
    fn amount(&self, v: f64) -> String {
        let mut unit = self.unit_long();
        if self.is_share() && round1(v) == 1.0 {
            unit = "Prozentpunkt";
        }
        format!("{} {unit}", num(round1(v))).trim().to_string()
    }

    /// Die Messgröße als Satz-Anfang; bei Anteilen „In c von n Läufen …“ (Go: `what`).
    fn what(&self) -> String {
        if !self.is_share() {
            return format!("Median von „{}“: {}.", self.name, self.amount(self.value));
        }
        let clause = measure_clause(&self.measure)
            .map(str::to_string)
            .unwrap_or_else(|| format!("traf „{}“ zu", self.name));
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

    /// Der Sollwert als Satz (Go: `target`).
    fn target(&self) -> String {
        let of = if self.is_share() { " der Läufe" } else { "" };
        let unit = self.entry.unit();
        match (self.lower, self.upper) {
            (Some(_), Some(_)) => format!("Sollwert: {}{of}.", self.entry.bounds_text()),
            (Some(l), None) => format!("Sollwert: mindestens {}{unit}{of}.", num(l)),
            (None, None) => "Kein Sollwert hinterlegt.".into(),
            (None, Some(u)) => format!("Sollwert: höchstens {}{unit}{of}.", num(u)),
        }
    }

    /// Satz zur Wirkung auf das Ziel; ehrlich, dass das Ausmaß nicht berechenbar ist (Go: `effect`).
    fn effect(&self) -> String {
        let way = if self.above() {
            "sinken"
        } else if self.below() {
            "steigen"
        } else {
            return format!(
                "Wirkung: „{}“ ({}) soll sich von der knappen Grenze entfernen, ohne den Korridor {} zu verlassen. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.",
                self.name,
                self.entry.value_text(),
                self.entry.bounds_text()
            );
        };
        format!(
            "Wirkung: „{}“ soll von {} Richtung Korridor {} {way}. Wie weit, ist nicht berechenbar; erst der volle Lauf zeigt es.",
            self.name,
            self.entry.value_text(),
            self.entry.bounds_text()
        )
    }

    /// Baut den Plan zu einem Hebel; `base` und `is_int` kommen aus data/ (`base == None`: nicht lesbar; Go: `planFor`).
    pub(crate) fn plan_for(&self, l: &Lever, base: Option<(f64, bool)>) -> LeverPlan {
        let mut p = LeverPlan {
            path: l.path.clone(),
            label: l.label.clone(),
            ..LeverPlan::default()
        };
        let Some((base, is_int)) = base else {
            p.short = "Aktueller Wert in data/ nicht lesbar: Bereich nicht abschätzbar, Wert von Hand wählen.".into();
            p.text = p.short.clone();
            return p;
        };
        let tp = crate::tuneparam::TuneParam {
            base,
            int: is_int,
            ..Default::default()
        };
        p.current = tp.format(base);
        let Some((start, lo, hi)) = lever_range(base, is_int, &l.direction, l.step) else {
            p.short = format!(
                "Aktuell {}. Bereich nicht seriös abschätzbar (Wert 0 oder ein Schritt von {} % ändert ihn nicht): Wert von Hand wählen.",
                p.current,
                step_text(l.step)
            );
            p.text = format!("{} {}", p.short, self.effect());
            return p;
        };
        p.known = true;
        (p.min, p.max, p.start) = (tp.format(lo), tp.format(hi), tp.format(start));
        let way = if l.direction == "-" { "runter" } else { "rauf" };
        p.short = format!(
            "Aktuell {}. Probierbereich {} … {} (1 bis {} Schritte à {} {way}). Startvorschlag: {}.",
            p.current,
            p.min,
            p.max,
            TUNE_RANGE as i64,
            step_text(l.step),
            p.start
        );
        p.text = format!("{} {}", p.short, self.effect());
        p
    }
}

/// Die Spalte „Abstand“ der Tabelle: „10 Pkt. zu hoch“ außerhalb, „5 Pkt. Reserve“ im Korridor (Go: `gapText`).
pub fn gap_text(t: &TargetInsight) -> String {
    if t.distance.is_none() {
        return "–".into();
    }
    if t.status == STATUS_VIOLATED {
        let dir = if t.above() { "zu hoch" } else { "zu niedrig" };
        let n = format!("{} {}", num(round1(t.gap_value())), t.unit_short());
        return format!("{} {dir}", n.trim());
    }
    if let Some((g, _)) = t.reserve() {
        let n = format!("{} {}", num(round1(g)), t.unit_short());
        return format!("{} Reserve", n.trim());
    }
    "keine Grenze nah".into()
}

/// Klartext zu einem Ziel in drei Sätzen: Messwert, Sollwert, Lage (Go: `plain`).
pub fn plain(t: &TargetInsight) -> String {
    if t.status == STATUS_INVALID {
        return format!(
            "{} Kein belastbarer Messwert: {} Läufe abgebrochen oder ohne Ergebnis.",
            t.target(),
            t.invalid_seeds.len()
        );
    }
    let s = format!("{} {} ", t.what(), t.target());
    if t.above() || t.below() {
        let over = if t.above() { "über" } else { "unter" };
        return format!(
            "{s}Damit {} {over} dem Korridor, also {}.",
            t.amount(t.gap_value()),
            t.word()
        );
    }
    match t.reserve() {
        None => format!("{s}Damit im Korridor."),
        Some((g, side)) if t.status == STATUS_NARROW => {
            format!("{s}Im Korridor, aber knapp: nur {} bis zur {side} Grenze.", t.amount(g))
        }
        Some((g, side)) => format!("{s}Im Korridor, {} von der {side} Grenze entfernt.", t.amount(g)),
    }
}

/// Vergleich mit der Baseline in Worten; leer ohne Baseline (Go: `trendPlain`).
pub fn trend_plain(t: &TargetInsight) -> String {
    let Some(tr) = &t.trend else { return String::new() };
    let unit = t.entry.unit();
    let before = num(tr.before);
    if tr.delta.abs() < 1e-9 {
        return format!("Unverändert gegenüber der Baseline ({before}{unit}).");
    }
    let (d, near) = (signed(tr.delta), if tr.better { "näher am" } else { "weiter weg vom" });
    format!("Gegenüber der Baseline ({before}{unit}) {near} Korridor: {d}{unit}.")
}

/// Startvorschlag (ein Schritt) und Probierbereich (bis zu `TUNE_RANGE` Schritte) ab dem Wert in data/; `None` bei Wert 0
/// oder wenn der Schritt (bei ganzen Zahlen gerundet) den Wert nicht ändert (Go: `leverRange`).
pub fn lever_range(base: f64, is_int: bool, dir: &str, step: f64) -> Option<(f64, f64, f64)> {
    let sign = if dir == "-" { -1.0 } else { 1.0 };
    if base == 0.0 || step <= 0.0 {
        return None;
    }
    let pick = |n: f64| -> f64 {
        let mut v = base * (1.0 + sign * n * step);
        if sign < 0.0 {
            v = v.max(0.0);
        }
        if !is_int {
            return v;
        }
        if sign > 0.0 {
            return v.ceil().max(base + 1.0);
        }
        v.floor().min(base - 1.0).max(0.0)
    };
    let (start, far) = (pick(1.0), pick(TUNE_RANGE));
    if start == base {
        return None;
    }
    Some((start, base.min(far), base.max(far)))
}

/// Ein Plan je Hebel; der Wert kommt aus data/ (Go: `plans`).
pub fn plans(t: &TargetInsight) -> Vec<LeverPlan> {
    t.levers
        .iter()
        .map(|l| {
            let base = parse_tune_param(&format!("{}=0..1", l.path))
                .ok()
                .map(|p| (p.base, p.int));
            t.plan_for(l, base)
        })
        .collect()
}
