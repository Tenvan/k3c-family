# B-427 · Frage: Aus welcher Quelle kommt das Mond-Bild für den Nacht-Hinweis?

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-370 (freigegeben, 2026-10-14) verlangt für den Nacht-Hinweis ein Mond-Bild „aus einem vorhandenen Pack“. BD1.4 hat alle Packs unter `public/grafik/` erneut durchgesehen (Dateinamen und Sichtprüfung der Sheets `kyrises-free-16x16-rpg-icon-pack`, `materials-pack`, `resource-icons`, `item-ruby-banana-star`, `16x16-rpg-items-db32`, `pixel-platformer`): Es gibt kein Mond-, Halbmond- oder Nacht-Symbol. Nächste Kandidaten: die Perle in `materials-pack/kandidaten/materials.png` (rund, weiß, kein Mond) und der Stern `part-star.png` (für Skillpunkte vergeben, von 🧑 als Ersatz ausgeschlossen).

## Ziel

Entscheidung, woher das Mond-Bild für `guideImage('dusk')` kommt, damit B-370 umgesetzt werden kann.

## Beteiligte und Zielgruppen

🧑 entscheidet; Agent in CLI trägt das Bild ein.

## Anforderungen

- Eine Quelle ist benannt: neues Pack (mit Lizenz-Eintrag in `docs/assets/zuordnung.md`), Eigenbau aus vorhandenen Pixeln mit Zustimmung 🧑, die Perle als Nacht-Symbol, oder Verzicht auf ein Bild.

## Nicht-Ziele

Die Umsetzung selbst (B-370); Bilder für Bauplatz und Bauer.

## Regeln und Einschränkungen

Neue Packs nur mit Lizenz-Eintrag (`docs/assets/zuordnung.md`) und Zustimmung 🧑; Pixel-Art ganzzahlig skaliert; keine selbst gemalte Grafik ohne Entscheidung 🧑 (B-370/Nicht-Ziele).

## Beispiele

Entscheidung „neues Pack X“ → B-370 bekommt eine Zeile in `docs/assets/zuordnung-welt.md`, `guideImage('dusk')` liefert das Bild. Entscheidung „kein Bild“ → B-370 wird verworfen, der Nacht-Hinweis bleibt Text.

## Ausnahme- und Fehlerfälle

nicht relevant: Das Ticket ist eine Frage.

## Akzeptanzkriterien

- **AC-01** 🧑 hat entschieden (Quelle benannt oder Verzicht); die Entscheidung steht in B-370.

## Offene Fragen

Welche Quelle? Optionen: (1) neues Pack mit Mond (z. B. von itch.io/Kenney, Lizenz prüfen), (2) Mond als kleines Eigenbau-Icon (16 × 16, wenige Pixel), (3) Perle aus `materials.png`, (4) kein Bild. Entscheidet 🧑.

## Notizen

Entstanden in BD1.4 (B-370, Schritt 3). Vorarbeit S8.2 (2026-10-09) fand ebenfalls keinen Mond.
