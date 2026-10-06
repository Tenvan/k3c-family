# M9 · SRV · k3c-dev in Worktrees und Markdown-Ansicht

- **Status:** geplant
- **Domäne:** SRV
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-275, B-213
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

k3c-dev bedient aus Worktrees unter `.claude/worktrees/` noch die Repo-Wurzel (B-275, Rückfall B-318); MarkdownView zeigt nummerierte Listen und Häkchen falsch (B-213).

## Ziel

Jede Session plant, prüft und startet Dienste in ihrem eigenen Checkout und liest Sessions in k3c-dev wie im Editor. Am Ende sichtbar: Planung, Prüfläufe und Dienste treffen den Worktree der Session; Session-Dateien lesbar im Detail-Panel.

## Beteiligte und Zielgruppen

🧑 und Agenten in parallelen Worktrees; der Agent baut in `tools/k3c-dev/`.

## Anforderungen

B-275 › Anforderungen; B-213 › Anforderungen.

## Nicht-Ziele

Neue MCP-Tools, Umbau der Planungsseite.

## Regeln und Einschränkungen

Nur `tools/k3c-dev/` (SRV). Eine k3c-dev-Instanz bleibt für alle Worktrees zuständig.

## Beispiele

Session in `.claude/worktrees/x` ruft `plan_set` → Datei im Worktree geändert, `workbench_status` nennt den Worktree.

## Ausnahme- und Fehlerfälle

Header `X-K3C-Root` fehlt oder zeigt außerhalb des Repos → Fehler mit Hinweis, kein Rückfall auf die Wurzel.

## Akzeptanzkriterien

- **AC-01** k3c-dev und Vite arbeiten in Worktrees unter `.claude/worktrees/` richtig (B-275/AC-01, B-275/AC-02, B-275/AC-03).
- **AC-02** MarkdownView in k3c-dev zeigt nummerierte Listen und Häkchen wie die alte Planungsseite (B-213/AC-01, B-213/AC-02).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- M9.1 Checkout je Aufruf aus dem Worktree, Vite-Reload im Worktree (AC-01).
- M9.2 MarkdownView: nummerierte Listen und Häkchen (AC-02).
- M9.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
