# tools/soundatlas/

## Responsibility
CLI `soundatlas` (SO6.2, B-494): packt die erste Präferenz jedes Effekt-Slots aus `data/sounds.json` (Bus `sfx`, ohne Schleifen) zu einem Sound-Atlas `public/audio/atlas.ogg` und `atlas.mp3` mit `atlas.json` (Start und Dauer je Slot). Die Ausgabe hängt nur von den Eingaben ab.

## Design
- `main.go`: Flags `-root` (Projektwurzel), `-ffmpeg` (Pfad zu ffmpeg); ruft `run` und meldet die Zahl der Effekte, Exit 1 bei Fehler.
- `atlas.go`:
  - Typen `slotFile`/`slot` (Ausschnitt der `sounds.json`), `entry` (Slot und Datei unter `public/`), `sprite` (Start und Dauer in Sekunden, auf Millisekunden gerundet), `atlasJSON`.
  - `loopSlots`: Slots, die je Spieler als Schleife laufen, gehören nicht in den Atlas.
  - `pickEntries` (rein): erste Präferenz jedes Effekt-Slots ohne eigenes Atlas-Sprite, sortiert nach Slot-ID.
  - `layout` (rein): rechnet aus den Samples je Effekt Start und Dauer, zwischen zwei Effekten `gapSeconds` Stille; `ms` rundet.
  - `decode`: ffmpeg liefert PCM s16le mono 44,1 kHz; `write` kodiert erst in temporäre Dateien und benennt danach um (kein halber Atlas bei Fehler).

## Flow
`main` → `run(root, ffmpeg)` → `data/sounds.json` lesen → `pickEntries` → je Eintrag `decode` → `layout` → `write` (OGG, MP3, `atlas.json` unter `public/audio/`).

## Integration
- Taskfile-Task `gen:soundatlas` in `tools/Taskfile.yml` (baut `bin/soundatlas`, braucht Go und ffmpeg; `sources` sind `tools/soundatlas/*.go` und `data/sounds.json`).
- Eingabe `data/sounds.json` und Sounddateien unter `public/`; Ausgabe `public/audio/atlas.{ogg,mp3,json}`.
- Abhängigkeiten: nur Standardbibliothek und das externe Programm ffmpeg.
- Test: `atlas_test.go` (`pickEntries`, `layout`).
