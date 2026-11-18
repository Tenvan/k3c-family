//! Port von `native/k3c-game/hud/seekarte_look.go`: Aussehen einer Insel im Diorama der Seekarte (B-964/R2): ohne neues
//! Asset aus den Angaben der Welt-Datei. Größe der Miniatur aus der Inselgröße (größte Chunk-Anzahl), Zahl der Schichten
//! unter der Insel aus der Stufenzahl, Aufsatz aus einem festen Hash der Insel-ID. Kein Zufall und kein Zustand:
//! Dieselbe Insel sieht immer gleich aus, bei beiden Spielern. Reine Logik; das HUD rechnet damit keine Regel, es wählt
//! nur Bilder.

use crate::view::frame::RouteIsland;

/// Chunk-Anzahlen der kleinsten und größten Inselgröße (data/island-sizes.json: sehr klein, sehr groß).
pub const SEA_CHUNKS_MIN: f64 = 16.0;
pub const SEA_CHUNKS_MAX: f64 = 100.0;
/// Maßstab der Miniatur bei kleinster und größter Insel; ohne Angabe gilt die Mitte.
pub const SEA_SCALE_MIN: f64 = 0.7;
pub const SEA_SCALE_MAX: f64 = 1.1;
/// SEA_LAYERS_MAX: mehr Schichten als diese passen nicht zwischen Insel und nächste Welle.
pub const SEA_LAYERS_MAX: i64 = 5;

/// SeaLook ist das Aussehen einer Insel: Welt-ID der Miniatur, Maßstab, Schichten unter der Insel, Wert für den Aufsatz.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SeaLook {
    pub world: String,
    pub scale: f64,
    pub layers: i64,
    pub seed: u32,
}

/// look_of leitet das Aussehen aus den Angaben der Route ab.
pub fn look_of(isl: &RouteIsland) -> SeaLook {
    SeaLook {
        world: isl.world.clone(),
        scale: sea_scale(isl.chunks),
        layers: sea_layers(isl.stages),
        seed: sea_seed(&isl.id),
    }
}

/// sea_scale ist der Maßstab der Miniatur zur Chunk-Anzahl: linear zwischen kleinster und größter Inselgröße, darüber
/// und darunter begrenzt; ohne Angabe (<= 0, Länge des Bioms) die Mitte.
pub fn sea_scale(chunks: i64) -> f64 {
    if chunks <= 0 {
        return (SEA_SCALE_MIN + SEA_SCALE_MAX) / 2.0;
    }
    let t = (chunks as f64 - SEA_CHUNKS_MIN) / (SEA_CHUNKS_MAX - SEA_CHUNKS_MIN);
    SEA_SCALE_MIN + t.clamp(0.0, 1.0) * (SEA_SCALE_MAX - SEA_SCALE_MIN)
}

/// sea_layers ist die Zahl der sichtbaren Schichten unter der Insel: je Stufe eine, mindestens eine, höchstens
/// `SEA_LAYERS_MAX`.
pub fn sea_layers(stages: i64) -> i64 {
    stages.clamp(1, SEA_LAYERS_MAX)
}

/// sea_seed ist der feste Hash der Insel-ID (FNV-1a, 32 Bit), ohne Allokation.
pub fn sea_seed(id: &str) -> u32 {
    id.bytes()
        .fold(2_166_136_261_u32, |h, b| (h ^ u32::from(b)).wrapping_mul(16_777_619))
}
