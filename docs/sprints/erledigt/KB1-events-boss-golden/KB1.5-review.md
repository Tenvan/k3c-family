# KB1.5 · Review KB1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** kb1/5-review
- **Abhängig von:** KB1.4
- **Tickets:** B-379, B-380, B-343, B-327, B-328
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Review des Sprints KB1: Kriterien AC-01 bis AC-04 geprüft, Golden-Diffs begründet, Sprint abgeschlossen oder Restpunkte als Ticket.

## Kontext

Commits des Sprints KB1 (KB1.1 bis KB1.4); Golden-Commits mit Begründung.

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`
- Fixes unter 30 Zeilen in den Dateien der Sessions

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Reviewer-Agent über den Sprint-Diff.
2. `task check:go` (gezielt `go:test`, `go:lint`).
3. Ergebnis, Tickets abschließen.

## Fertig, wenn

- [ ] Review ohne schwere Befunde, Prüfungen grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 bis AC-04: geprüft mit `task check` und `task check:go` (grün) sowie dem Review zu KB1.1 bis KB1.4.
- Die Abnahme steht in der Sprint-README.
