//! Fenster-Backend: winit-Fenster und wgpu. Einziger Ort im Crate, der `winit` und `wgpu` kennt.

mod gpu;
mod keys;
mod pads;
mod setup;
mod sound;

#[cfg(test)]
pub(crate) use sound::decode as decode_sound;
pub(crate) use sound::open as open_sound;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::{Fullscreen, Window, WindowId};

use crate::measure::SecondStat;
use crate::{App, Frame, Measure, Runner};

use gpu::Gfx;
use pads::Pads;

/// Bilder im Messring (rund 18 Minuten bei 60 Hz).
const RING: usize = 1 << 16;
/// Längster Bildabstand, der in die Zeitsammlung eingeht (danach gilt: Pause, kein Aufholen).
const MAX_DT: f64 = 0.25;

/// Einstellungen von [`run_window`].
#[derive(Clone, Debug, Default)]
pub struct Config {
    /// Fenstertitel (vorn) und Name des Laufs in der Messdatei.
    pub title: String,
    /// Schreibt am Ende die Messung (Median, p99, längstes Bild, späte je Sekunde) als JSON dorthin.
    pub measure: Option<PathBuf>,
    /// Beendet die Schleife nach so vielen Sekunden (für Messläufe).
    pub max_seconds: Option<f64>,
    /// Startet im Vollbild (randlos); danach schaltet die App über [`App::fullscreen`].
    pub fullscreen: bool,
}

/// Öffnet ein Fenster (in der Größe veränderbar) und treibt `app` im Takt [`crate::STEP`], bis `done()`,
/// Schließen oder `max_seconds`. Tasten gehen als [`crate::InputState`] an die App; das Backend wertet keine aus.
pub fn run_window(app: &mut dyn App, cfg: Config) -> Result<(), String> {
    let el = EventLoop::new().map_err(|e| e.to_string())?;
    el.set_control_flow(ControlFlow::Poll);
    let mut r = WinLoop::new(app, cfg);
    el.run_app(&mut r).map_err(|e| e.to_string())?;
    r.finish()
}

struct WinLoop<'a> {
    run: Runner<'a>,
    cfg: Config,
    win: Option<Arc<Window>>,
    gfx: Option<Gfx>,
    backend: String,
    frame: Frame,
    pads: Pads,
    /// Zuletzt umgesetzter Vollbild-Wunsch der App; nur eine Änderung des Wunsches schaltet (das System darf
    /// das Vollbild sonst selbst umschalten, ohne dass die Schleife es zurücksetzt).
    full_wish: Option<bool>,
    measure: Measure,
    start: Instant,
    last: Option<Instant>,
    second: Option<SecondStat>,
    error: Option<String>,
}

impl<'a> WinLoop<'a> {
    fn new(app: &'a mut dyn App, cfg: Config) -> Self {
        let full_wish = Some(cfg.fullscreen);
        Self {
            run: Runner::new(app, None),
            cfg,
            win: None,
            gfx: None,
            backend: String::new(),
            frame: Frame::new(),
            pads: Pads::open(),
            full_wish,
            measure: Measure::new(RING),
            start: Instant::now(),
            last: None,
            second: None,
            error: None,
        }
    }

    /// Schreibt die Messdatei (falls verlangt) und meldet einen Fehler aus dem Start.
    fn finish(&mut self) -> Result<(), String> {
        if let Some(e) = self.error.take() {
            return Err(e);
        }
        let Some(path) = &self.cfg.measure else {
            return Ok(());
        };
        let label = format!("{} {}", self.cfg.title, self.backend);
        let json = self.measure.summary().to_json(label.trim());
        std::fs::write(path, json).map_err(|e| format!("{}: {e}", path.display()))
    }

    fn create(&mut self, el: &ActiveEventLoop) -> Result<(), String> {
        let attrs = Window::default_attributes()
            .with_title(self.cfg.title.clone())
            .with_inner_size(LogicalSize::new(1280, 720))
            .with_fullscreen(self.cfg.fullscreen.then_some(Fullscreen::Borderless(None)));
        let win = Arc::new(el.create_window(attrs).map_err(|e| e.to_string())?);
        let (gfx, backend) = Gfx::new(win.clone())?;
        println!("backend: {backend}");
        self.backend = backend;
        self.gfx = Some(gfx);
        win.request_redraw();
        self.win = Some(win);
        Ok(())
    }

    fn redraw(&mut self, el: &ActiveEventLoop) {
        let now = Instant::now();
        let dt = self.last.map_or(0.0, |l| now.duration_since(l).as_secs_f64());
        self.pads.poll(self.run.input_mut());
        self.frame.begin();
        self.run.advance(dt.min(MAX_DT), &mut self.frame);
        self.frame.finish();
        self.sync_fullscreen();
        let mut cpu_ms = 0.0;
        if let Some(g) = &mut self.gfx {
            g.render(&mut self.frame, || cpu_ms = now.elapsed().as_secs_f64() * 1000.0);
        }
        if self.last.is_some() {
            self.second = self.measure.push(dt * 1000.0, cpu_ms).or(self.second);
            self.update_title();
        }
        self.last = Some(now);
        let over = self
            .cfg
            .max_seconds
            .is_some_and(|s| now.duration_since(self.start).as_secs_f64() >= s);
        if self.run.app().done() || over {
            el.exit();
        } else if let Some(w) = &self.win {
            w.request_redraw();
        }
    }

    /// Taste in den Eingabe-Zustand; Wiederholungen des Systems zählen nicht.
    fn key(&mut self, ev: &KeyEvent) {
        let PhysicalKey::Code(code) = ev.physical_key else {
            return;
        };
        let Some(k) = keys::map(code) else {
            return;
        };
        let keys = &mut self.run.input_mut().keys;
        match ev.state {
            ElementState::Pressed if !ev.repeat => keys.press(k),
            ElementState::Released => keys.release(k),
            ElementState::Pressed => {}
        }
    }

    /// Setzt das Vollbild, wenn sich der Wunsch der App geändert hat und das Fenster davon abweicht.
    fn sync_fullscreen(&mut self) {
        let (Some(want), Some(w)) = (self.run.app().fullscreen(), &self.win) else {
            return;
        };
        if self.full_wish == Some(want) {
            return;
        }
        self.full_wish = Some(want);
        if w.fullscreen().is_some() != want {
            w.set_fullscreen(want.then_some(Fullscreen::Borderless(None)));
        }
    }

    /// Titel je Sekunde: Backend, Bilder/s, späte Bilder der letzten vollen Sekunde.
    fn update_title(&mut self) {
        let (Some(st), Some(w)) = (self.second.take(), &self.win) else {
            return;
        };
        let pads = self.run.input_mut().pads().len();
        w.set_title(&format!(
            "{} | {} | {} Bilder/s | spät {} | Pads {pads}",
            self.cfg.title, self.backend, st.frames, st.late
        ));
    }
}

impl ApplicationHandler for WinLoop<'_> {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.win.is_none()
            && let Err(e) = self.create(el)
        {
            self.error = Some(e);
            el.exit();
        }
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _: WindowId, ev: WindowEvent) {
        match ev {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(s) => {
                if let Some(g) = &mut self.gfx {
                    g.resize(s.width, s.height);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => self.key(&event),
            WindowEvent::Focused(false) => self.run.input_mut().keys.release_all(),
            WindowEvent::RedrawRequested => self.redraw(el),
            _ => {}
        }
    }
}
