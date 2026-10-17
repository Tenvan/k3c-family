# B-465 · task lint ist auf develop rot, weil SprintsBacklog 61 Zeilen hat

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`task lint` (oxlint 1.86.0) scheitert auf `origin/develop` (`1328fca7`) mit `eslint(max-lines-per-function)`: Die Funktion `SprintsBacklog` in `tools/k3c-dev/frontend/src/planning/SprintsBacklog.tsx:26` hat 61 Zeilen, erlaubt sind 60. Die Datei kam zuletzt mit #329 (Workflow-Check für Worktree-Planung). Damit ist `task check` in jedem Checkout rot, auch bei reinen Doku-Änderungen (gefunden in WZ6.1). Typecheck und Tests sind grün.

## Ziel

`task lint` und damit `task check` sind auf `develop` wieder grün; `SprintsBacklog` bleibt in Verhalten und Aussehen gleich (Teil der Funktion herausziehen, Regel nicht abschwächen).

## Beteiligte und Zielgruppen

Agenten und 🧑, deren `task check` an dem Fehler scheitert; CI auf `develop`.

## Anforderungen

- `SprintsBacklog` unter 60 Zeilen, z. B. Kopfleiste oder Spaltenbereich als eigene Komponente.
- Keine Verhaltensänderung, keine Lint-Ausnahme (`oxlint-disable`).

## Nicht-Ziele

Die Warnungen in `src/tools/` (`no-unused-expressions`, `no-useless-spread`); neue Funktionen der Planungsseite (B-400, B-401).

## Regeln und Einschränkungen

Domäne DEV; Funktion ≤ 60, Datei ≤ 400 Zeilen (`CLAUDE.md`).

## Beispiele

Reine Doku-Änderung, `task check` → grün.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Umstrukturierung ohne neues Verhalten.

## Akzeptanzkriterien

- **AC-01** `task lint` meldet keinen Fehler; `max-lines-per-function` bleibt bei 60.
- **AC-02** `task check:dev` grün; die Planungsseite verhält sich unverändert (bestehende Frontend-Tests).

## Offene Fragen

keine

## Notizen

Gefunden in WZ6.1 (2026-10-17), oxlint 1.86.0.
