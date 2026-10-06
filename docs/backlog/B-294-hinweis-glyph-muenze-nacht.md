# B-294 · Münze und „Nacht naht“ zeigen in der geführten ersten Nacht keine Glyph

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** S8
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Spec-Beispiel von S6 („Hinweis ‚Aufheben‘ mit Glyph ‚A‘ über der Münze“) passt nicht zur Mechanik: Münzen werden beim Drüberlaufen aufgehoben (`collectCoins`), es gibt keine Taste. S6.3 zeigt für `coin` und `dusk` daher Text ohne Glyph (`src/scenes/guideOverlay.ts`, `guide.coin`, `guide.dusk`); nur Bauplatz und Bauer tragen die Bestätigen-Glyph.

## Ziel

Entschieden ist, ob Münze und „Nacht naht“ ohne Glyph richtig sind oder ob ein Bild (z. B. Münzsymbol, Mond) gewünscht ist.

## Beteiligte und Zielgruppen

🧑 entscheidet; Kinder am TV lesen die Hinweise.

## Anforderungen

- Entscheidung von 🧑 (bei S6.4 am TV), danach ggf. Umsetzung in der Domäne CLI.

## Nicht-Ziele

Neue Tastenbelegung; die Spec von S6 wird nicht umgeschrieben.

## Regeln und Einschränkungen

`CLAUDE.md`: Client zeichnet nur; B und View ohne Glyph; Glyphen selbst gezeichnet.

## Beispiele

Münze liegt vor dem Monarchen → heute Text „Münze einsammeln“ ohne Glyph.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Darstellungsfrage.

## Akzeptanzkriterien

- **AC-01** 🧑 hat entschieden (Text genügt oder Bild gewünscht); bei „Bild“ ist ein Folge-Ticket angelegt.

## Offene Fragen

Genügt Text ohne Glyph für Münze und „Nacht naht“? Entscheider: 🧑.

## Notizen

Herkunft: S6.3 › Ergebnis, Review S6.5.