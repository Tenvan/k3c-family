# B-484 · Worktree-Vergleich: Sessions und Projekt eines neuen Sprints fehlen in den Pills

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ20
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus dem Review von WZ12.2: `diffSessions` in `wtdiff.go` läuft nur für Sprints, die es in der Wurzel schon gibt. Ein neuer Sprint aus einem Worktree erscheint daher leer (keine Sessions), ohne Projekt und ohne Eintrag in `project.sprints`; ein neues Projekt mit neuem Sprint zeigt ein leeres Projekt und daneben einen losen Sprint. B-467/AC-04 („an ihrem üblichen Ort“) ist damit für neue Sprints nur teilweise erfüllt. Vorschlag: `wtdiff.go` liefert für neue Sprints Sessions und Projekt-Zuordnung mit (B-465 berühren).

## Ziel

Ein Sprint, der nur in einem Worktree existiert, erscheint auf der Planungsseite vollständig: mit seinen Sessions, im richtigen Projekt und mit der Pill „neu“. Ein neues Projekt mit neuem Sprint zeigt den Sprint in seiner Karte statt eines leeren Projekts neben einem losen Sprint. Damit ist B-467/AC-04 („an ihrem üblichen Ort“) für neue Sprints erfüllt.

## Beteiligte und Zielgruppen

Ralf (🧑) plant mit Worktrees und entscheidet, ob der Zuschnitt in WZ12 oder B-465 gehört. Agenten legen Sprints und Sessions in Worktrees an und müssen sie in der Planungsseite wiederfinden.

## Anforderungen

- `diffData` (`wtdiff.go`) liefert für einen Sprint, der in der Wurzel fehlt, auch seine Sessions als `neu` (heute nur für Sprints, die die Wurzel schon kennt, über `diffSessions`).
- `Change` (Go, `wtdiff.go`) und `WorktreeChange` (TS, `api/types.ts`) bekommen ein optionales Feld `project` (`omitempty`), das bei einem neuen Sprint sein Projekt trägt; `withWorktreeEntries` setzt es als `PlanSprint.project` und hängt den Sprint in `project.sprints`, sofern das Projekt in der Wurzel oder ebenfalls neu existiert.
- Ein neues Projekt mit neuem Sprint zeigt den Sprint in der Projektkarte.
- Ein Sprint mit unbekanntem Projekt bleibt ein loser Sprint: `PlanSprint.project` wird dann leer gelassen (`groupProjects` blendet einen Sprint mit gesetztem, aber unbekanntem Projekt aus).
- Deterministisch: gleiche Daten ergeben gleiche Reihenfolge der Änderungen (Reihenfolge des Worktrees, wie bisher).
- Die Wurzel-Daten bleiben unverändert: `project.sprints` wird dafür kopiert, nicht in der Eingabe verändert.

## Nicht-Ziele

- Schreibende Aktionen auf Worktree-Einträgen (siehe B-483, nur lesend).
- Änderungen am Format der Worktree-Prüfung oder des Prüfwerts außerhalb der Zuordnung neuer Sprints.
- Sprints, die in der Wurzel schon existieren (laufen bereits über `diffSessions`).

## Regeln und Einschränkungen

Domäne DEV; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen (`wtdiff.go` prüfen). Planung nur über MCP; die Anzeige ergänzt nur, schreibt nichts. Vertrag: `Change` (Go) und `WorktreeChange` (TS) wachsen um das optionale Feld `project` (`omitempty`); sonst bleibt das Format. Berührt B-465 (Zuordnung der Worktree-Prüfung); vor der Umsetzung klären, ob die Arbeit dort oder in WZ12 liegt.

## Beispiele

- Worktree legt Sprint `X1` mit zwei Sessions im Projekt `WZG` an → Sprint-Karte `X1` steht unter `WZG`, beide Sessions sind darunter sichtbar, alle mit Pill „neu“.
- Worktree legt Projekt `NEU` mit Sprint `X2` an → Projektkarte `NEU` enthält `X2`, kein loser Sprint daneben.

## Ausnahme- und Fehlerfälle

- Neuer Sprint nennt ein Projekt, das weder in der Wurzel noch im Worktree existiert → Sprint erscheint als loser Sprint wie bisher, keine Ausnahme.
- Worktree mit Lesefehler (`error`) → trägt nichts bei, wie heute.
- Sprint ohne Sessions → Karte ohne Sessions, ohne Fehler.

## Akzeptanzkriterien

- **AC-01** Go-Test in `tools/k3c-dev/internal/planning` (`wtdiff`): Ein Sprint, der nur im Worktree existiert, liefert seine Sessions als `neu`-Änderungen und sein Projekt im Feld `project` (Beispiel: zwei Sessions → zwei Änderungen).
- **AC-02** Vitest in `tools/k3c-dev/frontend/src/planning` (`wtPills`): `withWorktreeEntries` hängt einen neuen Sprint in `project.sprints` seines Projekts und seine Sessions an den Sprint; die Eingabedaten bleiben unverändert (Regressionstest für den Session-Teil, den der Client schon kann).
- **AC-03** Vitest über `groupProjects`: neues Projekt mit neuem Sprint ergibt eine Projektkarte mit dem Sprint und keinen losen Sprint.
- **AC-04** Vitest über `groupProjects`: neuer Sprint mit unbekanntem Projekt erscheint als loser Sprint, nicht ausgeblendet, ohne Fehler.
- **AC-05** `check_run` `task:check`, `task:check:go` und `dev:test` grün.

## Offene Fragen

1. Gehört die Arbeit in WZ12 (Nachtrag) oder in B-465, weil sie die Zuordnung der Worktree-Prüfung berührt? Entscheidet Ralf (🧑).
2. Annahme bis zur Antwort: eigenes Ticket, Umsetzung später im selben Sprint wie B-483.

## Notizen

Quelle: Review von WZ12.2. Code: `tools/k3c-dev/internal/planning/wtdiff.go` (`diffData`, `diffSessions`), `tools/k3c-dev/frontend/src/planning/wtPills.ts` (`withWorktreeEntries`). Verwandt: B-467, B-465, B-483.
