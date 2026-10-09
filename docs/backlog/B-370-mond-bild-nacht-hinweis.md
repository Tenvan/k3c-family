# B-370 · Der Hinweis „Die Nacht naht“ zeigt ein Mond-Bild vor dem Text

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

S8.2 (B-294) zeigt vor dem Hinweis „Hinlaufen: Münze aufheben“ das Münzbild (`guideImage` in `src/scenes/guideHints.ts`). Für „Die Nacht naht! Zurück zur Burg“ gibt es unter `public/` kein Mond-Bild: `find public -iname "*moon*"` ist leer, die Sammel-Sheets `kyrises-free-16x16-rpg-icon-pack`, `16x16-rpg-items-db32`, `resource-icons` und `item-ruby-banana-star` (Stern) enthalten keinen Mond (Durchsicht S8.2, 2026-10-09).

## Ziel

Auch der Nacht-Hinweis der geführten ersten Nacht hat ein Bild vor dem Text, wie im Beschluss 🧑 2026-10-06 zu B-294.

## Beteiligte und Zielgruppen

Spielende in der ersten Nacht; 🧑 entscheidet über das Bild; Agent trägt es in CLI ein.

## Anforderungen

- `guideImage('dusk')` liefert ein Bild aus einem Pack unter `public/`, mit Zeile in `docs/assets/zuordnung-welt.md`.
- Pixel-Art ganzzahlig skaliert, wie das Münzbild (`RowImage` in `src/scenes/glyphView.ts`).

## Nicht-Ziele

Selbst gemalte Grafik ohne Entscheidung 🧑; Bilder für Bauplatz und Bauer.

## Regeln und Einschränkungen

CLI zeichnet nur; neue Packs nur mit Lizenz-Eintrag in `docs/assets/zuordnung.md`.

## Beispiele

Dämmerung in der ersten Nacht → über dem Monarchen Mond-Bild + „Die Nacht naht! Zurück zur Burg“.

## Ausnahme- und Fehlerfälle

Textur nicht geladen → Hinweis ohne Bild (nur Text).

## Akzeptanzkriterien

- **AC-01** `task test -- guideHints`: `guideImage('dusk')` liefert das gewählte Bild; Zuordnung steht in `docs/assets/zuordnung-welt.md`.

## Offene Fragen

Welches Bild statt Mond (🧑): ein vorhandenes Pack-Bild als Ersatz (z. B. Stern `props/part-star.png` aus `item-ruby-banana-star`, schon für Skillpunkte vergeben) oder ein neues Pack mit Mond. Blockierend.

## Notizen

Aus S8.2 (B-294/AC-01, Mond-Teil verschoben).
