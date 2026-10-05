# engine/net/

## Responsibility

Transport- und API-Schicht des Go-Servers (Adapter zwischen HTTP/WebSocket und dem Raummodell `engine/room`): liefert den Build aus, bedient die REST-Endpunkte (Spielstände, Berichte, Health, Level, Diagnose) und die WebSocket-Verbindung `/ws` nach Protokoll v3. Enthält keine Spiellogik.

## Design

- Façade/Router: `handler.go` – `NewHandler(Config)` baut einen `http.ServeMux`; `Config` bündelt `store.Saves`, `store.Reports`, `room.Manager`, Logger, Token, `LogDir`.
- Middleware: `accessLog` (`accesslog.go`) umhüllt den Mux mit `statusWriter`; `clientGate` begrenzt `POST /api/clientlog` (Größe, Anzahl, Rate).
- Observer/Peer: `conn` (`ws.go`) implementiert `room.Peer` (`Joined`, `Level`, `State`, `Seats`, `Replaced`, `Closed`); Schreib-Goroutine mit Warteschlange: ein Zustand ersetzt einen wartenden Zustand direkt davor (Ereignisse wandern mit, B-276), volle Warteschlange (64 andere Nachrichten) = Abbruch. `handler.go` setzt `room.Manager.Snapshot` auf `stateOf`.
- Message Dispatch: `conn.handle` (`dispatch.go`) – `switch` über `inMsg.T` (`create`, `join`, `addSlot`, `removeSlot`, `input`, `leave`, `dev` …); Fehler werden mit `codeOf` auf Protokoll-Codes gemappt.
- Wire-Format: `protocol.go` (`inMsg`, `welcomeMsg`, `stateMsg`, `levelMsg`, `errorMsg`, `ProtocolVersion`); `stateOf` serialisiert `sim.World` zu `map[string]any`.
- Delta-Kompression: `deltaOf`/`listDelta` (`delta.go`) – nur geänderte Felder, Listen mit `id` als set/del.
- Diagnose (Bearer-Token, sonst 404): `status.go`, `status_actions.go` (POST disconnect/save), `status_log.go` (Log-Paging nach Offset), `cpu.go` (`cpuMeter`).
- `static.go`: `resolve` schützt gegen Path Traversal im `dist/`-Ordner; `level.go`: zustandslose Level-Berechnung über `engine/level`.

## Flow

1. `cmd/k3c-server` ruft `NewHandler(Config)`; Routen: `/api/health`, `/api/save`, `/api/save/backups`, `/api/save/restore`, `/api/status[/log|/disconnect|/save]`, `/api/report`, `/api/level`, `/api/clientlog`, `/ws`, `/` (static).
2. `GET /ws`: `websocket()` → `handshake` (hello/welcome) → `track` + `broadcastRooms` → Lese-Schleife ruft `conn.handle` je Nachricht.
3. `create`/`join` → `room.Manager.Create/Join` → `conn.enter` registriert die Verbindung als `Peer`; weitere Nachrichten gehen über `roomMessage` an `room.Room` (`AddSlot`, `RemoveSlot`, `Input`, `Leave`, `Dev`).
4. Takt im Raum → `conn.Level` (immer gefolgt von vollem Zustand) bzw. `conn.State(tick, Zustand der Stufe)` → `push` in die Warteschlange → `writer`-Goroutine: `stateData` (nach Level snap, sonst `deltaOf(prev, cur)` zum zuletzt gesendeten, `ack` beim Senden) → Client.
5. Verbindungsende: `conn.left` → `Room.Drop`; bei Ersetzung durch ein neues Gerät `Replaced` (Fehler senden, schließen).
6. `/api/save*`: Request → `store.Saves.Load/Store/Backups/Restore`; Fehler (`ErrInvalid`, `ErrNotFound`, `ErrTooLarge`, `ErrSlot`) → HTTP-Status via `fail`/`writeJSON`.

## Integration

- Importiert: `engine/room` (Manager, Room, Peer, Fehler), `engine/store` (Saves, Reports), `engine/sim` (`World`), `engine/level` (Layout, Level-API), `github.com/coder/websocket`.
- Konsumenten: `cmd/k3c-server` (`main.go`, Handler-Aufbau), `cmd/k3c-load` (`bots.go`, Lastgenerator über das Protokoll).
- Gegenstelle: Browser-Client `src/online/` (Protokoll v3, `docs/protocol.md`), k3c-dev (`/api/status*`, `/api/clientlog`).
