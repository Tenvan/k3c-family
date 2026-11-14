# M9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** k3c-dev in Worktrees und Markdown-Ansicht
- **Projekt:** –

## Ziel

Jede Session plant, prüft und startet Dienste in ihrem eigenen Checkout und liest Sessions in k3c-dev wie im Editor. Am Ende sichtbar: Planung, Prüfläufe und Dienste treffen den Worktree der Session; Session-Dateien lesbar im Detail-Panel.

## Ergebnis

- 2026-10-07, Review M9.4: AC-01 (Ergebnis M9.1: Test `TestWorkspaceAusHeader`; M9.2: Vite-Probe ohne Port, Live-Probe mit Port entfiel), AC-02 (M9.3: Vitest), AC-04 (`task check:dev`, `task check` grün) mit Nachweis; kein schwerer Befund. - AC-03: verschoben, die Ursache des fehlenden Headers ist offline eingegrenzt und nur live messbar: B-341 (die Log-Felder `header` und `checkout` entscheiden mit einem Aufruf aus einem Worktree). - B-275 und B-213 archiviert. Version: v0.14.1 vorgeschlagen (Patch: nur Entwickler-Werkzeug k3c-dev und Dev-Server-Konfiguration ohne Änderung am Spiel; aktuell v …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | k3c-dev und Vite arbeiten in Worktrees unter `.claude/worktrees/` richtig (B-275/AC-01, B- … |
| AC-02 | – | MarkdownView in k3c-dev zeigt nummerierte Listen und Häkchen wie die alte Planungsseite (B … |
| AC-03 | verschoben | Die Ursache, warum der Header `X-K3C-Root` aus Worktrees nicht ankommt, ist geklärt und im … |
| AC-04 | geprüft | `task check:dev` und `task check` grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-275 | erledigt | AC-01 – | k3c-dev und Vite arbeiten in Worktrees unter `.claude/worktrees/` richtig |
| B-213 | erledigt | AC-02 – | MarkdownView in k3c-dev zeigt nummerierte Listen und Häkchen wie die alte Planun … |
