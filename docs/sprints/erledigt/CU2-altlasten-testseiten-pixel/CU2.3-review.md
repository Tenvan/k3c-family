# CU2.3 · Review CU2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** cu2/3-review
- **Abhängig von:** CU2.2
- **Tickets:** B-822, B-824
- **Kriterien:** alle

## Ziel

Sprint CU2 ist abgenommen, der eine PR des Sprints ist offen mit Auto-Merge, der Sprint liegt in `erledigt/`.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Diff in Planungsdateien, `src/`, CI, `tools/Taskfile.yml`, Workbench). Prüfpunkte: nichts gelöscht, was eine bleibende Seite braucht (`monitor.html`, `lizenzen.html`, `game.html`); `tests/projectRules.test.ts` ohne Ausnahmen für entfernte Seiten; Suchen aus B-822 und B-824 ohne Treffer. Glossar-Änderung aus CU2.2 im letzten Commit nach `git merge origin/develop`. Die Listen aus AC-05 und AC-10 gehen gesammelt an 🧑 (Ralf-Queue), kein Statuswechsel fremder Planung.

## Erlaubte Dateien

- alle Erlaubten Dateien von CU2.1 und CU2.2
- `docs/glossar.md` (nur letzter Commit)
- Planung CU2, B-822, B-824, Abnahme-Ticket (über MCP)

## Nicht-Ziele

Neue Funktionen, Stil-Befunde, B-823.

## Schritte

1. `task check`, `task check:go` nacheinander; `task check:dev` (Workbench-Änderung).
2. Diff `origin/develop...sprint/cu2` lesen, `k3c-reviewer` parallel; schwere Befunde beheben.
3. Glossar nachziehen, Abnahme in der README, Abnahme-Ticket nach B-742 nur falls nötig (Gen-Lauf lief in CU2.2; Sim gesperrt), Sprint nach `erledigt/`, PR mit Auto-Merge.

## Fertig, wenn

- [ ] AC-01 bis AC-10: Nachweise in den Ergebnissen von CU2.1 und CU2.2 geprüft.
- [ ] Ralf-Queue: Listen aus AC-05 und AC-10 gemeldet.
- [ ] Abnahme eingetragen, PR offen, Auto-Merge an.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Review k3c-reviewer 2026-11-12: Spiel, Panel und CI brechen nicht. Behoben: `testScenarios.ts` samt verwaister `test.*`-/`tool.httpError`-Texte entfernt, tote oxlint-Regel und Kommentar in `gamedata/reports.go`, README (Testseiten), Lizenztexte um Schrift und CC-BY 4.0 ergänzt, `public/audio/README.md` (Probe-Stücke ungenutzt). B-823-Änderung als eigener Commit getrennt. `task check:diff` grün.
