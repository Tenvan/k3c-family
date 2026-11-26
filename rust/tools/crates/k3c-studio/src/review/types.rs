//! Typen mit den JSON-Namen des Vertrags (paper-mcp.md › Oberfläche); `assetsui` übernimmt sie als Alias.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// Eine abgelehnte Regel einer Spec (paper-spec.md › Regeln).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Fehler {
    pub feld: String,
    pub regel: String,
    pub text: String,
}

/// Ein geändertes Feld einer Spec gegenüber der Basis.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FeldAenderung {
    pub pfad: String,
    pub alt: Value,
    pub neu: Value,
}

/// Status eines Review-Eintrags.
pub const STATUS_NEU: &str = "neu";
pub const STATUS_GEAENDERT: &str = "geaendert";
pub const STATUS_GELOESCHT: &str = "geloescht";
pub const STATUS_UNGUELTIG: &str = "ungueltig";

/// Ein geändertes Asset; Status `neu` | `geaendert` | `geloescht` | `ungueltig`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Asset {
    pub id: String,
    pub rig: String,
    pub pfad: String,
    pub status: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fehler: Vec<Fehler>,
    pub felder: Vec<FeldAenderung>,
    pub teile: Vec<String>,
    pub stil: bool,
    #[serde(rename = "pixelAnteil", serialize_with = "go_zahl")]
    pub pixel_anteil: f64,
}

/// Vergleich des Arbeitsstands mit einer Basis; `basisStand` `aktuell` | `lokal`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Ergebnis {
    pub basis: String,
    #[serde(rename = "basisCommit")]
    pub basis_commit: String,
    #[serde(rename = "basisStand")]
    pub basis_stand: String,
    pub assets: Vec<Asset>,
}

/// Zahl wie Go `json.Marshal`: ganze Werte ohne `.0`.
pub(super) fn go_zahl<S: Serializer>(v: &f64, s: S) -> Result<S::Ok, S::Error> {
    if v.fract() == 0.0 && v.abs() < 9.0e15 {
        s.serialize_i64(*v as i64)
    } else {
        s.serialize_f64(*v)
    }
}

/// `null` wie Go als leerer Wert (CLI-Antworten).
pub(super) fn null_leer<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}
