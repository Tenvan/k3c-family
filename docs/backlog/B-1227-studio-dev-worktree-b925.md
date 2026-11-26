# B-1227 · Studio rechnet die Engine: Dev-Worktree-Instanz und Regel B-925 für das Studio

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WN3
- **Projekt:** WZN
- **Erstellt:** 2026-11-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Seit WZ63.5 rechnet das Studio (`k3c-studio`) die Engine im Prozess (level_generate, sim_run, replay_run, sim_test), nicht mehr die Workbench. Die Regeln B-924/B-925 in CLAUDE.md, in den Agenten `k3c-sim-dev` und `k3c-umsetzer` sowie in `.claude/agents` nennen noch „die Workbench“ und `task k3c-workbench:dev:worktree`. Das Studio hat kein `dev:worktree`; sein Dienst kennt nur einen Port (MCP), der UI-API-Port folgt dem MCP-Port (8193 + Versatz). Der Versatz 10 landet für den MCP-Port auf 5191 = Vite-Port der Workbench.

## Ziel

Sim-Code lässt sich im Worktree mit eigener Studio-Instanz auf eigenen Ports bearbeiten; die Regeln nennen das Studio statt der Workbench.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Siehe Akzeptanzkriterien.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `task k3c-studio:dev:worktree` startet eine Studio-Dev-Instanz auf eigenen, kollisionsfreien Ports (MCP, UI-API, Vite) analog `k3c-workbench:dev:worktree`.
- **AC-02** Port-Schema ohne Kollision zwischen Workbench- und Studio-Worktree-Diensten (5181+10 darf nicht 5191 sein).
- **AC-03** CLAUDE.md (B-925), `k3c-sim-dev`, `k3c-umsetzer`, `k3c-explorer` nennen das Studio als Rechenort der Engine.
- **AC-04** Die Workbench-instructions nennen sich nicht mehr zuständig für Balancing/Simulation.

## Offene Fragen

Entscheidung 🧑 nur zum Port-Schema (Versatz je App oder getrennte Port-Blöcke).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
