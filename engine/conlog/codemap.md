# engine/conlog/

## Responsibility

Infrastructure/Adapter: `slog.Handler` für lesbare, farbige Konsolenzeilen. Wird von Server und Entwickler-Werkzeug gemeinsam genutzt; JSON-Logdateien bleiben davon unberührt.

## Design

- `Handler` (`conlog.go`) implementiert `slog.Handler` (`Enabled`, `Handle`, `WithAttrs`, `WithGroup`); Writer und Mutex werden zwischen abgeleiteten Handlern geteilt.
- Zeilenformat: `HH:MM:SS.mmm LEVEL [ns] Text key=value …`, Level-Wörter bleiben im Text (auch ohne Farbe auffindbar).
- `Topics` (`map[string]string`): Emoji je Namespace (`ns`), wird vor `[ns]` gesetzt; neuer `ns` wird dort eingetragen.
- `Color()`: aus bei gesetztem `NO_COLOR`; `levelStyle` liefert ANSI-Farbe und 5-Zeichen-Name; `quote` quotet wie `slog.TextHandler`; `attr` löst Gruppen zu `key.sub=value` auf.

## Flow

1. `slog.New(conlog.New(os.Stderr, level, conlog.Color()))` beim Start.
2. `slog.Info(...)` → `Handler.Handle` formatiert Zeit, Level, Emoji aus `Topics[ns]`, `[ns]`, Meldung.
3. Attribute (`WithAttrs`-Vorbelegung + Record) laufen durch `attr`, Ausgabe unter Sperre in den Writer.

## Integration

- Abhängigkeiten: nur Stdlib (`log/slog`).
- Konsumenten: `cmd/k3c-server`, `tools/k3c-dev/internal/applog`.
- k3c-dev liest Level-Wörter (`markOf`, `outLevel`) aus den Zeilen.
