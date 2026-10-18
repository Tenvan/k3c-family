# tools/k3c-dev/internal/botdev/

## Responsibility

Bot-Gerät für Testläufe (B-348, TR1.2): eine WebSocket-Verbindung zum Go-Spielserver mit einem Slot, dessen Monarchen ein Bot des Balancing-Testers steuert. Das Gerät verhält sich wie ein Client: es hält den Serverzustand und antwortet mit `input`.

## Design

- `Device` (`device.go`): `Dial` (Handschlag), `Create`/`Join` (Raum mit Präfix `test-` anlegen bzw. per Code beitreten, über `enter`/`await`), `World()` (zuletzt empfangene Welt der eigenen Stufe plus eigener Monarch), `Stats()` (`Stats`: Zustände, Eingaben, Fehler, Trennung), `Stop`.
- `play` (eigene Goroutine): liest Nachrichten, `handle`/`onState` rufen den `balance.Bot` je Zustand der eigenen Stufe und senden `{"t":"input","seq","p":[pad]}`; `track` verfolgt Bewegung.
- Zustand (`state.go`): `applyDelta`/`mergeList` setzen `snap`/`delta` nach `docs/protocol.md` um (Listen mit `id` als `{set, del}`, `idLists` spiegelt `engine/net/delta.go`), `worldOf` dekodiert in eine `sim.World`.

## Flow

1. `Dial` -> `Create`/`Join` -> `enter` wartet auf die Bestätigung und liefert den Raumcode.
2. `play`: `Read` -> `handle` (Zustand einarbeiten) -> Bot-Kommando -> `send` -> `Stats.Inputs++`.
3. `Stop` verlässt den Raum, schließt die Verbindung und wartet auf `play`.

## Integration

- Konsumenten: `internal/mcpsrv/simtest_online.go` (Modus online headless), `simtest_clients.go` (Beobachter-Gerät je Client).
- Abhängigkeiten: `internal/balance` (`Bot`), `engine/sim`, `engine/net` (Protokollnamen), `github.com/coder/websocket`.
