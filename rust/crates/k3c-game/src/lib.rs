//! Natives Spiel: Client und Schleife, zeichnet nur über k3c-platform.

pub mod actors;
pub mod art;
pub mod demo;
pub mod fx;
pub mod input;
pub mod replay;
pub mod still;
pub mod view;
pub mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn crate_baut() {}
}
