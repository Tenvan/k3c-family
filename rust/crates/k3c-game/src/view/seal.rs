//! Siegel der Schwierigkeitsgrade (ECO1.1, ECO1.2): Zuordnung Grad → Papierschnitt-Asset, gemeinsam für Menü (Lobby,
//! Spielstand-Liste) und HUD, die einander nicht importieren dürfen.

/// Siegel der Grade in der Reihenfolge von `data/difficulty.json`: dev, easy, normal, hard, ultra.
pub const SEAL_ART: [(&str, &str); 5] = [
    ("dev", "icons/siegel-dev"),
    ("easy", "icons/siegel-easy"),
    ("normal", "icons/siegel-normal"),
    ("hard", "icons/siegel-hard"),
    ("ultra", "icons/siegel-ultra"),
];

/// Das Siegel-Asset des Grads; leer oder unbekannt (alter Stand) zeigt das Siegel des Standardgrads `normal`.
pub fn grade_seal(grade: &str) -> &'static str {
    let pick = |g: &str| SEAL_ART.iter().find(|(id, _)| *id == g).map(|(_, a)| *a);
    pick(grade).or_else(|| pick("normal")).unwrap_or("")
}
