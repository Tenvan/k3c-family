//! Controls der Seiten: erzeugen und zeichnen. Schaltflächen und Umschalter sind owner-draw (flach, mit Hover und
//! Gedrückt-Zustand), weil die Standardschaltfläche kein Dark-Theme hat; Auswahl und Eingabe nutzen `DarkMode_CFD`.

use std::ffi::c_void;
use std::sync::atomic::{AtomicIsize, Ordering};

use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    DT_CENTER, DT_NOPREFIX, DT_SINGLELINE, DT_VCENTER, DrawTextW, FillRect, FrameRect, HDC, HFONT, InvalidateRect,
    SetBkMode, SetTextColor, TRANSPARENT,
};
use windows::Win32::UI::Controls::{
    DRAWITEMSTRUCT, ODS_DISABLED, ODS_FOCUS, ODS_SELECTED, SetWindowTheme, WM_MOUSELEAVE,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{TME_LEAVE, TRACKMOUSEEVENT, TrackMouseEvent};
use windows::Win32::UI::WindowsAndMessaging::{
    BS_OWNERDRAW, CB_ADDSTRING, CB_GETCURSEL, CB_SETCURSEL, CBS_DROPDOWNLIST, CallWindowProcW, CreateWindowExW,
    ES_AUTOHSCROLL, GWLP_USERDATA, GWLP_WNDPROC, GetWindowLongPtrW, GetWindowTextW, HMENU, SendMessageW,
    SetWindowLongPtrW, WINDOW_EX_STYLE, WINDOW_STYLE, WM_MOUSEMOVE, WM_SETFONT, WNDPROC, WS_BORDER, WS_CHILD,
    WS_TABSTOP, WS_VISIBLE, WS_VSCROLL,
};
use windows::core::{PCWSTR, Result, w};

use super::theme;
use crate::layout::{Control, Kind};
use crate::style::{ACCENT, BG, Rgb, TEXT};

/// Flächen und Rahmen der Controls.
pub(super) const FIELD: Rgb = Rgb(0x2B, 0x2D, 0x31);
const HOVER: Rgb = Rgb(0x36, 0x38, 0x3E);
const PRESSED: Rgb = Rgb(0x42, 0x45, 0x4C);
pub(super) const BORDER: Rgb = Rgb(0x3F, 0x41, 0x48);
const DISABLED_TEXT: Rgb = Rgb(0x6B, 0x6D, 0x72);

const FLAG_HOT: isize = 1;
const FLAG_ON: isize = 2;

/// Ursprüngliche Fensterprozedur der Klasse BUTTON (für alle Schaltflächen gleich).
static BUTTON_PROC: AtomicIsize = AtomicIsize::new(0);

/// wide wandelt Text in nullterminiertes UTF-16.
pub(super) fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// create erzeugt das Control `c` als Kind von `parent` mit der Kennung `id` und setzt die Schrift.
pub(super) fn create(parent: HWND, inst: HINSTANCE, id: usize, c: &Control, font: HFONT) -> Result<HWND> {
    let (class, style) = match c.kind {
        Kind::Button | Kind::Toggle => (w!("BUTTON"), BS_OWNERDRAW as u32 | WS_TABSTOP.0),
        Kind::Select => (w!("COMBOBOX"), CBS_DROPDOWNLIST as u32 | WS_VSCROLL.0 | WS_TABSTOP.0),
        Kind::Input => (w!("EDIT"), ES_AUTOHSCROLL as u32 | WS_BORDER.0 | WS_TABSTOP.0),
        Kind::Label => (w!("STATIC"), 0),
    };
    let text = wide(&c.text);
    // SAFETY: class ist ein statischer String, text ist nullterminiert und lebt über den Aufruf, parent ist ein
    // gültiges Fenster dieses Threads; die Kennung geht als HMENU in das Kind (Konvention für Kind-Controls).
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            class,
            PCWSTR(text.as_ptr()),
            WINDOW_STYLE(WS_CHILD.0 | WS_VISIBLE.0 | style),
            0,
            0,
            10,
            10,
            Some(parent),
            Some(HMENU(id as *mut c_void)),
            Some(inst),
            None,
        )
    }?;
    send(hwnd, WM_SETFONT, font.0 as usize, 1);
    match c.kind {
        Kind::Button | Kind::Toggle => subclass_button(hwnd),
        Kind::Select => init_combo(hwnd, &c.options),
        Kind::Input => dark_theme(hwnd),
        Kind::Label => {}
    }
    Ok(hwnd)
}

/// send schickt eine Nachricht an ein Control.
pub(super) fn send(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> LRESULT {
    // SAFETY: nur Zahlenargumente; hwnd ist ein Control dieses Threads (die Nachrichten dieses Moduls nehmen
    // keine Zeiger, außer CB_ADDSTRING, das `init_combo` mit einem lebenden nullterminierten Puffer ruft).
    unsafe { SendMessageW(hwnd, msg, Some(WPARAM(wp)), Some(LPARAM(lp))) }
}

fn dark_theme(hwnd: HWND) {
    // SAFETY: Klassenname ist ein statischer String, hwnd ein gültiges Control.
    let r = unsafe { SetWindowTheme(hwnd, w!("DarkMode_CFD"), PCWSTR::null()) };
    if let Err(e) = r {
        eprintln!("🐛 Dark-Theme für Control nicht verfügbar: {e}");
    }
}

fn init_combo(hwnd: HWND, options: &[String]) {
    dark_theme(hwnd);
    for o in options {
        let s = wide(o);
        send(hwnd, CB_ADDSTRING, 0, s.as_ptr() as isize);
    }
    send(hwnd, CB_SETCURSEL, 0, 0);
}

/// flags liest die Zustandsbits (Hover, Umschalter an) aus dem Fenster.
fn flags(hwnd: HWND) -> isize {
    // SAFETY: GWLP_USERDATA der Schaltflächen gehört diesem Modul und enthält nur die Zustandsbits.
    unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) }
}

fn set_flag(hwnd: HWND, bit: isize, on: bool) {
    let f = flags(hwnd);
    let n = if on { f | bit } else { f & !bit };
    if n != f {
        // SAFETY: wie `flags`; danach zeichnet das Control neu.
        unsafe {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, n);
            let _ = InvalidateRect(Some(hwnd), None, false);
        }
    }
}

/// is_on sagt, ob ein Umschalter an ist.
pub(super) fn is_on(hwnd: HWND) -> bool {
    flags(hwnd) & FLAG_ON != 0
}

/// selected liefert den gewählten Index einer Auswahlliste.
pub(super) fn selected(hwnd: HWND) -> Option<usize> {
    usize::try_from(send(hwnd, CB_GETCURSEL, 0, 0).0).ok()
}

/// text liest den Text eines Controls (höchstens 512 Zeichen).
pub(super) fn text(hwnd: HWND) -> String {
    let mut buf = [0u16; 512];
    // SAFETY: buf ist ein beschreibbarer Puffer; GetWindowTextW schneidet auf dessen Länge.
    let n = unsafe { GetWindowTextW(hwnd, &mut buf) }.max(0) as usize;
    String::from_utf16_lossy(buf.get(..n).unwrap_or_default())
}

/// toggle schaltet den Zustand eines Umschalters um und liefert den neuen Wert.
pub(super) fn toggle(hwnd: HWND) -> bool {
    let on = flags(hwnd) & FLAG_ON == 0;
    set_flag(hwnd, FLAG_ON, on);
    on
}

fn subclass_button(hwnd: HWND) {
    let new_proc = button_proc as *const () as usize as isize;
    // SAFETY: tauscht die Fensterprozedur dieses Buttons gegen `button_proc`, die an die alte weiterreicht; die
    // alte Prozedur wird vorher gemerkt (für alle BUTTON-Fenster dieselbe).
    let old = unsafe { SetWindowLongPtrW(hwnd, GWLP_WNDPROC, new_proc) };
    if old != new_proc {
        BUTTON_PROC.store(old, Ordering::Relaxed);
    }
}

extern "system" fn button_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_MOUSEMOVE if flags(hwnd) & FLAG_HOT == 0 => {
            set_flag(hwnd, FLAG_HOT, true);
            let mut t = TRACKMOUSEEVENT {
                cbSize: size_of::<TRACKMOUSEEVENT>() as u32,
                dwFlags: TME_LEAVE,
                hwndTrack: hwnd,
                dwHoverTime: 0,
            };
            // SAFETY: t ist vollständig initialisiert und lebt über den Aufruf.
            let _ = unsafe { TrackMouseEvent(&mut t) };
        }
        WM_MOUSELEAVE => set_flag(hwnd, FLAG_HOT, false),
        _ => {}
    }
    // SAFETY: BUTTON_PROC ist die zuvor gemerkte Prozedur der Klasse BUTTON (oder 0 = None, dann tut der Aufruf
    // nichts); die Argumente sind die der Nachricht. Der Wert ist ein WNDPROC (Option<fn>) und gleich groß wie isize.
    unsafe {
        let old: WNDPROC = std::mem::transmute::<isize, WNDPROC>(BUTTON_PROC.load(Ordering::Relaxed));
        CallWindowProcW(old, hwnd, msg, wp, lp)
    }
}

/// fill_rect füllt ein Rechteck mit einer Farbe.
pub(super) fn fill_rect(hdc: HDC, r: &RECT, c: Rgb) {
    let Ok(b) = theme::solid_brush(c) else { return };
    // SAFETY: hdc und r stammen vom Aufrufer (WM_DRAWITEM) und gelten während des Aufrufs; b lebt bis zum Löschen.
    unsafe {
        FillRect(hdc, r, b);
    }
    theme::delete_object(b);
}

fn fill(dis: &DRAWITEMSTRUCT, c: Rgb) {
    fill_rect(dis.hDC, &dis.rcItem, c);
}

fn frame(dis: &DRAWITEMSTRUCT, c: Rgb) {
    let Ok(b) = theme::solid_brush(c) else { return };
    // SAFETY: wie `fill`.
    unsafe {
        FrameRect(dis.hDC, &dis.rcItem, b);
    }
    theme::delete_object(b);
}

/// draw_button zeichnet Schaltfläche oder Umschalter (flach): Hover, Gedrückt, Umschalter an (Akzentfläche),
/// Fokus (Akzentrahmen), deaktiviert (grauer Text).
pub(super) fn draw_button(dis: &DRAWITEMSTRUCT) {
    let f = flags(dis.hwndItem);
    let st = dis.itemState.0;
    let (disabled, pressed) = (st & ODS_DISABLED.0 != 0, st & ODS_SELECTED.0 != 0);
    let on = f & FLAG_ON != 0;
    let bg = match (pressed, on, f & FLAG_HOT != 0) {
        (true, _, _) => PRESSED,
        (_, true, _) => ACCENT,
        (_, _, true) => HOVER,
        _ => FIELD,
    };
    fill(dis, bg);
    frame(dis, if st & ODS_FOCUS.0 != 0 { ACCENT } else { BORDER });
    let mut buf = [0u16; 128];
    // SAFETY: buf ist ein beschreibbarer Puffer; GetWindowTextW schneidet auf dessen Länge.
    let n = unsafe { GetWindowTextW(dis.hwndItem, &mut buf) }.max(0) as usize;
    let color = if disabled {
        DISABLED_TEXT
    } else if on {
        BG
    } else {
        TEXT
    };
    let mut r: RECT = dis.rcItem;
    // SAFETY: hDC gilt während der Nachricht; buf[..n] und r leben über den Aufruf.
    unsafe {
        SetBkMode(dis.hDC, TRANSPARENT);
        SetTextColor(dis.hDC, windows::Win32::Foundation::COLORREF(color.colorref()));
        DrawTextW(
            dis.hDC,
            &mut buf[..n.min(128)],
            &mut r,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX,
        );
    }
}
