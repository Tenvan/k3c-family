use super::*;
use crate::{Key, NullCanvas, PadAxis, PadButton, run_headless};

/// Rechnet einen Zustand nur aus der Eingabe: Position, Tipps, Pad-Summe, Prüfsumme über alle Takte.
#[derive(Default, Debug, Clone, PartialEq)]
struct Walker {
    steps: u32,
    x: i64,
    taps: u32,
    pad: f64,
    sum: u64,
}

impl App for Walker {
    fn step(&mut self, input: &InputState, _dt: f64) {
        self.steps += 1;
        if input.held(Key::D) {
            self.x += 1;
        }
        if input.held(Key::A) {
            self.x -= 1;
        }
        if input.just_pressed(Key::Space) {
            self.taps += 1;
        }
        for p in input.pads() {
            self.pad += p.axis(PadAxis::LX);
            if p.just_pressed(PadButton::A) {
                self.taps += 100;
            }
        }
        let h = (self.x as u64) ^ ((self.taps as u64) << 20) ^ (self.pad.to_bits() >> 3);
        self.sum = self.sum.wrapping_mul(1_000_003).wrapping_add(h);
    }
    fn draw(&mut self, _c: &mut dyn Canvas, alpha: f64) {
        assert!((0.0..1.0).contains(&alpha));
    }
    fn done(&self) -> bool {
        self.steps >= 1000
    }
}

fn script() -> InputScript {
    let mut json = String::from("[");
    for k in 0..50u64 {
        let keys = match k % 4 {
            0 => r#"["D"]"#,
            1 => r#"["D", "Space"]"#,
            2 => r#"["A"]"#,
            _ => "[]",
        };
        let pad = if k % 3 == 0 {
            format!(r#"[{{"id": {}, "buttons": ["A"], "axes": {{"LX": 0.{k}}}}}]"#, k % 2)
        } else {
            "[]".into()
        };
        json += &format!(r#"{{"step": {}, "keys": {keys}, "pads": {pad}}},"#, k * 19 + k % 3);
    }
    json.pop();
    json.push(']');
    InputScript::from_json(&json).unwrap()
}

/// Simulierte Uhr: unregelmäßige Bildzeiten zwischen 2 ms und 70 ms, auch mal 0 Takte oder 4 Takte je Bild.
fn ragged(i: u32) -> f64 {
    [0.016, 0.002, 0.033, 0.07, 0.0069, 0.041, 0.016, 0.25][(i % 8) as usize]
}

#[test]
fn tausend_takte_mit_simulierter_uhr_gleich_headless() {
    let s = script();
    let mut a = Walker::default();
    let mut c = NullCanvas::default();
    let mut r = Runner::new(&mut a, Some(&s));
    let mut frames = 0;
    while !r.app().done() {
        r.advance(ragged(frames), &mut c);
        frames += 1;
    }
    assert_eq!(r.steps(), 1000);
    let mut b = Walker::default();
    run_headless(&mut b, 5000, Some(&s));
    assert_eq!(a, b);
    assert!(a.taps > 100 && a.pad > 0.0, "Skript wirkt: {a:?}");
    assert!(
        frames > 300,
        "simulierte Uhr hat Bilder ohne und mit mehreren Takten: {frames}"
    );
}

#[test]
fn flanke_aus_dem_backend_wird_nach_dem_takt_geloescht() {
    let mut a = Walker::default();
    let mut c = NullCanvas::default();
    let mut r = Runner::new(&mut a, None);
    r.input_mut().keys.press(Key::Space);
    r.input_mut().keys.release(Key::Space);
    assert_eq!(r.advance(0.001, &mut c), 0, "kein Takt: Flanke bleibt");
    assert_eq!(r.advance(STEP, &mut c), 1);
    assert_eq!(r.advance(STEP, &mut c), 1);
    assert_eq!(a.taps, 1);
}
