//! Port von `native/k3c-game/view/layer_test.go`.

use k3c_platform::Canvas;

use super::super::cell::Cell;
use super::super::frame::Frame;
use super::*;

struct FakeDrawer {
    layer: Layer,
}

impl Drawer for FakeDrawer {
    fn layer(&self) -> Layer {
        self.layer
    }
    fn draw(&self, _: &mut dyn Canvas, _: &Frame, _: &Cell) {}
}

fn fake(layer: Layer, _name: &'static str) -> Option<Box<dyn Drawer>> {
    Some(Box::new(FakeDrawer { layer }))
}

// Die Ebenen-Reihenfolge ist Teil des Vertrags: eine Änderung hier ist eine Vertragsänderung.
#[test]
fn layer_order_is_fixed() {
    let want = [
        "far",
        "mid",
        "near",
        "ground",
        "buildings",
        "actors",
        "river",
        "effects",
        "hud",
        "overlay",
    ];
    let got = layers();
    assert_eq!(got.len(), want.len(), "Ebenen: {}, erwartet {}", got.len(), want.len());
    for (i, l) in got.iter().enumerate() {
        assert_eq!(l.to_string(), want[i], "Ebene {i}");
    }
    assert!(
        !Layer::OVERLAY.per_cell() && Layer::HUD.per_cell() && !Layer(99).valid() && !Layer(-1).per_cell(),
        "per_cell/valid falsch"
    );
    assert_eq!(Layer(99).to_string(), "invalid", "ungültige Ebene ohne Namen");
}

#[test]
fn sorted_is_stable_by_layer() {
    let input: Vec<Option<Box<dyn Drawer>>> = vec![
        fake(Layer::HUD, "hud"),
        fake(Layer::FAR, "far1"),
        None,
        fake(Layer::ACTORS, "actors"),
        fake(Layer(42), "kaputt"),
        fake(Layer::FAR, "far2"),
        fake(Layer::OVERLAY, "overlay"),
    ];
    let got = sorted(&input);
    let want = ["far1", "far2", "actors", "hud", "overlay"];
    assert_eq!(got.len(), want.len(), "{} Drawer, erwartet {}", got.len(), want.len());
    // Namen über die Adresse zurückverfolgen (Anmelde-Reihenfolge der gültigen Drawer).
    let registered = ["hud", "far1", "actors", "kaputt", "far2", "overlay"];
    let addrs: Vec<*const dyn Drawer> = input.iter().flatten().map(|b| &**b as *const dyn Drawer).collect();
    let names: Vec<&str> = got
        .iter()
        .map(|d| {
            let p = addrs
                .iter()
                .position(|a| std::ptr::addr_eq(*a, *d as *const dyn Drawer));
            registered[p.expect("Drawer aus der Eingabe")]
        })
        .collect();
    assert_eq!(names, want);
}
