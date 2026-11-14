# WR2.1 · Server-Reste in engine/room, k3c-sim bot und simexe entfernen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/wr2
- **Abhängig von:** –
- **Tickets:** B-897
- **Kriterien:** AC-01, AC-02, AC-03, AC-08

## Ziel

Server-Teile in `engine/room`, `k3c-sim bot`, `simexe.StartBot`/`Pool` und `testdata/protocol/` sind entfernt; das native Spiel läuft unverändert.

## Kontext

B-897 › Anforderungen. Nutzer von `engine/room`: `native/k3c-game/local` (Manager, Room, Peer, Snapshot, Dev-Aktionen, Fehler-Werte) und `tools/k3c-workbench/internal/studioview`.

## Erlaubte Dateien

- `engine/room/**`
- `cmd/k3c-sim/**`
- `tools/k3c-workbench/internal/simexe/**`, `tools/k3c-workbench/internal/mcpsrv/fakesim_test.go`
- `testdata/protocol/**`

## Nicht-Ziele

- `native/k3c-game/**`, Beobachter, Vorschau, Spielmetrik-Report.

## Schritte

1. `k3c-sim bot` samt Test und Usage-Texten entfernen.
2. `simexe` `bot.go`, `pool.go`, Bot-Tests und Fake-Bot entfernen.
3. `engine/room`: Messreihen, Reopen und den Hook Changed samt Tests entfernen; Kommentare zu `engine/net` nachziehen.
4. `testdata/protocol/` entfernen, Beispiel in `autosave_test.go` einbetten.

## Fertig, wenn

- AC-01, AC-02, AC-03 erfüllt, `task check:diff` grün.

## Prüfen

- `check_run task:check:diff` (Worktree).
- `grep -rn "engine/net\|Spielserver\|k3c-server" --include=*.go engine cmd tools` ohne `_test.go`.

## Ergebnis

- Entfernt: `cmd/k3c-sim/bot.go` samt Test und Usage, `simexe/bot.go`, `pool.go`, `Event.Commands`, Bot-Tests, Fake-Bot in `mcpsrv/fakesim_test.go`; in `engine/room` `monitor.go` (Messreihen, `Sample`, `secTicks`), `reopen.go`, Hook `Manager.Changed` samt `notify`; `testdata/protocol/` (Beispiel `saved` steht jetzt in `autosave_test.go`).
- Behalten, weil das native Spiel sie nutzt: `Status` (Abstürze, `local/step.go`), `CloseRoom` und `DeleteSave` (`local/session.go`, `saves.go`), `Peer`, `Snapshot`.
- Behalten im Zweifel: `Summary` (Beobachtung in Tests), Beobachter (`observer.go`), Vorschau (`preview*.go`, Aufrufer entfallen), `Rooms`/`WorldInfo` (Raumliste), Spielmetrik-Report (`metrics.go`, Balancing gesperrt).
- Kommentare zu `engine/net` und `/api/*` in `engine/room`, `cmd/k3c-sim/world.go` und Codemap nachgezogen. `task check:diff` grün.
