//! Port von `native/k3c-game/game/parts.go`: Zusammenbau der Fachmodule zu den Teilen des Spiels.
//!
//! Go meldet die optionalen Rollen (Kamera-Regel, Asset-Quelle, Overlay, Schiffsfahrt) per Typ-Zusicherung an; Rust
//! nicht: Jedes Fachmodul nennt seine Rollen bei der Anmeldung als Trait-Objekt (siehe `view/roles.rs`), ein Beitrag
//! mit Zustand für mehrere Rollen teilt ihn über `Shared` (`shared.rs`). Nur game kennt alle Module (Importregel:
//! `imports_tests.rs`).

use super::levels::Loader;
use crate::view::input::{KeyboardSource, Source};
use crate::view::layer::{Drawer, Listener};
use crate::view::roles::{AssetSource, CameraRule, Finisher, Overlay};
use crate::view::screen::{MenuPages, Menus};
use crate::view::story::EndHolder;
use crate::view::voyage::VoyageDrawer;

/// Beitrag eines Fachmoduls; ungenutzte Felder bleiben leer.
#[derive(Default)]
pub struct Part {
    pub name: String,
    pub drawers: Vec<Box<dyn Drawer>>,
    pub listener: Option<Box<dyn Listener>>,
    /// Listener der Drawer (Go: ein Drawer, der zugleich `Listener` ist, hört mit), in Reihenfolge der Drawer
    pub drawer_listeners: Vec<Box<dyn Listener>>,
    pub source: Option<Box<dyn Source>>,
    /// Belegung der Tastatur der Eingabe-Quelle (Go: Typ-Zusicherung `Source` → `KeyboardSource`)
    pub keyboard: Option<Box<dyn KeyboardSource>>,
    pub menus: Option<Box<dyn Menus>>,
    /// Untermenüs für den Aufnahme-Modus `--menu` (Go: Typ-Zusicherung `Menus` → `MenuPages`)
    pub menu_pages: Option<Box<dyn MenuPages>>,
    pub camera: Option<Box<dyn CameraRule>>,
    pub assets: Vec<Box<dyn AssetSource>>,
    pub overlays: Vec<Box<dyn Overlay>>,
    pub voyage: Option<Box<dyn VoyageDrawer>>,
    /// Listener, die am Spielende noch etwas zeigen (Go: Typ-Zusicherung `Listener` → `EndHolder`)
    pub end_holders: Vec<Box<dyn EndHolder>>,
    /// Listener mit `Finish` beim Programmende (Go: Typ-Zusicherung `Listener` → `Finisher`)
    pub finishers: Vec<Box<dyn Finisher>>,
}

impl Part {
    /// Leerer Beitrag mit Namen.
    pub fn named(name: &str) -> Part {
        Part {
            name: name.to_string(),
            ..Part::default()
        }
    }
}

/// Das zusammengebaute Spiel: Drawer nach Ebene sortiert, Listener, Overlays und Asset-Quellen in Anmelde-Reihenfolge.
/// `art` setzt der Aufrufer (main: die Grafik-Bibliothek; `None` = ohne Laden, Tests).
pub struct Parts {
    pub names: Vec<String>,
    pub drawers: Vec<Box<dyn Drawer>>,
    pub listeners: Vec<Box<dyn Listener>>,
    pub input: Box<dyn Source>,
    pub keyboard: Option<Box<dyn KeyboardSource>>,
    pub menus: Box<dyn Menus>,
    /// Untermenüs für `--menu`; `None`, wenn das Menü sie nicht meldet
    pub menu_pages: Option<Box<dyn MenuPages>>,
    pub camera: Option<Box<dyn CameraRule>>,
    pub assets: Vec<Box<dyn AssetSource>>,
    pub overlays: Vec<Box<dyn Overlay>>,
    /// Schiffsfahrt im Vollbild (hud, B-977); `None`: Ladebildschirm
    pub voyage: Option<Box<dyn VoyageDrawer>>,
    pub end_holders: Vec<Box<dyn EndHolder>>,
    pub finishers: Vec<Box<dyn Finisher>>,
    pub art: Option<Box<dyn Loader>>,
}

/// Baut die Beiträge zusammen. Drawer, die zugleich Listener sind, hören mit (`drawer_listeners`). Genau eine
/// Eingabe-Quelle und genau ein Menü sind Pflicht, höchstens eine Kamera-Regel und eine Schiffsfahrt; ein Drawer mit
/// ungültiger Ebene ist ein Fehler. (Go prüft zusätzlich `nil`-Drawer; ein Trait-Objekt kann nicht fehlen.)
pub fn assemble(ps: Vec<Part>) -> Result<Parts, String> {
    let mut acc = Acc::default();
    for p in ps {
        acc.add(p)?;
    }
    acc.finish()
}

/// Sammler von `assemble`: Zwischenstand über die Beiträge.
#[derive(Default)]
struct Acc {
    names: Vec<String>,
    drawers: Vec<Box<dyn Drawer>>,
    listeners: Vec<Box<dyn Listener>>,
    input: Option<Box<dyn Source>>,
    keyboard: Option<Box<dyn KeyboardSource>>,
    finishers: Vec<Box<dyn Finisher>>,
    menus: Option<Box<dyn Menus>>,
    menu_pages: Option<Box<dyn MenuPages>>,
    end_holders: Vec<Box<dyn EndHolder>>,
    camera: Option<Box<dyn CameraRule>>,
    assets: Vec<Box<dyn AssetSource>>,
    overlays: Vec<Box<dyn Overlay>>,
    voyage: Option<Box<dyn VoyageDrawer>>,
}

impl Acc {
    /// Nimmt den Beitrag `p` auf (Reihenfolge wie in Go: Drawer-Listener, eigener Listener, Eingabe, Menü, Rollen).
    fn add(&mut self, p: Part) -> Result<(), String> {
        if p.drawers.iter().any(|d| !d.layer().valid()) {
            return Err(format!("❌ {}: Drawer ohne gültige Ebene", p.name));
        }
        self.listeners.extend(p.drawer_listeners);
        self.listeners.extend(p.listener);
        self.drawers.extend(p.drawers);
        set_once(&mut self.input, p.source, &p.name, "Eingabe")?;
        set_once(&mut self.keyboard, p.keyboard, &p.name, "Tastatur-Belegung")?;
        self.finishers.extend(p.finishers);
        set_once(&mut self.menus, p.menus, &p.name, "Menü")?;
        set_once(&mut self.menu_pages, p.menu_pages, &p.name, "Menü-Seiten")?;
        self.end_holders.extend(p.end_holders);
        set_once(&mut self.camera, p.camera, &p.name, "Kamera-Regel")?;
        set_once(&mut self.voyage, p.voyage, &p.name, "Schiffsfahrt")?;
        self.assets.extend(p.assets);
        self.overlays.extend(p.overlays);
        self.names.push(p.name);
        Ok(())
    }

    /// Prüft die Pflicht-Rollen und sortiert die Drawer nach Ebene (stabil: gleiche Ebene bleibt in Anmelde-Reihenfolge).
    fn finish(mut self) -> Result<Parts, String> {
        let (Some(input), Some(menus)) = (self.input, self.menus) else {
            return Err("❌ Eingabe-Quelle oder Menü fehlt".to_string());
        };
        self.drawers.sort_by_key(|d| d.layer());
        Ok(Parts {
            names: self.names,
            drawers: self.drawers,
            listeners: self.listeners,
            input,
            keyboard: self.keyboard,
            menus,
            menu_pages: self.menu_pages,
            end_holders: self.end_holders,
            camera: self.camera,
            assets: self.assets,
            overlays: self.overlays,
            voyage: self.voyage,
            finishers: self.finishers,
            art: None,
        })
    }
}

/// Setzt `dst` auf `v`, wenn `v` gesetzt ist; ein zweiter Beitrag derselben Art ist ein Fehler. `what` im
/// Fehlertext mit Artikel: „zweite Eingabe“, „zweites Menü“ (Go: „zweite Menü“).
fn set_once<T: ?Sized>(dst: &mut Option<Box<T>>, v: Option<Box<T>>, name: &str, what: &str) -> Result<(), String> {
    if v.is_none() {
        return Ok(());
    }
    if dst.is_some() {
        let zweite = if what == "Menü" { "zweites" } else { "zweite" };
        return Err(format!("❌ {name}: {zweite} {what}"));
    }
    *dst = v;
    Ok(())
}

#[cfg(test)]
#[path = "parts_tests.rs"]
mod parts_tests;
