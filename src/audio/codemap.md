# src/audio/

## Responsibility

Audio-Subsystem des Clients (Presentation Layer): erzeugt Ton aus Spiel-Ereignissen über Web Audio. Es rechnet keine Spielregel, sondern reagiert nur auf `GameEvent`s und Kamera-Ausschnitte.

## Design

- `audioCore.ts`: `AudioCore` (Fassade) mit injizierbaren `CoreDeps` (Context, `canPlay`, Fetch) für Tests; `audioCore()` liefert die gemeinsame Instanz. `AudioContext` entsteht erst bei der ersten Eingabe (Autoplay-Policy), `resume()` bis er läuft. Neben `play(name, bus, volume)` (Atlas-Sprite) spielt `playFile(url, {bus, loop, fade, onEnded})` ganze Dateien, je Bus ein Stück (`voices`); `preload`, `stop`, `isPlaying`, `getVolume`/`setVolume` ergänzen. Ein neues Stück ersetzt das alte per Crossfade, das alte blendet vom aktuellen Pegel aus (`release`).
- `crossfade.ts` (B-250): reine Kurven ohne Browser: `crossfadeCurves(n)` (gleiche Leistung, cos/sin), `fadeOutFrom(level)`, Konstanten `CROSSFADE_SECONDS` (2) und `QUICK_FADE_SECONDS` (0.03, kein Knacken beim Ersetzen/Stoppen).
- `mixer.ts`: `Mixer` mit Master-Gain und je einem Bus (`music`, `sfx`, `ambient`); Lautstärken je Gerät in den Settings (`volumeToGain`).
- `atlas.ts`: Sound-Atlas (`public/audio/atlas.json`, ein Sprite je Format): `parseAtlas`, `pickFile` nach `FORMAT_ORDER` (ogg, mp3, wav).
- `music.ts` (SO4.2): reiner Zustands-Automat `nextMusicState` (acht Zustände, Priorität Lobby > Ende > Boss > Kampf > Tiefe > Tageszeit), Tabelle `MUSIC_TRACKS` (Zustand → ein Stück oder Lücke = Stille) und `MusicDirector` (`lobby()`, `update(view, events)`, `reset()`; `musicDirector()` liefert die Instanz), der per `preload` + `playFile` mit Crossfade (`CROSSFADE_SECONDS`) auf dem Bus `music` spielt; `musicViewOf(world)` leitet den Auszug ab, `trackUrl` wählt das Format. Fehlt die Datei einer Zustands-Zeile (`null`), blendet das laufende Stück aus. Gespeist von `GameScene` (Welt-Auszug, Ereignisse) und `LobbyScene`.
- `events.ts`: reine Zuordnung `cueFor(GameEvent)` → `SoundCue` und Positionsdämpfung `attenuation` (`Listener` = Mitte/Breite der Kamera, `EDGE_GAIN`).

## Flow

1. `GameScene.trackLastDevice()` ruft bei Eingabe `audioCore().onInput()`: Context anlegen, `Mixer` bauen, `loadAtlas()` (Atlas-JSON + eine Audiodatei dekodieren), `resume()`.
2. `GameScene.spawnEffects()` ruft je Event `onEvent(e, x, listeners)`.
3. `cueFor` wählt den Sprite, `attenuation` rechnet die Lautstärke aus Quellposition und Listenern.
4. `MusicDirector.update` (je Frame aus `GameScene`/`LobbyScene`) bestimmt mit `nextMusicState` den Zustand, lädt das Stück und startet es per `playFile` mit `loop` und Crossfade.
5. `play(name, bus, volume)` startet einen `BufferSource` mit Sprite-Offset und Dauer am Bus-Knoten; ohne laufenden Context oder Atlas passiert nichts (Rückgabe `false`).

## Integration

- Konsumenten: `src/scenes/GameScene.ts` (`audioCore`, `Listener`, `musicDirector`), `src/scenes/LobbyScene.ts` (`musicDirector`), `src/tools/soundtest.ts`.
- Abhängigkeiten: `src/model/types` (`GameEvent`), `src/core/clientLog`, `src/core/settings` (`loadSettings`, `saveSettings`, `clampVolume`), Web Audio API, Assets `public/audio/`.
