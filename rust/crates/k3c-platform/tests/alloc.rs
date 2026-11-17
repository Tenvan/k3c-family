//! Im Leerlauf (nach dem Aufwärmen) darf ein Bild nichts allozieren: Takt, Eingabe, Sammeln, Ausschnitte,
//! Sortieren, Schrift.
// Zählender Allocator braucht `unsafe`; die Bibliothek bleibt `forbid(unsafe_code)`.
#![allow(unsafe_code)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use k3c_platform::{App, Canvas, Frame, InputState, Key, PadAxis, PadButton, Rect, Runner, TexId};

thread_local! {
    static COUNT: Cell<u64> = const { Cell::new(0) };
}

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let _ = COUNT.try_with(|c| c.set(c.get() + 1));
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        let _ = COUNT.try_with(|c| c.set(c.get() + 1));
        unsafe { System.realloc(p, l, n) }
    }
}

#[global_allocator]
static A: Counting = Counting;

struct Scene {
    tex: Option<TexId>,
    tick: u32,
}

impl App for Scene {
    fn step(&mut self, input: &InputState, _dt: f64) {
        self.tick += 1 + u32::from(input.just_pressed(Key::Space));
        for p in input.pads() {
            self.tick += u32::from(p.held(PadButton::A)) + p.axis(PadAxis::LX) as u32;
        }
    }
    fn draw(&mut self, c: &mut dyn Canvas, alpha: f64) {
        let t = *self.tex.get_or_insert_with(|| c.texture(&[255; 16 * 16 * 4], 16, 16));
        for i in 0..2000usize {
            if i.is_multiple_of(1000) {
                c.clip(Some(Rect::new((i / 1000) as f32 * 960.0, 0.0, 960.0, 1080.0)));
            }
            let x = (i * 7 + self.tick as usize) as f32;
            c.sprite(
                t,
                Rect::new(0.0, 0.0, 16.0, 16.0),
                Rect::new(x % 1900.0, (i % 1000) as f32, 16.0, 16.0),
                [1.0; 4],
            );
        }
        c.clip(None);
        c.rect(Rect::new(10.0, 10.0, 100.0, 100.0), [1.0, 0.8, 0.0, 1.0]);
        c.text("PROBE N2000 #1", 60.0, 60.0, 56.0, [1.0; 4]);
        assert!(alpha < 1.0);
    }
    fn done(&self) -> bool {
        false
    }
}

fn bild(run: &mut Runner, frame: &mut Frame, i: u32) {
    let input = run.input_mut();
    if i.is_multiple_of(7) {
        input.keys.press(Key::Space);
        input.pad_mut(1).set_button(PadButton::A, true);
    } else {
        input.keys.release(Key::Space);
        input.pad_mut(1).set_axis(PadAxis::LX, 0.5);
    }
    frame.begin();
    run.advance(1.0 / 60.0, frame);
    frame.finish();
}

#[test]
fn leerlauf_bild_alloziert_nichts() {
    let mut app = Scene { tex: None, tick: 0 };
    let (mut run, mut frame) = (Runner::new(&mut app, None), Frame::new());
    for i in 0..4 {
        bild(&mut run, &mut frame, i);
    }
    let _uploads = frame.take_pending().count();
    let before = COUNT.with(Cell::get);
    for i in 0..120 {
        bild(&mut run, &mut frame, i);
    }
    let allocs = COUNT.with(Cell::get) - before;
    assert_eq!(allocs, 0, "{allocs} Allokationen in 120 Bildern");
    assert_eq!(frame.runs().len(), 3, "zwei Bildhälften und Vollfarbe");
}
