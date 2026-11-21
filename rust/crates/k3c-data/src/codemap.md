# rust/crates/k3c-data/src/

## Responsibility

Lader und Datenstrukturen aller JSON-Dateien aus `data/` samt den Prüfungen beim Laden.

## Design

- `lib.rs`: Moduleinträge, `Fehler` (`Lesen`, `Json`, `Pruefung`; Text wie in Go), interne Helfer `decode` und `check`; reexportiert `Data`, `load_all`, `load_game`, `Source`.
- `source.rs`: `Source` (`Dir` oder `Embedded`), `from_env` (`K3C_DATA_DIR`, sonst `data` neben der EXE, sonst eingebettet), `choose`, `read(name)`.
- `data.rs`: `Data` (alle Dateien), `biome_ids`, `load_game`, `load_all`. `hub_layout.rs` (`spread`) rückt Hub-Plätze beim Laden nach der Gebäudebreite auseinander (nur Rust).
- Sim und Wirtschaft: `economy`, `troops`, `enemies`, `bosses`, `buildings`, `hub`, `monarch`, `waves`, `events`, `goals`, `story`, `difficulty`, `island_sizes`, `ship`, `guards`, `ruins`; neu für Rust (Go kennt sie nicht): `cargo`, `coop`, `festival`, `hermits`, `mounts`, `weather`.
- Welt und Level: `biome`, `world` (Kopf der Welt-Dateien, `path`, `valid_id`), `ordered` (`Ordered<V>`: JSON-Objekt in Datei-Reihenfolge).
- Balancing: `balance_hints`, `balance_targets` (`load_targets`).
- Darstellung und Spiel: `look` mit `look_glow`, `look_sky`, `look_world`; `sizes`, `paper`, `light`, `native`, `render`, `sounds`, `stresstests`.
- `omit.rs`: Hilfen für Go-`omitempty`. `*_tests.rs`, `testutil.rs`: Tests.

## Flow

Datei → `Source::read` → `decode` (serde) → Prüffunktion → Feld in `Data`; ein Fehler trägt das Präfix der Datei wie in Go (`data/bosses.json: …`).

## Integration

- Konsumenten: `k3c-level` (Biome, Regeln), `k3c-sim`, `k3c-room`, `k3c-game`, `k3c-balance`.
- Abhängigkeit: `k3c-det` (Go-Gleitkommatext).
