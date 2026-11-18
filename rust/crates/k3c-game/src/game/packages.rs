//! Port von `Packages` (`native/k3c-game/game/parts.go`): ein Eintrag je Fachmodul.

use std::cell::RefCell;
use std::rc::Rc;

use super::parts::Part;
use super::pending::devtools_part;
use super::shared::{HudOverlay, HudVoyage, Shared};
use crate::actors::{self, draw::ActorsDrawer};
use crate::art::Library;
use crate::audio::Sound;
use crate::view::keys::KeyTable;
use crate::{fx, hud, input, menu, props, world};

/// Grafik-Bibliotheken der Module. Go reicht eine `*art.Library` an alle; in Rust liest `world` sie nur (`Rc`), die
/// übrigen Module schreiben hinein (`Rc<RefCell>`). `Libs::default()` (ohne Grafik) gilt für Tests.
#[derive(Default)]
pub struct Libs {
    pub world: Option<Rc<Library>>,
    pub shared: Option<Rc<RefCell<Library>>>,
}

/// Liefert die Beiträge aller Fachmodule in der Reihenfolge von Go: world, props, actors, fx, devtools, hud, input,
/// menu, audio (devtools vor hud: das Cheat-Menü nimmt die Eingabe vor dem Skill-Menü). `keys` ist die Tastenbelegung
/// für das Menü (Go: `KeyRows`).
pub fn packages(libs: &Libs, keys: Option<KeyTable>) -> Vec<Part> {
    let lib = || libs.shared.clone();
    vec![
        world_part(libs),
        props_part(lib()),
        actors_part(lib()),
        fx_part(lib()),
        devtools_part(lib()),
        hud_part(lib()),
        input_part(),
        Part {
            menus: Some(menu::new(lib(), keys)),
            ..Part::named("menu")
        },
        Part {
            listener: Some(Box::new(Sound::new())),
            ..Part::named("audio")
        },
    ]
}

/// Die Eingabe-Quelle hinter zwei Rollen: `Source` (Poll) und `KeyboardSource` (Belegung der Tastatur, `game` stellt
/// sie vor jedem Poll).
fn input_part() -> Part {
    let i = Shared(Rc::new(RefCell::new(input::new())));
    Part {
        source: Some(Box::new(i.share())),
        keyboard: Some(Box::new(i)),
        ..Part::named("input")
    }
}

fn world_part(libs: &Libs) -> Part {
    let w = world::new(libs.world.clone());
    Part {
        drawers: w.drawers,
        camera: w.camera_rule,
        assets: w.asset_source.into_iter().collect(),
        ..Part::named("world")
    }
}

type SharedLib = Option<Rc<RefCell<Library>>>;

fn props_part(lib: SharedLib) -> Part {
    let p = Shared(Rc::new(RefCell::new(props::new_pending(lib))));
    Part {
        drawers: vec![Box::new(p.share())],
        drawer_listeners: vec![Box::new(p.share())],
        assets: vec![Box::new(p)],
        ..Part::named("props")
    }
}

fn actors_part(lib: SharedLib) -> Part {
    let a = Shared(Rc::new(RefCell::new(actors::new(lib))));
    let drawer = || ActorsDrawer { a: Rc::clone(&a.0) };
    Part {
        drawers: vec![Box::new(drawer())],
        drawer_listeners: vec![Box::new(drawer())],
        assets: vec![Box::new(a.share())],
        ..Part::named("actors")
    }
}

fn fx_part(lib: SharedLib) -> Part {
    let x = Shared(Rc::new(RefCell::new(fx::new(lib, fx::default_aura_source()))));
    Part {
        drawers: vec![Box::new(x.share())],
        drawer_listeners: vec![Box::new(x.share())],
        assets: vec![Box::new(x)],
        ..Part::named("fx")
    }
}

fn hud_part(lib: SharedLib) -> Part {
    let h = Shared(Rc::new(RefCell::new(hud::new_pending(lib))));
    Part {
        drawers: vec![Box::new(h.share())],
        drawer_listeners: vec![Box::new(h.share())],
        assets: vec![Box::new(h.share())],
        overlays: vec![Box::new(HudOverlay(h.share()))],
        voyage: Some(Box::new(HudVoyage(h))),
        ..Part::named("hud")
    }
}
