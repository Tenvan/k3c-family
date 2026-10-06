# engine/net/

## Responsibility

Transport- und API-Schicht des Go-Servers (Adapter zwischen HTTP/WebSocket und dem Raummodell `engine/room`): liefert den Build aus, bedient die REST-Endpunkte (Spielstände, Berichte, Health, Level, Diagnose, Messreihen, Dev) und die WebSocket-Verbindung `/ws` nach Protokoll v4. Enthält keine Spiellogik.

## Design

- Façade/Router: `handler.go` – `NewHandler(Config)` baut einen `http.ServeMux`; `Config` bündelt `store.Saves`, `store.Reports`, `room.Manager`, Logger, Token, `LogDir`, `CPU`. Setzt `Rooms.Changed`, `Rooms.Snapshot` (= `stateOf`) und `Rooms.Monitor.CPU` (eigener `cpuMeter`).
- Middleware: `accessLog` (`accesslog.go`) umhüllt den Mux mit `statusWriter`; `clientGate` begrenzt `POST /api/clientlog` (Größe, Anzahl, Rate).
- Observer/Peer: `conn` (`ws.go`) implementiert `room.Peer` (`Joined`, `Level`, `State`, `Seats`, `Replaced`, `Closed`); Schreib-Goroutine mit Warteschlange: ein Zustand ersetzt einen wartenden Zustand direkt davor (Ereignisse wandern per `mergeEvents` mit, B-276), volle Warteschlange (64 andere Nachrichten) = Abbruch; verworfene Zustände zählt `drops`.
- Message Dispatch: `conn.handle` (`dispatch.go`) – `switch` über `inMsg.T` (`create`, `join`, dann `roomMessage`: `addSlot`, `removeSlot`, `input`, `learn`/`respec` via `skillMessage`, `leave`, `dev`); Fehler werden mit `codeOf` auf Protokoll-Codes gemappt.
- Wire-Format: `protocol.go` (`ProtocolVersion` = 4, `inMsg`, `welcomeMsg`, `stateMsg`, `levelMsg`, `errorMsg`); `stateOf` serialisiert `sim.World` zu `map[string]any`; `actions.go` (`addActions`, `actionsOf`) ergänzt je Spieler `points` und die Aktionsliste (attack, skill je Slot, learn).
- Delta-Kompression: `deltaOf`/`listDelta` (`delta.go`) – nur geänderte Felder, Listen mit `id` als set/del.
- RTT-Messung: `pinger`/`probe` (`ping.go`) – eine Goroutine je Verbindung, WebSocket-Ping je Sekunde; RTT, Warteschlangenlänge und `drops` landen über `Monitor.Device` in der Messreihe des Geräts.
- Diagnose (Bearer-Token, sonst 404): `status.go`, `status_actions.go` (POST disconnect/save), `status_log.go` (Log-Paging nach Offset), `cpu.go` (`cpuMeter`), `metrics.go` (`GET /api/metrics?since=` → `Monitor.Since`, Polling statt Push).
- Dev-Seite: `dev.go` – `/api/dev` (`devBody`, `dev`, `devAction`): GET Raumliste/Diagnose, POST Dev-Aktion nur im Dev-Mode (sonst 403).
- `saves.go`: `GET /api/saves` listet Spielstände über `Saves.List`; `static.go`: `resolve` schützt gegen Path Traversal im `dist/`-Ordner; `level.go`: zustandslose Level-Berechnung über `engine/level`.

## Flow

1. `cmd/k3c-server` ruft `NewHandler(Config)`; Routen: `/api/health`, `/api/save`, `/api/saves`, `/api/save/backups`, `/api/save/restore`, `/api/status[/log|/disconnect|/save]`, `/api/metrics`, `/api/report`, `/api/level`, `/api/clientlog`, `/api/dev`, `/ws`, `/` (static).
2. `GET /ws`: `websocket()` → `handshake` (hello/welcome, Version 4) → `track` + `broadcastRooms` → `pinger` startet → Lese-Schleife ruft `conn.handle` je Nachricht.
3. `create`/`join` → `room.Manager.Create/Join` → `conn.enter` registriert die Verbindung als `Peer`; weitere Nachrichten gehen über `roomMessage` an `room.Room` (`AddSlot`, `RemoveSlot`, `Input`, `Learn`, `Respec`, `Leave`, `Dev`).
4. Takt im Raum → `conn.Level` (immer gefolgt von vollem Zustand) bzw. `conn.State(tick, Stufe, Zustand)` → `push` in die Warteschlange → `writer`-Goroutine: `stateData` (nach Level snap, sonst `deltaOf(prev, cur)` zum zuletzt gesendeten, `ack` beim Senden) → Client.
5. Verbindungsende: `conn.left` → `Room.Drop`; bei Ersetzung durch ein neues Gerät `Replaced` (Fehler senden, schließen).
6. `/api/save*`: Request → `store.Saves.Load/Store/Backups/Restore/List`; Fehler (`ErrInvalid`, `ErrNotFound`, `ErrTooLarge`, `ErrSlot`) → HTTP-Status via `fail`/`writeJSON`.

## Integration

- Importiert: `engine/room` (Manager, Room, Peer, Monitor, Fehler), `engine/store` (Saves, Reports), `engine/sim` (`World`, `AvailablePoints`), `engine/level` (Layout, Level-API), `github.com/coder/websocket`.
- Konsumenten: `cmd/k3c-server` (`main.go`, Handler-Aufbau), `cmd/k3c-load` (`bots.go`, Lastgenerator über das Protokoll).
- Gegenstelle: Browser-Client `src/online/` (Protokoll v4, `docs/protocol.md`), k3c-dev (`/api/status*`, `/api/metrics`, `/api/clientlog`), Dev-Seite `/dm` (`/api/dev`).
