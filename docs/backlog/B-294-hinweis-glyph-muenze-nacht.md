# B-294 · Münze und „Nacht naht“ zeigen in der geführten ersten Nacht keine Glyph

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** S8
- **Projekt:** –
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

Entschieden 2026-10-06 (🧑, Chat): Bild statt Glyph – Münzsymbol bzw. Mond vor dem Text, aus vorhandenen Packs unter `public/` (Zuordnung in `docs/assets/`); umgesetzt in S8.2.

## Notizen

Herkunft: S6.3 › Ergebnis, Review S6.5.