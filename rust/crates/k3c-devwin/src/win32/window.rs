//! Fensterklasse, Nachrichtenschleife und Fensterprozedur: verschiebbar, in der Größe änderbar, DPI-fest.

use std::cell::Cell;
use std::ffi::c_void;

use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, DT_NOPREFIX, DT_WORDBREAK, DrawTextW, EndPaint, HBRUSH, HFONT, InvalidateRect, PAINTSTRUCT,
    SelectObject, SetBkMode, SetTextColor, TRANSPARENT, UpdateWindow,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::HiDpi::{AdjustWindowRectExForDpi, GetDpiForSystem, GetDpiForWindow};
use windows::Win32::UI::WindowsAndMessaging::{
    CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DispatchMessageW,
    GWLP_USERDATA, GetClientRect, GetMessageW, GetSystemMetrics, GetWindowLongPtrW, IDC_ARROW, LoadCursorW, MINMAXINFO,
    MSG, PostQuitMessage, RegisterClassExW, SM_CYSCREEN, SW_SHOW, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOZORDER,
    SetWindowLongPtrW, SetWindowPos, ShowWindow, TranslateMessage, WINDOW_EX_STYLE, WINDOW_STYLE, WM_DESTROY,
    WM_DPICHANGED, WM_GETMINMAXINFO, WM_NCCREATE, WM_NCDESTROY, WM_PAINT, WM_SIZE, WNDCLASSEXW, WS_CLIPCHILDREN,
    WS_OVERLAPPEDWINDOW,
};
use windows::core::{Error, PCWSTR, Result, w};

use super::theme;
use crate::style::{self, GRID, MIN_SIZE, START_SIZE, scale};

const CLASS: PCWSTR = w!("K3CDevWin");
const STYLE: WINDOW_STYLE = WINDOW_STYLE(WS_OVERLAPPEDWINDOW.0 | WS_CLIPCHILDREN.0);

/// Ui ist der Zustand des Fensters; er hängt als Zeiger am Fenster (`GWLP_USERDATA`). Felder als `Cell`, weil
/// Aufrufe wie `SetWindowPos` die Fensterprozedur erneut betreten.
struct Ui {
    dpi: Cell<u32>,
    font: Cell<HFONT>,
}

impl Drop for Ui {
    fn drop(&mut self) {
        theme::delete_object(self.font.get());
    }
}

/// run öffnet das Fenster und läuft die Schleife; der Hintergrundpinsel lebt so lange wie der Prozess.
pub(super) fn run() -> Result<u8> {
    theme::enable_dpi_awareness();
    let bg = theme::solid_brush(style::BG)?;
    let r = open_and_loop(bg);
    theme::delete_object(bg);
    r
}

fn open_and_loop(bg: HBRUSH) -> Result<u8> {
    // SAFETY: GetModuleHandleW(None) liefert das Handle der eigenen EXE und braucht keine Zeiger.
    let hinst = HINSTANCE(unsafe { GetModuleHandleW(None) }?.0);
    register(hinst, bg)?;
    // SAFETY: Aufruf ohne Zeiger.
    let dpi = unsafe { GetDpiForSystem() };
    let ui = Box::new(Ui {
        dpi: Cell::new(dpi),
        font: Cell::new(theme::create_font(dpi)?),
    });
    let (w, h) = start_size(dpi);
    let title: Vec<u16> = style::TITLE.encode_utf16().chain(std::iter::once(0)).collect();
    // Die Box geht an die Fensterprozedur (WM_NCCREATE) und wird in WM_NCDESTROY freigegeben; scheitert die
    // Erzeugung vor WM_NCCREATE, endet der Prozess ohnehin.
    let param = Box::into_raw(ui).cast::<c_void>().cast_const();
    // SAFETY: Klasse ist registriert, title ist nullterminiert und lebt über den Aufruf, param ist ein
    // Box-Zeiger, den nur die Fensterprozedur dieses Fensters liest.
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            CLASS,
            PCWSTR(title.as_ptr()),
            STYLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            w,
            h,
            None,
            None,
            Some(hinst),
            Some(param),
        )
    }?;
    theme::dark_titlebar(hwnd);
    sync_dpi(hwnd);
    // SAFETY: hwnd wurde eben erzeugt und gehört diesem Thread.
    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOW);
        let _ = UpdateWindow(hwnd);
    }
    message_loop()
}

fn register(hinst: HINSTANCE, bg: HBRUSH) -> Result<()> {
    // SAFETY: Standardcursor des Systems, keine Zeiger.
    let cursor = unsafe { LoadCursorW(None, IDC_ARROW) }?;
    let wc = WNDCLASSEXW {
        cbSize: size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        hInstance: hinst,
        hCursor: cursor,
        hbrBackground: bg,
        lpszClassName: CLASS,
        ..Default::default()
    };
    // SAFETY: wc ist vollständig initialisiert und lebt über den Aufruf; CLASS ist ein statischer String.
    if unsafe { RegisterClassExW(&wc) } == 0 {
        return Err(Error::from_thread());
    }
    Ok(())
}

fn message_loop() -> Result<u8> {
    let mut msg = MSG::default();
    loop {
        // SAFETY: msg lebt über den Aufruf; None heißt Nachrichten aller Fenster dieses Threads.
        let r = unsafe { GetMessageW(&mut msg, None, 0, 0) }.0;
        if r == 0 {
            break;
        }
        if r < 0 {
            return Err(Error::from_thread());
        }
        // SAFETY: msg wurde von GetMessageW gefüllt und bleibt über beide Aufrufe gültig.
        unsafe {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok((msg.wParam.0 & 0xFF) as u8)
}

/// outer_size rechnet eine Client-Größe in dip in die Fenstergröße (mit Rahmen und Titelleiste) in Pixel.
fn outer_size(client: (i32, i32), dpi: u32) -> (i32, i32) {
    let mut r = RECT {
        left: 0,
        top: 0,
        right: scale(client.0, dpi),
        bottom: scale(client.1, dpi),
    };
    // SAFETY: r lebt über den Aufruf; STYLE ist ein gültiger Fensterstil ohne Menü.
    let ok = unsafe { AdjustWindowRectExForDpi(&mut r, STYLE, false, WINDOW_EX_STYLE(0), dpi) };
    if ok.is_err() {
        r.right += scale(2 * GRID, dpi);
        r.bottom += scale(6 * GRID, dpi);
    }
    (r.right - r.left, r.bottom - r.top)
}

/// start_size ist die Startgröße in Pixel, höchstens 90 % der Bildschirmhöhe (aber nie unter dem Minimum).
fn start_size(dpi: u32) -> (i32, i32) {
    let (w, h) = outer_size(START_SIZE, dpi);
    // SAFETY: Aufruf ohne Zeiger.
    let screen = unsafe { GetSystemMetrics(SM_CYSCREEN) };
    let (_, min_h) = outer_size(MIN_SIZE, dpi);
    (w, h.min(screen / 10 * 9).max(min_h))
}

fn dpi_of(hwnd: HWND) -> u32 {
    // SAFETY: Aufruf mit einem Fenster-Handle; bei ungültigem Handle liefert Windows 0.
    match unsafe { GetDpiForWindow(hwnd) } {
        0 => 96,
        d => d,
    }
}

fn ui_of<'a>(hwnd: HWND) -> Option<&'a Ui> {
    // SAFETY: GWLP_USERDATA ist 0 oder der Box-Zeiger aus `attach`, der bis WM_NCDESTROY lebt; die Fensterprozedur
    // läuft nur im UI-Thread, und Ui hat nur Cell-Felder, daher sind gleichzeitige `&Ui` zulässig.
    unsafe { (GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Ui).as_ref() }
}

fn attach(hwnd: HWND, lp: LPARAM) {
    // SAFETY: bei WM_NCCREATE zeigt lParam auf eine CREATESTRUCTW, deren lpCreateParams der Box-Zeiger ist.
    unsafe {
        if let Some(cs) = (lp.0 as *const CREATESTRUCTW).as_ref() {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, cs.lpCreateParams as isize);
        }
    }
}

fn detach(hwnd: HWND) {
    // SAFETY: der Zeiger stammt aus `attach` (Box::into_raw); nach dem Nullsetzen gibt es ihn nur noch hier.
    unsafe {
        let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Ui;
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        if !p.is_null() {
            drop(Box::from_raw(p));
        }
    }
}

extern "system" fn wndproc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_NCCREATE => attach(hwnd, lp),
        WM_GETMINMAXINFO => {
            min_info(hwnd, lp);
            return LRESULT(0);
        }
        WM_SIZE => {
            invalidate(hwnd);
            return LRESULT(0);
        }
        WM_PAINT => {
            paint(hwnd);
            return LRESULT(0);
        }
        WM_DPICHANGED => {
            dpi_changed(hwnd, wp, lp);
            return LRESULT(0);
        }
        WM_DESTROY => {
            // SAFETY: beendet die Schleife dieses Threads, keine Zeiger.
            unsafe { PostQuitMessage(0) };
            return LRESULT(0);
        }
        WM_NCDESTROY => detach(hwnd),
        _ => {}
    }
    // SAFETY: die Argumente sind die der Fensterprozedur und werden unverändert weitergereicht.
    unsafe { DefWindowProcW(hwnd, msg, wp, lp) }
}

fn invalidate(hwnd: HWND) {
    // SAFETY: None heißt das ganze Client-Rechteck; hwnd ist das eigene Fenster.
    let _ = unsafe { InvalidateRect(Some(hwnd), None, true) };
}

fn min_info(hwnd: HWND, lp: LPARAM) {
    // SAFETY: bei WM_GETMINMAXINFO zeigt lParam auf eine beschreibbare MINMAXINFO.
    let Some(mmi) = (unsafe { (lp.0 as *mut MINMAXINFO).as_mut() }) else {
        return;
    };
    let dpi = ui_of(hwnd).map_or_else(|| dpi_of(hwnd), |u| u.dpi.get());
    let (w, h) = outer_size(MIN_SIZE, dpi);
    mmi.ptMinTrackSize = POINT { x: w, y: h };
}

/// apply_dpi übernimmt eine neue Auflösung: erst die neue Schrift, dann der Wert (bei Fehler bleibt beides alt).
fn apply_dpi(ui: &Ui, dpi: u32) {
    match theme::create_font(dpi) {
        Ok(f) => {
            theme::delete_object(ui.font.replace(f));
            ui.dpi.set(dpi);
        }
        Err(e) => eprintln!("❌ Schrift für {dpi} dpi: {e}"),
    }
}

/// sync_dpi gleicht nach dem Erzeugen die Auflösung des Monitors ab, auf dem das Fenster landet (sonst käme sie
/// erst mit einem `WM_DPICHANGED`), und passt die Fenstergröße an.
fn sync_dpi(hwnd: HWND) {
    let (Some(ui), dpi) = (ui_of(hwnd), dpi_of(hwnd)) else {
        return;
    };
    if ui.dpi.get() == dpi {
        return;
    }
    apply_dpi(ui, dpi);
    let (w, h) = start_size(dpi);
    // SAFETY: hwnd ist das eigene Fenster; nur die Größe ändert sich.
    let r = unsafe { SetWindowPos(hwnd, None, 0, 0, w, h, SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE) };
    if let Err(e) = r {
        eprintln!("❌ Fenstergröße nicht angepasst: {e}");
    }
}

fn dpi_changed(hwnd: HWND, wp: WPARAM, lp: LPARAM) {
    let dpi = (wp.0 & 0xFFFF) as u32;
    if let Some(ui) = ui_of(hwnd) {
        apply_dpi(ui, dpi);
    }
    // SAFETY: bei WM_DPICHANGED zeigt lParam auf das vom System vorgeschlagene RECT, gültig während der Nachricht.
    if let Some(r) = unsafe { (lp.0 as *const RECT).as_ref() } {
        // SAFETY: hwnd ist das eigene Fenster, das RECT ist gelesen.
        let moved = unsafe {
            SetWindowPos(
                hwnd,
                None,
                r.left,
                r.top,
                r.right - r.left,
                r.bottom - r.top,
                SWP_NOZORDER | SWP_NOACTIVATE,
            )
        };
        if let Err(e) = moved {
            eprintln!("❌ Fenster nicht verschoben: {e}");
        }
    }
    invalidate(hwnd);
}

/// paint zeichnet das Gerüst: einen Hinweistext in Schrift und Farbe des Looks (RU56.3 ersetzt ihn durch Controls).
fn paint(hwnd: HWND) {
    let Some(ui) = ui_of(hwnd) else { return };
    let mut ps = PAINTSTRUCT::default();
    let mut r = RECT::default();
    let text = format!("K3C Cheat-Fenster (Gerüst)\n{} dpi", ui.dpi.get());
    let mut wide: Vec<u16> = text.encode_utf16().collect();
    // SAFETY: ps und r leben über die Aufrufe, BeginPaint/EndPaint sind gepaart, die Schrift gehört ui; wide ist
    // ein beschreibbarer Puffer ohne Nullterminator (DrawTextW nimmt die Länge aus der Slice).
    unsafe {
        let hdc = BeginPaint(hwnd, &mut ps);
        let _ = GetClientRect(hwnd, &mut r);
        let pad = scale(2 * GRID, ui.dpi.get());
        r.left += pad;
        r.top += pad;
        r.right -= pad;
        SelectObject(hdc, ui.font.get().into());
        SetBkMode(hdc, TRANSPARENT);
        SetTextColor(hdc, COLORREF(style::TEXT.colorref()));
        DrawTextW(hdc, &mut wide, &mut r, DT_WORDBREAK | DT_NOPREFIX);
        let _ = EndPaint(hwnd, &ps);
    }
}
