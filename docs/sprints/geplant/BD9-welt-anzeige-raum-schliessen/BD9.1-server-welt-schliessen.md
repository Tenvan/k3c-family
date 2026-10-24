# BD9.1 · Server: Welt in der Raumliste und Raum schließen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** bd9/1-server
- **Abhängig von:** –
- **Tickets:** B-604, B-581
- **Kriterien:** AC-02, AC-03

## Ziel

`rooms[]` trägt die Welt jedes Raums, und `POST api/room/close?slot=NAME` schließt einen offenen Raum mit Speichern und Rückkehr der Geräte in die Lobby; beides steht in `docs/protocol.md`.

## Kontext

- `room.Info` und `info()` in `engine/room/actions.go`; Welt über `r.isl.WorldID()`; `rooms` gesendet in `engine/net/dispatch.go`, `ws.go`.
- Schließen wie `lockedSweep` in `engine/room/manager.go` (`saveNow()`, `closed = true`, aus `m.rooms` entfernen, `peer.Closed`); Löschen als Muster: `engine/net/saves.go`, `handler.go`, `engine/room/deletesave.go`.
- Fehlercode `room_closed` (`engine/net/protocol.go`, `docs/protocol.md`), Client kehrt damit in die Lobby zurück.
- Protokollversion in `engine/net` und `src/online/clientProtocol.ts`.

## Erlaubte Dateien

- `engine/room/actions.go`, `manager.go`, neue `engine/room/closeroom.go` und Tests
- `engine/net/handler.go`, `saves.go` bzw. neue `engine/net/closeroom.go`, `protocol.go` und Tests
- `docs/protocol.md`
- `src/online/clientProtocol.ts` (nur Typ `RoomInfo.world` und Version)

## Nicht-Ziele

Lobby-Oberfläche (BD9.2); Host-Rolle.

## Schritte

1. `Info.World` in Go und `rooms[].world` im Protokoll, Version + 1.
2. `Manager.CloseRoom(name)`: Ablauf wie Sweep (speichern, schließen, Geräte benachrichtigen, entfernen), Fehler bei unbekanntem Raum.
3. HTTP-Endpunkt `POST api/room/close?slot=NAME` (200/404/400), Loopback- und Origin-Regeln wie `api/save`.
4. `docs/protocol.md` ergänzen.
5. Go-Tests: Welt in `rooms`, Schließen mit verbundenem Gerät (Gerät bekommt `room_closed`, Stand gespeichert, danach `DeleteSave` erfolgreich), unbekannter Raum.

## Fertig, wenn

- [ ] AC-02 (Server): Go-Test `rooms[].world` grün.
- [ ] AC-03 (Server): Go-Test Schließen grün.
- [ ] `task check:diff` grün, Review-Agent ohne schwere Befunde.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
