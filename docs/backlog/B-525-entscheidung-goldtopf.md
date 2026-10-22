# B-525 · Entscheidung: Wortlaut und Symbol des Goldtopfs in der Spielerkarte

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** –
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Spielerkarte in der HUD-Leiste (`src/scenes/HudScene.ts`, `CARD.pot`) zeigt neben dem Lebensbalken einen gezeichneten Goldtopf mit Stand, ohne Textwort. Offen aus dem POC zu B-413.

## Ziel

🧑 bestätigt oder ändert Wortlaut und Symbol des Goldtopfs.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Default (von Claude autonom gewählt, BD4): bleibt wie im Spiel, gezeichneter Topf plus Zahl, kein Textwort.
- Alternative: Münz-Symbol oder Text „Gold“.

## Nicht-Ziele

Neue Grafiken (Projekt GRA).

## Regeln und Einschränkungen

Texte zentral in `src/core/texts.*.ts`.

## Beispiele

Karte: `P1 ▬▬▬ 🍯 12`.

## Ausnahme- und Fehlerfälle

nicht relevant, reine Darstellungsfrage.

## Akzeptanzkriterien

- **AC-01** 🧑 entscheidet Default oder Alternative; bei Alternative entsteht ein Umsetzungsticket.

## Offene Fragen

Default oder Alternative?

## Notizen

Aus B-413 › Notizen (BD4, 2026-10-22). Autonom entschieden, Claude.
