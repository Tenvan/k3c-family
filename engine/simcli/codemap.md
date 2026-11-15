# engine/simcli/

## Responsibility

Application Service: die Unterbefehle des Engine-CLI (B-726, B-924) als Go-Paket. `cmd/k3c-sim` ist die EXE dazu, die
Workbench ruft dieselbe Logik im Prozess auf. `engine/` importiert nichts aus `tools/`.

## Design

- `simcli.go`: `Exec(ctx, args, stdin, stderr, out, files)` führt einen Unterbefehl aus und liefert den Exit-Code
  (`ExitOK`, `ExitFailed`, `ExitUsage`, `ExitAborted`); `run` fängt jede Panik und macht sie zum Fehler (`💥 Absturz in der
  Engine`). `Emitter` schreibt die JSON-Zeilen (`progress`, `result`, `report`, `error`). Abbruch über den Context:
  `checkAbort` im Fortschritts-Rückruf von `balance --targets`/`--tune` (Panik `aborted`, gefangen in `engine/balance/parallel.go`).
- `call.go`: `Stream` (Events an einen Rückruf, abbrechbar, Rechnung in einer Goroutine) und `Call` (Zeile `result`);
  `Event`, `ParseEvent`, `DirFiles` (Daten eines Checkouts von der Platte für `world` und `hub`).
- `input.go` (`run`, `level`), `balance.go`, `targets.go`, `tune.go`, `sensitivity.go`, `world.go`, `hub.go`: je Unterbefehl.

## Integration

- Abhängigkeiten: `engine/enginetools`, `engine/balance`, `engine/level`, `engine/sim`, `data`.
- Konsumenten: `cmd/k3c-sim`; Workbench (`internal/mcpsrv`, `internal/simrun`, `internal/world`, `internal/hubplan`).
- Grenze: `level` und `run` lesen `data.Files` des Prozesses (eingebettet oder `K3C_DATA_DIR` beim Start), `world` und `hub`
  die übergebenen Dateien (Checkout-`data/`).
