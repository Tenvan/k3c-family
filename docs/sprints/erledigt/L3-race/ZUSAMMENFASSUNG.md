# L3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Race-Detector für die nebenläufigen Go-Pakete
- **Projekt:** –

## Ziel

Datenrennen in `engine/` fallen in der CI auf. Am Ende sichtbar: Der Go-Job der CI hat einen Schritt mit `go test -race ./engine/...`, und `task check:go` führt `-race` aus oder meldet, warum nicht.

## Ergebnis

2026-10-01 · Review L3.2, `task check` und `task check:go` grün. - AC-01 umgesetzt (CI-Schritt im Job `go` auf ubuntu-latest, nach `go test ./...`); erster grüner CI-Lauf steht aus (🧑). - AC-02 geprüft (L3.1: ohne Compiler Hinweis und Exit 0, mit scheiterndem `gcc` Exit 1). - Befunde: keine. Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Die CI führt `go test -race ./engine/...` aus (B-077/AC-01). |
| AC-02 | geprüft | Lokal ohne C-Compiler meldet die Task den Verzicht auf `-race` und scheitert nicht (B-077/ … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-077 | erledigt | AC-01 –, AC-02 geprüft | Die nebenläufigen Go-Pakete werden mit dem Race-Detector geprüft |
