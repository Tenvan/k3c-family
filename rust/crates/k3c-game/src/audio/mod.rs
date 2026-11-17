//! Port des Pakets `native/k3c-game/audio` (Sprint RU17, B-1014): Ton im nativen Spiel.
//!
//! Teil 1 (RU17.3): Slot-Register (`register`), Ton-Atlas (`atlas`), Ducking (`duck`) und die Zuordnung Ereignis →
//! Klang samt Lautstärke je Spielerhälfte (`events`). Mischer, Musik, Bank und Ausgabe folgen in RU17.4 und RU17.5.
//!
//! Wegfall von `sink.go`: Die Senke kapselt oto (Ebitengine). In Rust übernimmt `k3c_platform::AudioOut` (RU4.8,
//! `open_audio()`) diese Aufgabe; es gibt keine `sink.rs`, Tests nehmen `NullAudio` (`testutil`).

pub mod atlas;
pub mod duck;
pub mod events;
pub mod register;

#[cfg(test)]
mod atlas_tests;
#[cfg(test)]
mod events_tests;
#[cfg(test)]
mod testutil;
