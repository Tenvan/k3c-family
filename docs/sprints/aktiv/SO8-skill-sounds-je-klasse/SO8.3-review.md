# SO8.3 · Review SO8

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** so8/3-review
- **Abhängig von:** SO8.2
- **Tickets:** B-576
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Alle Kriterien AC-01 bis AC-03 sind geprüft, die Abnahme steht in der Sprint-README.

## Kontext

Diff von SO8.1 und SO8.2; `task check`, `task check:go`, `task check:race` nacheinander (check:go und check:race nur, weil die Review-Regel es verlangt).

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`
- Fixes unter 30 Zeilen in den Dateien von SO8.1 und SO8.2

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Prüfläufe.
2. Diff lesen (Credits, Katalog, Abbildung, Anbindung), nur schwere Befunde beheben.
3. Abnahme-Abschnitt der Sprint-README füllen (höchstens fünf Zeilen); AC-04 steht bei SO8.4 offen.

## Fertig, wenn

- [ ] AC-01 bis AC-03 mit Nachweis oder Ticket.

## Prüfen

`task check`, `task check:go`, `task check:race` (nacheinander).

## Ergebnis

- AC-01 geprüft: vier Slots mit je drei Kenney-CC0-Kandidaten und Credit, Atlas im Diff, Vitest `skills.test.ts` (Katalog je Klasse) (SO8.1).
- AC-02, AC-03 geprüft: Vitest `skills.test.ts` und `skillCasts.test.ts` (SO8.2).
- Prüfläufe nacheinander: `task check` grün (9,9 s), `task check:go` grün (2 min 18 s), `task check:race` grün (aus dem Cache, keine Go-Änderung).
- Review (Sonnet, getrennt): keine schweren Befunde; kein doppelter Ton im Split-Screen, kein Ton aus dem ersten Zustand oder nach `reset`.
