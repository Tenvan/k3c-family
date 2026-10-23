# B-575 · Die Menü-Szenen haben UI-Klick-Töne

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** SND
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-558 (S5, KR4.1): Die Menü-Szenen bekommen UI-Klicks. Bisher gibt es keinen Klick-Ton, weil UI kein Server-Ereignis ist (Q08).

## Ziel

Auswahl und Bestätigen in den Menü-Szenen spielen einen kurzen Ton.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Ton in den Menü-Szenen (Client), kein Server-Ereignis.
- Bedienbar mit Gamepad und Tastatur; B-Taste bleibt unbelegt.
- Kandidat im Asset-Studio wählen (CC0 oder CC-BY); Slot in `data/assets.json`.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Navigieren und Bestätigen in Lobby und Spielmenü spielt je einen Ton.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus B-558 (KR4.1). Spec noch Entwurf.
