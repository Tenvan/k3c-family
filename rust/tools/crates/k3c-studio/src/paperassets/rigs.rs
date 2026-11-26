//! Rig-Typen v1 und die Vorlage einer neuen Spec (Port von `rigs.go`; paper-spec.md › Rig-Typen). Gegen check.ts
//! geprüft über die Tests der Rig-Vorlagen.

use super::Error;
use super::patch::format as formatiere;
use crate::kitx::{go_float, go_quote, marshal_raw};

/// Pflicht-Teile, Standard-Pivots, Platzhalter-Teil der Vorlage (für Rigs ohne Pflicht-Teile) und die Ausgabe-Vorgabe
/// einer neuen Spec.
#[derive(Debug)]
pub struct Rig {
    pub name: &'static str,
    pub pflicht: &'static [&'static str],
    pub pivots: &'static [(&'static str, [f64; 2])],
    /// Kette der Vorlage (Vierbeiner: Beinpfad braucht eltern an oberbein-*).
    pub eltern: &'static [(&'static str, &'static str)],
    /// Gangarten-Animationen (Regel gangart-fehlt).
    pub animationen: Option<&'static str>,
    /// Vorgabe für `bein` (nur Vierbeiner, aus der Fixture pferd).
    pub bein: Option<&'static str>,
    pub pivot_text: &'static str,
    pub platzhalter: &'static str,
    pub hoehe_px: i32,
    pub rahmen: [f64; 4],
}

const LEER: Rig = Rig {
    name: "",
    pflicht: &[],
    pivots: &[],
    eltern: &[],
    animationen: None,
    bein: None,
    pivot_text: "",
    platzhalter: "",
    hoehe_px: 0,
    rahmen: [0.0; 4],
};

const FIGUR: Rig = Rig {
    name: "figur",
    pflicht: &[
        "rumpf",
        "kopf",
        "oberarm-v",
        "unterarm-v",
        "oberarm-h",
        "unterarm-h",
        "oberschenkel-v",
        "unterschenkel-v",
        "oberschenkel-h",
        "unterschenkel-h",
    ],
    pivots: &[
        ("rumpf", [0.0, -50.0]),
        ("oberschenkel-v", [0.0, -50.0]),
        ("oberschenkel-h", [0.0, -50.0]),
        ("oberarm-v", [0.0, -84.0]),
        ("oberarm-h", [0.0, -84.0]),
        ("unterarm-v", [0.0, -70.0]),
        ("unterarm-h", [0.0, -70.0]),
        ("unterschenkel-v", [0.0, -28.0]),
        ("unterschenkel-h", [0.0, -28.0]),
        ("kopf", [0.0, -90.0]),
    ],
    pivot_text: "Hüfte (0,-50) rumpf, oberschenkel-*; Schulter (0,-84) oberarm-*; Ellbogen (0,-70) unterarm-*; \
                 Knie (0,-28) unterschenkel-*; Hals (0,-90) kopf",
    hoehe_px: 192,
    rahmen: [-60.0, -145.0, 85.0, 151.0],
    ..LEER
};

/// Schema-Pflicht (paper-spec.md › 4) plus Beinteile und `bein`, die der Renderer verlangt (RD1, pruefeVierbeiner).
const VIERBEINER: Rig = Rig {
    name: "vierbeiner",
    pflicht: &[
        "oberbein-hf",
        "unterbein-hf",
        "huf-hf",
        "oberbein-vf",
        "unterbein-vf",
        "huf-vf",
        "schweif",
        "rumpf-hinten",
        "rumpf-vorn",
        "oberbein-hn",
        "unterbein-hn",
        "huf-hn",
        "oberbein-vn",
        "unterbein-vn",
        "huf-vn",
        "hals",
    ],
    pivots: &[
        ("oberbein-hf", [-26.0, -56.0]),
        ("unterbein-hf", [-26.0, -31.0]),
        ("huf-hf", [-26.0, -9.0]),
        ("oberbein-vf", [30.0, -56.0]),
        ("unterbein-vf", [30.0, -31.0]),
        ("huf-vf", [30.0, -9.0]),
        ("schweif", [-38.0, -68.0]),
        ("rumpf-hinten", [0.0, -66.0]),
        ("rumpf-vorn", [0.0, -80.0]),
        ("oberbein-hn", [-30.0, -56.0]),
        ("unterbein-hn", [-30.0, -31.0]),
        ("huf-hn", [-30.0, -9.0]),
        ("oberbein-vn", [26.0, -56.0]),
        ("unterbein-vn", [26.0, -31.0]),
        ("huf-vn", [26.0, -9.0]),
        ("hals", [30.0, -70.0]),
    ],
    eltern: &[
        ("oberbein-hf", "rumpf-hinten"),
        ("unterbein-hf", "oberbein-hf"),
        ("huf-hf", "unterbein-hf"),
        ("oberbein-vf", "rumpf-vorn"),
        ("unterbein-vf", "oberbein-vf"),
        ("huf-vf", "unterbein-vf"),
        ("schweif", "rumpf-hinten"),
        ("rumpf-hinten", "rumpf-vorn"),
        ("oberbein-hn", "rumpf-hinten"),
        ("unterbein-hn", "oberbein-hn"),
        ("huf-hn", "unterbein-hn"),
        ("oberbein-vn", "rumpf-vorn"),
        ("unterbein-vn", "oberbein-vn"),
        ("huf-vn", "unterbein-vn"),
        ("hals", "rumpf-vorn"),
    ],
    animationen: Some(
        r#"{"schritt":{"frames":8,"gangart":"schritt"},"trab":{"frames":8,"gangart":"trab"},"galopp":{"frames":8,"gangart":"galopp"}}"#,
    ),
    bein: Some(r#"{"breiten_vorn":[16,14,11.5,8.2,5.4,6.8],"breiten_hinten":[19,17,12.5,8,5.4,6.8]}"#),
    pivot_text: "Hüfte (-26/-30 hinten, 26/30 vorn; -56) oberbein-*; Knie -31 unterbein-*; Fessel -9 huf-*; \
                 Rückengelenk (0,-66) rumpf-hinten",
    hoehe_px: 192,
    rahmen: [-68.0, -122.0, 145.0, 126.0],
    ..LEER
};

/// Die Rig-Typen in der Reihenfolge des Schemas (enum rig).
static RIGS: [Rig; 5] = [
    FIGUR,
    VIERBEINER,
    Rig {
        name: "gebaeude",
        pflicht: &["korpus"],
        pivot_text: "frei; Stufen-Zusätze hängen an Teilen der Basis",
        hoehe_px: 192,
        rahmen: [-64.0, -128.0, 128.0, 128.0],
        ..LEER
    },
    Rig {
        name: "requisite",
        platzhalter: "korpus",
        pivot_text: "frei",
        hoehe_px: 64,
        rahmen: [-16.0, -32.0, 32.0, 32.0],
        ..LEER
    },
    Rig {
        name: "hintergrund",
        platzhalter: "flaeche",
        pivot_text: "frei",
        hoehe_px: 256,
        rahmen: [-128.0, -256.0, 256.0, 256.0],
        ..LEER
    },
];

/// Die Rig-Typen in Schema-Reihenfolge.
pub fn rig_namen() -> Vec<&'static str> {
    RIGS.iter().map(|r| r.name).collect()
}

/// Der Rig-Typ oder ein Fehler mit den erlaubten Namen.
pub fn finde_rig(name: &str) -> Result<&'static Rig, Error> {
    RIGS.iter()
        .find(|r| r.name == name)
        .ok_or_else(|| Error::Sonst(format!("Rig „{name}“ unbekannt: {}", rig_namen().join(", "))))
}

impl Rig {
    /// Die Teile einer neuen Spec: Pflicht-Teile oder das Platzhalter-Teil.
    pub fn vorlage_teile(&self) -> Vec<&'static str> {
        if self.pflicht.is_empty() {
            return vec![self.platzhalter];
        }
        self.pflicht.to_vec()
    }

    fn teil(&self, name: &str) -> String {
        let pivot = self
            .pivots
            .iter()
            .find(|(n, _)| *n == name)
            .map_or([0.0; 2], |(_, p)| *p);
        let eltern = self
            .eltern
            .iter()
            .find(|(n, _)| *n == name)
            .map_or_else(String::new, |(_, e)| format!(r#""eltern":{},"#, text(e)));
        format!(
            r#"{{"id":{},{eltern}"pivot":{},"lagen":[]}}"#,
            text(name),
            zahlen(&pivot)
        )
    }
}

fn text(s: &str) -> String {
    marshal_raw(&go_quote(s))
}

fn zahlen(werte: &[f64]) -> String {
    let teile: Vec<String> = werte.iter().map(|v| go_float(*v)).collect();
    format!("[{}]", teile.join(","))
}

/// Eine neue Spec des Rig-Typs mit leeren Lagen (formatiert wie die Fixtures).
pub fn vorlage(rig: &str, id: &str, beschreibung: &str) -> Result<String, Error> {
    let r = finde_rig(rig)?;
    let teile: Vec<String> = r.vorlage_teile().iter().map(|n| r.teil(n)).collect();
    let mut spec = format!(
        r#"{{"$schema":"../../../tools/paper/schema/paper-spec.schema.json","id":{},"rig":{},"version":1,"#,
        text(id),
        text(rig)
    );
    spec += &format!(
        r#""beschreibung":{},"teile":[{}],"#,
        text(beschreibung),
        teile.join(",")
    );
    if let Some(bein) = r.bein {
        spec += &format!(r#""bein":{bein},"#);
    }
    if let Some(animationen) = r.animationen {
        spec += &format!(r#""animationen":{animationen},"#);
    }
    spec += &format!(
        r#""ausgabe":{{"hoehe_px":{},"rahmen":{}}}}}"#,
        r.hoehe_px,
        zahlen(&r.rahmen)
    );
    formatiere(&spec)
}

#[cfg(test)]
#[path = "rigs_tests.rs"]
mod tests;
