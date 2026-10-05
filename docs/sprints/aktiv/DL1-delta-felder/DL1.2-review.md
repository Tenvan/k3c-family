# DL1.2 · Review und Abnahme des Sprints DL1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Branch:** dl1/2-review
- **Abhängig von:** DL1.1
- **Tickets:** B-297
- **Kriterien:** alle

## Ziel

DL1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, liegt in `docs/sprints/erledigt/`, B-297 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff). Besonders: `null` bleibt ein Wert, `events` nie in `unset`, Client und
Server verhalten sich gleich, der Delta-Test deckt weiter alle geprüften Felder ab.

## Erlaubte Dateien

- `engine/net/delta*.go`, `src/online/clientDelta*`, `docs/protocol.md` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung.

## Schritte

1. `Status: in Arbeit`. `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/dl1` lesen, Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-04 aus dem Ergebnis von DL1.1 prüfen.
4. Abnahme (höchstens fünf Zeilen) mit Versionsvorschlag in die Sprint-README.
5. B-297 auf `erledigt` und nach `docs/backlog/archiv/`; Sprint nach `docs/sprints/erledigt/`, `Status: erledigt`,
   Fahrplan anpassen, PR öffnen.

## Fertig, wenn

- [ ] AC-01 bis AC-04 haben einen Nachweis im Ergebnis von DL1.1.
- [ ] Schwere Befunde behoben oder als Ticket; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

–
