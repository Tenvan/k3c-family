//! Port von native/k3c-rig/paint: lädt die Bilder der Rig-Figuren und zeichnet sie über `platform::Canvas`
//! (Teile als Bild mit Matrix, Bänder und Zügel als Dreiecksnetz über `Canvas::mesh`).

pub mod actor;
pub mod assets;
pub mod painter;

pub use actor::{Actor, State};
pub use assets::{Assets, ElementImage, ImageRef, Skin, read_png};
pub use painter::{Painter, parse_color};

#[cfg(test)]
mod painter_tests;
#[cfg(test)]
mod rustref_tests;
#[cfg(test)]
mod testutil;
