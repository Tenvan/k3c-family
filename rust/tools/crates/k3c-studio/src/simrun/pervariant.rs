//! Option „je Variante“ (Port von `pervariant.go`, B-662): Ein Auftrag mit n festen Varianten wird n + 1 Läufe in der
//! Warteschlange: ein Basis-Lauf ohne Varianten und je Variante einer, mit gleichen Seeds und gemeinsamem Label-Präfix.

use super::spec::{MAX_LABEL, Spec};

/// Präfix der Labels, wenn der Auftrag keines hat.
const PER_VARIANT_PREFIX: &str = "je Variante";

impl Spec {
    /// Teilt den Auftrag: zuerst der Basis-Lauf, dann je Variante ein Lauf; keiner trägt `per_variant` weiter.
    pub(super) fn per_variant_specs(&self) -> Vec<Spec> {
        let prefix = if self.label.is_empty() {
            PER_VARIANT_PREFIX
        } else {
            &self.label
        };
        let mut base = self.clone();
        base.per_variant = false;
        base.vary = Vec::new();
        base.label = variant_label(prefix, "Basis");
        let ones = self.vary.iter().map(|v| Spec {
            vary: vec![v.clone()],
            label: variant_label(prefix, v),
            ..base.clone()
        });
        std::iter::once(base.clone()).chain(ones).collect()
    }
}

/// „<präfix> · <teil>“ in höchstens [`MAX_LABEL`] Zeichen; zu lang kürzt zuerst das Präfix (mindestens 8 Zeichen
/// bleiben), dann das Ende.
fn variant_label(prefix: &str, part: &str) -> String {
    const SEP: &str = " · ";
    let tail = format!("{SEP}{part}");
    let room = MAX_LABEL.saturating_sub(tail.chars().count()).max(8);
    clip_chars(&format!("{}{tail}", clip_chars(prefix, room)), MAX_LABEL)
}

/// Kürzt auf höchstens `n` Zeichen, das letzte ist dann „…“.
fn clip_chars(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        return s.to_owned();
    }
    let mut out: String = s.chars().take(n.saturating_sub(1)).collect();
    out.push('…');
    out
}

#[cfg(test)]
#[path = "pervariant_tests.rs"]
mod tests;
