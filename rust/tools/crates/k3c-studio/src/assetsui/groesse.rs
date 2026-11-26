//! Größenvergleich (B-733): Spiel-Pixel je Einheit eines Assets und die Hintergrund-Ebenen der Biome. Die Regeln
//! folgen dem Client (`src/scenes/render/sprites.ts` › targetHeight, `paperAssets.ts` › paperScale): eine Figur mit
//! Spielrolle wird so skaliert, dass ihr Rahmen über dem Fuß `height · scale` Pixel hoch ist; Gebäude, Objekte und
//! Hintergründe zeichnet das Spiel in Originalgröße (1 px je Einheit).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Dienst, data_url, datei, go_zahl, id_von, lies_json, objekt};
use crate::review;

#[cfg(test)]
#[path = "groesse_tests.rs"]
mod tests;

/// Der Maßstab eines Assets im Spiel.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Groesse {
    /// Spiel-Pixel je Einheit der Spec (Rahmen `ausgabe.rahmen`).
    #[serde(rename = "pxProEinheit", serialize_with = "go_zahl")]
    pub px_pro_einheit: f64,
    /// Woher der Maßstab kommt (Text der Oberfläche).
    pub quelle: String,
}

/// Ein Asset im Stand der Basis für den Größenvergleich (B-853): Spec dieses Stands (ohne `$schema`) und sein Maßstab.
/// Rollen und Monarch-Maßstab kommen aus dem Arbeitsstand, der Rahmen aus der Spec der Basis.
#[derive(Debug, Clone, Serialize)]
pub struct GroesseBasis {
    #[serde(flatten)]
    pub groesse: Groesse,
    pub spec: Value,
}

/// Die drei Ebenen eines Biom-Hintergrunds (Variante 1) als Data-URLs, hinten nach vorn.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Hintergrund {
    pub fern: String,
    pub mitte: String,
    pub nah: String,
}

/// Gruppen, deren Assets wie Figuren skaliert werden; ohne Spielrolle gilt der Maßstab des Monarchen.
const FIGUR_GRUPPEN: [&str; 5] = ["figuren", "gegner", "truppen", "bosse", "reittiere"];
const MONARCH_REF: &str = "figuren/monarch";
const HINTERGRUND_ORDNER: &str = "public/grafik/hintergruende";

#[derive(Debug)]
struct Rolle {
    name: String,
    hoehe: f64,
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(default)]
struct Figur {
    hoehe: f64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Figuren {
    spieler: Option<Vec<Figur>>,
    truppen: Option<BTreeMap<String, Figur>>,
    gegner: Option<BTreeMap<String, Figur>>,
}

/// Die Teile von `data/paper.json`, die Spielrollen nennen.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Paper {
    spieler: Option<Vec<String>>,
    truppen: Option<BTreeMap<String, String>>,
    gegner: Option<BTreeMap<String, String>>,
    figuren: Option<Figuren>,
}

/// Merkt die erste Rolle mit Höhe je Asset (`gruppe/id`, ohne `@variante`).
fn setze(out: &mut BTreeMap<String, Rolle>, verweis: &str, name: &str, figur: Figur) {
    if figur.hoehe <= 0.0 {
        return;
    }
    let verweis = verweis.split('@').next().unwrap_or_default();
    out.entry(verweis.to_owned()).or_insert_with(|| Rolle {
        name: name.to_owned(),
        hoehe: figur.hoehe,
    });
}

impl Dienst {
    /// Der Spiel-Maßstab eines Assets.
    pub fn assets_groesse(&self, id: &str) -> Result<Groesse, String> {
        let p = self.pfad_zu(id, "")?;
        self.groesse_von(&p, None)
    }

    /// Spec und Spiel-Maßstab eines Assets im Stand der Basis (Pflicht: `HEAD`, `HEAD~1`, Commit-SHA oder
    /// `origin/main`). Fehlt das Asset in der Basis, ist das ein Fehler (das Review zeigt dann nur den Arbeitsstand).
    pub fn assets_groesse_basis(&self, id: &str, basis: &str) -> Result<GroesseBasis, String> {
        if basis.is_empty() {
            return Err(review::fehler_basis(basis).to_string());
        }
        let q = self.quelle(id, basis)?;
        let spec = objekt(&q.spec)?;
        let groesse = self.groesse_von(&q.pfad, Some(&q.spec))?;
        Ok(GroesseBasis { groesse, spec })
    }

    /// Der Maßstab des Assets am Pfad `p`; `spec` ist der Inhalt seiner Spec (`None` = Datei des Arbeitsstands).
    fn groesse_von(&self, p: &str, spec: Option<&[u8]>) -> Result<Groesse, String> {
        let gruppe = p.split('/').nth(2).unwrap_or_default();
        let verweis = format!("{gruppe}/{}", id_von(p));
        let rollen = self.rollen_hoehen()?;
        if let Some(r) = rollen.get(&verweis) {
            let quelle = format!("Spielrolle {} (data/paper.json)", r.name);
            return self.figur_groesse(p, spec, r.hoehe, quelle);
        }
        if !FIGUR_GRUPPEN.contains(&gruppe) {
            return Ok(Groesse {
                px_pro_einheit: 1.0,
                quelle: "1 px je Einheit wie im Spiel (Gebäude, Objekte, Hintergründe)".to_owned(),
            });
        }
        let Some(monarch) = rollen.get(MONARCH_REF) else {
            return Err("Spielrolle des Monarchen fehlt in data/paper.json".to_owned());
        };
        let mp = self.pfad_zu(id_von(MONARCH_REF), "")?;
        let quelle = "Maßstab des Monarchen (keine Spielrolle)".to_owned();
        self.figur_groesse(&mp, None, monarch.hoehe, quelle)
    }

    /// Zielhöhe in Spiel-Pixeln geteilt durch die Höhe des Rahmens über dem Fuß.
    fn figur_groesse(&self, pfad: &str, spec: Option<&[u8]>, hoehe: f64, quelle: String) -> Result<Groesse, String> {
        #[derive(Default, Deserialize)]
        #[serde(default)]
        struct Ausgabe {
            rahmen: Option<Vec<f64>>,
        }
        let wert: Value = match spec {
            Some(raw) => serde_json::from_slice(raw).map_err(|e| e.to_string())?,
            None => lies_json(&datei(&self.root, pfad))?,
        };
        let ausgabe: Option<Ausgabe> =
            serde_json::from_value(wert.get("ausgabe").cloned().unwrap_or_default()).map_err(|e| e.to_string())?;
        match ausgabe.and_then(|a| a.rahmen).as_deref() {
            Some(&[_, oben, _, _]) if oben < 0.0 => Ok(Groesse {
                px_pro_einheit: hoehe / -oben,
                quelle,
            }),
            _ => Err(format!("{pfad}: ausgabe.rahmen fehlt oder liegt nicht über dem Fuß")),
        }
    }

    /// Ordnet jedem Asset mit Spielrolle die Figurhöhe in Spiel-Pixeln zu (`data/paper.json`: spieler, truppen, gegner
    /// und deren Maße unter figuren, CU1.2). Zeigen zwei Rollen auf dasselbe Asset, gewinnt immer dieselbe (sortiert).
    fn rollen_hoehen(&self) -> Result<BTreeMap<String, Rolle>, String> {
        let paper: Paper = lies_json(&datei(&self.root, "data/paper.json"))?;
        let figuren = paper.figuren.unwrap_or_default();
        let mut out = BTreeMap::new();
        let spieler = figuren.spieler.unwrap_or_default();
        for (verweis, figur) in paper.spieler.unwrap_or_default().iter().zip(spieler) {
            setze(&mut out, verweis, "Spieler", figur);
        }
        let rollen = [(paper.truppen, figuren.truppen), (paper.gegner, figuren.gegner)];
        for (verweise, masse) in rollen {
            let masse = masse.unwrap_or_default();
            for (name, verweis) in verweise.unwrap_or_default() {
                let figur = masse.get(&name).copied().unwrap_or_default();
                setze(&mut out, &verweis, &name, figur);
            }
        }
        Ok(out)
    }

    /// Die Biome mit Hintergrund (Namensteil `hg-<teil>-…`, z. B. wald, hoehle).
    pub fn assets_hintergruende(&self) -> Result<Vec<String>, String> {
        let ordner = datei(&self.root, HINTERGRUND_ORDNER);
        let fehler = |e: std::io::Error| format!("open {}: {e}", ordner.display());
        let mut teile = Vec::new();
        for eintrag in std::fs::read_dir(&ordner).map_err(fehler)? {
            let eintrag = eintrag.map_err(fehler)?;
            let name = eintrag.file_name();
            let teil = name
                .to_str()
                .and_then(|n| n.strip_prefix("hg-"))
                .and_then(|n| n.strip_suffix("-nah-v1"))
                .filter(|t| !t.is_empty() && t.bytes().all(|b| b.is_ascii_lowercase()));
            if let Some(teil) = teil
                && eintrag.file_type().map_err(fehler)?.is_dir()
            {
                teile.push(teil.to_owned());
            }
        }
        teile.sort();
        Ok(teile)
    }

    /// Die Ebenen des Biom-Hintergrunds `teil` (aus `AssetsHintergruende`; ein fremder Name wird abgelehnt).
    pub fn assets_hintergrund(&self, teil: &str) -> Result<Hintergrund, String> {
        if !self.assets_hintergruende()?.iter().any(|t| t == teil) {
            return Err(format!("Hintergrund „{teil}“ fehlt"));
        }
        let ebene = |ebene: &str| {
            let pfad = format!("{HINTERGRUND_ORDNER}/hg-{teil}-{ebene}-v1/stand.png");
            data_url(&datei(&self.root, &pfad))
        };
        Ok(Hintergrund {
            fern: ebene("fern")?,
            mitte: ebene("mitte")?,
            nah: ebene("nah")?,
        })
    }
}
