# B-925 · Änderungen am Sim-Code laufen im eigenen Worktree mit eigener Dev-Workbench auf eigenen Ports

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WE1
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Mit B-924 läuft der Sim-Code in der Workbench selbst. Eine Session, die `engine/sim`, `engine/level`, `engine/room` oder `cmd/k3c-sim` ändert, würde sonst die laufende Workbench der Repo-Wurzel (Ports 5180/5191) mit fremdem Stand testen. Die Workbench kennt schon `K3C_DEV_PORT` (MCP) und `K3C_DEV_UI_API_PORT` (UI-API) sowie Port-Versatz je Worktree für Dienste. Hinweis 🧑 2026-11-15, dazu: als Agent definieren, der das ausführt.

## Ziel

Wer Sim-Code ändert, arbeitet in einem eigenen Worktree mit einer eigenen Dev-Instanz der Workbench auf eigenen Ports; ein Projekt-Agent richtet das ein und führt die Arbeit dort aus.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Regel in `docs/arbeitsweise.md` (Worktrees) und Kurzform in `CLAUDE.md`: Änderung an Sim-Code → eigener Worktree, eigene `task k3c-workbench` (dev) mit Ports nach Versatz des Worktrees; die Wurzel-Workbench bleibt unberührt. Diese Regel ist die ausdrückliche Anweisung für den Worktree.
- Ein Task startet die Dev-Workbench eines Worktrees mit berechneten Ports (MCP, UI-API, Vite) und nennt sie.
- Agent `.claude/agents/k3c-sim-dev` (schreibend): legt Worktree und Branch an, startet die Dev-Workbench dort, setzt die Sim-Änderung um, prüft gegen die eigene Instanz, stoppt sie am Ende; Sim- und Balancing-Läufe bleiben gesperrt, solange die Sperre gilt.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `docs/arbeitsweise.md`, `CLAUDE.md`, `tools/k3c-workbench/Taskfile.yml`, `tools/k3c-workbench/headless.go`, `.claude/agents/k3c-sim-dev/**`
- **Abhängig von:** B-924
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Regel steht in `docs/arbeitsweise.md` und `CLAUDE.md`.
- **AC-02** Der Task startet eine Dev-Workbench in einem Worktree auf Ports ungleich 5180/5191 (Test oder Probe).
- **AC-03** Agent `k3c-sim-dev` ist angelegt und per Smoke-Test geprüft.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
