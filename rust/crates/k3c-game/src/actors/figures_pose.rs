//! Zweiter Teil von `native/k3c-game/actors/figures.go` (ab `figure`): Figuren anlegen, Actors und Pose einstellen,
//! Melden. Eigene Datei, damit `figures.rs` unter 400 Zeilen bleibt (Leitfaden § 2).

use k3c_platform::Canvas;

use super::Actors;
use super::anim::{ANIM_IDLE, GAIT_STAND, Pose, Want, next, shift};
use super::catalog::Look;
use super::figures::{Figure, Key, WarnKey};
use super::pose::advance;
use crate::art::{Actor, State};

/// defaultOnce ist die Länge einer Einmal-Animation, wenn weder Rig noch Sheet sie nennen (ohne Grafik, Tests).
pub const DEFAULT_ONCE: f64 = 0.5;

impl Actors {
    /// figure liefert die Figur zu k (aus der Liste genommen; der Aufrufer legt sie zurück) und legt sie an, wenn sie
    /// neu ist oder sich ihr Look geändert hat (Landstreicher wird Bauer). mounted: Monarch auf dem Reittier des
    /// Katalogs.
    pub fn figure(&mut self, k: Key, look: Look, mounted: bool) -> Figure {
        let mut fg = match self.figs.remove(&k) {
            Some(f) if f.look == look => f,
            _ => Figure {
                alpha: look.alpha,
                pending: !look.sheet && self.lib.is_some(),
                look,
                actor: None,
                broken: false,
                mounted,
                pose: Pose::default(),
                left: false,
                shift: shift(k.id),
                lift: 0.0,
                ui: 1.0,
                generation: 0,
                known: false,
                x: 0.0,
                cd: 0.0,
                posed: false,
                dirty: false,
                posed_at: 0.0,
            },
        };
        fg.generation = self.generation;
        fg
    }

    /// ensureActors legt die Rig-Actors der Figuren an, die noch keinen haben (Zeichenfläche für die Texturen), und
    /// stellt ihre Pose am Actor ein.
    pub fn ensure_actors(&mut self, canvas: &mut dyn Canvas) {
        let keys: Vec<Key> = self.figs.iter().filter(|(_, f)| f.pending).map(|(k, _)| *k).collect();
        for k in keys {
            let Some(mut fg) = self.figs.remove(&k) else {
                continue;
            };
            fg.pending = false;
            let (actor, broken) = self.new_actor(canvas, &fg.look, fg.mounted);
            fg.actor = actor;
            fg.broken = broken;
            if fg.actor.is_some() && !fg.pose.anim.is_empty() {
                let p = fg.pose.clone();
                let once = p.dur > 0.0;
                fg.pose = self.settle(&mut fg, p, once);
                fg.dirty = true;
            }
            // Go legt den Actor schon in Listen an und stellt dort auch seine Pose; hier gleich nach dem Anlegen,
            // damit die Figur im selben Frame gepostet ist (pose.rs).
            if fg.actor.is_some()
                && !fg.broken
                && let Err(e) = advance(self.now, &mut fg)
            {
                fg.posed = false;
                let key = WarnKey {
                    what: fg.look.asset.clone(),
                    name: fg.pose.anim.clone(),
                };
                self.warn(key, &e);
            }
            self.figs.insert(k, fg);
        }
    }

    /// newActor legt den Rig-Actor an: Monarch beritten, Vierbeiner als Reittier ohne Reiter, sonst figur zu Fuß.
    /// Fehlt das Rig, steht es einmal je Asset im Log und die Figur bleibt unsichtbar (keine stille Ersatz-Grafik).
    /// Ohne Bibliothek (lib `None`) gibt es keinen Actor und keine Meldung. Liefert den Actor und ob das Rig fehlt.
    pub fn new_actor(&mut self, canvas: &mut dyn Canvas, l: &Look, mounted: bool) -> (Option<Actor>, bool) {
        let (Some(lib), Some(mount)) = (self.lib.clone(), self.cat.as_ref().map(|c| c.mount.clone())) else {
            return (None, false); // ohne Grafik (Tests): nur Posen
        };
        let res = {
            let mut lb = lib.borrow_mut();
            if mounted {
                lb.new_actor(canvas, &l.asset, &mount, &l.variant)
            } else {
                let mut r = lb.new_actor(canvas, &l.asset, "", &l.variant);
                let four = match &r {
                    Err(_) => true,
                    Ok(a) => a.rider.as_ref().is_some_and(|x| x.kind == "vierbeiner"),
                };
                if four && let Ok(q) = lb.new_actor(canvas, "", &l.asset, &l.variant) {
                    r = Ok(q);
                }
                r
            }
        };
        match res {
            Ok(a) => (Some(a), false),
            Err(e) => {
                self.warn(
                    WarnKey {
                        what: "rig".to_string(),
                        name: l.asset.clone(),
                    },
                    &e,
                );
                (None, true)
            }
        }
    }

    /// apply setzt den Wunsch w als Pose; kennt die Figur die Animation nicht, zeigt sie die Ruhe-Pose (Meldung
    /// einmal).
    pub fn apply(&mut self, fg: &mut Figure, w: &Want) {
        let (p, changed) = next(&fg.pose, w, self.now);
        if !changed {
            return;
        }
        fg.pose = self.settle(fg, p, w.once);
        fg.dirty = true;
    }

    /// settle stellt die Pose p am Figur-Actor ein (Rückfall auf die Ruhe-Pose mit einer Meldung) und setzt bei
    /// Einmal-Animationen ihre Länge.
    fn settle(&mut self, fg: &mut Figure, mut p: Pose, once: bool) -> Pose {
        match self.set_state(fg, &p) {
            Err(err) => {
                self.warn(
                    WarnKey {
                        what: fg.look.asset.clone(),
                        name: p.anim.clone(),
                    },
                    &err,
                );
                p = Pose {
                    anim: ANIM_IDLE.to_string(),
                    start: self.now,
                    ..Pose::default()
                };
                if fg
                    .actor
                    .as_ref()
                    .is_some_and(|a| a.mount.is_some() && a.rider.is_some())
                {
                    p = Pose {
                        anim: GAIT_STAND.to_string(),
                        gait: GAIT_STAND.to_string(),
                        start: self.now,
                        ..Pose::default()
                    };
                }
                let _ = self.set_state(fg, &p); // idle bzw. stand hat jedes Rig und Sheet
            }
            Ok(()) if once => p.dur = self.duration(fg, &p),
            Ok(()) => {}
        }
        p
    }

    /// setState stellt die Pose am Actor ein bzw. prüft das Sheet.
    fn set_state(&self, fg: &mut Figure, p: &Pose) -> Result<(), String> {
        let sheet = fg.look.sheet;
        let Some(a) = fg.actor.as_mut() else {
            if sheet
                && let Some(lib) = &self.lib
                && lib.borrow().sheet(&fg.look.asset, &p.anim).is_none()
            {
                return Err(format!("sheet {} hat keine Animation „{}“", fg.look.asset, p.anim));
            }
            return Ok(());
        };
        if !p.gait.is_empty() {
            let m = a.mount.as_ref();
            if !m.is_none_or(|m| m.animations.get(&p.gait).is_some_and(|x| !x.gait.is_empty())) {
                let id = m.map_or("", |m| m.id.as_str());
                return Err(format!("reittier {id} hat keine Gangart „{}“", p.gait));
            }
            a.state = State {
                name: p.anim.clone(),
                gait: p.gait.clone(),
            };
        } else if let Some(r) = &a.rider {
            r.figure_freq(&p.anim).map_err(|e| e.to_string())?;
            a.state = State {
                name: p.anim.clone(),
                gait: String::new(),
            };
        } else {
            return a.play(&p.anim).map_err(|e| e.to_string());
        }
        Ok(())
    }

    /// duration: Länge der eingestellten Einmal-Animation (ein Zyklus des Rigs bzw. alle Bilder des Sheets).
    #[allow(clippy::cast_precision_loss)]
    pub fn duration(&self, fg: &Figure, p: &Pose) -> f64 {
        if let Some(a) = &fg.actor
            && let Ok(f) = a.freq()
            && f > 0.0
        {
            return 1.0 / f;
        }
        if fg.look.sheet
            && let Some(lib) = &self.lib
            && let Some(s) = lib.borrow().sheet(&fg.look.asset, &p.anim)
            && s.fps > 0.0
        {
            return s.frames.len() as f64 / s.fps;
        }
        DEFAULT_ONCE
    }

    /// warn meldet einen Fehler einmal je Schlüssel.
    pub fn warn(&mut self, k: WarnKey, err: &dyn std::fmt::Display) {
        if self.warned.get(&k).copied().unwrap_or(false) {
            return;
        }
        let (what, name) = (k.what.clone(), k.name.clone());
        self.warned.insert(k, true);
        k3c_platform::log::error(
            "❌ Figur ohne Grafik, Ruhe-Pose bzw. unsichtbar",
            &[("art", &what), ("name", &name), ("err", &err)],
        );
    }
}
