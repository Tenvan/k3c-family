# B-838 · Regel für Sessions desselben Sprints in einer Welle ist entschieden und in Code und Doku gleich

- **Domäne:** DEV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ58
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: alle offenen angehen)

## Ausgangslage

`docs/arbeitsweise.md` › Domänen sagt: Sessions eines Sprints laufen immer nacheinander (ein Sprint-Branch). `plan_waves` (`tools/k3c-workbench/internal/planning/waves.go`, `collision()`) und `plan_next` (`next_lanes.go`, `clash()`, Kommentar dort) kennen keine Ausnahme für denselben Sprint: Zwei Sessions eines Sprints ohne Sperre und mit getrennten Erlaubten Dateien landen in derselben Welle bzw. unter „Parallel möglich“. Mit Sperre in der Sprint-README (jede Session erbt sie, `waves_nodes.go`) laufen sie nacheinander; `Abhängig von` der Sprint-README gilt für die eigenen Sessions nicht. Offene Regelfrage aus WZ44.2, benannt im Review WZ44.5.

## Ziel

Eine Regel gilt in Code und Doku gleich.

## Beteiligte und Zielgruppen

🧑 entscheidet; planende Agenten.

## Anforderungen

- Entscheidung 🧑: (a) Sessions eines Sprints immer nacheinander (Werkzeug ordnet sie in Folgewellen), oder (b) parallel erlaubt bei getrennten Dateien, dann Worktrees je Session auf Unter-Branches und Satz in `docs/arbeitsweise.md` anpassen.
- Danach Code (`waves.go`, `next_lanes.go`) oder Doku angleichen, mit Test.

## Nicht-Ziele

Andere Wellen-Regeln.

## Regeln und Einschränkungen

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `docs/arbeitsweise.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Sprint C1 mit C1.1 (`a/**`) und C1.2 (`b/**`), ohne Sperre: heute beide in Welle 1; nach (a) C1.2 in Welle 2.

## Ausnahme- und Fehlerfälle

–

## Akzeptanzkriterien

- **AC-01** Entscheidung mit Datum notiert; `plan_waves` und `plan_next` folgen ihr, Test für den Fall oben.

## Offene Fragen

keine. Entschieden (autonom Claude, 2026-11-14, Vorschlag a): Sessions desselben Sprints laufen nie parallel in einer Welle, weil ein Sprint einen Branch hat und zwei Worktrees ihn nicht zugleich auschecken.

## Notizen

Review WZ44.5 (k3c-reviewer, 2026-11-13).
