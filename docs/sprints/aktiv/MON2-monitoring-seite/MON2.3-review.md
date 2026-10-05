# MON2.3 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Branch:** mon2/3-review
- **Abhängig von:** MON2.2
- **Tickets:** B-282
- **Kriterien:** alle

## Ziel

Sprint MON2 ist nach `docs/arbeitsweise.md` › Review-Session abgenommen, der eine PR des Sprints ist offen.

## Kontext

Diff `origin/develop...origin/sprint/mon2`. AC-05 ist eine Hardware-Abnahme (MON2.4) und keine Abhängigkeit des Reviews:
als `angenommen, Validierung offen (MON2.4)` führen. Ist AC-04 aus MON2.2 noch ohne Nachweis, mit Freigabe 🧑 im Browser-Pane
nachholen oder als offen führen.

## Erlaubte Dateien

- Dateien der Domäne PLAT aus MON2.1 und MON2.2 (nur schwere Befunde)
- Planungsdateien

## Nicht-Ziele

Stil, Benennung, Vereinfachungen; Änderungen am Server.

## Schritte

1. `task check` grün.
2. Diff lesen, nur schwere Befunde nach `docs/arbeitsweise.md` beheben oder als Ticket anlegen.
3. Abnahme in der Sprint-README, Ordner nach `sprints/erledigt/`, Fahrplan (MON2.4 unter „Offen am Gerät“), Versionsvorschlag.
4. Commit, `git merge origin/develop`, push, PR des Sprints öffnen.

## Fertig, wenn

- [ ] Alle Kriterien mit Nachweis oder `angenommen`/`verschoben` in der Abnahme.
- [ ] `task check` grün, PR offen.

## Prüfen

```bash
task check
```

## Ergebnis

–
