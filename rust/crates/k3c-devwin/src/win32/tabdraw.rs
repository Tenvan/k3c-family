//! Eigene Zeichnung der Reiterleiste (Subclass von `SysTabControl32`): Hintergrund, Reiter und Akzentlinie im
//! Dark-Look. Auswahl, Größen und Benachrichtigungen bleiben beim Control.

use std::ffi::c_void;
use std::sync::atomic::{AtomicIsize, Ordering};

use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, DT_CENTER, DT_NOPREFIX, DT_SINGLELINE, DT_VCENTER, DrawTextW, EndPaint, HDC, HFONT, PAINTSTRUCT,
    SelectObject, SetBkMode, SetTextColor, TRANSPARENT,
};
use windows::Win32::UI::Controls::{TCM_GETCURSEL, TCM_GETITEMCOUNT, TCM_GETITEMRECT};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, GWLP_WNDPROC, GetClientRect, SetWindowLongPtrW, WM_ERASEBKGND, WM_GETFONT, WM_PAINT, WNDPROC,
};

use super::controls::{self, FIELD, fill_rect};
use crate::pages::PAGES;
use crate::style::{ACCENT, BG, Rgb, TEXT, scale};

const MUTED: Rgb = Rgb(0x9A, 0x9D, 0xA3);

/// Ursprüngliche Fensterprozedur der Klasse SysTabControl32.
static TAB_PROC: AtomicIsize = AtomicIsize::new(0);

/// subclass übernimmt Hintergrund und Zeichnung der Leiste.
pub(super) fn subclass(tab: HWND) {
    let new_proc = tab_proc as *const () as usize as isize;
    // SAFETY: tauscht die Fensterprozedur dieser Leiste gegen `tab_proc`, die an die alte weiterreicht; die alte
    // Prozedur wird gemerkt (nur eine Leiste im Prozess).
    let old = unsafe { SetWindowLongPtrW(tab, GWLP_WNDPROC, new_proc) };
    if old != new_proc {
        TAB_PROC.store(old, Ordering::Relaxed);
    }
}

extern "system" fn tab_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_ERASEBKGND => return LRESULT(1),
        WM_PAINT => {
            paint(hwnd);
            return LRESULT(0);
        }
        _ => {}
    }
    // SAFETY: TAB_PROC ist die zuvor gemerkte Prozedur der Klasse (0 = None, dann tut der Aufruf nichts); die
    // Argumente sind die der Nachricht. WNDPROC (Option<fn>) und isize sind gleich groß.
    unsafe {
        let old: WNDPROC = std::mem::transmute::<isize, WNDPROC>(TAB_PROC.load(Ordering::Relaxed));
        CallWindowProcW(old, hwnd, msg, wp, lp)
    }
}

fn paint(hwnd: HWND) {
    let mut ps = PAINTSTRUCT::default();
    let mut cli = RECT::default();
    // SAFETY: ps und cli leben über die Aufrufe; BeginPaint/EndPaint sind gepaart, hdc gilt bis EndPaint.
    let hdc = unsafe {
        let hdc = BeginPaint(hwnd, &mut ps);
        let _ = GetClientRect(hwnd, &mut cli);
        hdc
    };
    fill_rect(hdc, &cli, BG);
    let font = HFONT(controls::send(hwnd, WM_GETFONT, 0, 0).0 as *mut c_void);
    // SAFETY: die Schrift gehört dem Hauptfenster und lebt über das Zeichnen; None-Handles wählt GDI nicht.
    unsafe { SelectObject(hdc, font.into()) };
    let count = controls::send(hwnd, TCM_GETITEMCOUNT, 0, 0).0;
    let cur = controls::send(hwnd, TCM_GETCURSEL, 0, 0).0;
    // SAFETY: Aufruf mit dem eigenen Fenster; liefert 0 bei Fehler.
    let dpi = unsafe { GetDpiForWindow(hwnd) };
    for i in 0..count.max(0) {
        let mut r = RECT::default();
        controls::send(hwnd, TCM_GETITEMRECT, i as usize, (&raw mut r) as isize);
        let title = PAGES.get(i as usize).map_or("", |p| p.title);
        draw_tab(hdc, r, title, i == cur, dpi);
    }
    // SAFETY: gepaart mit BeginPaint.
    let _ = unsafe { EndPaint(hwnd, &ps) };
}

/// draw_tab zeichnet einen Reiter: gewählt mit Fläche und Akzentlinie, sonst gedämpfter Text.
fn draw_tab(hdc: HDC, mut r: RECT, title: &str, sel: bool, dpi: u32) {
    if sel {
        fill_rect(hdc, &r, FIELD);
        let line = RECT {
            top: r.bottom - scale(3, dpi),
            ..r
        };
        fill_rect(hdc, &line, ACCENT);
    }
    let mut t: Vec<u16> = title.encode_utf16().collect();
    let color = if sel { TEXT } else { MUTED };
    // SAFETY: hdc gilt während des Zeichnens; t und r leben über den Aufruf.
    unsafe {
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, COLORREF(color.colorref()));
        DrawTextW(
            hdc,
            &mut t,
            &mut r,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX,
        );
    }
}
