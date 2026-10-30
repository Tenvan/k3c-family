# B-639 · Prompt-Buttons der Workbench schicken nur kurze Prompts ohne Anweisungen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** BS1
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑 mit BS1

## Ausgangslage

Die Prompt-Buttons der Workbench (Planung: `tools/k3c-workbench/frontend/src/planning/prompts.ts`, Git: `frontend/src/git/prompts.ts`, Teile in `PromptParts.tsx`) bauen lange Prompts mit Arbeitsanweisungen („Lies zuerst die Sprint-README … fasse in drei Sätzen zusammen … Status nur per `plan_set`“). Der Ablauf steht aber schon im Skill `k3c-session` und in `CLAUDE.md`; die Anweisungen doppeln das und veralten.

## Ziel

Ein Klick schickt einen kurzen, einfachen Prompt ohne Anweisungen, z. B. „Session BS1.1“, „Sprint BS1“, „Projekt BAS“, „Sessions BS1.1, BS1.2“, „Tickets B-634, B-635 in Sprint BS1“. Den Ablauf liefert der Skill.

## Beteiligte und Zielgruppen

🧑 (klickt), Claude (liest den Prompt).

## Anforderungen

- Alle Prompt-Erzeuger in `planning/prompts.ts` und `git/prompts.ts` liefern eine Zeile: Art + IDs (+ Ziel-Sprint bzw. Worktree, falls nötig), keine Arbeitsanweisungen.
- Reihenfolge mehrerer Sessions bleibt (nach Rang).
- Tests (`*.test.ts`) auf die kurzen Prompts umstellen.
- Der Skill `k3c-session` erkennt die Kurzform („Session <ID>“, „Sprint <ID>“, „Projekt <Kürzel>“); falls die Trigger fehlen, Beschreibung ergänzen.

## Nicht-Ziele

Neue Buttons, Layout-Änderungen.

## Regeln und Einschränkungen

Oberfläche nur über `@webkit` (Atomic Design). ⚠️ Die EXE bettet die Oberfläche ein: Nach der Änderung muss die Workbench neu gebaut/gestartet werden (`task k3c-workbench:build` bzw. `k3c-workbench:web` neu starten) – deutlich melden.

## Beispiele

Button „Session starten“ an BS1.1 → Prompt `Session BS1.1`. Auswahl BS1.1 und BS2.1 → `Sessions BS1.1, BS2.1`. Backlog B-634 → Sprint BS1 → `Tickets B-634 in Sprint BS1`. Neuer Sprint aus Tickets → `Neuer Sprint aus B-634, B-635`.

## Ausnahme- und Fehlerfälle

Sprint im Worktree → `Sprint BS1 (Worktree <name>)`.

## Akzeptanzkriterien

- **AC-01** Kein Prompt-Erzeuger in `planning/prompts.ts` und `git/prompts.ts` liefert mehr als eine Zeile oder Arbeitsanweisungen; Tests prüfen die Beispiele.
- **AC-02** Skill `k3c-session` startet mit der Kurzform richtig (Trigger ergänzt, falls nötig).
- **AC-03** Neustart der Workbench deutlich gemeldet (Ergebnis, `notify_ui`, Chat).

## Offene Fragen

keine

## Notizen

Wunsch 🧑 2026-10-26: „Prompt-Buttons in der Workbench vereinfachen, kein langer Prompt, einfache ohne Anweisungen.“
