# engine/internal/

## Responsibility

Test-Hilfen und Repo-Regeln für die Go-Engine; kein Laufzeitcode des Spiels. Jeder Unterordner hat eine eigene Aufgabe.

## Design

| Ordner | Rolle |
|--------|-------|
| `golden/` | Golden-Dateien unter `testdata/golden/` lesen, vergleichen und mit `task go:golden` neu schreiben |
| `rules/` | Repo-Regeln für Go als Tests: Importgrenzen, Verschachtelungstiefe, Map-Iteration |
| [`detvec/`](detvec/codemap.md) | Prüf-Vektoren (Gleitkomma, Spiel-Daten) für den Rust-Port |
| [`trace/`](trace/codemap.md) | Aufzeichnung eines Sim-Laufs als `.jsonl.gz` für den Rust-Port |

## Integration

- Nur aus Tests der Engine genutzt; `internal` verhindert den Import von außen.
