# B-487 · Chunk-Editor: abgelehnte Aufrufe von planLoad und planCheck bleiben stumm

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ22
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Befund des Reviews zu B-486 (2026-10-18): In `tools/k3c-studio/frontend/src/world/usePlan.ts` bleibt eine Ablehnung von `backend.planLoad` (`void load()`) und `backend.planCheck` (`.then` ohne `.catch`) unbehandelt. Beim Laden bleibt der Editor ohne Meldung leer, bei der Prüfung zeigt die Oberfläche still den alten Stand. B-486 hat dieselbe Fehlerklasse nur für `planSave` behoben.

## Ziel

Jede abgelehnte Bindung des Chunk-Editors zeigt ihren Grund am Editor: Laden als Meldung, Prüfung als Fehler am Entwurf; der Entwurf bleibt erhalten.

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

- **AC-01** Frontend-Test: Lehnt `planLoad` ab, steht „Laden fehlgeschlagen: <Grund>“ am Editor.
- **AC-02** Frontend-Test: Lehnt `planCheck` ab, ist Speichern gesperrt und der Grund steht am Editor; das letzte gültige Bild bleibt.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
