# native/k3c-rig/

## Responsibility

Rig-Prototyp `k3c-rig` (Projekt NAT, B-778): zeigt Monarch und Pferd zur Laufzeit aus Rig-Teilen neben den Papier-Sheets und liefert Bildvergleich, Screenshots und Messungen; eigenes Go-Modul `k3c/native/k3c-rig`, dessen Pakete `rig` und `paint` auch das native Spiel nutzt.

## Design

- `main.go`: Einstieg; `Args` aus Flags (`k3c-rig`), `Game` implementiert `ebiten.Game` (`Update`, `Draw`, `Layout`) und hält eine `Scene`.
- Szenen je Modus: `demo.go` (`Demo`, Vorführung: links Sheet, rechts Rig, zwei Monarchen), `shots.go` (`Shots`, `--shots`), `compare.go` (`Compare`, `--vergleich`, Kennzahlen und Differenzbilder je Frame), `measure.go` (`Measure`, `--measure --figuren --art rig|sheet`, FPS/Rechenzeit/Speicher).
- `assets.go`: Monarch und Pferd mit allen Spieler-Varianten über `paint.LoadAssets`; `sheets.go`: Monarch-Sheets aus `public/grafik/figuren/monarch/manifest.json` zum Vergleich.
- `input.go`: je Spieler ein Gamepad, die Tastatur steuert einen wählbaren Monarchen (`Action`).
- `clock_windows.go`/`clock_other.go`: hochauflösende Uhr (Windows-Zähler statt grobem Systemtakt).
- `export/*.ts`: Bun-Skripte, die Teil-PNGs, Rig-Dateien, Elemente und Sollwerte aus `tools/paper` nach `assets/` schreiben.
- `assets/`: eingecheckte Rig-Daten je Gruppe (`figuren`, `reittiere`, `truppen`, `gegner`, `bosse`, `elemente`, `soll`).

## Flow

1. `task native:rig` → `main` → `parseArgs` → `LoadAssets(dir)` und `LoadSheets(root)`.
2. `newScene(g)` wählt Demo, Shots, Compare oder Measure; `ebiten.RunGame(g)` mit `SyncWithFPS`.
3. Je Bild `Game.Update` → `Scene.Update` (Eingabe, `paint.Actor.Update`), `Game.Draw` → `Scene.Draw` mit `paint.Painter`.
4. Aufnahme-Modi schreiben nach `reports/native-rig-shots/`, `reports/native-rig-vergleich/` bzw. Berichte `reports/native-rig-*.json` (Messung über `native/measure.ps1`).

## Integration

- Abhängigkeiten: `native/k3c-rig/rig`, `native/k3c-rig/paint`, Ebitengine; Export braucht `tools/paper`.
- Konsumenten: `native/k3c-game` bindet das Modul per `replace k3c/native/k3c-rig => ../k3c-rig` ein.
- Tasks in `native/Taskfile.yml`: `rig`, `rig:build`, `rig:test`, `rig:export`, `rig:export:check`, `rig:shots`, `rig:vergleich`, `rig:measure`, `rig:paket`.
