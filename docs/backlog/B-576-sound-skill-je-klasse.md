# B-576 · Jede Klasse hat einen eigenen Skill-Sound

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** SO7
- **Projekt:** SND
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-558 (S5, KR4.1): Statt des gemeinsamen `sfx:skill` bekommt jede Klasse einen eigenen Skill-Sound.

## Ziel

Der Skill jeder Klasse klingt eigen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Ein Slot je Klasse in `data/assets.json`, Auslöser wie bisher `skill`.
- Kandidaten je Klasse im Asset-Studio wählen (CC0 oder CC-BY).
- Gemeinsamer `sfx:skill` bleibt als Rückfall; `task gen:soundatlas` nach der Slot-Änderung.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Jede Klasse spielt beim Skill ihren eigenen Sound; fehlt einer, greift der gemeinsame.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus B-558 (KR4.1). Spec noch Entwurf.
