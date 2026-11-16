//! Driver: JSON-Skript für die Demo (Ebenen, Gebäude, Figuren, Kamera); keine Sim, nur Bewegung nach Skript.

use std::path::Path;

use serde::Deserialize;

fn d_skala() -> f64 {
    0.6
}

fn d_ebene_skala() -> f64 {
    1.5
}

fn d_lauf() -> String {
    "lauf".to_string()
}

fn d_galopp() -> String {
    "galopp".to_string()
}

/// Hintergrund-Ebene mit Parallaxe (0 = steht am Bildschirm, 1 = bewegt sich mit der Welt).
#[derive(Debug, Clone, Deserialize)]
pub struct Ebene {
    pub asset: String,
    pub parallaxe: f64,
    #[serde(default = "d_ebene_skala")]
    pub skala: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Gebaeude {
    pub x: f64,
    pub asset: String,
}

/// Eine Gruppe gleichartiger Figuren, die zwischen `von` und `bis` hin und her laufen (Units, Tempo in Units/s).
#[derive(Debug, Clone, Deserialize)]
pub struct Figur {
    pub asset: String,
    pub anzahl: u32,
    pub tempo: f64,
    pub von: f64,
    pub bis: f64,
    #[serde(default = "d_lauf")]
    pub anim: String,
}

/// Gerittene Figur, der die Kamera folgt; ohne `tempo` gilt das Tempo des Monarchen aus `data/monarch.json`.
#[derive(Debug, Clone, Deserialize)]
pub struct Kamera {
    pub tempo: Option<f64>,
    pub figur: String,
    #[serde(default = "d_galopp")]
    pub anim: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Driver {
    /// Länge der Strecke in Units.
    pub laenge: f64,
    pub biom: String,
    pub ebenen: Vec<Ebene>,
    /// Bodenfarbe `#rrggbb`.
    pub boden: String,
    #[serde(default)]
    pub gebaeude: Vec<Gebaeude>,
    #[serde(default)]
    pub figuren: Vec<Figur>,
    pub kamera: Kamera,
    /// Verkleinerung der Figuren und Gebäude gegenüber dem Rahmen des Assets.
    #[serde(default = "d_skala")]
    pub skala: f64,
}

impl Driver {
    /// Liest und prüft ein Driver-Skript.
    pub fn read(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let d: Driver = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        d.check().map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(d)
    }

    fn check(&self) -> Result<(), String> {
        if self.laenge <= 0.0 {
            return Err("laenge muss größer 0 sein".into());
        }
        for f in &self.figuren {
            if f.von >= f.bis {
                return Err(format!("{}: von muss kleiner als bis sein", f.asset));
            }
        }
        parse_color(&self.boden).map(|_| ())
    }

    /// Alle Assets des Skripts, ohne Doppelte, in fester Reihenfolge.
    pub fn assets(&self) -> Vec<String> {
        let all = self
            .ebenen
            .iter()
            .map(|e| &e.asset)
            .chain(self.gebaeude.iter().map(|g| &g.asset))
            .chain(self.figuren.iter().map(|f| &f.asset))
            .chain([&self.kamera.figur]);
        let mut out: Vec<String> = Vec::new();
        for a in all {
            if !out.contains(a) {
                out.push(a.clone());
            }
        }
        out
    }
}

/// `#rrggbb` → RGBA (0..1).
pub fn parse_color(s: &str) -> Result<[f32; 4], String> {
    let hex = s
        .strip_prefix('#')
        .filter(|h| h.len() == 6)
        .ok_or_else(|| format!("Farbe `{s}` ist nicht #rrggbb"))?;
    let v = u32::from_str_radix(hex, 16).map_err(|_| format!("Farbe `{s}` ist nicht #rrggbb"))?;
    let c = |shift: u32| ((v >> shift) & 0xff) as f32 / 255.0;
    Ok([c(16), c(8), c(0), 1.0])
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIN: &str = r##"{"laenge":100,"biom":"wald","ebenen":[],"boden":"#336600",
        "kamera":{"figur":"figuren/monarch"}}"##;

    #[test]
    fn minimales_skript_bekommt_vorgaben() {
        let d: Driver = serde_json::from_str(MIN).unwrap();
        d.check().unwrap();
        assert_eq!(d.skala, 0.6);
        assert_eq!(d.kamera.anim, "galopp");
        assert!(d.kamera.tempo.is_none());
        assert_eq!(d.assets(), ["figuren/monarch"]);
    }

    #[test]
    fn falsche_werte_werden_abgelehnt() {
        let bad = MIN.replace("100", "0");
        assert!(serde_json::from_str::<Driver>(&bad).unwrap().check().is_err());
        assert!(parse_color("336600").is_err());
        assert_eq!(parse_color("#ff0000").unwrap(), [1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn mitgeliefertes_skript_ist_gueltig() {
        let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../demo/reiten.json");
        let d = Driver::read(&p).unwrap();
        assert!(d.ebenen.len() >= 3 && !d.figuren.is_empty());
    }
}
