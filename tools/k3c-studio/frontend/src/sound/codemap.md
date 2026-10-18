# tools/k3c-studio/frontend/src/sound/

## Responsibility
Seite Sound des Asset-Studios: Reiter Slots, Szenen und Hörprobe; spielt die Zuordnungen der Sound-Slots mit dem Audio-Kern des Spiels.

## Design
- `SoundPage.tsx`: Reiter; stoppt beim Verlassen alle Busse. Reiter Slots nutzt `../assets/AssetsPage` mit Art Sound und `SoundProbe` (Hörprobe am Slot).
- `SoundScenes.tsx`: Szenen sfx, musik, ambient.
- `Hoerprobe.tsx`: Liste aus `@game/tools/soundtestLogic` (ganze Dateien nach Musik-Zuständen und Ereignissen), hart oder mit Crossfade, Stopp, Lautstärke ±10 %, Tastatur.
- `audio.ts`: ein `audioCore` je Fenster, `playChoice`, `stopAll`; Aufrufe nur aus Klick oder Taste.
- `playback.ts`: reine Abspiel-Zuordnung je Zuordnung (`playPlan`: Atlas-Ton, „fern“ mit `FAR_GAIN`, Musik mit Crossfade, Umgebung als Schleife; `playStatus`).
- Tests: `playback.test.ts`, `SoundPage.test.tsx`.

## Flow
Klick auf Hörprobe → `playPlan(slot, choice)` → `playChoice` über den Audio-Kern; Wechsel der Seite oder des Reiters → `stopAll`.

## Integration
- Abhängigkeiten: `../assets/`, `../api`, Audio-Kern und `@game/tools/soundtestLogic` aus dem Spiel.
- Eingehängt in `../App.tsx` (Seite `sound`).
