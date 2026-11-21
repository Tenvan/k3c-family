# rust/crates/k3c-sim/src/hub/

## Responsibility

Hub und Ausbau (Port von `engine/sim/hub_level.go`, `hub_validate*.go`, Teile von `world_hub_sites.go`): Ausbau-Stufen von Burg, Mauer, Turm
und Tor sowie die Prüfung einer Kandidaten-`hub.json` (Plätze, Zahlziele, Abstände).

## Design

- `level.rs`: Ausbau wie jeder Bau (Gold, Material aus dem Vorrat, ein Bauer): `new_hub_site`, `site_cost`, `next_level`, `hub_gate`, `pay_upgrade`,
  `step_upgrade`, `upgrade_job`, `site_hub_level`. Während des Ausbaus gilt die alte Stufe; der Hub ist ein Platz ohne Liste (`World::hub_site`).
- `sites.rs`: `check_world_hub_sites`, `check_site_bounds`, `RULE_BOUNDS` (Regel „hub-breite“).
- `validate.rs`: `HubFinding`, `HubReport`, Regel-Konstanten (`RULE_JSON`, `RULE_LEVELS`, `RULE_STAIRS` …), `STAIRS_GAP_UNITS`, `required_site_kinds`;
  ändert keinen globalen Zustand.
- `validate_layout.rs`: `HubTarget`, `zone_targets`, `fixed_targets`, `each_layout`, `check_gaps`: Abstandsregeln allein aus Kandidat und
  `buildings.json`, in jeder Tiefe.
- `mod.rs`: nur Modul-Zeilen. Abweichung zu Go: Plätze über ID, die Insel kommt als `isl: Option<&IslandRef>`.

## Flow

Zur Laufzeit: Spieler zahlt am Platz, `pay_upgrade` bucht, `step_upgrade`/`hub_step` baut in Schritten, `finish_upgrade` setzt die neue Stufe. Zur
Prüfung: Kandidat und eingebettete Daten (`world::base`) rein, `HubReport` mit `HubFinding`s raus.

## Integration

- Abhängigkeiten: `state`, `world` (`hub_of`, `vals`), `island` (Vorrat), `k3c-data` (`hub`, `buildings`, `biome`), `k3c-det`.
- Daten: `data/hub.json`, `data/buildings.json`, `data/biomes/*.json`.
- Konsumenten: `world` (Tick, Wertesatz-Prüfung), `hermit/validate.rs`, Workbench über `k3c-room` (Gebäude-Planer).
