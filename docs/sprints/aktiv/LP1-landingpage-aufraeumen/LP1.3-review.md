# LP1.3 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** lp1/3-review
- **Abhängig von:** LP1.1, LP1.2
- **Tickets:** B-335, B-292
- **Kriterien:** alle

## Ziel

Sprint LP1 ist nach `docs/arbeitsweise.md` › Review-Session abgenommen, der eine PR des Sprints ist offen.

## Kontext

Diff `origin/develop...origin/sprint/lp1`. AC-04 ist eine Abnahme durch 🧑 (LP1.4) und keine Abhängigkeit des Reviews: als `angenommen, Validierung offen (LP1.4)` führen. Grenzfall `tests/projectRules.test.ts` (INF) laut Sprint-README prüfen: nur die Seiten-Prüfung geändert.

## Erlaubte Dateien

- Dateien aus LP1.1 (nur schwere Befunde)
- Planungsdateien

## Nicht-Ziele

Stil, Benennung, Vereinfachungen; Server.

## Schritte

1. `task check` grün.
2. Diff lesen, nur schwere Befunde nach `docs/arbeitsweise.md` beheben oder als Ticket anlegen.
3. Abnahme in der Sprint-README, Fahrplan (LP1.4 unter „Offen am Gerät“, falls noch offen), Versionsvorschlag; B-292 archivieren.
4. Commit, `git merge origin/develop`, push, PR des Sprints öffnen.

## Fertig, wenn

- [ ] Alle Kriterien mit Nachweis oder `angenommen`/`verschoben` in der Abnahme.
- [ ] `task check` grün, PR offen.

## Prüfen

```bash
task check
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
