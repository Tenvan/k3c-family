# B-789 · Der Ebitengine-Prototyp zeigt den Fluss in der unteren Bildhälfte, einen festen Fern-Hintergrund und alle Figuren als Rig

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-11-11 🧑 Chat (Revision 3: Fluss unteres Drittel, eine Farbe, sechs Wellen-Reihen nach links, FPS-Anzeige)

## Ausgangslage

Der zweite Prototyp (`native/k3c-ebiten`, B-777) zeigt ein Wald-Level aus Papier-Sheets: drei Hintergrund-Schichten je Spieler mit Scroll-Faktor (fern 0,15, mitte 0,35, nah 0,5), Boden, Gebäude, Natur, Truppen, Gegner, zwei berittene Monarchen. Der Boden liegt in der Mitte der Sicht eines Spielers, darunter ist nur eine braune Fläche; einen Fluss gibt es nicht, und alle Schichten bewegen sich. Der dritte Prototyp (`native/k3c-rig`, B-778) setzt nur Monarch und Pferd zur Laufzeit aus Rig-Teilen zusammen (Rig-Typen `figur` und `vierbeiner`). In den Specs unter `public/grafik` stehen darüber hinaus: 6 Truppen, 7 Gegner und 5 Bosse als `figur`, 11 Reittiere und 2 Gegner als `vierbeiner`, 8 Gegner und 1 Boss als `requisite` (ohne Teile-Baum).

## Ziel

Der Prototyp zeigt das Bild, wie es im Spiel gemeint ist: Der Fluss füllt die untere Hälfte der Szene und fließt im Vordergrund, der hinterste Hintergrund steht fest, und alle Figuren laufen als Rig. Danach lässt sich am Bild und an Messwerten entscheiden, ob dieser Aufbau die Grundlage des nativen Spiels wird.

## Beteiligte und Zielgruppen

🧑 Ralf (Sichtprüfung am PC, Entscheidung), Claude (Bau, Messung).

## Anforderungen

- **Fluss:** Er belegt das untere Drittel der Szene (je Sicht eines Spielers), die Welt mit Boden und Figuren die oberen zwei Drittel (Änderung 🧑 2026-11-11 nach dem ersten Bild; vorher untere Hälfte). Die Fläche ist so angelegt, dass später Schiffe auf bis zu drei Ebenen darauf fahren können (Bosskämpfe).
- **Fließen:** Eine Farbe, keine hervorgehobenen Bahnen. Sechs Wellen-Reihen fließen in Flussrichtung nach links und bewegen sich perspektivisch richtig: Reihen weiter vorn sind höher, ihre Wellen breiter und schneller. Der Fluss liegt im Vordergrund vor dem Ufer. Platzhalter-Grafik genügt.
- **Fester Fern-Hintergrund:** Die hinterste der drei Schichten (`fern`) bewegt sich nicht mit der Kamera; das bisherige Bild genügt für den Test. `mitte` und `nah` scrollen wie bisher.
- **FPS-Anzeige:** groß und deutlich im Bild.
- **Rigs:** Alle Figuren mit Teile-Baum (Rig-Typen `figur` und `vierbeiner`: 6 Truppen, 7 Gegner, 5 Bosse, 11 Reittiere, 2 Gegner) bekommen Teile und Rig-Dateien aus dem Export; im Level laufen die bisher gezeigten Figuren als Rig. Figuren vom Typ `requisite` bleiben in diesem Prototyp Sheets.
- Gebaut wird in `native/k3c-ebiten`, das Paket `native/k3c-rig/rig` wird eingebunden.
- Zwei Spieler im Split-Screen mit je einem Gamepad wie bisher; Messlauf (Bildrate, Grafikspeicher, Rechenzeit) gegen den Stand mit Sheets.

## Nicht-Ziele

Spiel-Logik, Schiffe und Bosskämpfe selbst; Shader (B-790); neue Figuren zeichnen; Umstellung des Browser-Clients.

## Regeln und Einschränkungen

Entscheidung 005 und 008; Rig-Format `native/k3c-rig/RIG.md`. Neue Grafik (Fluss-Kacheln, Fern-Hintergrund) entsteht nur über Papier-Specs und `assets_*`; bis dahin Platzhalter im Prototyp. Der Begriff „Fluss“ und die Aufteilung der Szene kommen zuerst ins Glossar.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-ebiten/**`, `native/k3c-rig/**`, `native/Taskfile.yml`
- **Abhängig von:** B-778
- **Braucht 🧑:** Freigabe
- **Größe:** L
- **Sperre:** –

## Beispiele

Monarch reitet nach rechts: Ufer, Gebäude und nahe Schichten ziehen vorbei, der Vulkan am Horizont bleibt stehen, die Fluss-Kacheln laufen in der unteren Bildhälfte weiter, auch wenn der Monarch steht.

## Ausnahme- und Fehlerfälle

Eine Figur hat keinen Teile-Baum (`requisite`) → siehe Offene Fragen. Rig-Datei oder Teil-PNG fehlt → klare Fehlermeldung beim Start, kein stiller Ersatz.

## Akzeptanzkriterien

- **AC-01** Im Fenster füllt der Fluss das untere Drittel jeder Sicht, sechs Wellen-Reihen in einer Farbe fließen nach links, vorn schneller und größer (Screenshot im Bericht).
- **AC-02** Die Schicht `fern` bleibt bei Kamerafahrt stehen, `mitte` und `nah` scrollen (Screenshots an zwei Kamera-Positionen).
- **AC-03** Alle Figuren des Levels mit Teile-Baum werden als Rig gezeichnet; `go test` vergleicht ihre Posen mit den Sollwerten aus `tools/paper` (Toleranz 0,01).
- **AC-04** Messbericht Entwicklungs-PC: Bildrate, Grafikspeicher und Rechenzeit je Bild, Rig gegen Sheet, im selben Level.
- **AC-05** Sichtprüfung 🧑 am PC, Entscheidung in den Notizen.

## Offene Fragen

keine. Entschieden von 🧑 am 2026-11-11 (Chat): Platzhalter-Kacheln für den Fluss; `fern` ist der feste Hintergrund mit dem bisherigen Bild (keine vierte Schicht); untere Hälfte je Sicht eines Spielers; `requisite`-Figuren bleiben Sheets; alle Figuren mit Teile-Baum exportieren, im Level die bisher gezeigten; Bau in `native/k3c-ebiten` mit dem Paket `native/k3c-rig/rig`.

## Notizen

Angelegt 2026-11-11 aus dem Chat mit 🧑. Schiffe auf bis zu drei Ebenen bei Bosskämpfen sind der Grund für die untere Bildhälfte. Messwerte B-778: ein Reiter mit Pferd kostet als Rig etwa 0,037 ms; viele Gegner als Rig sind die offene Frage aus B-780.
