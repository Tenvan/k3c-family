//! Anwendungszustand des Fensters: Modell, Pipe, Statuszeile, Überwachung des Spielprozesses. Die Pipe-Threads
//! melden sich per `PostMessageW` beim Hauptfenster (Nachrichten `WM_APP_*`), das sie hier auswertet.

use std::cell::{Cell, RefCell};
use std::ffi::c_void;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread;

use k3c_devproto::Msg;
use windows::Win32::Foundation::{COLORREF, CloseHandle, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{HBRUSH, HDC, SetBkColor, SetTextColor};
use windows::Win32::System::Threading::{INFINITE, OpenProcess, PROCESS_SYNCHRONIZE, WaitForSingleObject};
use windows::Win32::UI::WindowsAndMessaging::{
    GetParent, PostMessageW, SW_RESTORE, SetForegroundWindow, SetWindowTextW, ShowWindow, WM_APP, WM_CLOSE,
};
use windows::core::PCWSTR;

use super::controls::wide;
use super::page;
use super::pipe::{self, Sink};
use crate::args::Args;
use crate::state::{Change, Model, set_id};
use crate::style::{BG, Rgb};

/// Nachricht des Spiels (lParam: `Box<Msg>`).
pub(super) const WM_APP_MSG: u32 = WM_APP + 1;
/// Die Pipe-Verbindung ist zu Ende.
pub(super) const WM_APP_LOST: u32 = WM_APP + 2;
/// Befehl einer Seite (lParam: `Box<(Msg, String)>`: Nachricht ohne Kennung und Beschriftung).
pub(super) const WM_APP_CMD: u32 = WM_APP + 3;

/// App hängt am Hauptfenster; Felder als `Cell`/`RefCell` (Re-Entrancy der Fensterprozedur).
pub(super) struct App {
    model: RefCell<Model>,
    tx: RefCell<Option<Sender<Msg>>>,
    status: Cell<HWND>,
}

impl App {
    pub(super) fn new() -> Self {
        App {
            model: RefCell::new(Model::default()),
            tx: RefCell::new(None),
            status: Cell::new(HWND::default()),
        }
    }

    pub(super) fn set_status_window(&self, h: HWND) {
        self.status.set(h);
    }

    pub(super) fn status_window(&self) -> HWND {
        self.status.get()
    }

    /// is_error sagt, ob die Statuszeile rot sein soll.
    pub(super) fn is_error(&self) -> bool {
        self.model.try_borrow().is_ok_and(|m| m.status_is_error())
    }

    /// start erzeugt die Pipe und startet die Überwachung des Spielprozesses; ein Fehler steht in der Statuszeile und
    /// das Ergebnis ist `true` (Exit-Code ≠ 0).
    pub(super) fn start(&self, hwnd: HWND, args: &Args, page: HWND) -> bool {
        let mut failed = false;
        let sink = Arc::new(WinSink(hwnd.0 as isize));
        match pipe::start(&args.pipe, sink) {
            Ok(tx) => {
                if let Ok(mut t) = self.tx.try_borrow_mut() {
                    *t = Some(tx);
                }
            }
            Err(e) => {
                eprintln!("❌ Pipe {} nicht erzeugt: {e}", args.pipe_path());
                if let Ok(mut m) = self.model.try_borrow_mut() {
                    m.fatal = Some(format!("Pipe {} nicht erzeugt: {e}", args.pipe_path()));
                }
                failed = true;
            }
        }
        if let Some(pid) = args.exit_with_game {
            watch_process(pid, hwnd);
        }
        self.refresh(
            page,
            Change {
                catalog: false,
                status: true,
            },
        );
        failed
    }

    /// on_msg nimmt eine Nachricht des Spiels entgegen.
    pub(super) fn on_msg(&self, page: HWND, msg: Msg) {
        if matches!(msg, Msg::Focus { .. }) {
            // Zweites F4 im Spiel: das Fenster nach vorn holen (das Betriebssystem darf es verweigern, dann blinkt es).
            bring_to_front(page);
            return;
        }
        let change = self
            .model
            .try_borrow_mut()
            .map(|mut m| m.on_msg(msg))
            .unwrap_or_default();
        self.refresh(page, change);
    }

    /// on_lost verarbeitet das Ende der Verbindung.
    pub(super) fn on_lost(&self, page: HWND) {
        let change = self.model.try_borrow_mut().map(|mut m| m.on_lost()).unwrap_or_default();
        self.refresh(page, change);
    }

    /// on_command schickt den Befehl einer Seite an das Spiel (mit neuer Kennung); ohne Verbindung geschieht nichts.
    pub(super) fn on_command(&self, page: HWND, mut msg: Msg, label: &str) {
        let Ok(mut m) = self.model.try_borrow_mut() else { return };
        if !m.enabled() {
            return;
        }
        set_id(&mut msg, m.next_command(label));
        drop(m);
        let sent = self
            .tx
            .try_borrow()
            .ok()
            .and_then(|t| t.as_ref().map(|t| t.send(msg).is_ok()));
        if sent != Some(true) {
            eprintln!("❌ Befehl {label} nicht gesendet");
        }
        self.refresh(page, Change::default());
    }

    /// take_msg übernimmt eine Nachricht aus der Pipe (lParam: `Box<Msg>` aus `WinSink::msg`).
    pub(super) fn take_msg(&self, page: HWND, lp: LPARAM) {
        // SAFETY: lParam ist der Box-Zeiger aus `WinSink::msg` (Box::into_raw); die Nachricht kommt genau einmal an.
        let msg = unsafe { Box::from_raw(lp.0 as *mut Msg) };
        self.on_msg(page, *msg);
    }

    /// take_cmd übernimmt den Befehl einer Seite (lParam: `Box<(Msg, String)>` aus der Seitenfläche).
    pub(super) fn take_cmd(&self, page: HWND, lp: LPARAM) {
        // SAFETY: lParam ist der Box-Zeiger aus der Seitenfläche (Box::into_raw); die Nachricht kommt genau einmal an.
        let cmd = unsafe { Box::from_raw(lp.0 as *mut (Msg, String)) };
        let (msg, label) = *cmd;
        self.on_command(page, msg, &label);
    }

    /// status_color färbt die Statuszeile (Fehler rot) für WM_CTLCOLORSTATIC; `None`, wenn das Control nicht
    /// die Statuszeile ist.
    pub(super) fn status_color(&self, wp: WPARAM, lp: LPARAM, bg: HBRUSH) -> Option<LRESULT> {
        if HWND(lp.0 as *mut c_void) != self.status.get() {
            return None;
        }
        let color = if self.is_error() {
            Rgb(0xF2, 0x5F, 0x5C)
        } else {
            Rgb(0xB5, 0xB8, 0xBD)
        };
        let hdc = HDC(wp.0 as *mut c_void);
        // SAFETY: hdc stammt aus der Nachricht und gilt während ihr.
        unsafe {
            SetTextColor(hdc, COLORREF(color.colorref()));
            SetBkColor(hdc, COLORREF(BG.colorref()));
        }
        Some(LRESULT(bg.0 as isize))
    }

    fn refresh(&self, page: HWND, change: Change) {
        let (catalog, enabled, text) = match self.model.try_borrow() {
            Ok(m) => (change.catalog.then(|| m.catalog.clone()), m.enabled(), m.status_text()),
            Err(_) => return,
        };
        if let Some(c) = catalog {
            page::set_catalog(page, c);
        }
        page::set_enabled(page, enabled);
        let t = wide(&text);
        // SAFETY: t ist nullterminiert und lebt über den Aufruf; status ist ein Control dieses Threads (oder leer,
        // dann scheitert der Aufruf folgenlos).
        let _ = unsafe { SetWindowTextW(self.status.get(), PCWSTR(t.as_ptr())) };
    }
}

/// bring_to_front holt das Hauptfenster (Elternfenster der Seite) nach vorn und stellt es aus der Symbolleiste her.
fn bring_to_front(page: HWND) {
    // SAFETY: page ist das eigene Seitenfenster; GetParent liefert das Hauptfenster; die Aufrufe haben nur Zahlen- und
    // Handle-Argumente.
    unsafe {
        let main = GetParent(page).unwrap_or_default();
        let _ = ShowWindow(main, SW_RESTORE);
        let _ = SetForegroundWindow(main);
    }
}

/// WinSink reicht Pipe-Ereignisse per PostMessage an das Hauptfenster (Handle als Zahl, weil HWND nicht Send ist).
struct WinSink(isize);

impl WinSink {
    fn hwnd(&self) -> HWND {
        HWND(self.0 as *mut c_void)
    }
}

impl Sink for WinSink {
    fn msg(&self, m: Msg) -> bool {
        let p = Box::into_raw(Box::new(m));
        // SAFETY: lParam trägt den Box-Zeiger; das Fenster gibt ihn in WM_APP_MSG frei.
        if unsafe { PostMessageW(Some(self.hwnd()), WM_APP_MSG, WPARAM(0), LPARAM(p as isize)) }.is_ok() {
            return true;
        }
        // SAFETY: die Nachricht wurde nicht zugestellt, der Zeiger gehört noch uns und stammt aus Box::into_raw.
        drop(unsafe { Box::from_raw(p) });
        false
    }

    fn lost(&self) -> bool {
        // SAFETY: Nachricht ohne Zeiger an das Hauptfenster.
        unsafe { PostMessageW(Some(self.hwnd()), WM_APP_LOST, WPARAM(0), LPARAM(0)) }.is_ok()
    }
}

/// watch_process schließt das Fenster, sobald der Prozess `pid` endet (oder schon beendet ist).
fn watch_process(pid: u32, hwnd: HWND) {
    let raw = hwnd.0 as isize;
    let spawned = thread::Builder::new().name("k3c-devwin-watch".into()).spawn(move || {
        // SAFETY: OpenProcess/WaitForSingleObject mit eigenem Handle, das nach dem Warten geschlossen wird; Nachricht
        // ohne Zeiger an das Hauptfenster.
        unsafe {
            if let Ok(h) = OpenProcess(PROCESS_SYNCHRONIZE, false, pid) {
                WaitForSingleObject(h, INFINITE);
                let _ = CloseHandle(h);
            }
            let _ = PostMessageW(Some(HWND(raw as *mut c_void)), WM_CLOSE, WPARAM(0), LPARAM(0));
        }
    });
    if let Err(e) = spawned {
        eprintln!("❌ Überwachung des Spiels nicht gestartet: {e}");
    }
}
