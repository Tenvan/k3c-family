//! Port von `native/k3c-game/world/world.go` (Paketkopf): zeichnet je Zelle Himmel, Kulisse (fern, mitte, nah), Boden,
//! Fluss und Licht und führt die Kamera (B-795). Rechnung (Kamera, Culling, Schichten, Fluss, Licht) liegt in reinen
//! Funktionen ohne Zeichenfläche; die Drawer sind dünn und legen je Bild nichts an.
//!
//! RU13.1 legt nur die Module von backdrop, camera, cull, ground und layers an; den Inhalt von `world.go` (World,
//! `new`, Drawer) liefert RU13.3.

pub mod backdrop;
pub mod camera;
pub mod cull;
pub mod ground;
pub mod layers;
pub mod pending;
