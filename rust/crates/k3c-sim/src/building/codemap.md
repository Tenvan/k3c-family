# rust/crates/k3c-sim/src/building/

## Responsibility

Bau-Regeln der Gebäude (Port von `engine/sim/building_*.go`): Kaserne mit Kämpfer-Limit, Heilplatz, Vorposten, Plantage,
Reparatur, Ruinen, Zaubertum, Taverne, Elite-Schmiede und Rüstung. Go hat keine `building.go`, `mod.rs` sammelt nur.

## Design

- `barracks.rs`: `fighters`, `troop_limit` (Basis plus Zuschlag je gebauter Kaserne), `barracks_bonus`, `weapons_in_flight`;
  EQ1.4: `island_fighters`, `island_limit` für die ganze Insel.
- `healer.rs`: `step_healing` (Bürger und Truppen im Radius, bis `max_hp`; Monarchen haben keine HP).
- `outpost.rs`: `outermost_wall`, `outpost`, `outpost_job` (Platz jenseits der äußersten Mauer-Lage laut Layout).
- `plantation.rs`: `step_plantations`, `plot_x` (Bäume der Baumschule wachsen nach, ohne Münzen, nicht gespeichert).
- `repair.rs`: `repair_job`, `repair`, `level_seconds`, `REPAIR_GOLD` (2); Dauer = fehlender HP-Anteil mal Bauzeit.
- `ruin.rs`: `is_ruin`, `ruin_site`, `add_ruin_sites`, `kind_hub_level`, `line_before_ruin` (Ruinen der Planung als Plätze).
- `spell_tower.rs`: `is_spell_tower`, `step_spell_towers` (Turm ab `spell.from_level` schießt selbst mit Splash).
- `tavern.rs`: `tavern_dawn`, `vagrants_at`, `wander_radius` (Landstreicher, an `anchor_x` gebunden).
- `upgrades.rs`: `place_payable`, `offer_at`, `elite_offer_at`, `pay_any_offer`, `step_crafts` (Elite-Upgrades der Schmiede,
  Rüstungsstufen), `armor_stage`, `armor_bonus`, `armored_hp`.

## Flow

`world::step::step` ruft `step_plantations`, `step_healing` und `step_spell_towers` je Tick; `step_crafts` läuft aus
`economy::sites::step_sites`; `tavern_dawn` kommt aus dem Zyklus (`world/cycle.rs`) am Morgengrauen. Reparatur und
Vorposten-Prüfung laufen über die Aufträge der Bauern (`unit`).

## Integration

- Abhängigkeiten: `state` (`World`, `Site`, `Job`, `WorldValues`), `economy`, `k3c_data`.
- Konsumenten: `links.rs` (`barracks`, `outpost`, `repair`, `spell_tower`, `tavern`, `upgrades::armored_hp`), `unit`, `economy`, `world`.
- Daten: `data/buildings.json` (barracks, healer, nursery, tower.spell, tavern), `data/troops.json` (Elite, Rüstung).
