# B-858 · Der readonly-Vitest der Workbench bleibt unter Last grün

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ52
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

WZ50.1: `tools/k3c-workbench/frontend/src/assets/readonly.test.tsx` war unter paralleler Last einmal rot („Buttons gefunden: expected 0 to be greater than 0“). Gleiche Ursache wie B-837: `flush()` reicht nicht, bis die Mock-Fixtures (lazy `import.meta.glob`) geladen sind.

## Ziel

Der readonly-Vitest wartet auf Bedingungen statt auf feste Runden und bleibt unter Last grün.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

- `until` aus `src/assets/catalog/catalogTesting.tsx` nutzen oder nach `src/assets/testing.tsx` heben.
- Keine festen Wartezeiten außer für Gegenproben.

## Nicht-Ziele

Readonly-Logik ändern.

## Regeln und Einschränkungen

Klasse A (Problem, DEV).

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/readonly.test.tsx`, `tools/k3c-workbench/frontend/src/assets/testing.tsx`, `tools/k3c-workbench/frontend/src/assets/catalog/catalogTesting.tsx`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

20 readonly-Läufe parallel zu drei vollen Vitest-Läufen → alle grün.

## Ausnahme- und Fehlerfälle

nicht relevant: Test-Stabilität.

## Akzeptanzkriterien

- **AC-01** 20 readonly-Läufe unter paralleler Last grün (Nachweis im Ergebnis).

## Offene Fragen

keine

## Notizen

Befund WZ50.1 (2026-11-14).
