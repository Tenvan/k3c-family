# rust/crates/k3c-devproto/

## Responsibility

Protokoll zwischen dem Rust-Spiel und dem Cheat-Fenster `k3c-devwin` (RU56, B-1131): Nachrichtentypen und Zeilen-JSON-Kodierung. Transport ist eine Named Pipe (Byte-Strom, eine JSON-Nachricht je Zeile); den Transport selbst enthält das Crate nicht.

## Design

- `Cargo.toml`: nur `serde` und `serde_json`.
- Quellcode in `src/` (siehe [`src/codemap.md`](src/codemap.md)).

## Flow

Beide Seiten senden `Hello`; das Spiel schickt `Catalog` und laufend `State`, das Fenster `Dev`, `Ui`, `Goto`, `Focus`; das Spiel antwortet mit `Ack`; `Bye` beendet.

## Integration

- Konsumenten: `k3c-devwin` (Pipe-Server) und `k3c-game`.
- Abhängigkeiten: `serde`, `serde_json`.
