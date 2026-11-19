//! Seitenfläche unter den Reitern: ein Kindfenster, das die Controls der aktiven Seite trägt, Gruppen zeichnet und
//! senkrecht scrollt (Scrollleiste, Mausrad). Die Anordnung kommt aus `crate::layout`.

use std::cell::{Cell, RefCell};
use std::ffi::c_void;

use k3c_devproto::Catalog;
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{HBRUSH, HDC, HFONT, InvalidateRect, SetBkColor, SetTextColor};
use windows::Win32::UI::Controls::{DRAWITEMSTRUCT, SetWindowTheme};
use windows::Win32::UI::Input::KeyboardAndMouse::EnableWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    CB_SETITEMHEIGHT, CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW, CreateWindowExW, DefWindowProcW, DestroyWindow,
    GWLP_USERDATA, GetClientRect, GetParent, GetWindowLongPtrW, GetWindowRect, IDC_ARROW, LoadCursorW, PostMessageW,
    RegisterClassExW, SWP_NOACTIVATE, SWP_NOZORDER, SetWindowLongPtrW, SetWindowPos, WINDOW_EX_STYLE, WM_COMMAND,
    WM_CTLCOLOREDIT, WM_CTLCOLORLISTBOX, WM_CTLCOLORSTATIC, WM_DRAWITEM, WM_MOUSEWHEEL, WM_NCCREATE, WM_NCDESTROY,
    WM_PAINT, WM_SETFONT, WM_SIZE, WM_VSCROLL, WNDCLASSEXW, WS_CHILD, WS_CLIPCHILDREN, WS_VISIBLE, WS_VSCROLL,
};
use windows::core::{Error, PCWSTR, Result, w};

use super::app::WM_APP_CMD;
use super::controls::{self, FIELD};
use super::pageevents::{self, ID_BASE};
use super::{pagedraw, pagescroll, theme};
use crate::layout::{Group, Kind, Layout, Rect, layout};
use crate::pages::PAGES;
use crate::style::{self, TEXT, scale, unscale};

const CLASS: PCWSTR = w!("K3CDevPage");

/// Content ist der Inhalt der aktiven Seite: Gruppen, ihre Control-Fenster (flach, Gruppe für Gruppe) und das
/// zuletzt berechnete Layout.
#[derive(Default)]
struct Content {
    groups: Vec<Group>,
    hwnds: Vec<HWND>,
    layout: Layout,
}

/// PageState hängt als Zeiger am Seitenfenster; Felder als `Cell`/`RefCell`, weil Aufrufe wie `SetWindowPos`
/// die Fensterprozedur erneut betreten.
struct PageState {
    inst: HINSTANCE,
    dpi: Cell<u32>,
    font: Cell<HFONT>,
    bg: HBRUSH,
    field: HBRUSH,
    page: Cell<usize>,
    enabled: Cell<bool>,
    scroll: Cell<i32>,
    catalog: RefCell<Catalog>,
    content: RefCell<Content>,
}

impl Drop for PageState {
    fn drop(&mut self) {
        theme::delete_object(self.field);
    }
}

/// create registriert die Klasse und erzeugt die Seitenfläche mit der ersten Seite.
pub(super) fn create(parent: HWND, inst: HINSTANCE, bg: HBRUSH, dpi: u32, font: HFONT) -> Result<HWND> {
    register(inst, bg)?;
    let state = Box::new(PageState {
        inst,
        dpi: Cell::new(dpi),
        font: Cell::new(font),
        bg,
        field: theme::solid_brush(FIELD)?,
        page: Cell::new(0),
        enabled: Cell::new(false),
        scroll: Cell::new(0),
        catalog: RefCell::new(Catalog::default()),
        content: RefCell::new(Content::default()),
    });
    let param = Box::into_raw(state).cast::<c_void>().cast_const();
    // SAFETY: die Klasse ist registriert, param ist ein Box-Zeiger, den nur die Fensterprozedur dieses Fensters
    // liest (WM_NCCREATE) und in WM_NCDESTROY freigibt; parent ist ein gültiges Fenster dieses Threads.
    let hwnd = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            CLASS,
            PCWSTR::null(),
            WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_VSCROLL,
            0,
            0,
            10,
            10,
            Some(parent),
            None,
            Some(inst),
            Some(param),
        )
    }?;
    // SAFETY: Klassennamen sind statische Strings; ein Fehler (alte Windows-Version) lässt die Leiste hell.
    let _ = unsafe { SetWindowTheme(hwnd, w!("DarkMode_Explorer"), PCWSTR::null()) };
    select(hwnd, 0);
    Ok(hwnd)
}

fn register(inst: HINSTANCE, bg: HBRUSH) -> Result<()> {
    // SAFETY: Standardcursor des Systems, keine Zeiger.
    let cursor = unsafe { LoadCursorW(None, IDC_ARROW) }?;
    let wc = WNDCLASSEXW {
        cbSize: size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        hInstance: inst,
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

fn state_of<'a>(hwnd: HWND) -> Option<&'a PageState> {
    // SAFETY: GWLP_USERDATA ist 0 oder der Box-Zeiger aus WM_NCCREATE, der bis WM_NCDESTROY lebt; die Prozedur
    // läuft nur im UI-Thread, PageState hat nur Cell/RefCell-Felder, daher sind gleichzeitige `&` zulässig.
    unsafe { (GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const PageState).as_ref() }
}

/// select zeigt die Seite `idx` (Reiter) und setzt das Scrollen zurück.
pub(super) fn select(hwnd: HWND, idx: usize) {
    if let Some(st) = state_of(hwnd) {
        st.page.set(idx);
        st.scroll.set(0);
        rebuild(hwnd, st);
    }
}

/// update_font übernimmt Auflösung und Schrift nach einem DPI-Wechsel (die alte Schrift gibt der Aufrufer danach frei).
pub(super) fn update_font(hwnd: HWND, dpi: u32, font: HFONT) {
    let Some(st) = state_of(hwnd) else { return };
    st.dpi.set(dpi);
    st.font.set(font);
    let hwnds = st.content.try_borrow().map(|c| c.hwnds.clone()).unwrap_or_default();
    for h in hwnds {
        controls::send(h, WM_SETFONT, font.0 as usize, 1);
    }
    relayout(hwnd, st);
}

fn rebuild(hwnd: HWND, st: &PageState) {
    let old = st
        .content
        .try_borrow_mut()
        .map(|mut c| std::mem::take(&mut c.hwnds))
        .unwrap_or_default();
    for h in old {
        // SAFETY: h ist ein von diesem Fenster erzeugtes Kind; nach dem Zerstören wird das Handle verworfen.
        let _ = unsafe { DestroyWindow(h) };
    }
    let groups = {
        let cat = st.catalog.try_borrow().map(|c| c.clone()).unwrap_or_default();
        PAGES.get(st.page.get()).map(|p| (p.build)(&cat)).unwrap_or_default()
    };
    let mut hwnds = Vec::new();
    for c in groups.iter().flat_map(|g| g.controls.iter()) {
        let id = ID_BASE + hwnds.len();
        match controls::create(hwnd, st.inst, id, c, st.font.get()) {
            Ok(h) => hwnds.push(h),
            Err(e) => {
                eprintln!("❌ Control {} nicht erzeugt: {e}", c.key);
                hwnds.push(HWND::default());
            }
        }
    }
    if let Ok(mut c) = st.content.try_borrow_mut() {
        *c = Content {
            groups,
            hwnds,
            layout: Layout::default(),
        };
    }
    apply_enabled(st);
    relayout(hwnd, st);
}

fn flat_index(groups: &[Group], group: usize, control: usize) -> usize {
    groups.iter().take(group).map(|g| g.controls.len()).sum::<usize>() + control
}

/// relayout berechnet das Layout für die aktuelle Fenstergröße neu, stellt die Scrollleiste ein und setzt die Controls.
fn relayout(hwnd: HWND, st: &PageState) {
    let dpi = st.dpi.get();
    let (mut win, mut cli) = (RECT::default(), RECT::default());
    // SAFETY: win und cli leben über die Aufrufe; hwnd ist das eigene Fenster.
    unsafe {
        let _ = GetWindowRect(hwnd, &mut win);
        let _ = GetClientRect(hwnd, &mut cli);
    }
    let (full_w, page_h) = (unscale(win.right - win.left, dpi), cli.bottom - cli.top);
    if let Ok(mut c) = st.content.try_borrow_mut() {
        c.layout = layout(full_w, unscale(page_h, dpi), &c.groups);
        let content_px = scale(c.layout.content_height, dpi);
        let max = (content_px - page_h).max(0);
        st.scroll.set(st.scroll.get().clamp(0, max));
        pagescroll::set_scrollbar(hwnd, content_px, page_h, st.scroll.get());
    }
    position(st);
    // SAFETY: hwnd ist das eigene Fenster; None heißt das ganze Client-Rechteck.
    let _ = unsafe { InvalidateRect(Some(hwnd), None, true) };
}

/// position setzt jedes Control an seinen Platz (Layout in dip, Scroll-Versatz in Pixel).
fn position(st: &PageState) {
    let Ok(c) = st
        .content
        .try_borrow()
        .map(|c| (c.layout.clone(), c.groups.clone(), c.hwnds.clone()))
    else {
        return;
    };
    let (lay, groups, hwnds) = c;
    let (dpi, off) = (st.dpi.get(), st.scroll.get());
    for p in &lay.controls {
        let Some(h) = hwnds.get(flat_index(&groups, p.group, p.control)).copied() else {
            continue;
        };
        let r = p.rect;
        let (x, y, w, mut hgt) = (scale(r.x, dpi), scale(r.y, dpi) - off, scale(r.w, dpi), scale(r.h, dpi));
        let is_combo = groups
            .get(p.group)
            .and_then(|g| g.controls.get(p.control))
            .map(|c| c.kind)
            == Some(Kind::Select);
        if is_combo {
            controls::send(h, CB_SETITEMHEIGHT, usize::MAX, (hgt - scale(6, dpi)) as isize);
            hgt += scale(200, dpi);
        }
        // SAFETY: h ist ein Kind dieses Fensters (oder ein ungültiges Handle, dann scheitert der Aufruf folgenlos).
        let _ = unsafe { SetWindowPos(h, None, x, y, w, hgt, SWP_NOZORDER | SWP_NOACTIVATE) };
    }
}

fn scroll_to(hwnd: HWND, st: &PageState, pos: i32) {
    let (dpi, ch) = (
        st.dpi.get(),
        st.content.try_borrow().map(|c| c.layout.content_height).unwrap_or(0),
    );
    let mut cli = RECT::default();
    // SAFETY: cli lebt über den Aufruf; hwnd ist das eigene Fenster.
    let _ = unsafe { GetClientRect(hwnd, &mut cli) };
    let max = (scale(ch, dpi) - (cli.bottom - cli.top)).max(0);
    let pos = pos.clamp(0, max);
    if pos != st.scroll.get() {
        st.scroll.set(pos);
        pagescroll::set_scrollbar(hwnd, scale(ch, dpi), cli.bottom - cli.top, pos);
        position(st);
        // SAFETY: hwnd ist das eigene Fenster.
        let _ = unsafe { InvalidateRect(Some(hwnd), None, true) };
    }
}

fn on_vscroll(hwnd: HWND, st: &PageState, code: u16) {
    let line = scale(40, st.dpi.get());
    if let Some(to) = pagescroll::target(hwnd, code, st.scroll.get(), line) {
        scroll_to(hwnd, st, to);
    }
}

fn on_command(hwnd: HWND, st: &PageState, wp: WPARAM) {
    let (id, note) = (wp.0 & 0xFFFF, (wp.0 >> 16) & 0xFFFF);
    let found = st
        .content
        .try_borrow()
        .ok()
        .and_then(|c| pageevents::command(&c.groups, &c.hwnds, id, note));
    let Some(cmd) = found else { return };
    let boxed = Box::into_raw(Box::new(cmd));
    // SAFETY: lParam trägt den Box-Zeiger, den das Hauptfenster in WM_APP_CMD freigibt; GetParent liefert das
    // Hauptfenster dieses Kindfensters.
    let sent = unsafe {
        let parent = GetParent(hwnd).unwrap_or_default();
        PostMessageW(Some(parent), WM_APP_CMD, WPARAM(0), LPARAM(boxed as isize))
    };
    if sent.is_err() {
        // SAFETY: die Nachricht wurde nicht zugestellt, der Zeiger stammt aus Box::into_raw und gehört noch uns.
        drop(unsafe { Box::from_raw(boxed) });
    }
}

/// set_catalog übernimmt den Katalog des Spiels und baut die Seite neu auf.
pub(super) fn set_catalog(hwnd: HWND, catalog: Catalog) {
    if let Some(st) = state_of(hwnd) {
        if let Ok(mut c) = st.catalog.try_borrow_mut() {
            *c = catalog;
        }
        rebuild(hwnd, st);
    }
}

/// set_enabled sperrt oder gibt alle Controls der Seite frei (gesperrt, solange das Spiel nicht verbunden ist).
pub(super) fn set_enabled(hwnd: HWND, on: bool) {
    let Some(st) = state_of(hwnd) else { return };
    st.enabled.set(on);
    apply_enabled(st);
}

fn apply_enabled(st: &PageState) {
    let hwnds = st.content.try_borrow().map(|c| c.hwnds.clone()).unwrap_or_default();
    for h in hwnds {
        // SAFETY: h ist ein Kind dieses Fensters oder ein ungültiges Handle (dann scheitert der Aufruf folgenlos).
        let _ = unsafe { EnableWindow(h, st.enabled.get()) };
    }
}

fn paint(hwnd: HWND, st: &PageState) {
    let groups: Vec<(Rect, String)> = st
        .content
        .try_borrow()
        .map(|c| {
            c.layout
                .groups
                .iter()
                .copied()
                .zip(c.groups.iter().map(|g| g.title.clone()))
                .collect()
        })
        .unwrap_or_default();
    pagedraw::paint(hwnd, (st.dpi.get(), st.scroll.get()), st.font.get(), &groups);
}

/// ctl_color färbt Beschriftung (BG), Eingabe und Liste (FIELD) für WM_CTLCOLOR*.
fn ctl_color(st: &PageState, msg: u32, wp: WPARAM) -> LRESULT {
    let hdc = HDC(wp.0 as *mut c_void);
    let (back, brush) = if msg == WM_CTLCOLORSTATIC {
        (style::BG, st.bg)
    } else {
        (style::Rgb(0x2B, 0x2D, 0x31), st.field)
    };
    // SAFETY: hdc stammt aus der Nachricht und gilt während ihr; die Pinsel gehören dem Zustand.
    unsafe {
        SetTextColor(hdc, COLORREF(TEXT.colorref()));
        SetBkColor(hdc, COLORREF(back.colorref()));
    }
    LRESULT(brush.0 as isize)
}

extern "system" fn wndproc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    if msg == WM_NCCREATE {
        attach(hwnd, lp);
    } else if msg == WM_NCDESTROY {
        detach(hwnd);
    } else if let Some(st) = state_of(hwnd) {
        match msg {
            WM_SIZE => relayout(hwnd, st),
            WM_VSCROLL => on_vscroll(hwnd, st, (wp.0 & 0xFFFF) as u16),
            WM_MOUSEWHEEL => {
                let delta = i32::from((wp.0 >> 16) as u16 as i16);
                scroll_to(hwnd, st, st.scroll.get() - delta * scale(40, st.dpi.get()) / 120);
            }
            WM_PAINT => paint(hwnd, st),
            WM_COMMAND => on_command(hwnd, st, wp),
            WM_CTLCOLOREDIT | WM_CTLCOLORLISTBOX | WM_CTLCOLORSTATIC => return ctl_color(st, msg, wp),
            WM_DRAWITEM => {
                // SAFETY: bei WM_DRAWITEM zeigt lParam auf eine DRAWITEMSTRUCT, gültig während der Nachricht.
                if let Some(dis) = unsafe { (lp.0 as *const DRAWITEMSTRUCT).as_ref() } {
                    controls::draw_button(dis);
                }
                return LRESULT(1);
            }
            _ => {}
        }
        if matches!(msg, WM_SIZE | WM_VSCROLL | WM_MOUSEWHEEL | WM_PAINT | WM_COMMAND) {
            return LRESULT(0);
        }
    }
    // SAFETY: die Argumente sind die der Fensterprozedur und werden unverändert weitergereicht.
    unsafe { DefWindowProcW(hwnd, msg, wp, lp) }
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
        let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut PageState;
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        if !p.is_null() {
            drop(Box::from_raw(p));
        }
    }
}
