# tools/k3c-studio/frontend/src/sound/

## Responsibility
Bausteine des Sounds im Asset-Studio: Szenen-Zeilen, Mischproben, Audio-Kern und Abspiel-Zuordnung. Die Seite Sound (Reiter Slots, Szenen, Proben, Hörprobe) ist seit B-677 in `../assetpage/` aufgegangen; Abspielen läuft dort über die eine Abspielleiste.

## Design
- `SoundScenes.tsx`: Szenen sfx, musik, ambient und die Zeilen-Szenen Skills, Gegner und Bosse, Bauen, Reiten, UI, Warnungen (`SoundRowList.tsx`, Aufbereitung in `soundRows.ts`; `skillSlot` vorläufig, WZ29.2 ersetzt sie durch SO8.2). Eingebunden von `../assetpage/SceneView.tsx`.
- `Proben.tsx` / `probes.ts`: Misch- (Ducking `DUCK`), Lautheits- und Abstandsprobe (`attenuation`); eingebunden von `../assetpage/ToolsPanel.tsx`.
- `audio.ts`: ein `audioCore` je Fenster, `playChoice`, `playAtGain`, `stopAll`; Aufrufe nur aus Klick oder Taste.
- `playback.ts`: reine Abspiel-Zuordnung (`playPlan`: Atlas-Ton, „fern“ mit `FAR_GAIN`, Musik mit Crossfade, Umgebung als Schleife; `playStatus`).
- Entfallen mit den Seiten Sound und Hörprobe: `SoundPage.tsx`, `Hoerprobe.tsx`, `SoundProbe.tsx`. Die Hörprobe-Liste ist durch ▶ in `../assetpage/` ersetzt.
- Tests: `playback.test.ts`, `soundRows.test.ts`, `soundScenes.test.tsx`.

## Flow
▶ in der Seite Assets → `transport.playOn(slot, choice)` (bzw. `playFile`) → `playChoice` über den Audio-Kern; Verlassen der Seite → `stopAll`.

## Integration
- Abhängigkeiten: `../assets/` (`useAssets`), `../graphics/scenesLogic` (`licenseFlag`, `slotsOf`, `resolveSlot`), `../api`, Audio-Kern und `@game/tools/soundtestLogic` (`stepVolume`).
- Konsumenten: `../assetpage/` (`SceneView`, `ToolsPanel`, `transport`), `../chat/SuggestionCard.tsx` (`playChoice`, `stopAll`).
