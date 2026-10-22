# Task Completion

1. Vor dem Commit einmal `task check:diff` (via k3c-workbench `check_run task:check:diff`, sonst Shell): wählt Client, Go, k3c-workbench, Studio und Starter nach dem Diff. Während der Arbeit nur gezielte Tests. Den vollen Satz (`check`, `check:go`, `check:race`) fährt die Review-Session, `check:all` der Release.
2. Planungs-Änderungen: `check_run task:test` mit `pattern: planning`; mehrere Abschnitte mit `plan_sections`.
3. Code-Review durch separaten Reviewer-Agent nur bei SIM/SRV, Protokoll, Spielstand oder ab ~150 Code-Zeilen, parallel zu `check:diff`; schwere Befunde vor dem Commit beheben (`docs/arbeitsweise.md` › Tempo im Lauf).
4. Ein Commit je Session auf `sprint/<präfix>`; PR erst öffnen, wenn vollständig gepusht; PR-Status vor Nachpush prüfen.
5. Wert-/Regeländerung mit Golden-Diff: `task go:golden` und Begründung in den Commit.
6. Sprint erst `erledigt`, wenn alle Sessions fertig (offene Hardware-Sessions halten ihn aktiv).
