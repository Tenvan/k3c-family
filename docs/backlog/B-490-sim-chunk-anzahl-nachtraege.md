# B-490 · Sim: Chunk-Anzahl mit Kandidaten-hub.json und gemeinsame Prüfung der Inselgröße

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Befunde der Umsetzung WZ17.3 (2026-10-18): (1) `room.BuildPreview` lehnt die Chunk-Anzahl zusammen mit einer Kandidaten-`hub.json` ab („Chunk-Anzahl mit Kandidaten-hub.json noch nicht möglich“), weil `sim.CreateWorldWith` keine Anzahl nimmt und `sim.Options.hub` nicht exportiert ist. (2) Die Prüfung „Anzahl ist eine Inselgröße“ steht zweimal: `sim.checkChunkCount` (nicht exportiert) und `room.checkPreviewCount`; die Meldungen sind gleich formuliert, kommen aber nicht aus einer Funktion.

## Ziel

Die Vorschau akzeptiert die Chunk-Anzahl auch mit Kandidaten-`hub.json`, und die Prüfung der Inselgröße lebt an einer Stelle (`sim.CheckChunkCount`, exportiert), die `room` nutzt.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
