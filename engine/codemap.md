# engine/

## Responsibility
Go-Spiel-Engine (ADR 001): Simulation, Level-Generierung, Raumverwaltung, Netzwerk und Persistenz. Der Browser ist reiner Client, alle Spielregeln liegen hier. Keine eigenen Quelldateien.

## Design
Schichten von innen nach außen:

| Ordner | Rolle |
|--------|-------|
| [`rng/`](rng/codemap.md) | deterministischer mulberry32-Generator, einzige Zufallsquelle |
| [`sim/`](sim/codemap.md) | Domain Model und deterministischer Tick-Core (World, Island, Campaign, Saves), ohne I/O |
| [`level/`](level/codemap.md) | deterministischer Level-Generator mit Validator und Biome-Loader |
| [`room/`](room/codemap.md) | Room/Manager: Geräte, Slots, Takt, Fristen, Speichern, Crash-Supervision über Peer/Store-Ports |
| [`store/`](store/codemap.md) | File-Repository für Spielstände (`saves/`) und Berichte (`reports/`) |
| [`net/`](net/codemap.md) | HTTP/WebSocket-Adapter (Protokoll v4) auf `room`, Build-Auslieferung, Diagnose |
| [`conlog/`](conlog/codemap.md) | farbiger `slog.Handler` mit Emoji-Topics |

## Flow
1. `cmd/k3c-server` → `net` nimmt Verbindungen an → `room.Manager` ordnet Geräte einem Raum zu.
2. Der Raum taktet `sim` (Seed aus `rng`, Level aus `level`) und schickt Snapshots/Deltas über `net` an die Clients.
3. Spielstände laufen über die Store-Ports von `room` nach `store`.

## Integration
- Konsumenten: `cmd/k3c-server`, `cmd/k3c-load`; in k3c-dev `internal/enginetools`, `internal/balance`, `internal/mcpsrv`, `internal/serverapi`, `internal/applog` (`conlog`).
- Daten: `data/` per `go:embed`.
