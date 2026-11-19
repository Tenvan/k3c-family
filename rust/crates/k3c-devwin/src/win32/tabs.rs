//! Reiterleiste (`SysTabControl32`, owner-draw im Dark-Look) und Seitenfläche darunter.

use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM};
use windows::Win32::Graphics::Gdi::{HBRUSH, HFONT};
use windows::Win32::UI::Controls::{
    ICC_TAB_CLASSES, INITCOMMONCONTROLSEX, InitCommonControlsEx, NMHDR, TCIF_TEXT, TCITEMW, TCM_GETCURSEL,
    TCM_INSERTITEMW, TCM_SETITEMSIZE, TCN_SELCHANGE, TCS_BUTTONS, TCS_FIXEDWIDTH, TCS_FLATBUTTONS, TCS_FOCUSNEVER,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, SWP_NOACTIVATE, SWP_NOZORDER, SetWindowPos, WINDOW_EX_STYLE, WINDOW_STYLE, WM_SETFONT, WS_CHILD,
    WS_CLIPSIBLINGS, WS_VISIBLE,
};
use windows::core::{PWSTR, Result, w};

use super::controls::{self, wide};
use super::{page, tabdraw};
use crate::pages::PAGES;
use crate::style::scale;

/// Höhe der Reiterleiste in dip.
const TAB_H: i32 = 36;

/// create erzeugt Reiterleiste und Seitenfläche als Kinder von `parent`.
pub(super) fn create(parent: HWND, inst: HINSTANCE, bg: HBRUSH, dpi: u32, font: HFONT) -> Result<(HWND, HWND)> {
    let icc = INITCOMMONCONTROLSEX {
        dwSize: size_of::<INITCOMMONCONTROLSEX>() as u32,
        dwICC: ICC_TAB_CLASSES,
    };
    // SAFETY: icc ist vollständig initialisiert und lebt über den Aufruf.
    let _ = unsafe { InitCommonControlsEx(&icc) };
    let style = WS_CHILD.0 | WS_VISIBLE.0 | WS_CLIPSIBLINGS.0;
    let tab_style = TCS_BUTTONS | TCS_FLATBUTTONS | TCS_FIXEDWIDTH | TCS_FOCUSNEVER;
    // SAFETY: Klassenname ist ein statischer String, parent ein gültiges Fenster dieses Threads.
    let tab = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("SysTabControl32"),
            None,
            WINDOW_STYLE(style | tab_style),
            0,
            0,
            10,
            10,
            Some(parent),
            None,
            Some(inst),
            None,
        )
    }?;
    controls::send(tab, WM_SETFONT, font.0 as usize, 1);
    tabdraw::subclass(tab);
    for (i, p) in PAGES.iter().enumerate() {
        let mut text = wide(p.title);
        let item = TCITEMW {
            mask: TCIF_TEXT,
            pszText: PWSTR(text.as_mut_ptr()),
            ..Default::default()
        };
        controls::send(tab, TCM_INSERTITEMW, i, (&raw const item) as isize);
    }
    let page = page::create(parent, inst, bg, dpi, font)?;
    Ok((tab, page))
}

/// arrange ordnet Leiste und Seitenfläche in einem Client-Bereich von `w` x `h` Pixeln an.
pub(super) fn arrange(tab: HWND, page: HWND, (w, h): (i32, i32), dpi: u32) {
    let th = scale(TAB_H, dpi).min(h);
    let n = PAGES.len() as i32;
    let item = ((th as isize) << 16) | ((w - scale(20, dpi)) / n - scale(10, dpi)).max(1) as isize;
    controls::send(tab, TCM_SETITEMSIZE, 0, item);
    // SAFETY: tab und page sind Kinder des Hauptfensters; die Aufrufe ändern nur Lage und Größe.
    unsafe {
        let _ = SetWindowPos(tab, None, 0, 0, w, th, SWP_NOZORDER | SWP_NOACTIVATE);
        let _ = SetWindowPos(page, None, 0, th, w, (h - th).max(0), SWP_NOZORDER | SWP_NOACTIVATE);
    }
}

/// set_font gibt Reiterleiste und Seitenfläche eine neue Schrift und Auflösung.
pub(super) fn set_font(tab: HWND, page: HWND, dpi: u32, font: HFONT) {
    controls::send(tab, WM_SETFONT, font.0 as usize, 1);
    page::update_font(page, dpi, font);
}

/// on_notify wertet WM_NOTIFY aus: wechselt die Seite, wenn der Reiter geändert wurde.
pub(super) fn on_notify(tab: HWND, page: HWND, lp: LPARAM) {
    // SAFETY: bei WM_NOTIFY zeigt lParam auf eine NMHDR, gültig während der Nachricht.
    let Some(nm) = (unsafe { (lp.0 as *const NMHDR).as_ref() }) else {
        return;
    };
    if nm.hwndFrom == tab && nm.code == TCN_SELCHANGE {
        let sel = controls::send(tab, TCM_GETCURSEL, 0, 0).0;
        if let Ok(i) = usize::try_from(sel) {
            page::select(page, i);
        }
    }
}
