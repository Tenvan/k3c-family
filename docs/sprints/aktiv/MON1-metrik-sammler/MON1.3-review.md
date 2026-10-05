# MON1.3 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Branch:** mon1/3-review
- **Abhängig von:** MON1.1, MON1.2
- **Tickets:** B-281
- **Kriterien:** alle

## Ziel

Der Sprint ist nach `docs/arbeitsweise.md` › Review-Session abgenommen, B-281 archiviert und der eine PR des Sprints gegen `develop` offen.

## Kontext

- Ablauf: `docs/arbeitsweise.md` › Review-Session (nur schwere Befunde zählen).
- Diff: `git diff origin/develop...origin/sprint/mon1`.
- Sicherheit besonders: `/api/metrics` nur mit Token (404/401/405), Geräte nur als Kürzel, Parameter `since` ungeprüft von außen.
- Leistung: Benchmark-Ergebnis aus MON1.1 (B-281 › Notizen) gegen AC-03 halten.

## Erlaubte Dateien

- Dateien des Sprints (`engine/room/`, `engine/net/`, `docs/protocol.md`) nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Stil, Benennung, Vereinfachungen; Seite (MON2).

## Schritte

1. `task check` und `task check:go` → grün.
2. Diff lesen, schwere Befunde in der Domäne beheben, sonst Ticket.
3. Abnahme in der Sprint-README (höchstens fünf Zeilen, Versionsvorschlag), B-281 `erledigt` und archivieren.
4. Sprint nach `docs/sprints/erledigt/`, `Status: erledigt`, Fahrplan; committen, `git merge origin/develop`, pushen, PR öffnen.

## Fertig, wenn

- [ ] Alle Kriterien AC-01 … AC-07 mit Nachweis (Session-Ergebnisse), AC-07: `task check:go` grün.
- [ ] Abnahme ausgefüllt, PR offen.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

–
