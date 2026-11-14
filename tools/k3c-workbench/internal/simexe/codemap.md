# tools/k3c-workbench/internal/simexe/

## Responsibility

Ruft das Engine-CLI `bin/k3c-sim` als Kindprozess auf (B-726, RS2.2) für `sim_run`, `level_generate`, `replay_run`, die Offline-Testläufe und den Welten-Editor; die Workbench importiert die Engine dadurch nicht.

## Design

- Protokoll: k3c-sim schreibt JSON-Zeilen auf stdout; `Event` mit `ParseEvent(line)` (nur Zeilen mit `event`). Schnittstelle laut Kopfkommentar von `cmd/k3c-sim/main.go`.
- `Rel(slot)`: Pfad der EXE relativ zum Checkout; `slot > 0` sind eigene Plätze `bin/k3c-sim-<slot>` der Offline-Läufe, damit ein laufender Prozess den Bau eines anderen Platzes nicht sperrt.
- `Env(root)`: setzt `data.DirEnv` auf `<root>/data`, damit k3c-sim die Daten des Checkouts von der Platte liest statt der eingebetteten Kopie.
- `Check(root)`: EXE vorhanden und nicht älter als ihre Quellen (`sources`: `cmd/k3c-sim`, `engine`, `go.mod`, `go.sum`; `newerSource`), sonst Fehler mit `BuildHint` (`task k3c-sim:build`).
- `Call(ctx, root, args, input, timeout)`: kurzer Unterbefehl (`run`, `level`, `balance --play`) über `internal/proc`, Eingabe auf stdin, Ergebnis ist das `result`-Event; Fehler nennen die Meldung von k3c-sim, die letzten `tailLines` stderr-Zeilen (`crashError`) oder das Zeitlimit.

## Flow

1. MCP-Tool oder Paket ruft `Check(root)`.
2. `Call` startet `bin/k3c-sim <args>` mit `Env(root)`, schreibt `input`, liest stdout zeilenweise per `ParseEvent`.
3. Das `result`-Event geht an den Aufrufer zurück.

## Integration

- Konsumenten: `internal/mcpsrv/tools_engine.go` (`sim_run`, `level_generate`), `internal/mcpsrv/tools_replay.go` (`replay_run`), `internal/simrun/offline.go`, `internal/world/level.go`.
- Abhängigkeiten: `internal/proc`, `internal/console` (`LineWriter`), `k3c/data` (`DirEnv`); Binary `bin/k3c-sim` aus `cmd/k3c-sim`.
