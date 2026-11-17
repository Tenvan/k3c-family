//! Port von native/k3c-rig/rig/load.go: Rig-Dateien laden, Indizes, Eltern und Zeichenreihenfolge vorbereiten.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;
use serde::de::DeserializeOwned;

use super::fehler::Fehler;
use super::gait::{LegIndex, RIDER_PARTS};
use super::keyframes::keyframe_ids;
use super::matrix::Point;
use super::order::Item;

/// Image ist das Bild eines Teils, Bands oder Elements: Datei, linke obere Ecke in Einheiten, Größe in Pixeln.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Image {
    #[serde(rename = "bild")]
    pub file: String,
    #[serde(rename = "versatz")]
    pub offset: Point,
    #[serde(rename = "groesse")]
    pub size: [i64; 2],
}

/// Part ist ein starres Teil; Teile ohne Lagen (Bein-Knochen) haben kein Bild.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Part {
    pub id: String,
    #[serde(rename = "eltern")]
    pub parent: String,
    pub pivot: Point,
    #[serde(rename = "ebene")]
    pub layer: f64,
    #[serde(flatten)]
    pub image: Image,
}

/// Slot ist ein Platz für ein optionales Element oder eine andere Figur (Reiter am Sattel).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Slot {
    pub name: String,
    #[serde(rename = "teil")]
    pub part: String,
    #[serde(rename = "punkt")]
    pub point: Point,
    #[serde(rename = "drehung")]
    pub rotation: f64,
    #[serde(rename = "ebene")]
    pub layer: f64,
}

/// Band ist ein Bild der Ruhelage, das entlang von Stützpunkten gebogen wird (Umhang, Bein).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Band {
    pub id: String,
    #[serde(rename = "teil")]
    pub part: String,
    #[serde(rename = "ebene")]
    pub layer: f64,
    #[serde(rename = "mitte")]
    pub center: f64,
    #[serde(rename = "ruhe")]
    pub rest: Vec<f64>,
    /// Abstand der linken und rechten Kante von der Mittellinie je Stützpunkt.
    #[serde(rename = "breite")]
    pub width: Vec<[f64; 2]>,
    #[serde(flatten)]
    pub image: Image,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Keyframe {
    pub t: f64,
    #[serde(rename = "winkel")]
    pub angles: BTreeMap<String, f64>,
    #[serde(rename = "sichtbar")]
    pub visible: BTreeMap<String, bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Animation {
    pub frames: i64,
    pub fps: f64,
    #[serde(rename = "gangart")]
    pub gait: String,
    pub keyframes: Vec<Keyframe>,
    /// Teile mit Winkeln in den Keyframes.
    #[serde(skip)]
    pub(super) ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct CloakSpec {
    #[serde(rename = "anker")]
    pub anchor: String,
    #[serde(rename = "punkt")]
    pub point: Point,
    #[serde(rename = "laenge")]
    pub length: f64,
    #[serde(rename = "breite")]
    pub width: [f64; 2],
    #[serde(rename = "segmente")]
    pub segments: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Proportions {
    #[serde(rename = "arme")]
    pub arms: Option<f64>,
    #[serde(rename = "beine")]
    pub legs: Option<f64>,
    #[serde(rename = "rumpf")]
    pub torso: Option<f64>,
    #[serde(rename = "kopf")]
    pub head: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct LegSpec {
    #[serde(rename = "spitze")]
    pub tip: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Reins {
    #[serde(rename = "farbe")]
    pub color: String,
    #[serde(rename = "deckkraft")]
    pub opacity: f64,
    #[serde(rename = "breite")]
    pub width: f64,
    #[serde(rename = "durchhang")]
    pub sag: f64,
    #[serde(rename = "ebene")]
    pub layer: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Riding {
    #[serde(rename = "sitz")]
    pub seat: Point,
    #[serde(rename = "ausblenden")]
    pub hide: Vec<String>,
}

/// Rig ist eine Figur aus Teilen (native/k3c-rig/RIG.md).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Rig {
    pub id: String,
    #[serde(rename = "rig")]
    pub kind: String,
    #[serde(rename = "dichte")]
    pub density: f64,
    #[serde(rename = "massstab")]
    pub scale: f64,
    #[serde(rename = "rahmen")]
    pub frame: [f64; 4],
    #[serde(rename = "teile")]
    pub parts: Vec<Part>,
    #[serde(rename = "plaetze")]
    pub slots: Vec<Slot>,
    #[serde(rename = "baender")]
    pub bands: Vec<Band>,
    #[serde(rename = "umhang")]
    pub cloak: Option<CloakSpec>,
    #[serde(rename = "bein")]
    pub leg: Option<LegSpec>,
    #[serde(rename = "proportionen")]
    pub proportions: Option<Proportions>,
    #[serde(rename = "animationen")]
    pub animations: BTreeMap<String, Animation>,
    #[serde(rename = "varianten")]
    pub variants: BTreeMap<String, BTreeMap<String, String>>,
    #[serde(rename = "standard")]
    pub default: BTreeMap<String, String>,
    #[serde(rename = "reiten")]
    pub riding: Option<Riding>,
    #[serde(rename = "zuegel")]
    pub reins: Option<Reins>,

    #[serde(skip)]
    pub dir: String,
    /// Zeichenreihenfolge: Teile, Bänder, Plätze und Zügel nach Ebene.
    #[serde(skip)]
    pub order: Vec<Item>,
    #[serde(skip)]
    pub(super) index: BTreeMap<String, usize>,
    #[serde(skip)]
    pub(super) parent: Vec<i64>,
    /// Teile so geordnet, dass Eltern vor ihren Kindern stehen.
    #[serde(skip)]
    pub(super) topo: Vec<usize>,
    #[serde(skip)]
    pub(super) factor: Vec<f64>,
    /// Teile, die beim Reiten entfallen.
    #[serde(skip)]
    pub(super) hide: Vec<bool>,
    #[serde(skip)]
    pub(super) legs: LegIndex,
    #[serde(skip)]
    pub(super) rider: [i64; RIDER_PARTS.len()],
}

/// Element ist ein optionales Teil an einem Platz; Offset zählt ab dem Punkt des Platzes.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Element {
    pub id: String,
    #[serde(rename = "platz")]
    pub slot: String,
    #[serde(rename = "dichte")]
    pub density: f64,
    #[serde(flatten)]
    pub image: Image,
}

fn read_json<T: DeserializeOwned>(path: &Path, what: &str) -> Result<T, Fehler> {
    let shown = path.display();
    let text = std::fs::read(path).map_err(|e| {
        Fehler::Fehlt(format!(
            "{what} fehlt: {shown} (erst `task native:rig:export` ausführen): {e}"
        ))
    })?;
    serde_json::from_slice(&text).map_err(|e| Fehler::Kaputt(format!("{what} kaputt: {shown}: {e}")))
}

impl Rig {
    /// `load` liest <dir>/rig.json und bereitet Indizes, Eltern und Zeichenreihenfolge vor.
    pub fn load(dir: &Path) -> Result<Rig, Fehler> {
        let mut r: Rig = read_json(&dir.join("rig.json"), "Rig-Datei")?;
        r.dir = dir.to_string_lossy().into_owned();
        if r.scale == 0.0 {
            r.scale = 1.0;
        }
        r.link()?;
        for a in r.animations.values_mut() {
            a.keyframes
                .sort_by(|x, y| x.t.partial_cmp(&y.t).unwrap_or(std::cmp::Ordering::Equal));
            a.ids = keyframe_ids(&a.keyframes);
        }
        r.hide = vec![false; r.parts.len()];
        if let Some(riding) = &r.riding {
            for id in &riding.hide {
                if let Some(&i) = r.index.get(id) {
                    r.hide[i] = true;
                }
            }
        }
        r.legs = r.leg_indices();
        for (i, id) in RIDER_PARTS.iter().enumerate() {
            r.rider[i] = r.index_of(id);
        }
        r.build_order()?;
        Ok(r)
    }

    /// `link` baut den Index der Teile, die Eltern-Verweise und die Pose-Faktoren.
    fn link(&mut self) -> Result<(), Fehler> {
        self.index = self.parts.iter().enumerate().map(|(i, p)| (p.id.clone(), i)).collect();
        self.parent = vec![0; self.parts.len()];
        self.factor = vec![0.0; self.parts.len()];
        for i in 0..self.parts.len() {
            let p = &self.parts[i];
            self.parent[i] = -1;
            if !p.parent.is_empty() {
                let Some(&j) = self.index.get(&p.parent) else {
                    return Err(Fehler::Rig(format!(
                        "Rig {}: Teil {} nennt unbekannte Eltern {}",
                        self.id, p.id, p.parent
                    )));
                };
                self.parent[i] = j as i64;
            }
            self.factor[i] = self.pose_factor(&p.id);
        }
        self.sort_topo()
    }

    /// `sort_topo` ordnet die Teile so, dass Eltern vor ihren Kindern stehen (für World ohne Rekursion).
    fn sort_topo(&mut self) -> Result<(), Fehler> {
        let mut done = vec![false; self.parts.len()];
        while self.topo.len() < self.parts.len() {
            let before = self.topo.len();
            for i in 0..self.parts.len() {
                if !done[i] && (self.parent[i] < 0 || done[self.parent[i] as usize]) {
                    done[i] = true;
                    self.topo.push(i);
                }
            }
            if self.topo.len() == before {
                return Err(Fehler::Rig(format!(
                    "Rig {}: Eltern der Teile bilden einen Kreis",
                    self.id
                )));
            }
        }
        Ok(())
    }

    /// `index_of` liefert die Position eines Teils in `parts` oder -1 (Go: `Index`).
    pub fn index_of(&self, id: &str) -> i64 {
        self.index.get(id).map_or(-1, |&i| i as i64)
    }

    /// `slot` liefert den Platz mit diesem Namen.
    pub fn slot(&self, name: &str) -> Option<&Slot> {
        self.slots.iter().find(|s| s.name == name)
    }

    /// `image_file` nennt die Bilddatei eines Teils oder Bands in einer Variante (unbekannte Variante = Standardbild).
    pub fn image_file<'a>(&'a self, id: &str, file: &'a str, variant: &str) -> &'a str {
        match self.variants.get(variant).and_then(|v| v.get(id)) {
            Some(f) => f,
            None => file,
        }
    }
}

/// `load_elements` liest <dir>/elemente.json.
pub fn load_elements(dir: &Path) -> Result<Vec<Element>, Fehler> {
    read_json(&dir.join("elemente.json"), "Elemente-Datei")
}
