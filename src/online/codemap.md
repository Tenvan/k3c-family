# src/online/

## Responsibility

Network-Client-Schicht (Protocol Adapter) zum Go-Server: WebSocket `/ws`, Protokoll v3 (`docs/protocol.md`). Der Browser sendet nur Eingaben und empfängt Zustand; er simuliert nichts und würfelt nicht. Phaser-frei und per `ClientEnv` testbar.

## Design

- State Machine: `RoomClient` (`clientConnection.ts`), `Status` = `connecting | lobby | room | reconnecting | ended | lost`. Öffentliche Operationen `create`, `join`, `addSlot`, `removeSlot`, `leave`, `sendInput`, `sendDev`, `takeFrames`, `retry`, `close`; `onChange` meldet jede Änderung an die Szene.
- Dependency Injection: `ClientEnv` (Socket-Factory, Uhr, Timer, Geräte-ID) und `SocketLike` ersetzen Browser-APIs in Tests; `createRoomClient()` baut die Browser-Variante (`ws(s)://<Host>/ws`).
- Reconnect: `dropped()` mit Backoff (`RETRY_FIRST_MS` 500 bis `RETRY_MAX_MS` 4000), Aufgabe nach `RECONNECT_LIMIT_MS` (`lost`); Geräte-ID dauerhaft in `localStorage` (`getDeviceId`).
- Nachrichtentypen: `clientProtocol.ts` (`PROTOCOL_VERSION` 3, `WS_PATH`, `ServerMessage`, `ClientMessage`, `DevMessage`, `WorldState`, `ErrorCode`, `INPUT_KEEPALIVE_MS`).
- Delta-Anwendung: `clientDelta.ts` `applyDelta()` (Listen mit `id` als `{set, del}` gemerged, `null` ist ein Wert, `events` pro Tick neu).
- Darstellung: `clientInterpolation.ts` (`interpolate`, `blendAlpha`; ein Tick Verzögerung, `TELEPORT_UNITS` ohne Überblendung), `clientWorld.ts` (`createViewWorld`, `applyState` schreibt in dasselbe `World`-Objekt).
- `protocol.ts`: ältere Snapshot-Hilfen (`ONLINE_PATH`, `snapshotWorld`, `applySnapshot`, `sanitizeInput`); nur in `src/scenes/noSim.test.ts` referenziert.

## Flow

1. `createRoomClient()` → `open()` → `hello {v, device}` → `welcome {tickHz, limits}` → Status `lobby`.
2. `rooms` füllt die Lobby; `create`/`join` → `joined {room, you}` → Status `room`; `level {depth, layout}` setzt `LevelInfo`.
3. `snap` setzt den vollen Zustand, `delta` wird per `applyDelta` darauf angewendet (ohne vorherigen `snap` verworfen); jeder Tick landet als `Frame {tick, ack, receivedAt, state}` in der Queue.
4. Szene holt pro Frame `takeFrames()`, interpoliert mit `blendAlpha`/`interpolate` und schreibt über `applyState` ins `World`.
5. Eingabe: `sendInput(SlotInput[])` sendet bei Änderung (höchstens `tickHz`) oder als Keepalive alle 500 ms `input {seq, p}`.
6. `seats` aktualisiert Slots und Monarchen; `error` → `fail(code, message)` je Fehler-Code; Abbruch → `reconnecting`, erneut `hello` und `join` auf den gemerkten Raum.

## Integration

- Konsumenten: `src/main.ts`, `src/scenes/` (`GameScene`, `LoadScene`, `LobbyScene`, `lobbyLogic`, `debugActions`, `debugOverlay`, `debugOverlayView`, `effectRules`).
- Abhängigkeiten: `src/model/` (`World`, `GameEvent`, `LevelLayout`, `BIOMES`), `src/core/clientLog.ts`, `src/core/texts.ts`; Endpoint WebSocket `/ws`; Vertrag `docs/protocol.md`.
- `src/scenes/noSim.test.ts` erzwingt, dass `client*.ts` nur aus `src/model` importieren.
