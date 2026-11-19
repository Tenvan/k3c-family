//! Zeichnen der Seitenfläche: Gruppen als Rahmen mit Titel (die Controls zeichnen sich selbst).

use windows::Win32::Foundation::{COLORREF, HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, CreatePen, DT_NOPREFIX, DT_SINGLELINE, DT_VCENTER, DrawTextW, EndPaint, GetStockObject, HDC, HFONT,
    HGDIOBJ, NULL_BRUSH, PAINTSTRUCT, PS_SOLID, RoundRect, SelectObject, SetBkMode, SetTextColor, TRANSPARENT,
};

use super::controls::BORDER;
use super::theme;
use crate::layout::Rect;
use crate::style::{ACCENT, GRID, scale};

/// paint zeichnet alle Gruppen (Rechtecke in dip) mit Scroll-Versatz `off` in Pixel; `dpi_off` = (dpi, off).
pub(super) fn paint(hwnd: HWND, (dpi, off): (u32, i32), font: HFONT, groups: &[(Rect, String)]) {
    let mut ps = PAINTSTRUCT::default();
    // SAFETY: ps lebt über Begin/EndPaint (gepaart); hdc gilt bis EndPaint.
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
    // SAFETY: Schrift gehört dem Hauptfenster und lebt über das Zeichnen.
    unsafe { SelectObject(hdc, font.into()) };
    for (r, title) in groups {
        let (x, y, w, h) = (scale(r.x, dpi), scale(r.y, dpi) - off, scale(r.w, dpi), scale(r.h, dpi));
        draw_group(hdc, (x, y, w, h), title, dpi);
    }
    // SAFETY: gepaart mit BeginPaint.
    let _ = unsafe { EndPaint(hwnd, &ps) };
}

fn draw_group(hdc: HDC, (x, y, w, h): (i32, i32, i32, i32), title: &str, dpi: u32) {
    // SAFETY: Stift wird erzeugt, gewählt, zurückgetauscht und gelöscht; Hohlpinsel ist ein Stock-Objekt.
    unsafe {
        let pen = CreatePen(PS_SOLID, 1, COLORREF(BORDER.colorref()));
        let old_pen = SelectObject(hdc, pen.into());
        let old_brush = SelectObject(hdc, GetStockObject(NULL_BRUSH));
        let rad = scale(6, dpi);
        let _ = RoundRect(hdc, x, y + scale(2 * GRID, dpi), x + w, y + h, rad, rad);
        SelectObject(hdc, old_brush);
        SelectObject(hdc, old_pen);
        theme::delete_object(HGDIOBJ(pen.0));
    }
    let mut t: Vec<u16> = title.encode_utf16().collect();
    let mut r = RECT {
        left: x + scale(2, dpi),
        top: y,
        right: x + w - scale(GRID, dpi),
        bottom: y + scale(2 * GRID - 2, dpi),
    };
    // SAFETY: hdc gilt während des Zeichnens; t und r leben über den Aufruf.
    unsafe {
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, COLORREF(ACCENT.colorref()));
        DrawTextW(hdc, &mut t, &mut r, DT_SINGLELINE | DT_VCENTER | DT_NOPREFIX);
    }
}
