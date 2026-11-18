//! Port von `native/k3c-game/menu/testing_text.go`: der angezeigte Wert je Zeile der Seite Testing.

use super::testing::{
    TST_BUILD, TST_FACTOR, TST_GROWTH, TST_MINUTES, TST_MODE, TST_PRESET, TST_SEED, TST_TIMESCALE, TST_TROOPS,
    TST_WAVES, TestingScreen, stress_field,
};
use super::texts::{TextKey, tr};
use crate::view::stress::GROWTH_EXP;

impl TestingScreen {
    /// Der angezeigte Wert der Zeile `r` der Seite Testing.
    pub fn value_text(&self, lang: &str, r: usize) -> String {
        let t = &self.cur;
        match r {
            TST_PRESET => match usize::try_from(self.preset).ok().and_then(|i| self.presets.get(i)) {
                Some(p) => p.name(lang).to_string(),
                None => tr(lang, TextKey::TstOwn),
            },
            TST_SEED if t.seed == 0 => tr(lang, TextKey::SeedAuto),
            TST_GROWTH => tr(
                lang,
                if t.growth == GROWTH_EXP {
                    TextKey::TstExp
                } else {
                    TextKey::TstLinear
                },
            ),
            TST_FACTOR => {
                // Rust `{}` zeigt 1.5, 2 und 0.25 wie Gos `FormatFloat(f, 'f', -1, 64)`
                let sign = if t.growth == GROWTH_EXP { "×" } else { "+" };
                format!("{sign} {}", t.factor)
            }
            TST_MODE => tr(
                lang,
                if t.pure {
                    TextKey::TstModePure
                } else {
                    TextKey::TstModeNormal
                },
            ),
            TST_TIMESCALE => format!("{}×", t.timescale),
            _ => {
                let mut c = t.clone();
                let v = stress_field(&mut c, r).map_or(0, |p| *p);
                let none = matches!(r, TST_WAVES | TST_MINUTES | TST_BUILD | TST_TROOPS);
                if none && v == 0 {
                    tr(lang, TextKey::TstNone)
                } else {
                    v.to_string()
                }
            }
        }
    }
}
