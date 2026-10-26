# BS1.3 · Prompt-Buttons: kurze Prompts ohne Anweisungen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bs1/3-prompt-buttons
- **Abhängig von:** –
- **Tickets:** B-639
- **Kriterien:** AC-07, AC-08

## Ziel

Die Prompt-Buttons der Workbench schicken nur noch kurze Prompts ohne Anweisungen, z. B. `Session BS1.1`.

## Kontext

- Prompt-Erzeuger: `tools/k3c-workbench/frontend/src/planning/prompts.ts` (`promptSprint`, `promptProject`, `promptSession`, `promptSessions`, `backlogPrompt`), `frontend/src/git/prompts.ts`, Anzeige `planning/PromptParts.tsx`; Tests daneben (`*.test.ts`).
- Den Ablauf liefert der Skill `k3c-session` (`.claude/skills/k3c-session/`); Kurzformen müssen dort als Trigger stehen.
- Spec: B-639 (Beispiele dort).
- ⚠️ Die EXE bettet die Oberfläche ein → Workbench neu bauen/starten.

## Erlaubte Dateien

- `tools/k3c-workbench/frontend/src/planning/prompts.ts`, `planning/PromptParts.tsx`, `planning/*.test.ts(x)`
- `tools/k3c-workbench/frontend/src/git/prompts.ts`, `git/*.test.ts(x)`
- `.claude/skills/k3c-session/SKILL.md` (nur Trigger/Beschreibung)

## Nicht-Ziele

Neue Buttons, Layout.

## Schritte

1. Jeden Erzeuger auf eine Zeile ohne Anweisungen umstellen (Beispiele aus B-639).
2. Tests anpassen.
3. Skill-Trigger für die Kurzformen prüfen/ergänzen.
4. `check_run task:check:diff` (inkl. `dev:test`), Commit.
5. **⚠️ Workbench-Neustart nötig** melden: Ergebnis Zeile 1, `notify_ui` (warn), Chat mit Befehl.

## Fertig, wenn

- [ ] AC-07: Tests prüfen die kurzen Prompts aus B-639, keine Anweisungen mehr.
- [ ] AC-08: Skill erkennt die Kurzform; Neustart gemeldet.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
