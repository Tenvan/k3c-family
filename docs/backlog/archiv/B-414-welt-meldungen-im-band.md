# B-414 · Welt-Meldungen erscheinen für alle im globalen Band oben

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** BD1
- **Projekt:** BED
- **Erstellt:** 2026-10-12
- **Spec:** freigegeben
- **Revision:** 4
- **Freigabe:** 2026-10-14, Chat 🧑 (Revision 4)

## Ausgangslage

Seit B-413 liegt oben über die ganze Breite eine gemeinsame HUD-Leiste (`BAR_HEIGHT` 134 px, `src/scenes/layout.ts`, `HudScene.ts`). Meldungen („Nacht“, Welle, Ereignisse) erscheinen weiterhin als Banner in der Mitte (`HudScene` › `banner`, Rolle `banner` 56 px, `bannerQueue`) und liegen damit über den Spielerfeldern. Ralf (Chat, 2026-10-12, beim Sprint GR7): Welt-Meldungen sollen für alle im globalen Band ganz oben erscheinen.

## Ziel

Welt-Meldungen (Ereignisse, die alle betreffen) stehen im globalen Band oben und verdecken keine Spielerfelder; die Felder behalten nur Spielerwerte.

## Beteiligte und Zielgruppen

Spielende am PC und an der Xbox (höchstens 2 Spieler), die Welt-Meldungen lesen; ein Agent baut in `src/scenes/`; 🧑 nimmt am Gerät ab.

## Anforderungen

- Welt-Meldungen laufen über die Leiste statt über das Mittel-Banner; Schrift als Rolle im Katalog `src/scenes/fontRules.ts` (Pflicht-Info, ≥ 28 px, Leiste ist volle Breite).
- Mehrere Meldungen hintereinander (Warteschlange wie `bannerQueue`, `HudScene.showBanner`) bleiben lesbar, ohne die drei Zeilen der Leiste zu überdecken (Platz und Verdrängung klären).
- Funktioniert bei 1 und 2 Spielern und mit Partner-Zelle (dort stehen keine Texte der Stufe, B-197).

## Nicht-Ziele

Neue Meldungsarten oder neue Server-Ereignisse; Änderung der Meldungstexte; persönliche Hinweise einzelner Spieler (Führungs-Hinweise über der Figur, B-294); Änderung der Leistenzeilen selbst (B-413); Touch, Handy und 3–4 Spieler (entfallen seit 2026-10-11).

## Regeln und Einschränkungen

Domäne CLI; der Client zeichnet nur Server-Zustand (`src/scenes/noSim.test.ts`). Texte zentral in `src/core/texts.de.ts`. Mindest-Schriftgrößen je Katalog `fontRules.ts` (B-136). Keine Änderung an Protokoll oder Server. B nicht belegen, View + Menu reserviert. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

- Die Nacht beginnt, 2 Spieler im Split-Screen → „Nacht“ steht für beide im Band oben über die volle Breite, in der Mitte und über den Spielerfeldern erscheint nichts.
- Eine Welle endet kurz nach „Nacht“ → die zweite Meldung wartet in der Schlange und erscheint nach der ersten; die drei Zeilen der Leiste bleiben sichtbar.
- 1 Spieler mit Partner-Zelle → die Meldung steht im Band; die Partner-Zelle zeigt keinen Text der Stufe.

## Ausnahme- und Fehlerfälle

- Mehr als 2 Meldungen warten → die Anzeigedauer verkürzt sich wie bisher (`HudScene.showBanner`: halbe Dauer bei Warteschlange > 2); es geht keine Meldung verloren, eine neue ersetzt die laufende nicht.
- Sehr langer Text → Umbruch innerhalb der Leistenhöhe; passt er nicht, wird er mit „…“ gekürzt. Er läuft nie über den Leistenrand und überdeckt keine Zeile.
- Szenenneustart oder Raumwechsel → die Schlange wird geleert (wie bisher, `bannerQueue = []`), keine Meldung von der alten Stufe.
- Mehrere lokale Spieler lösen dieselbe Meldung aus → sie erscheint einmal, nicht je Spieler.

## Akzeptanzkriterien

- **AC-01** Welt-Meldungen erscheinen im Band oben, nicht mehr in der Mitte; ein Test in `hudLayout.test.ts` belegt die Lage (innerhalb von `BAR_HEIGHT`, ohne Überlagerung der Leistenzeilen) für 1 und 2 Spieler.
- **AC-02** Die Schrift der Meldung ist eine Rolle in `FONTS` und erreicht die Mindestgröße (`fontRules.test.ts`).
- **AC-03** Beobachtung am Gerät: Meldungen sind lesbar, die Spielerfelder bleiben frei.

## Offene Fragen

keine. Entschieden 2026-10-14 (🧑, Chat; die Punkte 2 und 3 überließ 🧑 dem Agenten, Vorschläge übernommen):
- Projekt BED statt WRT bestätigt.
- Verdrängung: Warteschlange wie bisher, eine neue Meldung ersetzt die laufende nicht.
- Langer Text: Umbruch innerhalb der Leistenhöhe, passt er nicht, Kürzung mit „…“.

## Notizen

Entstanden im Sprint GR7 (GR7.1, B-197): Die Partner-Zelle zeigt keine Texte der Stufe; Meldungen für alle gehören ins Band. Nicht Teil von GR7 (Spec Revision 1). Eingeplant im Sprint BD1 (Projekt BED, bestätigt 2026-10-14). Anzeige-Sprint W6 berührt dieselben Dateien (`HudScene.ts`, `layout.ts`): vor der Umsetzung Reihenfolge prüfen.
