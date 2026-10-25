# KB2.2 · Raum speichert nach „Komplett verloren“ nicht über den letzten Spielstand

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb2/2-gameover-nicht-speichern
- **Abhängig von:** KB2.1
- **Tickets:** B-344
- **Kriterien:** AC-02

## Ziel

Nach „Komplett verloren“ (`Island.Over`) schreibt der Raum keinen Spielstand mehr; der zuletzt gespeicherte Stand bleibt Byte für Byte gleich (B-344).

## Kontext

`engine/sim/island_defeat.go` setzt bei Niederlage-Modus `lost` `Island.Over = true` und meldet `gameOver`; `StepIsland` ändert danach nichts mehr. `engine/room/saver.go` speichert über `save()` (Stufenwechsel, Verlassen, Abbruch) und `saveNow()` (Aufräumen des leeren Raums, Herunterfahren); `Room.SaveNow` (Diagnose, B-088) schreibt direkt. Alle drei Wege brauchen den Riegel. Spielstand-Format bleibt (v5).

## Erlaubte Dateien

- `engine/room/saver.go`, `engine/room/room.go` (nur `SaveNow`)
- `engine/room/gameover_save_test.go` (neu)
- `engine/room/codemap.md`

## Nicht-Ziele

Anzeige des Endes (K5), Neustart nach Game Over, Protokollfeld für `gameOver` (K4), Änderungen in `engine/sim/`. Offene Frage aus B-344 (Raum sofort schließen?) bleibt: Default „Raum bleibt offen, bis alle gegangen sind“.

## Schritte

1. Riegel `r.isl.Over` in `save()`, `saveNow()` und `SaveNow` (Fehler `ErrGameOver`), Log 🚫.
2. Test: Stand vor Game Over bleibt bei Verlassen, Aufräumen und Herunterfahren gleich; ohne vorherigen Stand entsteht keine Datei.
3. `task check:diff`, Reviewer parallel.

## Fertig, wenn

- [ ] AC-02: `go test ./engine/room -run GameOver` grün: Datei nach `Island.Over` bei Verlassen, Aufräumen und Herunterfahren unverändert, ohne vorherigen Stand keine Datei, Log mit 🚫.
- [ ] `task check:diff` grün (bekannte rote Studio-Tests: B-606).

## Prüfen

```bash
go test ./engine/room -run GameOver
task check:diff
```

## Ergebnis

- **AC-02** umgesetzt: Riegel `gameOver` in `save`, `saveNow` und `SaveNow` (`ErrGameOver`, Log 🚫); Stände von vor dem Game Over schreibt die Schreib-Goroutine noch. Geprüft: `gameover_save_test.go` (Verlassen, Abbruch, Diagnose, Aufräumen, Herunterfahren; ohne vorherigen Stand keine Datei), `go test ./engine/room ./engine/net ./engine/store` grün, `golangci-lint` 0 Befunde.
- `task check:diff` rot außerhalb des Diffs: `engine/*` zieht die Frontends von Workbench, Studio und Starter mit, im Worktree fehlen deren `node_modules` (TS2307 `react`); bekannte rote Studio-Tests B-606. `task check:go` im Zeitlimit, einziger Fehler `engine/devrun` `TestStopBrichtBeiZweitemSignalAb` (Wanduhr unter Last, B-284), einzeln grün.
- Review (Sonnet): keine schweren Befunde. Hinweis: `afterDisconnect` loggt nach Game Over weiter „💾 Raum leer und pausiert, speichert“ (folgenlos, nicht in den Erlaubten Dateien).
- Offene Frage B-344 mit Default entschieden: Raum bleibt nach Game Over offen, bis alle gegangen sind. `POST /api/status/save` antwortet nach Game Over mit 500 „Spielstand nicht gesichert“ (kein eigener Protokoll-Code).
