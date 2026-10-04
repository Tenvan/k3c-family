# tools/atlas/

## Responsibility

Build-Time-CLI (Asset Pipeline): packt die im Spiel genutzten Figuren-Frames aus `data/sprites.json` zu Atlas-PNGs und einer Phaser-Multiatlas-Beschreibung `public/atlas/atlas.json`. Die Ausgabe ist deterministisch (B-163).

## Design

- `main.go`: Flags `-root` (Projektwurzel), `-out` (Standard `public/atlas`), `-max` (Kantenlänge, 4096); ruft `run`.
- `atlas.go`: Datentypen `sheet`, `spriteData`, `frame`, `jFrame`, `texture` (Phaser-JSON); `loadFrames` liest genutzte Sheets und schneidet Streifen per `cut` in Frames; `pack` ist ein Shelf-Packer (Frames nach Höhe absteigend, dann Name; bei Überlauf beginnt eine neue `page`); `render` zeichnet die Seiten.
- Stabile Sortierung macht die Ausgabe reproduzierbar.

## Flow

1. `main` -> `run(root, out, maxSize)`.
2. `loadFrames`: `data/sprites.json` + `public/sprites/<sheet>/<anim>.png` -> `[]frame`.
3. `pack` -> Seiten; alte `atlas*`-Dateien im Zielordner werden entfernt.
4. Je Seite `atlas-<i>.png`, am Ende `atlas.json` (Format `RGBA8888`).
5. Ausgabe `atlas: N Atlas/Atlanten nach <out>`, bei Fehler Exit 1.

## Integration

- Eingaben: `data/sprites.json`, `public/sprites/**/*.png`.
- Ausgabe: `public/atlas/atlas.json` und `atlas-*.png`; der Client lädt sie über `src/scenes/sprites.ts` (`LoadScene`).
- Taskfile-Task `atlas` (baut `bin/atlas`, Dependency von `dev` und `build`, mit Quellen/Erzeugnissen als Task-Cache).
