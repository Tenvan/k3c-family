//! Port von native/k3c-rig/rig/keyframes.go: Winkel und Sichtbarkeit aus Keyframes.

use super::load::{Animation, Keyframe, Rig};

/// `keyframe_ids` nennt alle Teile, die in den Keyframes einen Winkel haben (sortiert, einmal beim Laden).
pub(super) fn keyframe_ids(kf: &[Keyframe]) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for f in kf {
        for id in f.angles.keys() {
            if !ids.contains(id) {
                ids.push(id.clone());
            }
        }
    }
    ids.sort_unstable();
    ids
}

impl Rig {
    /// `add_keyframes` addiert die Winkel der Keyframes bei Phase p (0..1) auf angles: linear und zyklisch, Teile ohne
    /// Wert in einem Keyframe zählen dort 0 (tools/paper/render/pose.ts › keyframeWinkel). Die Keyframes sind nach t
    /// sortiert.
    pub(super) fn add_keyframes(&self, a: &Animation, p: f64, angles: &mut [f64]) {
        let kf = &a.keyframes;
        if kf.is_empty() {
            return;
        }
        let mut last: i64 = -1;
        for (j, f) in kf.iter().enumerate() {
            if f.t <= p {
                last = j as i64;
            }
        }
        let tail = &kf[kf.len() - 1];
        let (mut from, mut from_t) = (tail, tail.t - 1.0);
        if last >= 0 {
            (from, from_t) = (&kf[last as usize], kf[last as usize].t);
        }
        let (mut to, mut to_t) = (&kf[0], kf[0].t + 1.0);
        if ((last + 1) as usize) < kf.len() {
            (to, to_t) = (&kf[(last + 1) as usize], kf[(last + 1) as usize].t);
        }
        let mut share = 0.0;
        if to_t != from_t {
            share = (p - from_t) / (to_t - from_t);
        }
        for id in &a.ids {
            if let Some(&i) = self.index.get(id) {
                let va = from.angles.get(id).copied().unwrap_or(0.0);
                let vb = to.angles.get(id).copied().unwrap_or(0.0);
                angles[i] += va + (vb - va) * share;
            }
        }
    }

    /// `keyframe_hidden` sagt, ob ein Teil bei Phase p ausgeblendet ist: Es gilt der letzte Keyframe mit
    /// sichtbar-Eintrag und t ≤ p, sonst (zyklisch) der letzte überhaupt; ohne Eintrag sichtbar
    /// (pose.ts › keyframeAusgeblendet).
    pub fn keyframe_hidden(&self, name: &str, id: &str, p: f64) -> bool {
        let (mut visible, mut before) = (true, false);
        let Some(anim) = self.animations.get(name) else {
            return !visible;
        };
        for f in &anim.keyframes {
            let Some(&v) = f.visible.get(id) else {
                continue;
            };
            if before && f.t > p {
                continue;
            }
            visible = v;
            before = before || f.t <= p;
        }
        !visible
    }
}
