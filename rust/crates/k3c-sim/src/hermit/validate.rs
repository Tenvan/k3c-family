//! Prüfung des Hub-Platzes des Bauwerks (RU62.10): Der Platz steht nicht in data/hub.json, sondern kommt aus hermits.json. Die
//! Prüfung hängt ihn als Insel-Platz an die hub.json der Daten und lässt die Hub-Prüfung laufen (Stufen, Arten, Lage in der
//! Hub-Breite, Abstand der Zahlziele, Abstand zu den Treppen); der Bauplatz zählt dabei als Gebäude der Art `hubSite.id`.
use k3c_data::Data;
use k3c_data::buildings::BuildingData;
use k3c_data::hub::HubSite;

use crate::hub::sites::check_world_hub_sites;

/// Die hub.json der Daten samt Platz des Bauwerks und die Gebäude samt seinem Eintrag.
pub fn hub_with_hall(data: &Data) -> (k3c_data::hub::HubData, std::collections::BTreeMap<String, BuildingData>) {
    let site = &data.hermits.hub_site;
    let mut hub = data.hub.clone();
    hub.island_sites.push(HubSite {
        kind: site.id.clone(),
        offset_units: site.offset_units,
        hub_level: site.hub_level,
        ..HubSite::default()
    });
    let mut buildings = data.buildings.clone();
    let hp = data.hermits.hermits.iter().map(|h| h.hp).fold(0.0, f64::max);
    buildings.insert(
        site.id.clone(),
        BuildingData {
            hp,
            ..BuildingData::default()
        },
    );
    (hub, buildings)
}

/// Prüft den Platz des Bauwerks mit der Hub-Prüfung (`check_world_hub_sites`); der Fehler nennt Datei, Regel und Platz.
pub fn check_hall_site(data: &Data) -> Result<(), String> {
    let (hub, buildings) = hub_with_hall(data);
    check_world_hub_sites(&hub, &data.economy, &buildings, &data.biomes)
}
