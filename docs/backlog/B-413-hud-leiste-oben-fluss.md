# B-413 · Gemeinsame HUD-Leiste oben für alle Spieler, Kompass oben, Fluss mit Spiegelung unten

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** BD4
- **Projekt:** BED
- **Erstellt:** 2026-10-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Vorrat, Hub, Tageszeit und Kampf standen oben rechts bzw. bei 3–4 Spielern mittig am Kreuzpunkt der Spielerfelder (B-084), der Kompass (Radar) unten in jedem Feld. Die Werte gelten für alle Spieler, belegen aber Platz in den Spielerfeldern. Unten ist der Boden eine einfarbige Fläche.

## Ziel

Eine Leiste oben über die ganze Breite zeigt die Werte für alle Spieler (Vorrat und Hub, Tageszeit, Kampf). Die Spielerfelder behalten nur Spielerwerte. Der Kompass wandert nach oben. Unten liegt ein Fluss mit gespiegeltem Parallax-Hintergrund wie in K2C.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Bei 1 bis 4 Spielern stehen Vorrat/Hub, Tageszeit und Kampf nur in der Leiste oben (`hudLayout.test.ts`), nie in einem Spielerfeld.
- **AC-02** Der Kompass steht je Feld oben, im obersten Feld unter der Leiste, ohne Überlagerung (`hudLayout.test.ts`).
- **AC-03** Unter der Bodenlinie liegt ein Fluss mit kopfüber gespiegeltem Hintergrund und strömenden Wellen; Beobachtung am Gerät.
- **AC-04** Die Leiste weicht dem Home-Button und den Touch-Flächen aus (`hudLayout.test.ts`).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

POC auf Branch `poc/hud-leiste-fluss` (2026-10-11), Stand nach Rückmeldungen 🧑: Leiste oben dunkelblau, drei Zeilen (`BAR_HEIGHT` 134 px): Raum · Tageszeit · Kampf / Spielerkarten / Vorrat, Hub, Bürger; rechts die Orts-Tabelle (ein Spieler je Zeile). Spielerkarte: Name, Lebensbalken, Goldtopf mit Stand (kein Text „Gold/HP/Ort“ mehr). Die Felder beginnen unter der Leiste, der Himmel entfällt (`WORLD_TOP` 420, `viewZoom`). Welt-Schrift (Preisschild, Ausgang, Bürger, Münzbeutel) erscheint durch Zoom-Ausgleich (`StageView.scaleTexts`) in 32 px statt 56 px × Zoom. Fluss in `stageView.ts` (`addRiver`): nur Hintergrund gespiegelt. Offen für 🧑: Wortlaut und Symbol des Goldtopfs; Hintergrund-Höhe je Biom gegen `WORLD_TOP` prüfen; Spiegelung der Figuren.
