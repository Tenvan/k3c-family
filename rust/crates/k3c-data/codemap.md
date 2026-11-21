# rust/crates/k3c-data/

## Responsibility

Spiel-Daten: typisierte Strukturen und Lader für `data/*.json`, wertgleich zu den Go-Strukturen (RU3/AC-04). Einzige Rust-Quelle für Balancing-Werte; das Crate rechnet nichts, es liest, prüft und liefert.

## Design

- `Cargo.toml`: `k3c-det`, `serde`, `serde_json` (mit `float_roundtrip`).
- Quellcode in `src/` (siehe [`src/codemap.md`](src/codemap.md)); ein Modul je Datei oder Dateigruppe von `data/`.
- Feldnamen der Ausgabe wie Gos `encoding/json`; gelesen wird der Schlüssel der Datei (Alias statt Groß-/Kleinschreibung). Nur die Balancing-Dateien lehnen unbekannte Felder ab.

## Flow

`Source::from_env()` wählt Ordner oder Einbettung → `load_game`/`load_all` lesen jede Datei, dekodieren und prüfen sie → `Data` hält alle Strukturen → Level, Sim und Spiel lesen daraus.

## Integration

- Abhängigkeiten: `k3c-det`.
- Konsumenten: `k3c-level`, `k3c-sim`, `k3c-room`, `k3c-balance`, `k3c-game`.
- Vorlage: die Go-Pakete `engine/sim`, `engine/level`, `engine/balance` und `data/embed.go`.
