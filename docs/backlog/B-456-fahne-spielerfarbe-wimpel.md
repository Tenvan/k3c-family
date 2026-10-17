# B-456 · Markierungsfahne als Wimpel in Spielerfarbe

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Markierungsfahnen über Knoten (`worldRenderer.ts`) sind heute neutral. Ralf (2026-10-17): in Spielerfarbe und als Dreieck/Wimpel.

## Ziel

Die Fahne ist ein Wimpel in der Farbe des markierenden Spielers; mit zwei Spielern unterscheidbar.

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

- **AC-01** Wimpel-Form, Farbe je Spieler (aus der vorhandenen Spielerfarbe, keine neue Farbliste), Vitest für die Farbwahl.
- **AC-02** Sitzt über Baum, Fels und Ader; zwei Spieler gleichzeitig unterscheidbar (🧑 prüft).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
