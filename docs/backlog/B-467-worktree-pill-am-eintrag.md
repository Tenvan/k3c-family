# B-467 · Worktree-Änderungen stehen als Pill am Projekt, Sprint oder an der Session statt als Leiste oben

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ12
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Wunsch von 🧑 (2026-10-17): Auf der Planungsseite der Workbench zeigt `WorktreeBar` (`tools/k3c-dev/frontend/src/planning/WorktreeBar.tsx`, eingebunden in `SprintsBacklog.tsx`) oben unter „Worktrees“ je Worktree eine Zeile mit allen Änderungen als Chips (`+ GEB6.1 …`, `~ WZ6 · Status geplant → aktiv`). Bei mehreren Worktrees wird das eine lange, unübersichtliche Menge.

## Ziel

Die Leiste oben entfällt. Jede Änderung aus einem Worktree erscheint am betroffenen Eintrag (Projekt, Sprint, Session, Ticket) als kleine Worktree-Pill; der Tooltip nennt Worktree/Branch und die Änderung (z. B. „sprint/wz6: Status offen → in Arbeit“).

## Beteiligte und Zielgruppen

🧑 liest die Planungsseite mit mehreren parallelen Worktrees.

## Anforderungen

- Daten aus dem bestehenden Workflow-Check (`WorktreeCheck`, `internal/planning/wtdiff.go`), kein neues Backend-Format, wenn vermeidbar.
- Pill im Stil der vorhandenen `Pill`-Bausteine, Tooltip über `title`/`Tip`.

## Nicht-Ziele

Änderungen am Vergleich selbst (Merge-Base, `wtdiff`); Lint-Befunde dort (B-465).

## Regeln und Einschränkungen

Domäne DEV; Funktion ≤ 60, Datei ≤ 400 Zeilen.

## Beispiele

Worktree `sprint/wz6` setzt WZ6.2 auf „in Arbeit“ → an der Session WZ6.2 steht eine Pill, Tooltip „sprint/wz6: Status offen → in Arbeit“.

## Ausnahme- und Fehlerfälle

Workflow-Check scheitert → keine Pills, Hinweis wie heute.

## Akzeptanzkriterien

- **AC-01** Die Planungsseite zeigt keine Worktree-Leiste mit Änderungs-Chips mehr.
- **AC-02** Ein Projekt, ein Sprint, eine Session oder ein Ticket mit Änderung in einem Worktree trägt eine Worktree-Pill; der Tooltip nennt Branch und Änderung(en), mehrere Worktrees ergeben mehrere Pills oder einen Tooltip mit allen.
- **AC-03** Neu im Worktree angelegte Einträge (`+`), die es im Checkout noch nicht gibt, bleiben sichtbar (Ort im Ticket klären).
- **AC-04** Komponententest in `task test`, `task check:dev` grün.

## Offene Fragen

Wo stehen Einträge, die nur im Worktree existieren (neues Ticket, neue Session)? Entscheidet 🧑.

## Notizen

Aufgenommen während WZ6.2 (Screenshot der Planungsseite, 2026-10-17).
