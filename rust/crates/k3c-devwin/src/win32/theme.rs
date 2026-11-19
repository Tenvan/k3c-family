//! Dark-Look: DPI-Modus, dunkle Titelleiste, Pinsel und die Systemschrift (Segoe UI 9 pt, DPI-skaliert).

use std::ffi::c_void;
use std::mem::size_of;

use windows::Win32::Foundation::COLORREF;
use windows::Win32::Graphics::Dwm::{DWMWA_USE_IMMERSIVE_DARK_MODE, DwmSetWindowAttribute};
use windows::Win32::Graphics::Gdi::{
    CreateFontIndirectW, CreateSolidBrush, DeleteObject, HBRUSH, HFONT, HGDIOBJ, LOGFONTW,
};
use windows::Win32::UI::HiDpi::{
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext, SystemParametersInfoForDpi,
};
use windows::Win32::UI::WindowsAndMessaging::{NONCLIENTMETRICSW, SPI_GETNONCLIENTMETRICS};
use windows::core::{BOOL, Error, Result};

use crate::style::{self, Rgb};

/// enable_dpi_awareness schaltet den Prozess auf Per-Monitor-DPI v2 (vor dem ersten Fenster). Ein Fehler ist
/// harmlos (der Modus kann schon gesetzt sein); das Fenster ist dann nur unscharf skaliert.
pub(super) fn enable_dpi_awareness() {
    // SAFETY: Aufruf ohne Zeiger, wirkt nur auf den eigenen Prozess; die Konstante ist ein gültiger Kontext.
    let r = unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
    if let Err(e) = r {
        eprintln!("🐛 DPI-Modus nicht gesetzt: {e}");
    }
}

/// dark_titlebar färbt die Titelleiste dunkel; ohne Unterstützung (ältere Windows-Version) bleibt sie hell.
pub(super) fn dark_titlebar(hwnd: windows::Win32::Foundation::HWND) {
    let on = BOOL::from(true);
    // SAFETY: on lebt über den Aufruf, die Größe passt zum Typ BOOL, hwnd ist ein gültiges Fenster des Aufrufers.
    let r = unsafe {
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            (&raw const on).cast::<c_void>(),
            size_of::<BOOL>() as u32,
        )
    };
    if let Err(e) = r {
        eprintln!("🐛 dunkle Titelleiste nicht verfügbar: {e}");
    }
}

/// solid_brush legt einen einfarbigen Pinsel an; der Aufrufer gibt ihn mit `delete_object` frei.
pub(super) fn solid_brush(c: Rgb) -> Result<HBRUSH> {
    // SAFETY: reine GDI-Allokation ohne Zeiger; das Handle gehört dem Aufrufer.
    let b = unsafe { CreateSolidBrush(COLORREF(c.colorref())) };
    if b.is_invalid() {
        Err(Error::from_thread())
    } else {
        Ok(b)
    }
}

/// delete_object gibt ein GDI-Objekt (Pinsel, Schrift) frei; ein Fehler ist nicht behebbar und wird ignoriert.
pub(super) fn delete_object(h: impl Into<HGDIOBJ>) {
    // SAFETY: das Handle stammt aus einem Create-Aufruf dieses Moduls und wird danach nicht mehr benutzt.
    let _ = unsafe { DeleteObject(h.into()) };
}

/// create_font legt die Menüschrift des Systems für die Auflösung dpi an (laut Windows Segoe UI 9 pt); wenn sie
/// nicht lesbar ist, Segoe UI mit 12 px bei 96 dpi.
pub(super) fn create_font(dpi: u32) -> Result<HFONT> {
    let mut ncm = NONCLIENTMETRICSW {
        cbSize: size_of::<NONCLIENTMETRICSW>() as u32,
        ..Default::default()
    };
    // SAFETY: pvparam zeigt auf ncm mit gesetzter cbSize und bleibt über den Aufruf gültig.
    let got = unsafe {
        SystemParametersInfoForDpi(
            SPI_GETNONCLIENTMETRICS.0,
            ncm.cbSize,
            Some((&raw mut ncm).cast::<c_void>()),
            0,
            dpi,
        )
    };
    let lf = if got.is_ok() {
        ncm.lfMessageFont
    } else {
        fallback_font(dpi)
    };
    // SAFETY: lf lebt über den Aufruf (CreateFontIndirectW kopiert die Beschreibung); das Handle gehört dem Aufrufer.
    let f = unsafe { CreateFontIndirectW(&lf) };
    if f.is_invalid() {
        Err(Error::from_thread())
    } else {
        Ok(f)
    }
}

fn fallback_font(dpi: u32) -> LOGFONTW {
    let mut lf = LOGFONTW {
        lfHeight: -style::scale(12, dpi),
        ..Default::default()
    };
    for (d, s) in lf.lfFaceName.iter_mut().zip("Segoe UI".encode_utf16()) {
        *d = s;
    }
    lf
}
