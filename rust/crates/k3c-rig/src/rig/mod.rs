//! Port von native/k3c-rig/rig: Papierschnitt-Figuren zur Laufzeit aus Teilen rechnen (Teile-Baum, Posen, Gangarten,
//! Umhang, Bänder). Formeln und Konstanten wie tools/paper (render/, rig/); ohne Zeichen-Stack.

pub mod band;
pub mod cloak;
pub mod curve;
pub mod fehler;
pub mod figure;
pub mod frames;
pub mod gait;
pub mod keyframes;
pub mod load;
pub mod matrix;
pub mod order;
pub mod ride;

pub use band::{BAND_SUB, Vertex};
pub use cloak::{CLOAK_DT, Cloak, cloak_run, frame_steps, step_time};
pub use fehler::Fehler;
pub use figure::FigurePose;
pub use frames::{Ride, frame_time, ride_cloak_frames, ride_floor};
pub use gait::{GaitPose, LEG_NAMES, RIDER_PARTS, gait_freq, ride_matrix};
pub use load::{Animation, Band, Element, Image, Keyframe, Part, Rig, Slot, load_elements};
pub use matrix::{IDENTITY, M, Point};
pub use order::{Item, ItemKind};
pub use ride::{rein_points, rider_base};

#[cfg(test)]
mod band_leg_tests;
#[cfg(test)]
mod band_tests;
#[cfg(test)]
mod expected_tests;
#[cfg(test)]
mod rustref_gait_tests;
#[cfg(test)]
mod rustref_load_tests;
#[cfg(test)]
mod rustref_tests;
#[cfg(test)]
mod testutil;
