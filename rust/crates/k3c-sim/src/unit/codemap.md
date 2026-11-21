# rust/crates/k3c-sim/src/unit/

## Responsibility

Eigene Truppen (Port von `engine/sim/unit_*.go`, `units.ts`): Landstreicher, Bauern mit Aufträgen und Berufen, Krieger, Bogenschützen, Handwerker;
dazu Flucht, Begleitschutz, Rüstung und Lager.

## Design

- `mod.rs`: `ARRIVE`, `ARROW_SPEED`, `spawn_vagrant`, `step_camps`, `step_troops`, `wander`.
- `jobs.rs`: `find_job`, `site_job`, `build_job`, `release_job`, `ascend_job`/`ascend`. `professions.rs`: `OfferTarget`, Berufe an Angebots-Zahlzielen.
- Kämpfer: `warrior.rs` (Schwert, Verlust, `pay_sword`, `is_fighter`), `archer.rs` (`step_archer`, Turm), `armor.rs` (`fetchArmor`, `assign`,
  `step_errand`), `escort.rs` (Begleitschutz im Endboss-Kampf).
- `hammer.rs`: `Product`, `PlaceProduct`, `belt_cost` (Hammer als Werkzeuggürtel, nur Bauer mit Hammer baut). `flee.rs`: Flucht getroffener Bürger.
- `camp_guard.rs`: Landstreicher-Lager zwischen Bäumen, Taverne an der Stelle (`step_native_rules`).
- Truppen sind Indizes (`ti`), Türme ihre Platz-ID.

## Flow

`step_troops` je Truppe: Flucht prüfen, dann Rolle (Landstreicher wandern, Bauer `find_job` und arbeiten, Krieger und Schütze kämpfen, Aufträge holen
Ausrüstung). Bei Gefahr bleiben Bauern in der Burg.

## Integration

- Abhängigkeiten: `state`, `world` (`WorldValues`), `island` (`IslandRef`, `errand`), `economy`, `building`, `k3c-data` (`troops`, `buildings`).
- Daten: `data/troops.json`, `data/buildings.json`, `data/economy.json`.
- Konsumenten: `world/step.rs`, `island/errand.rs`, `portal`, `combat`.
