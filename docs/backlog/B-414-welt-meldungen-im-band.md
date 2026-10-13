# B-414 · Welt-Meldungen erscheinen für alle im globalen Band oben

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BD1
- **Projekt:** BED
- **Erstellt:** 2026-10-12
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit B-413 liegt oben über die ganze Breite eine gemeinsame HUD-Leiste (`BAR_HEIGHT` 134 px, `src/scenes/layout.ts`, `HudScene.ts`). Meldungen („Nacht“, Welle, Ereignisse) erscheinen weiterhin als Banner in der Mitte (`HudScene` › `banner`, Rolle `banner` 56 px, `bannerQueue`) und liegen damit über den Spielerfeldern. Ralf (Chat, 2026-10-12, beim Sprint GR7): Welt-Meldungen sollen für alle im globalen Band ganz oben erscheinen.

## Ziel

Welt-Meldungen (Ereignisse, die alle betreffen) stehen im globalen Band oben und verdecken keine Spielerfelder; die Felder behalten nur Spielerwerte.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Welt-Meldungen laufen über die Leiste statt über das Mittel-Banner; Schrift als Rolle im Katalog `src/scenes/fontRules.ts` (Pflicht-Info, ≥ 28 px, Leiste ist volle Breite).
- Mehrere Meldungen hintereinander (Warteschlange wie `bannerQueue`) bleiben lesbar, ohne die drei Zeilen der Leiste zu überdecken (Platz und Verdrängung klären).
- Funktioniert bei 1 bis 4 Spielern und mit Partner-Zelle (dort stehen keine Texte der Stufe, B-197).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Welt-Meldungen erscheinen im Band oben, nicht mehr in der Mitte; ein Test in `hudLayout.test.ts` belegt die Lage (innerhalb von `BAR_HEIGHT`, ohne Überlagerung der Leistenzeilen) für 1 bis 4 Spielern.
- **AC-02** Die Schrift der Meldung ist eine Rolle in `FONTS` und erreicht die Mindestgröße (`fontRules.test.ts`).
- **AC-03** Beobachtung am Gerät: Meldungen sind lesbar, die Spielerfelder bleiben frei.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Entstanden im Sprint GR7 (GR7.1, B-197): Die Partner-Zelle zeigt keine Texte der Stufe; Meldungen für alle gehören ins Band. Nicht Teil von GR7 (Spec Revision 1), eigener Sprint im Projekt WRT. Anzeige-Sprint W6 (aktiv) berührt dieselben Dateien: vor dem Einplanen Reihenfolge prüfen.
