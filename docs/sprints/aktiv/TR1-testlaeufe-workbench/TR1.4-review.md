# TR1.4 · Review und Abnahme des Sprints TR1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** tr1/4-review
- **Abhängig von:** TR1.3
- **Tickets:** B-348
- **Kriterien:** alle

## Ziel

TR1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, B-348 archiviert, der eine PR des Sprints offen.

## Kontext

Leichtes Review, nur schwere Befunde im Diff. Besonders prüfen: Sicherheit (keine Shell, Parameter als Positivliste, Bot-Feed und Browser nur auf Loopback, Token nie in Ausgabe oder Bericht), Läufe enden sicher (`stop`, Zeitlimit, Workbench-Ende), Ausgabe von `status` ≤ 10 Zeilen, Läufe nur im Checkout der Session.

## Erlaubte Dateien

- Dateien der Sessions TR1.1 bis TR1.3 nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, neue Funktionen.

## Schritte

1. `Status: in Arbeit`. `task check`, `task check:go`, `task check:dev` grün.
2. `git diff origin/develop...origin/sprint/tr1` lesen, Befunde behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 prüfen (AC-01, AC-02, AC-06: TR1.1; AC-03: TR1.2; AC-04: TR1.3; AC-05: alle).
4. Abnahme (≤ 5 Zeilen, `Version: v… vorgeschlagen`), B-348 erledigt und archiviert, Sprint nach `erledigt/`, Fahrplan, `git merge origin/develop`, push, PR öffnen.

## Fertig, wenn

- [ ] AC-01 bis AC-06 haben Nachweis oder sind mit Grund und Ticket verschoben.
- [ ] Schwere Befunde behoben oder als Ticket.
- [ ] Checks grün, PR offen.

## Prüfen

```bash
task check
task check:go
task check:dev
```

## Ergebnis

–
