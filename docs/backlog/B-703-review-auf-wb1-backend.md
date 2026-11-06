# B-703 · Die Review-Seite nutzt backend.assets* und fillPrompt aus WB1 statt eigener Kopien

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

RV1 baut die Review-Seite auf `feature/pap` ohne WB1. Deshalb hat `tools/k3c-workbench/frontend/src/assets/review/` einen eigenen Transport (`api.ts`, Wails-Binding oder `POST /api/dev/rpc/<Methode>`), eigene Typen (`types.ts`, gleich benannt wie `api/typesAssets.ts`) und eine eigene Prompt-Füllung (`prompt.ts`, gleiche Regeln wie `assets/prompts/fill.ts`). WB1 liefert `backend.assets*`, `api/typesAssets.ts` und `fillPrompt`. B-701 D-RV1-8.

## Ziel

Nach dem Merge von WB1 und RV1 in `feature/pap` nutzt die Review-Seite `backend.assets*`, die Typen aus `api/typesAssets.ts` und `fillPrompt` aus `assets/prompts/fill.ts`; `assets/review/api.ts`, `types.ts` und die Füllfunktion in `prompt.ts` entfallen (nur `zustandVon`/`reviewWerte` bleiben).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `assets/review/` importiert keinen eigenen Transport mehr; die Tests mocken `../../api` wie die übrigen Seiten.
- **AC-02** Eine Prompt-Füllfunktion für Katalog und Review; `prompt.test.ts` und `fill.test.ts` grün.
- **AC-03** Vitest k3c-workbench und `tsc --noEmit` grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
