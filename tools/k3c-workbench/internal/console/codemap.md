# tools/k3c-workbench/internal/console/

## Responsibility

In-Memory-Konsolenpuffer je Quelle (Prüfläufe `check:<ziel>`, eigenes Log, Dienste, Tasks, Sim-Läufe) mit Ring-Puffer und Live-Benachrichtigung für die Oberfläche.

## Design

- `Store` (`console.go`): `New(capacity, onLine)`, `Add(source, stream, text)`, `Tail`, `Reset`, `Sources`, `Stats` (Quellen, Zeilen, Bytes für das Ressourcen-Monitoring), `Close`; je Quelle ein `ring` fester Kapazität, `Line` als Eintrag.
- Asynchrones `dispatch`: der `onLine`-Callback ist vom schreibenden Prozess entkoppelt.
- `LineWriter` (`writer.go`): `io.Writer`-Adapter, der Byte-Ströme in Zeilen zerlegt (`NewLineWriter(emit)`, `Flush`, `clean` entfernt Steuerzeichen).

## Flow

1. Produzent (Prozess-stdout via `LineWriter`, `slog`-Handler) ruft `Store.Add`.
2. `ringFor(source)` legt den Ring an oder nutzt ihn; die Zeile wird abgelegt und an `onLine` gereicht (Event `console:line`, gebündelt in `console_batch.go`).
3. Lesen: `Tail(source, n)` (MCP `console_tail` in `internal/mcpsrv/tools_console.go`, Binding `ConsoleTail`), `Sources()` für die Quellenliste.

## Integration

- Konsumenten: `internal/applog`, `internal/services` (`controller.go`, `runtime.go`), `internal/taskrun`, `internal/simexe`, `internal/simrun` (`offline.go`), `internal/mcpsrv` (`check_run.go`, `server.go`, `tools_console.go`), `main.go`, `app.go`, `app_logs.go`, `console_batch.go`.
- Keine internen Abhängigkeiten.
