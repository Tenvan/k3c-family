//! Port von native/k3c-rig/paint/actor.go: Eine Figur aus Rig-Teilen: figur zu Fuß, Reiter auf seinem Reittier oder
//! Vierbeiner allein. Pose je Bild aus der Zeit, Umhang mit festem Zeitschritt, Waffe und Rüstung als optionale
//! Elemente an ihren Plätzen.

use std::rc::Rc;

use k3c_platform::Canvas;

use super::assets::{Assets, ElementImage, Skin};
use super::painter::{Painter, parse_color};
use crate::rig::gait::{gait_freq, ride_matrix};
use crate::rig::matrix::translate;
use crate::rig::{CLOAK_DT, Cloak, Fehler, FigurePose, IDENTITY, ItemKind, M, Point, Ride, Rig, rein_points};

/// State ist ein Zustand: Animation der Figur zu Fuß (`name`) oder Gangart des Reittiers (`gait`, leer = zu Fuß).
/// `name` ist zugleich der Name der Animation im Sheet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct State {
    pub name: String,
    pub gait: String,
}

/// MAX_CLOAK_STEPS begrenzt die Physik-Schritte je Bild (nach einem Hänger holt der Umhang nicht alles nach).
const MAX_CLOAK_STEPS: f64 = 8.0;

/// Actor ist eine Figur mit Zustand. `rider` ist die figur (None = Vierbeiner ohne Reiter), `mount` das Reittier bzw.
/// der Vierbeiner (None = nur zu Fuß). `weapon` und `armor` sind Element-Ids an den Plätzen, "" = leer.
#[derive(Debug)]
pub struct Actor {
    pub rider: Option<Rc<Rig>>,
    pub mount: Option<Rc<Rig>>,
    pub state: State,
    pub variant: String,
    pub weapon: String,
    pub armor: String,
    pub tempo: f64,
    /// schaut nach links (gespiegelt; Spiegeln liegt in der Sicht des Painters)
    pub left: bool,
    /// Zeit der Animation in Sekunden
    pub t: f64,

    phase: f64,
    skin: Option<Rc<Skin>>,
    mount_skin: Option<Rc<Skin>>,
    pose: FigurePose,
    world: Vec<M>,
    ride: Ride,
    cloak: Option<Cloak>,
    cloak_on: i32,
    cloak_acc: f64,
    chain: Vec<Point>,
    legs: Vec<Point>,
}

impl Actor {
    /// `new` legt eine Figur im Zustand idle an; die Waffe ist die Standard-Waffe des Rigs (Monarch: Schwert).
    pub fn new(
        a: &mut Assets,
        c: &mut dyn Canvas,
        rider: Option<Rc<Rig>>,
        mount: Option<Rc<Rig>>,
        variant: &str,
    ) -> Result<Actor, Fehler> {
        let mut ac = Actor::bare(rider, mount, variant)?;
        ac.play("idle")?;
        ac.refresh(a, c)?;
        Ok(ac)
    }

    /// `start` wie `new`, aber im Zustand `state` (statt idle) und mit Waffe und Rüstung: Für Tests und Sheets, die
    /// den Zustand vor dem ersten Lösen kennen.
    pub fn start(
        a: &mut Assets,
        c: &mut dyn Canvas,
        (rider, mount): (Option<Rc<Rig>>, Option<Rc<Rig>>),
        variant: &str,
        state: State,
    ) -> Result<Actor, Fehler> {
        let mut ac = Actor::bare(rider, mount, variant)?;
        ac.state = state;
        ac.refresh(a, c)?;
        Ok(ac)
    }

    fn bare(rider: Option<Rc<Rig>>, mount: Option<Rc<Rig>>, variant: &str) -> Result<Actor, Fehler> {
        if rider.is_none() && mount.is_none() {
            return Err(Fehler::Rig("Figur ohne Rig".to_string()));
        }
        let weapon = rider
            .as_ref()
            .and_then(|r| r.default.get("waffe"))
            .cloned()
            .unwrap_or_default();
        Ok(Actor {
            rider,
            mount,
            state: State::default(),
            variant: variant.to_string(),
            weapon,
            armor: String::new(),
            tempo: 1.0,
            left: false,
            t: 0.0,
            phase: 0.0,
            skin: None,
            mount_skin: None,
            pose: FigurePose::default(),
            world: Vec::new(),
            ride: Ride::default(),
            cloak: None,
            cloak_on: 0,
            cloak_acc: 0.0,
            chain: Vec::new(),
            legs: Vec::new(),
        })
    }

    /// `play` stellt eine Animation ein, ohne die Zeit zurückzusetzen: figur nach Namen, Vierbeiner ohne Reiter in der
    /// Gangart seiner Animation (Gegner: idle = schritt, lauf = trab).
    pub fn play(&mut self, name: &str) -> Result<(), Fehler> {
        if self.rider.is_some() {
            self.state = State {
                name: name.to_string(),
                gait: String::new(),
            };
            return Ok(());
        }
        let mount = self.mount.as_ref().expect("Vierbeiner");
        match mount.animations.get(name) {
            Some(anim) if !anim.gait.is_empty() => {
                self.state = State {
                    name: name.to_string(),
                    gait: anim.gait.clone(),
                };
                Ok(())
            }
            _ => Err(Fehler::Rig(format!(
                "animation-unbekannt: Vierbeiner {} hat keine Animation „{name}“ mit Gangart",
                mount.id
            ))),
        }
    }

    /// `refresh` holt die Bilder der gewählten Variante und rechnet die Pose neu.
    pub fn refresh(&mut self, a: &mut Assets, c: &mut dyn Canvas) -> Result<(), Fehler> {
        if let Some(r) = &self.rider {
            self.skin = Some(a.skin(c, r, &self.variant)?);
        }
        if let Some(m) = &self.mount {
            self.mount_skin = Some(a.skin(c, m, &self.variant)?);
        }
        self.solve()
    }

    pub fn riding(&self) -> bool {
        !self.state.gait.is_empty()
    }

    /// `phase` liefert die Phase 0..1 des laufenden Zyklus.
    pub fn phase(&self) -> f64 {
        self.phase
    }

    /// `reset_cloak` lässt den Umhang beim nächsten Schritt neu hängen.
    pub fn reset_cloak(&mut self) {
        self.cloak = None;
    }

    /// `freq`: Zyklen je Sekunde des laufenden Zustands.
    pub fn freq(&self) -> Result<f64, Fehler> {
        if self.riding() {
            return gait_freq(&self.state.gait);
        }
        self.rider
            .as_ref()
            .expect("Figur zu Fuß hat einen Reiter-Rig")
            .figure_freq(&self.state.name)
    }

    /// `solve` rechnet Pose und Welt-Matrizen zur Zeit t (Figur am Ursprung, Füße bei y = 0).
    fn solve(&mut self) -> Result<(), Fehler> {
        if self.riding() && self.mount.is_none() {
            return Err(Fehler::Rig(format!("Gangart {} ohne Reittier", self.state.gait)));
        }
        let freq = self.freq()?;
        self.phase = ((self.t * freq) % 1.0 + 1.0) % 1.0;
        if self.riding() {
            let m = self.mount.clone().expect("Reittier");
            return match self.rider.clone() {
                None => {
                    m.gait_pose(&self.state.gait, self.phase, &mut self.ride.pose)?;
                    let base = ride_matrix(self.ride.pose.dy, self.ride.pose.pitch, m.scale);
                    m.world(&self.ride.pose.angles, base, &mut self.ride.mount_world);
                    Ok(())
                }
                Some(r) => self.ride.set(&m, &r, &self.state.gait, self.phase, IDENTITY),
            };
        }
        let r = self.rider.clone().expect("Reiter-Rig");
        r.figure_pose(&self.state.name, self.t, &mut self.pose)?;
        r.world(
            &self.pose.angles,
            translate(self.pose.offset[0], self.pose.offset[1]),
            &mut self.world,
        );
        Ok(())
    }

    /// `cloak_state`: Anker und Wind des Umhangs und sein Boden; reitend in der Ride-Gruppe, zu Fuß in der Welt.
    fn cloak_state(&self) -> (Point, f64, f64) {
        let rider = self.rider.as_ref().expect("Reiter-Rig");
        if self.riding() {
            return (
                self.ride.cloak_anchor(rider),
                self.ride.pose.wind * self.tempo,
                crate::rig::ride_floor(),
            );
        }
        (rider.cloak_anchor(&self.world), self.pose.wind * self.tempo, 0.0)
    }

    /// `update` dreht die Animation um dt weiter und rechnet den Umhang in festen Schritten nach.
    pub fn update(&mut self, dt: f64) -> Result<(), Fehler> {
        self.t += dt * self.tempo;
        self.solve()?;
        self.step_cloak(dt);
        Ok(())
    }

    fn step_cloak(&mut self, dt: f64) {
        let Some(spec) = self.rider.as_ref().and_then(|r| r.cloak.clone()) else {
            return;
        };
        let (anchor, wind, floor) = self.cloak_state();
        let mode = if self.riding() { 2 } else { 1 };
        if self.cloak.is_none() || self.cloak_on != mode {
            (self.cloak, self.cloak_on, self.cloak_acc) = (Some(Cloak::new(&spec, anchor)), mode, 0.0);
        }
        self.cloak_acc = f64::min(self.cloak_acc + dt, MAX_CLOAK_STEPS * CLOAK_DT);
        let cloak = self.cloak.as_mut().expect("Umhang");
        while self.cloak_acc >= CLOAK_DT {
            cloak.step(anchor, wind, floor);
            self.cloak_acc -= CLOAK_DT;
        }
        cloak.points(&mut self.chain);
    }

    /// `set_frame` stellt den Zustand eines Sheet-Frames ein: Pose von Frame n und die Umhang-Kette des Sheets.
    pub fn set_frame(&mut self, n: i64, frames: i64, chain: &[Point]) -> Result<(), Fehler> {
        let freq = self.freq()?;
        self.t = crate::rig::frame_time(n, frames, freq);
        self.chain.clear();
        self.chain.extend_from_slice(chain);
        self.solve()
    }

    /// `draw` zeichnet die Figur am Ursprung der Welt; Ort, Maßstab und Spiegelung liegen in der Sicht des Painters.
    pub fn draw(&mut self, p: &mut Painter, c: &mut dyn Canvas, a: &Assets) {
        if self.riding() {
            self.draw_mount(p, c, a);
            return;
        }
        let world = std::mem::take(&mut self.world);
        self.draw_rider(p, c, a, &world, IDENTITY, false);
        self.world = world;
    }

    /// `draw_mount` zeichnet das Reittier in Ebenen-Reihenfolge; am Platz `sattel` kommt der Reiter, danach Hals und Zügel.
    fn draw_mount(&mut self, p: &mut Painter, c: &mut dyn Canvas, a: &Assets) {
        let (r, s) = (
            self.mount.clone().expect("Reittier"),
            self.mount_skin.clone().expect("Bilder"),
        );
        for it in &r.order {
            match it.kind {
                ItemKind::Part => {
                    let (part, img) = (&r.parts[it.index], s.parts[it.index]);
                    if let Some(img) = img {
                        p.image(c, img, self.ride.mount_world[it.index], part.image.offset, r.density);
                    }
                }
                ItemKind::Band => {
                    r.leg_points(it.index, &self.ride.pose.angles, &mut self.legs);
                    let at = usize::try_from(s.band_part[it.index]).expect("Band hängt an einem Teil");
                    if let Some(img) = s.bands[it.index] {
                        p.band(
                            c,
                            img,
                            &r.bands[it.index],
                            &self.legs,
                            self.ride.mount_world[at],
                            r.density,
                        );
                    }
                }
                ItemKind::Slot => {
                    if self.rider.is_some() && r.slots[it.index].name == "sattel" {
                        let (world, base) = (std::mem::take(&mut self.ride.rider_world), self.ride.base);
                        self.draw_rider(p, c, a, &world, base, true);
                        self.ride.rider_world = world;
                    }
                }
                ItemKind::Reins => {
                    if self.rider.is_some() {
                        self.draw_reins(p, c);
                    }
                }
            }
        }
    }

    fn draw_reins(&self, p: &mut Painter, c: &mut dyn Canvas) {
        let (mount, rider) = (
            self.mount.as_ref().expect("Reittier"),
            self.rider.as_ref().expect("Reiter"),
        );
        let ends = rein_points(
            mount,
            &self.ride.mount_world,
            rider,
            &self.ride.rider_world,
            &self.ride.base,
        );
        if let (Some((hand, ctrl, mouth)), Some(re)) = (ends, mount.reins.as_ref()) {
            p.curve(c, (hand, ctrl, mouth), re.width, parse_color(&re.color), re.opacity);
        }
    }

    /// `draw_rider` zeichnet die Figur: Umhang (in cloak_base), Teile und Elemente an den Plätzen; reitend ohne das
    /// hintere Bein.
    fn draw_rider(&self, p: &mut Painter, c: &mut dyn Canvas, a: &Assets, world: &[M], cloak_base: M, mounted: bool) {
        let (r, s) = (
            self.rider.as_ref().expect("Reiter"),
            self.skin.as_ref().expect("Bilder"),
        );
        for it in &r.order {
            match it.kind {
                ItemKind::Part => {
                    if let (false, Some(img)) = (mounted && r.hidden(it.index), s.parts[it.index]) {
                        p.image(c, img, world[it.index], r.parts[it.index].image.offset, r.density);
                    }
                }
                ItemKind::Band => {
                    let b = &r.bands[it.index];
                    if let (true, Some(img)) = (self.chain.len() == b.rest.len(), s.bands[it.index]) {
                        p.band(c, img, b, &self.chain, cloak_base, r.density);
                    }
                }
                ItemKind::Slot => {
                    let slot = &r.slots[it.index];
                    if let Some(e) = self.element_at(a, &slot.name) {
                        p.image(
                            c,
                            e.img,
                            r.slot_world(slot, world),
                            e.element.image.offset,
                            e.element.density,
                        );
                    }
                }
                ItemKind::Reins => {}
            }
        }
    }

    /// `element_at` liefert das gewählte Element eines Platzes oder None (Platz leer).
    fn element_at<'a>(&self, a: &'a Assets, slot: &str) -> Option<&'a ElementImage> {
        match slot {
            "waffe" => a.elements.get(&self.weapon),
            "ruestung" => a.elements.get(&self.armor),
            _ => None,
        }
    }
}
