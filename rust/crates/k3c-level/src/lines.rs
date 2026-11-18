//! Hub- und Insel-Plätze (Port von `engine/level/lines.go`). Gos Paket-Variablen `hubSites` und `payGapUnits` liegen
//! in `Rules` (`rules.rs`); hier stehen die Typen und die Auswahl der Plätze.

/// Go: `hubSite`, ein Hub- oder Insel-Platz aus `data/hub.json`: Art und Lage als Abstand zur Hub-Mitte, links negativ.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HubSiteRef {
    pub kind: String,
    pub offset: f64,
}

/// Go: `HubSite`, ein Hub- oder Insel-Platz einer Welt (`hub.sites`, `hub.islandSites`): Art und Lage als Abstand zur
/// Hub-Mitte, links negativ. Der Generator bekommt die Plätze als Wert (`Options::hub_sites`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HubSite {
    pub kind: String,
    pub offset_units: f64,
}

/// Go: `sitesOf`, wählt die Plätze einer Erzeugung oder Prüfung: die der Welt (`own`), ohne sie (`None`) die der
/// Basis (`base`).
pub fn sites_of(own: Option<&[HubSite]>, base: &[HubSiteRef]) -> Vec<HubSiteRef> {
    match own {
        None => base.to_vec(),
        Some(list) => list
            .iter()
            .map(|s| HubSiteRef {
                kind: s.kind.clone(),
                offset: s.offset_units,
            })
            .collect(),
    }
}
