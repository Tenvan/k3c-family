//! Port von `native/k3c-game/actors/pose.go`.
//!
//! Pose der Rig-Figuren im festen Takt: Listen schreibt einmal je Frame Animation und Umhang aller Rig-Figuren fort,
//! mit dem Zeitschritt aus Frame.Clock (ein Takt = 1/30 s), unabhängig davon, ob und wie oft Draw die Figur zeichnet.
//! So ist das Bild eines Frames bildgleich, egal wie viele Draws je Update laufen oder welche Figur vorher sichtbar war.
//!
//! Abweichung von Go: Eine neue Rig-Figur hat in Listen noch keinen Actor (er braucht die Zeichenfläche, siehe
//! figures.rs); `ensure_actors` ruft `advance` für sie gleich nach dem Anlegen, im selben Frame wie in Go.

use super::Actors;
use super::anim::anim_time;
use super::figures::{Figure, WarnKey};

/// maxPoseStep begrenzt den Zeitschritt zwischen zwei Posen; ein größerer Sprung lässt den Umhang neu einschwingen.
pub const MAX_POSE_STEP: f64 = 0.25;

// Einschwingen des Umhangs einer neuen Figur: settleSteps Schritte zu je settleStep Sekunden. settleStep entspricht
// der Obergrenze der Umhang-Schritte je Update im Actor (8 × 1/120 s), so geht keine Zeit verloren; 8 Schritte statt
// 30 begrenzen die Spitze, wenn viele Figuren zugleich erscheinen (Laden, Welle).
pub const SETTLE_STEPS: i64 = 8;
pub const SETTLE_STEP: f64 = 8.0 / 120.0;

impl Actors {
    /// advanceAll schreibt die Pose aller Rig-Figuren fort (Reihenfolge egal: jede Figur rechnet nur für sich).
    pub fn advance_all(&mut self) {
        let now = self.now;
        let mut errs: Vec<(WarnKey, String)> = Vec::new();
        for fg in self.figs.values_mut() {
            if fg.actor.is_none() || fg.broken {
                continue;
            }
            if let Err(e) = advance(now, fg) {
                fg.posed = false;
                errs.push((
                    WarnKey {
                        what: fg.look.asset.clone(),
                        name: fg.pose.anim.clone(),
                    },
                    e,
                ));
            }
        }
        for (k, e) in errs {
            self.warn(k, &e);
        }
    }
}

/// advance rechnet die Pose des Actors für die Darstellungszeit `now`: höchstens einmal je Zeit und Pose; der Umhang
/// läuft mit der vergangenen Zeit seit der letzten Pose. Beim ersten Mal (oder nach einem großen Sprung) schwingt er
/// mit SETTLE_STEPS festen Schritten ein, damit er nicht senkrecht hängt.
pub fn advance(now: f64, fg: &mut Figure) -> Result<(), String> {
    if fg.posed && !fg.dirty && fg.posed_at == now {
        return Ok(());
    }
    let target = anim_time(&fg.pose, now, fg.shift);
    let (dt, had) = (now - fg.posed_at, fg.posed);
    fg.posed = true;
    fg.dirty = false;
    fg.posed_at = now;
    let Some(a) = fg.actor.as_mut() else {
        return Ok(());
    };
    if had && (0.0..=MAX_POSE_STEP).contains(&dt) {
        a.t = target - dt * a.tempo;
        return a.update(dt).map_err(|e| e.to_string());
    }
    a.reset_cloak();
    #[allow(clippy::cast_precision_loss)]
    {
        a.t = target - SETTLE_STEPS as f64 * SETTLE_STEP * a.tempo;
    }
    for _ in 0..SETTLE_STEPS {
        a.update(SETTLE_STEP).map_err(|e| e.to_string())?;
    }
    Ok(())
}
