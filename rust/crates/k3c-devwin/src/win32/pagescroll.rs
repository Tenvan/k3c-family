//! Scrollleiste der Seitenfläche (Win32-`SCROLLINFO`).

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Controls::SetScrollInfo;
use windows::Win32::UI::WindowsAndMessaging::{
    GetScrollInfo, SB_VERT, SCROLLINFO, SIF_PAGE, SIF_POS, SIF_RANGE, SIF_TRACKPOS,
};

pub(super) fn set_scrollbar(hwnd: HWND, content_px: i32, page_px: i32, pos: i32) {
    let si = SCROLLINFO {
        cbSize: size_of::<SCROLLINFO>() as u32,
        fMask: SIF_RANGE | SIF_PAGE | SIF_POS,
        nMin: 0,
        nMax: (content_px - 1).max(0),
        nPage: page_px.max(0) as u32,
        nPos: pos,
        ..Default::default()
    };
    // SAFETY: si ist vollständig initialisiert und lebt über den Aufruf.
    unsafe { SetScrollInfo(hwnd, SB_VERT, &si, true) };
}

/// target liefert die neue Scroll-Position für einen Scrollcode (`SB_*`) bei Position `cur` und Zeilenhöhe `line`;
/// `None` bei unbekanntem Code.
pub(super) fn target(hwnd: HWND, code: u16, cur: i32, line: i32) -> Option<i32> {
    let mut si = SCROLLINFO {
        cbSize: size_of::<SCROLLINFO>() as u32,
        fMask: SIF_RANGE | SIF_PAGE | SIF_POS | SIF_TRACKPOS,
        ..Default::default()
    };
    // SAFETY: si lebt über den Aufruf und trägt cbSize und Maske.
    let _ = unsafe { GetScrollInfo(hwnd, SB_VERT, &mut si) };
    let page = si.nPage as i32;
    match code {
        0 => Some(cur - line),
        1 => Some(cur + line),
        2 => Some(cur - page),
        3 => Some(cur + page),
        4 | 5 => Some(si.nTrackPos),
        6 => Some(0),
        7 => Some(i32::MAX),
        _ => None,
    }
}
