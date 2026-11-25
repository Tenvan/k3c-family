# B-1284 · ESLint-Befunde in Spieltest-Seite und Welten-Editor machen task check auf main rot

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ65
- **Projekt:** WZG
- **Erstellt:** 2026-11-25
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`task check` (ESLint) ist auf main rot, Tests grün (2205). 15 Befunde `complexity`/`max-lines-per-function`: Workbench `tools/k3c-workbench/frontend/src/playtest/` (TimelinePanel.tsx 29/177, SessionDetail.tsx 21, RecordBar.tsx 26, PlaytestPage.tsx 19, PlaytestPage.test.tsx 45/239, mock.ts 32, FindingsPanel.tsx 70, DescribeBar.tsx 55; Spieltest #850, WZ64–WZ66) und Studio `tools/k3c-studio/frontend/src/world/useWorld.ts:244`, `WorldPage.tsx:62` (SI7.1 #861, WE6.4 #865). Gefunden in RU40.5.

## Ziel

`task check` ist auf main wieder grün, damit Review-Sessions ihn als Nachweis nutzen können.

## Beteiligte und Zielgruppen

Agenten der Review-Sessions; Ralf merged.

## Anforderungen

- Die genannten Funktionen in kleinere Komponenten oder Hooks teilen (Komplexität ≤ 15, ≤ 60 Zeilen), Verhalten gleich.
- Atomic Design der Wails-Frontends einhalten (`docs/arbeitsweise.md` › Oberflächen der Wails-Programme).

## Nicht-Ziele

Neue Funktionen der Spieltest-Seite oder des Welten-Editors; Lockern der ESLint-Grenzen.

## Regeln und Einschränkungen

Klasse A (Schuld, DEV, ohne Spielregeln). Komplexitäts-Budget aus `docs/arbeitsweise.md`.

- **Bereich:** `tools/k3c-workbench/frontend/src/playtest/**`, `tools/k3c-studio/frontend/src/world/useWorld.ts`, `tools/k3c-studio/frontend/src/world/WorldPage.tsx`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`check_run task:lint` → 0 Befunde.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Umstrukturierung.

## Akzeptanzkriterien

- **AC-01** `check_run task:check` grün (Lint, Typecheck, Tests).
- **AC-02** Vitest der Workbench und des Studios (`check_run task:check:dev`) grün, Verhalten der Seiten unverändert.

## Offene Fragen

keine

## Notizen

Umgesetzt in RU40.5 (Branch sprint/ru40), getrennter Commit: Playtest-Komponenten in Teilkomponenten und Hooks zerlegt (neu `TimelineEntries.tsx`, `useTimeline.ts`, `useRecorder.ts`); `mock.ts` aus Teilfunktionen über einen gemeinsamen Zustand; im Studio `useSetValue` und `WorldTitle`/`NOTICE_COLOR`.
- AC-01: `check_run task:check` grün (Lint, Typecheck, Tests).
- AC-02: Vitest grün (Workbench 347, Studio 319).
- Reviewer: keine Verhaltensänderung.
- Nicht im Browser geprüft. Stichprobe für 🧑: Spieltest-Seite (Aufnahmeleiste, Auswerten, Zeitleiste) und Kopfzeile des Welten-Editors.
