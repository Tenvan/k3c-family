//! Demo-Client (RU2.3): zeichnet eine Papierschnitt-Welt, durch die die Kamera reitet; Figuren laufen nach
//! einem Skript (Driver), nicht nach der Sim. Zeichnet nur über `k3c-platform`.

mod cli;
mod driver;
mod scene;
mod world;

use k3c_platform::{App, Canvas, Config, InputScript, InputState, Key, Rect, STEP, run_headless, run_window};

use crate::art::Library;
pub use cli::{Options, parse_args};
use driver::{Driver, parse_color};
use scene::{Gpu, sprite_at};
use world::World;

/// Höhe der Bodenlinie (Fußpunkte) in logischen Pixeln.
const GROUND_Y: f32 = 860.0;
/// Pixel je Unit (1 Unit = 32 px).
const UNIT_PX: f64 = 32.0;
const SKY: [f32; 4] = [0.56, 0.72, 0.85, 1.0];
/// Ebenen: Himmel unten, dann die Hintergründe, Boden, Gebäude, Figuren, Zusatzlast, Schrift.
const L_SKY: u32 = 0;
const L_BACK: u32 = 1;
const L_GROUND: u32 = 20;
const L_BUILD: u32 = 21;
const L_ACTORS: u32 = 22;
const L_EXTRA: u32 = 23;
const L_TEXT: u32 = 30;
/// Größe der Zusatzlast-Sprites relativ zum Rahmen.
const EXTRA_SCALE: f32 = 0.3;

/// Zusatzlast: N kleine bewegte Sprites aus einem Figuren-Asset (`--sprites N`).
struct Extra {
    n: usize,
    sprite: Option<(k3c_platform::TexId, Rect)>,
}

pub struct Demo {
    driver: Driver,
    ground: [f32; 4],
    world: World,
    cpu: Vec<crate::art::Asset>,
    gpu: Vec<Gpu>,
    extra: Extra,
    /// Esc gedrückt: Schleife endet.
    quit: bool,
    /// Gewünschtes Vollbild; F schaltet um.
    full: bool,
}

impl Demo {
    /// `lib` = None zeichnet Platzhalter-Rechtecke (Tests ohne Grafik).
    pub fn new(driver: Driver, lib: Option<&Library>, rider_speed: f64, sprites: usize) -> Result<Self, String> {
        let world = World::new(&driver, rider_speed);
        let mut cpu = Vec::new();
        if let Some(lib) = lib {
            for a in &world.assets {
                cpu.push(lib.load(a).map_err(|e| format!("Asset {a}: {e}"))?);
            }
        }
        let ground = parse_color(&driver.boden)?;
        Ok(Self {
            driver,
            ground,
            world,
            cpu,
            gpu: Vec::new(),
            extra: Extra {
                n: sprites,
                sprite: None,
            },
            quit: false,
            full: false,
        })
    }

    /// Lädt beim ersten Bild alle Sheets auf die Zeichenfläche.
    fn ready(&mut self, c: &mut dyn Canvas) {
        if self.gpu.is_empty() && !self.cpu.is_empty() {
            self.gpu = std::mem::take(&mut self.cpu)
                .into_iter()
                .map(|a| Gpu::upload(c, a))
                .collect();
        }
        if self.extra.n > 0 && self.extra.sprite.is_none() {
            let first = self.world.actors.get(1).map_or(0, |a| a.asset);
            let pick = self.gpu.get(first).and_then(|g| g.pick("idle", 0.0));
            self.extra.sprite = pick.map(|(t, f, _)| {
                (
                    t,
                    Rect::new(f.rect[0] as f32, f.rect[1] as f32, f.rect[2] as f32, f.rect[3] as f32),
                )
            });
        }
    }

    fn draw_backdrop(&self, c: &mut dyn Canvas, cam_px: f64) {
        let (w, h) = c.size();
        c.layer(L_SKY);
        c.rect(Rect::new(0.0, 0.0, w, h), SKY);
        for (i, e) in self.driver.ebenen.iter().enumerate() {
            c.layer(L_BACK + i as u32);
            let Some(g) = self.index_of(&e.asset).and_then(|k| self.gpu.get(k)) else {
                continue;
            };
            let Some((t, f, rahmen)) = g.pick("stand", 0.0) else {
                continue;
            };
            let s = e.skala as f32;
            let tile = rahmen[2] as f32 * s;
            let off = (cam_px * e.parallaxe) as f32;
            let mut k = (off / tile).floor();
            while k * tile - off < w {
                sprite_at(c, t, f, rahmen, (k * tile - off, GROUND_Y), s, false);
                k += 1.0;
            }
        }
        c.layer(L_GROUND);
        c.rect(Rect::new(0.0, GROUND_Y, w, h - GROUND_Y), self.ground);
    }

    fn index_of(&self, asset: &str) -> Option<usize> {
        self.world.assets.iter().position(|a| a == asset)
    }

    fn draw_buildings(&self, c: &mut dyn Canvas, cam: f64) {
        c.layer(L_BUILD);
        let s = self.driver.skala as f32;
        for b in &self.driver.gebaeude {
            let x = 960.0 + ((b.x - cam) * UNIT_PX) as f32;
            if !(-500.0..2420.0).contains(&x) {
                continue;
            }
            if let Some(g) = self.index_of(&b.asset).and_then(|k| self.gpu.get(k)) {
                g.draw(c, "stand", 0.0, (x, GROUND_Y), s, false);
            } else {
                c.rect(Rect::new(x - 40.0, GROUND_Y - 120.0, 80.0, 120.0), [0.5, 0.3, 0.2, 1.0]);
            }
        }
    }

    fn draw_actors(&self, c: &mut dyn Canvas, cam: f64, alpha: f64) {
        c.layer(L_ACTORS);
        let s = self.driver.skala as f32;
        for a in &self.world.actors {
            let x = 960.0 + ((a.x_at(alpha) - cam) * UNIT_PX) as f32;
            if !(-400.0..2320.0).contains(&x) {
                continue;
            }
            let foot = (x, GROUND_Y + 20.0 + a.row);
            if let Some(g) = self.gpu.get(a.asset) {
                let t = self.world.tick as f64 + alpha + a.phase;
                g.draw(c, &a.anim, t, foot, s, a.dir < 0.0);
            } else {
                c.rect(
                    Rect::new(foot.0 - 12.0, foot.1 - 60.0, 24.0, 60.0),
                    [0.9, 0.8, 0.2, 1.0],
                );
            }
        }
    }

    fn draw_extra(&self, c: &mut dyn Canvas, alpha: f64) {
        let Some((t, src)) = self.extra.sprite else {
            return;
        };
        c.layer(L_EXTRA);
        let (w, h) = (src.w * EXTRA_SCALE, src.h * EXTRA_SCALE);
        let time = self.world.tick as f64 + alpha;
        for i in 0..self.extra.n {
            let fi = i as f64;
            let x = (fi * 37.3 + time * 2.0 * (1.0 + (i % 5) as f64)) % 1920.0;
            let y = (fi * 13.7) % 700.0;
            c.sprite(t, src, Rect::new(x as f32, y as f32, w, h), [1.0; 4]);
        }
    }
}

impl App for Demo {
    fn step(&mut self, input: &InputState, _dt: f64) {
        self.quit |= input.just_pressed(Key::Escape);
        if input.just_pressed(Key::F) {
            self.full = !self.full;
        }
        self.world.step();
    }

    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        self.ready(c);
        let cam = self.world.camera(alpha);
        self.draw_backdrop(c, cam * UNIT_PX);
        self.draw_buildings(c, cam);
        self.draw_actors(c, cam, alpha);
        self.draw_extra(c, alpha);
        c.layer(L_TEXT);
        let line = format!(
            "K3C DEMO  BIOM {}  X {:.1}  FIGUREN {}",
            self.driver.biom,
            cam,
            self.world.actors.len()
        );
        c.text(&line, 40.0, 30.0, 28.0, [1.0; 4]);
    }

    fn done(&self) -> bool {
        self.quit
    }

    fn fullscreen(&self) -> Option<bool> {
        Some(self.full)
    }
}

/// Startet die Demo nach den Optionen: Fenster, Messlauf oder headless.
pub fn run(o: &Options) -> Result<(), String> {
    let driver = Driver::read(&o.driver)?;
    let lib = Library::open(&o.grafik, &o.data.join("paper.json")).map_err(|e| e.to_string())?;
    let speed = driver.kamera.tempo.map_or_else(|| cli::monarch_speed(&o.data), Ok)?;
    let mut demo = Demo::new(driver, Some(&lib), speed, o.sprites)?;
    demo.full = o.fullscreen;
    if o.headless {
        let script = o.script.as_ref().map(|p| read_script(p)).transpose()?;
        let n = run_headless(&mut demo, o.steps, script.as_ref());
        println!(
            "headless: {} Schritte, {} Sprites, {} Flächen, {} Texturen",
            o.steps, n.sprites, n.rects, n.textures
        );
        return Ok(());
    }
    let cfg = Config {
        title: format!("k3c demo ({:.0} Hz)", 1.0 / STEP),
        measure: o.measure.map(|_| o.out.clone()),
        max_seconds: o.measure,
        fullscreen: o.fullscreen,
    };
    run_window(&mut demo, cfg)
}

fn read_script(p: &std::path::Path) -> Result<InputScript, String> {
    let text = std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?;
    InputScript::from_json(&text)
}

#[cfg(test)]
mod tests;
