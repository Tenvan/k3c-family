# tools/k3c-dev/internal/console/

## Responsibility

In-Memory-Konsolenpuffer je Quelle (Prüfläufe `check:<ziel>`, eigenes Log, Dienste, Tasks) mit Ring-Puffer und Live-Benachrichtigung für die Oberfläche.

## Design

- `Store` (`console.go`): `New(capacity, onLine)`, `Add(source, stream, text)`, `Tail`, `Reset`, `Sources`, `Close`; je Quelle ein `ring` fester Kapazität, `Line` als Eintrag.
- Asynchrones `dispatch`: der `onLine`-Callback ist vom schreibenden Prozess entkoppelt.
- `LineWriter` (`writer.go`): `io.Writer`-Adapter, der Byte-Ströme in Zeilen zerlegt (`NewLineWriter(emit)`, `Flush`, `clean` entfernt Steuerzeichen).

## Flow

1. Produzent (Prozess-stdout via `LineWriter`, `slog`-Handler) ruft `Store.Add`.
2. `ringFor(source)` legt den Ring an oder nutzt ihn; die Zeile wird abgelegt und an `onLine` gereicht (Event an das Frontend).
3. Lesen: `Tail(source, n)` (MCP `console_tail`, Wails `ConsoleTail`), `Sources()` für die Quellenliste.

## Integration

- Konsumenten: `internal/applog` (Spiegel), `internal/services` (`controller.go`, `runtime.go`, `outlog.go`), `internal/taskrun`, `internal/mcpsrv` (`check_run.go`, `server.go`), `app.go`, `app_logs.go`, `main.go`.
- MCP-Tool: `console_tail`. Keine internen Abhängigkeiten.
