# US1.5 · Review US1

- **Status:** fertig
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

- Review Opus über `094be01c..HEAD`: 0 kritisch, 4 wichtig, 6 klein.
- Behoben: W1 fehlende Einmal-Animation startet idle nicht neu (Schlag zu Fuß), W2 Oberkante beritten aus dem Gang-Rahmen, W3 `-check-data` prüft nach `data/paper.json` (`TestCheckPaperNurGenutzteGruppen`, `TestCheckPaperUseRepoStand`, `TestCheckDataPaperFehlerExit1`), W4 keine alte Einfärbung für Truppen und Gegner (`paperTint.test.ts` RED→GREEN), K5 Überlage im Bau ausgeblendet, K10 Kommentar.
- Zurückgestellt: K6–K9 als D-US1-06.
- `origin/feature/pap` gemergt (Konflikt `docs/sprints/README.md` gelöst); danach Vitest 3193/3193, tsc, Go-Tests grün; `check:diff` rot nur durch einen Workbench-Frontend-Flake unter Last, Frontend allein 432/432 (B-704).
