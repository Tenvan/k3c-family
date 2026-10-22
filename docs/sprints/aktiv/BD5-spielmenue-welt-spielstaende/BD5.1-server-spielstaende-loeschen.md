# BD5.1 · Server: Spielstände mit Welt auflisten und löschen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** sprint/bd5
- **Abhängig von:** –
- **Tickets:** B-506, B-533
- **Kriterien:** AC-01

## Ziel

Der Server nennt die Welt je Spielstand in `GET /api/saves` und löscht einen Spielstand über `DELETE /api/save?slot=NAME`, außer er ist in einem Raum offen.

## Kontext

Spielstände: `engine/store/saves.go` (`Saves`, `Delete` entfernt auch Sicherungen – nur für Testräume), `saves_list.go` (`SaveInfo`), Sicherungen `backups.go` (`rotate`, BackupKeep 5). HTTP: `engine/net/handler.go` (`/api/save` GET/POST). Räume: `engine/room/manager.go` (Sperre `m.mu`, `ErrSaveInUse`). Welt steht im Stand als `world` (Version 8, `engine/sim/island_save.go`). Entscheidung B-533: Löschen verschiebt in die Sicherungen.

## Erlaubte Dateien

- `engine/store/saves.go`, `saves_list.go`, `*_test.go` in `engine/store/`
- `engine/room/manager.go` oder neue Datei `engine/room/deletesave.go` (+ Test)
- `engine/net/handler.go`, `engine/net/saves.go`, `engine/net/*_test.go`
- `docs/protocol.md`

## Nicht-Ziele

Keine WebSocket-Nachricht, keine neue Protokoll-Version; kein Wiederherstellen im Spiel.

## Schritte

1. Tests: `store.Saves.Discard` (Stand weg, Sicherung da, alte Sicherungen bleiben, fehlend → ErrNotFound), `SaveInfo.World`, `Manager.DeleteSave` (offen → ErrSaveInUse), HTTP `DELETE` (200, 404, 400, 409).
2. Umsetzen.
3. `docs/protocol.md` › HTTP: Spielstände nachtragen.

## Fertig, wenn

- [ ] AC-01: Go-Tests in `engine/store`, `engine/room`, `engine/net` grün, Löschen eines offenen Stands ergibt 409.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 umgesetzt: `DELETE /api/save?slot=` (`engine/net/saves.go`), `store.Saves.Discard` verschiebt in die Sicherungen (B-533), `room.Manager.DeleteSave` lehnt offene Stände ab (409), `GET /api/saves` nennt `world`; Anlegen bleibt `create` fresh. Protokoll-Version bleibt 8, `docs/protocol.md` nachgetragen.
- AC-01 geprüft: Go-Tests `TestSavesDiscardBehaeltSicherungen`, `TestSavesListNenntWelt`, `TestDeleteSaveLehntOffenenStandAb`, `TestSaveLoeschenUeberHTTP`; `task check:diff` grün; Reviewer (Sonnet): keine schweren Befunde.
