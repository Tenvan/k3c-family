//! Natives Spiel: Client und Schleife, zeichnet nur über k3c-platform.

pub mod actors;
pub mod art;
pub mod audio;
pub mod demo;
pub mod devtools;
pub mod devwin;
pub mod fx;
pub mod game;
pub mod hud;
pub mod input;
pub mod local;
pub mod menu;
pub mod playtest;
pub mod props;
pub mod replay;
pub mod runs;
pub mod still;
pub mod view;
pub mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_baut() {}
}
