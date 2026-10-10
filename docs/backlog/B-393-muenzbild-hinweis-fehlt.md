# B-393 · Das Münzbild vor „Hinlaufen: Münze aufheben“ erscheint im Browser nicht

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-10
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Browser-Abnahme S8.4 (🧑, 2026-10-10): Beim ersten Münz-Hinweis der geführten ersten Nacht war kein Münzbild zu sehen. S8.2 (B-294) belegt es nur per Test (`guideImage('coin')` → `grafik:coinIcon`, `src/scenes/guideHints.ts`, Textur in `src/scenes/worldSprites.ts`, Zeichnen in `src/scenes/guideOverlay.ts`/`glyphView.ts`). Ob der Hinweis selbst erschien und nur das Bild fehlte, ist offen; ursächlich möglich: Textur nicht geladen (Ausnahmefall „nur Text“) oder `RowImage` wird nicht gezeichnet.

## Ziel

Das Münzbild steht sichtbar vor dem Hinweistext „Hinlaufen: Münze aufheben“ (B-294/AC-01, S8/AC-03).

## Beteiligte und Zielgruppen

Spielende in der ersten Nacht; 🧑 hat den Fehler in der Abnahme gemeldet.

## Anforderungen

- Im Browser erscheint das Bild vor dem Text, mit 1 und 2 Spielern; ein Test oder Log belegt, dass die Textur geladen und gezeichnet wird.

## Nicht-Ziele

Mond-Bild für „Die Nacht naht“ (B-370).

## Regeln und Einschränkungen

CLI zeichnet nur Snapshots; Pixel-Art ganzzahlig skaliert; Komplexitäts-Budget.

## Beispiele

Neuer Spielstand, erste Münze in Sicht → über der Münze steht Münz-Icon + „Hinlaufen: Münze aufheben“.

## Ausnahme- und Fehlerfälle

Textur nicht geladen → Hinweis nur mit Text (bisher gewollt, ohne Meldung; ein Log 🐛 wäre hilfreich).

## Akzeptanzkriterien

- **AC-01** Im Browser (`game.html`, neuer Spielstand) zeigt der Münz-Hinweis das Münzbild vor dem Text, mit 1 und 2 Spielern.

## Offene Fragen

Erschien der Hinweistext selbst? Möglich: Er war schon als gesehen markiert (`guideSeen`, Browser-Speicher) und kam deshalb gar nicht (🧑 prüft mit frischem Profil oder `?`-Reset).

## Notizen

Aus S8.4 (Browser-Abnahme).
