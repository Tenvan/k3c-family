# GR7.2 · Sattelsitz des Reiters beim Laufen und Sprinten

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** gr7/2-sattelsitz
- **Abhängig von:** –
- **Tickets:** B-320
- **Kriterien:** AC-05

## Ziel

Die Sattelpunkte von `horse` und `blackHorse` in `data/sprites.json` stimmen für `run` mit dem Sattel im Bild überein, ein Test belegt den Abstand zum `idle`-Punkt (≤ 4 px im Sheet); der Reiter bleibt beim Wechsel Stehen → Laufen → Sprint über dem Sattel.

## Kontext

- **Befund B-320** (Abnahme S7.3, 🧑 am PC): Im Stand sitzt der Monarch mittig, beim Laufen und Sprinten hinten auf der Kruppe.
- **Daten:** `data/sprites.json › mounts.horse.saddle` (`blackHorse` nutzt dieselben Punkte): `idle: [[49,26]]`, `run`: 11 Punkte mit x = 38 bis 42 und y = 21 bis 29. Sheet `horse` hat `frameWidth` 100, `frameHeight` 65, `originX` 0.55, `originY` 0.969, `scale` 2.2; `idle` 1 Frame, `run` 11 Frames; Bilder `public/sprites/horse/idle.png` und `run.png`. Der Unterschied x 49 gegen x ≈ 40 sind ~9 Sheet-Pixel, ×2,2 rund 20 px auf dem Bildschirm.
- **Lage-Rechnung:** `src/scenes/render/mountPose.ts › saddleOf` und `mountPose` (`sx = (sattel.x − originX × frameWidth) × scale`); die Rechnung ist durch `mountPose.test.ts` abgedeckt, nur die Datenwerte sind verdächtig (ungeprüft). Zu klären ist: stimmen die Run-Punkte nicht, oder sitzt der Pferdekörper in den Lauf-Frames tatsächlich weiter links im Frame (anderer Ursprung zwischen Steh- und Lauf-Frame)?
- **Prüfen am Bild:** Pferdekörper je Frame aus `idle.png` und `run.png` auslesen (nicht transparente Pixel, Rumpf-Mitte oberhalb der Beine, z. B. mit Python und Pillow oder einem kleinen Skript im Scratchpad), daraus Sattelpunkte je Frame bestimmen und mit den Daten vergleichen. Das Bild ansehen (Read-Tool auf die PNG) ist erlaubt.
- **Test-Idee (AC-05/B-320 AC-01):** Test liest `data/sprites.json`, prüft für `horse`: `|run[i][0] − idle[0][0]| ≤ 4` und `|run[i][1] − idle[0][1]| ≤ ~6` je Frame, mit Begründung für jede Ausnahme im Test (Kommentar), sofern das Bild den Sattel dort hat.
- Andere Reittiere sind kein Ziel, nur `horse` und `blackHorse`; `horseWalk` hat eigene, stimmige Werte (x 27 bis 28).

## Erlaubte Dateien

- `data/sprites.json` (nur `mounts.horse`, `mounts.blackHorse`, bei Bedarf `sheets.horse`)
- `src/scenes/render/mountPose.ts`, `src/scenes/render/mountPose.test.ts`, bei Bedarf neuer Test `src/scenes/mountData.test.ts`
- `docs/sprints/geplant/GR7-figuren-schrift/`, `docs/sprints/aktiv/GR7-figuren-schrift/` (nur Status), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Beine des Reiters (nur Oberkörper, so in S7.2), andere Reittiere, neue Animationen, Sichtprüfung am PC (🧑 sieht Stehen, Laufen, Sprinten, B-320/AC-02), Server und Protokoll.

## Schritte

1. Auf dem Sprint-Branch `sprint/gr7` `Status: in Arbeit` setzen, pushen. `docs/arbeitsweise.md`, `docs/glossar.md` lesen.
2. Ursache klären: Pferdekörper in `public/sprites/horse/idle.png` und `run.png` je Frame vermessen, mit den Datenpunkten vergleichen; Befund (falsche Daten oder anderer Ursprung) im Ergebnis festhalten.
3. Test zuerst: Abstand der `run`-Sattelpunkte zum `idle`-Punkt ≤ 4 px (x), Ausnahmen mit Begründung; er muss mit den alten Daten rot sein.
4. Sattelpunkte (und nur bei nachgewiesenem Ursprungs-Fehler `originX`/`frameWidth`) in `data/sprites.json` für `horse` und `blackHorse` korrigieren; bei Bedarf Gegentest, dass `mountPose` den Reiter beim Wechsel `idle` → `run` nicht mehr als den Abstand aus dem Test verschiebt.
5. `task check` ausführen. Ergebnis mit Nachweis AC-05 schreiben; **AC-02 aus B-320 (Sichtprüfung am PC) bleibt offen und wird von 🧑 in GR7.5 oder bei der nächsten Abnahme bestätigt**; `Status: fertig`, Tabelle der Sprint-README anpassen.

## Fertig, wenn

- [x] AC-05 (B-320/AC-01): Ein Test belegt, dass für `horse` der Sattelpunkt jedes `run`-Frames höchstens 4 px (Sheet-Pixel) vom `idle`-Punkt entfernt liegt, Ausnahmen sind im Test begründet; `blackHorse` gilt gleich.
- [x] Die Ursache (Daten oder Ursprung) steht im Ergebnis; B-320/AC-02 (Sichtprüfung 🧑) ist als offen geführt.
- [x] `task check` grün (geänderte Dateien: Tests, `tsc`, `oxlint` grün; `task check` im Worktree wegen B-415 rot); Dateien ≤ 400 Zeilen.

## Prüfen

```bash
task test -- mount
task check
```

Keine manuellen Prüfungen (Browser, Xbox, TV).

## Ergebnis

**Fertig (2026-10-12).**

- **Ursache:** falsche Daten, nicht der Ursprung. Beide Sheets haben dieselbe Frame-Breite (100) und `originX` 0,55. Im Bild (`public/sprites/horse/idle.png`, `run.png`, je Frame vergrößert mit eingezeichneten Punkten angesehen) liegen die alten `run`-Punkte (x 38 bis 42) auf der Kruppe, rund 18 px hinter dem Mähnenansatz; der `idle`-Punkt (x 49) sitzt 8 px dahinter am Widerrist. Der Galopp-Körper steht im Frame nicht weiter links, die Punkte waren ~9 px zu weit hinten.
- **Korrektur:** `data/sprites.json › mounts.horse.saddle.run` und `blackHorse` (gleiche Punkte): x 47 bis 52 statt 38 bis 42, y 21 bis 30 (Rücken hebt und senkt sich im Galopp). Abstand zum Mähnenansatz wie im `idle`-Frame; x je Frame am Bild gesichtet und mit dem Rückenprofil abgeglichen (Frames 2 und 3 von Hand, die Mähne ließ sich dort nicht per Farbe finden: ungeprüft auf ±2 px genau).
- **AC-05 geprüft:** neuer Test `src/scenes/mountData.test.ts` (rot mit den alten Daten, `run[0]`: 9 px; grün mit den neuen): x-Abstand zum `idle`-Punkt ≤ 4 px für alle 11 Frames beider Reittiere, y-Abstand ≤ 4 px, Ausnahme `run[2]` (Sammelphase, Rücken 5 px höher als im Stand) begründet im Test. `npx vitest run src/scenes src/tools tests` grün (1652 Tests), `tsc` und `oxlint` ohne Befund.
- **Offen:** B-320/AC-02 (Sichtprüfung Stehen, Laufen, Sprinten am PC) bleibt bei 🧑; das Ticket bleibt `eingeplant`.
- **Hinweis:** `horseWalk` hat eigene, stimmige Punkte (nicht angefasst).
