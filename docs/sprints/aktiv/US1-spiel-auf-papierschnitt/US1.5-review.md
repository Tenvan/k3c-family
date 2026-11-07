# US1.5 · Review US1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** us1/5-review
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Review des Sprints mit Opus, Befunde behoben oder als D-US1-xx notiert, Abnahme in der README.

## Kontext

Diff `094be01c..HEAD`.

## Erlaubte Dateien

- alle Dateien der Sessions US1.1–US1.4
- Sprint-README

## Nicht-Ziele

Neue Features.

## Schritte

1. Reviewer-Agent (Opus) über den Diff.
2. Wichtige Befunde beheben.
3. `origin/feature/pap` mergen, Prüfungen, Commit, Push.

## Fertig, wenn

- [ ] Abnahme in der README, alle Sessions fertig.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
