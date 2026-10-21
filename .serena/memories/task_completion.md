# Task Completion

1. `task check` muss grün sein (via k3c-dev `check_run`, sonst Shell). Bei Go-Änderungen zusätzlich `task check:go`; bei `tools/k3c-dev` `check:dev`, bei Studio `check:studio`; vor Release `check:all`.
2. Planungs-Änderungen: `check_run task:test` mit `pattern: planning`.
3. Code-Review der geänderten Dateien durch separaten Reviewer-Agent (`/code-review`); schwere Befunde vor dem Commit beheben.
4. Ein Commit je Session auf `sprint/<präfix>`; PR erst öffnen, wenn vollständig gepusht; PR-Status vor Nachpush prüfen.
5. Wert-/Regeländerung mit Golden-Diff: `task go:golden` und Begründung in den Commit.
6. Sprint erst `erledigt`, wenn alle Sessions fertig (offene Hardware-Sessions halten ihn aktiv).
