# src/audio/

## Responsibility

Audio-Subsystem des Clients (Presentation Layer): erzeugt Ton aus Spiel-Ereignissen über Web Audio. Es rechnet keine Spielregel, sondern reagiert nur auf `GameEvent`s und Kamera-Ausschnitte.

## Design

- `audioCore.ts`: `AudioCore` (Fassade) mit injizierbaren `CoreDeps` (Context, `canPlay`, Fetch) für Tests; `audioCore()` liefert die gemeinsame Instanz. `AudioContext` entsteht erst bei der ersten Eingabe (Autoplay-Policy), `resume()` bis er läuft.
- `mixer.ts`: `Mixer` mit Master-Gain und je einem Bus (`music`, `sfx`, `ambient`); Lautstärken je Gerät in den Settings (`volumeToGain`).
- `atlas.ts`: Sound-Atlas (`public/audio/atlas.json`, ein Sprite je Format): `parseAtlas`, `pickFile` nach `FORMAT_ORDER` (ogg, mp3, wav).
- `events.ts`: reine Zuordnung `cueFor(GameEvent)` → `SoundCue` und Positionsdämpfung `attenuation` (`Listener` = Mitte/Breite der Kamera, `EDGE_GAIN`).

## Flow

1. `GameScene.trackLastDevice()` ruft bei Eingabe `audioCore().onInput()`: Context anlegen, `Mixer` bauen, `loadAtlas()` (Atlas-JSON + eine Audiodatei dekodieren), `resume()`.
2. `GameScene.spawnEffects()` ruft je Event `onEvent(e, x, listeners)`.
3. `cueFor` wählt den Sprite, `attenuation` rechnet die Lautstärke aus Quellposition und Listenern.
4. `play(name, bus, volume)` startet einen `BufferSource` mit Sprite-Offset und Dauer am Bus-Knoten; ohne laufenden Context oder Atlas passiert nichts (Rückgabe `false`).

## Integration

- Konsument: `src/scenes/GameScene.ts` (`audioCore`, `Listener`).
- Abhängigkeiten: `src/model/types` (`GameEvent`), `src/core/clientLog`, `src/core/settings` (`loadSettings`, `saveSettings`, `clampVolume`), Web Audio API, Assets `public/audio/`.
