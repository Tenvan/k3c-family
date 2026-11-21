# rust/crates/k3c-sim/src/mounts/

## Responsibility

Welt-Reittiere (RU62.8, B-970): Eine Welt mit Eintrag in `mounts.json` hat ein eigenes Reittier. Wer am Gatter der Insel `price` Münzen einwirft,
reitet es bis zum Ende der Welt. Neue Mechanik nur in Rust.

## Design

- `mod.rs`: `EDGE_MARGIN_UNITS`, `world_mount` (Reittier-ID der Welt), `mount_factors` (Faktoren für Tempo und mehr je Spieler), `gate_x` (Gatter aus
  Seed, Tiefe, Breite, Hub-Position), `Island::enable_mounts`.
- `buy.rs`: `step_gate` (Einwurf mit Zahlen-Taste; zu wenig Münzen oder schon gekauft: abgelehnt, nichts abgezogen; beide Monarchen kaufen in
  Reihenfolge der Spieler-Indizes).
- Gatter nur in der Oberflächen-Stufe (`World::mount_gate`, `state/mount_view.rs`).

## Flow

`enable_mounts` setzt das Gatter beim Anlegen; der Spielerschritt ruft `step_gate`; danach liefert `mount_factors` die Stärke in die Bewegung.

## Integration

- Abhängigkeiten: `state`, `monarch` (`mount_of`), `k3c-rng`, `k3c-data` (`mounts.rs`).
- Daten: `data/mounts.json`.
- Konsumenten: `economy/players.rs`, `monarch/passives.rs`, `island`.
