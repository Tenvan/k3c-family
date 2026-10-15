# B-370 · Der Hinweis „Die Nacht naht“ zeigt ein Mond-Bild vor dem Text

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** BD1
- **Projekt:** BED
- **Erstellt:** 2026-10-09
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-14, Chat 🧑 (Revision 2)

## Ausgangslage

S8.2 (B-294) zeigt vor dem Hinweis „Hinlaufen: Münze aufheben“ das Münzbild (`guideImage` in `src/scenes/guideHints.ts`). Für „Die Nacht naht! Zurück zur Burg“ gibt es unter `public/` kein Mond-Bild: `find public -iname "*moon*"` ist leer, die Sammel-Sheets `kyrises-free-16x16-rpg-icon-pack`, `16x16-rpg-items-db32`, `resource-icons` und `item-ruby-banana-star` (Stern) enthalten keinen Mond (Durchsicht S8.2, 2026-10-09).

## Ziel

Auch der Nacht-Hinweis der geführten ersten Nacht hat ein Bild vor dem Text, wie im Beschluss 🧑 2026-10-06 zu B-294.

## Beteiligte und Zielgruppen

Spielende in der ersten Nacht; 🧑 entscheidet über das Bild; Agent trägt es in CLI ein.

## Anforderungen

- `guideImage('dusk')` liefert ein Mond-Bild aus einem vorhandenen Pack unter `public/`, mit Zeile in `docs/assets/zuordnung-welt.md`.
- Findet die Session in den vorhandenen Packs kein Mond-Bild (S8.2 fand keines), nimmt sie keinen Ersatz (kein Stern) und blockiert mit einer Frage an 🧑.
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

keine. Entschieden 2026-10-14 (🧑, Chat): ein Mond-Icon aus den vorhandenen Packs; Stern (`part-star.png`) ist kein Ersatz. Hinweis: S8.2 fand in den vorhandenen Packs keinen Mond; die Session sucht erneut und fragt bei Misserfolg nach, ein neues Pack braucht Lizenz-Eintrag und Zustimmung 🧑.

## Notizen

Entschieden 2026-10-15 (🧑, Chat, B-427): Kein Pack hat einen Mond. Das Bild ist das Emoji 🌙, zur Laufzeit in die Canvas-Textur `guide:moon` gezeichnet (`guideOverlay.ts` › `ensureMoon`, Systemfont, 28 px, ×1). Ausnahme zu „aus vorhandenem Pack“ und „ganzzahlig skaliert“ (kein Asset, keine Lizenz). Zeile `guide:moon` in `docs/assets/zuordnung-welt.md`, Test in `guideHints.test.ts`; Optik am Gerät nimmt 🧑 ab.

Aus S8.2 (B-294/AC-01, Mond-Teil verschoben).
