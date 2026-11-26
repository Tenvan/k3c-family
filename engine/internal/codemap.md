# engine/internal/

## Responsibility

Repo-Regeln für den Go-Rest als Tests; kein Laufzeitcode. Golden-, Prüfvektor- und Trace-Hilfen der früheren Go-Engine
sind mit RU41.1 entfernt (letzter Stand im Tag `go-referenz-final`).

## Design

| Ordner | Rolle |
|--------|-------|
| `rules/` | Repo-Regeln für Go als Tests: Importgrenzen (`imports_test.go`), Verschachtelungstiefe (`nesting_test.go`) |

## Integration

- Läuft mit `task go:test`; `internal` verhindert den Import von außen.
