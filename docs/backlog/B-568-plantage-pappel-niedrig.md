# B-568 · Plantage bekommt eine eigene niedrige Pappel vor der Farm

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-542 (D5, KR2.1): Die hohe Plantage-Pappel hinter der Farm gefällt nicht; Ralf wählt eine eigene niedrige Pappel vor der Farm. Seit GEB10 ist die Pappel so hoch wie alle Bäume, der Setzling bleibt 1 × 1 davor.

## Ziel

Die Plantage zeigt eine eigene niedrige Pappel vor der Farm.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Neue JSON und PNG für die niedrige Plantagen-Pappel in `tools/buildings` (DEV).
- Ebene in `src/scenes/render/plantation.ts` anpassen (CLI); Spielcode sonst unberührt.
- Vorschau und Spiel prüfen (Abnahme am Gerät durch 🧑).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die Plantage zeigt die niedrige Pappel vor der Farm in Vorschau und Spiel.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus B-542 (KR2.1, Alternative). Spec noch Entwurf.
