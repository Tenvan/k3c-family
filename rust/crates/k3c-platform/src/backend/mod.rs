//! Fenster-Backend: winit-Fenster und wgpu. Einziger Ort im Crate, der `winit` und `wgpu` kennt.

mod gpu;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::measure::SecondStat;
use crate::{App, Clock, Frame, Measure, STEP};

use gpu::Gfx;

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
}

/// Öffnet ein Fenster und treibt `app` im Takt [`STEP`], bis `done()`, Schließen oder `max_seconds`.
pub fn run_window(app: &mut dyn App, cfg: Config) -> Result<(), String> {
    let el = EventLoop::new().map_err(|e| e.to_string())?;
    el.set_control_flow(ControlFlow::Poll);
    let mut r = Runner::new(app, cfg);
    el.run_app(&mut r).map_err(|e| e.to_string())?;
    r.finish()
}

struct Runner<'a> {
    app: &'a mut dyn App,
    cfg: Config,
    win: Option<Arc<Window>>,
    gfx: Option<Gfx>,
    backend: String,
    clock: Clock,
    frame: Frame,
    measure: Measure,
    start: Instant,
    last: Option<Instant>,
    second: Option<SecondStat>,
    error: Option<String>,
}

impl<'a> Runner<'a> {
    fn new(app: &'a mut dyn App, cfg: Config) -> Self {
        Self {
            app,
            cfg,
            win: None,
            gfx: None,
            backend: String::new(),
            clock: Clock::new(),
            frame: Frame::new(),
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
            .with_inner_size(LogicalSize::new(1280, 720));
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
        let (steps, alpha) = self.clock.advance(dt.min(MAX_DT));
        for _ in 0..steps {
            self.app.step(STEP);
        }
        self.frame.begin();
        self.app.draw(&mut self.frame, alpha);
        self.frame.finish();
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
        if self.app.done() || over {
            el.exit();
        } else if let Some(w) = &self.win {
            w.request_redraw();
        }
    }

    /// Titel je Sekunde: Backend, Bilder/s, späte Bilder der letzten vollen Sekunde.
    fn update_title(&mut self) {
        let (Some(st), Some(w)) = (self.second.take(), &self.win) else {
            return;
        };
        w.set_title(&format!(
            "{} | {} | {} Bilder/s | spät {}",
            self.cfg.title, self.backend, st.frames, st.late
        ));
    }
}

impl ApplicationHandler for Runner<'_> {
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
            WindowEvent::RedrawRequested => self.redraw(el),
            _ => {}
        }
    }
}
