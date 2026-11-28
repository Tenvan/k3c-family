# tools/soundatlas/

## Responsibility
CLI `soundatlas` (SO6.2, B-494): packt die erste Präferenz jedes Effekt-Slots aus `data/sounds.json` (Bus `sfx`, ohne Schleifen) zu einem Sound-Atlas `public/audio/atlas.ogg` und `atlas.mp3` mit `atlas.json` (Start und Dauer je Slot). Die Ausgabe hängt nur von den Eingaben ab.

## Design
TypeScript unter bun, nur Node-APIs (RU71); Aufruf `bun tools/soundatlas/main.ts [-root <ordner>] [-ffmpeg <pfad>]`.

- `main.ts`: Flags, ruft `run` und meldet die Zahl der Effekte; Exit 1 bei Fehler, 2 bei falschem Flag.
- `atlas.ts`:
  - `pickEntries` (rein): erste Präferenz jedes Effekt-Slots ohne eigenes Atlas-Sprite, sortiert nach Slot-ID; Schleifen-Slots bleiben außen vor.
  - `layout` (rein): rechnet aus den Samples je Effekt Start und Dauer (Sekunden, auf Millisekunden gerundet), zwischen zwei Effekten 0,15 s Stille; `atlasJson` schreibt den Index mit sortierten Schlüsseln.
  - `isLocal`: Sounddateien müssen unter `public/` liegen.
  - ffmpeg läuft ohne Shell: dekodiert zu PCM s16le mono 44,1 kHz, kodiert erst in temporäre Dateien unter `.work/tmp` der Projektwurzel; danach werden alle drei umbenannt (kein halber Atlas bei Fehler).

## Flow
`main` → `run(root, ffmpeg)` → `data/sounds.json` lesen → `pickEntries` → je Eintrag dekodieren → `layout` → schreiben (OGG, MP3, `atlas.json` unter `public/audio/`).

## Integration
- Taskfile-Task `gen:soundatlas` in `tools/Taskfile.yml` (braucht bun und ffmpeg; `sources` sind `tools/soundatlas/*.ts` ohne Tests und `data/sounds.json`).
- Eingabe `data/sounds.json` und Sounddateien unter `public/`; Ausgabe `public/audio/atlas.{ogg,mp3,json}`.
- Test: `atlas.test.ts` (Vitest der Wurzel).