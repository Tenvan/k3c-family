# src/scenes/

## Responsibility

Phaser-Szenen des Browser-Clients (Laden, Lobby, Spiel, HUD, Optionen) samt Split-Screen-Layout und lokalen Slots. Keine Spielregeln: die Szenen zeichnen Server-Snapshots und reichen Eingaben weiter. Logik, die ohne Phaser testbar sein soll, liegt in `*Logic.ts`.

## Design

- **Szenen:** `LoadScene.ts` (Atlas und Grafiken laden) → `LobbyScene.ts` (Raumwahl) → `GameScene.ts` (Eingabe, Frames, Kameras) + parallel `HudScene.ts` (bildschirmfeste Anzeigen); `OptionsScene.ts` als Overlay.
- **Scene + reine Logik (Humble Object):** `LoadScene`/`loadLogic.ts` (`progressText`, `barFill`, `errorText`), `LobbyScene`/`lobbyLogic.ts` (`parseStartParams`, `applyCommand`, `LobbyFlow`: Auswahl und Befehle), `OptionsScene`/`optionsLogic.ts` (`OPTION_IDS`, Menü-Eingaben); die Szene zeichnet nur.
- **Split-Screen:** `layout.ts` (`computeLayout`, 1–4 Zellen, `BAR_HEIGHT` der Leiste oben, `viewZoom`), `cellStages.ts` (`cellStages`: Stufe je Zelle), `localSlots.ts` (`LocalSlots`: Eingabegerät ↔ Slot, `addSlot`/`removeSlot`/`leave`, `pageBots`).
- **Regel-Tests des Ordners:** `noSim.test.ts` (Client importiert nur aus `src/model`, keine Sim), `textRule.test.ts` (Spieltexte nur in `src/core/texts.*.ts`), `goldOnlyInHud.test.ts` (Gold nur im HUD, B-398), `mountData.test.ts` (Sattelpunkte des Reittiers, B-320); weitere Tests `cellStages`, `layout`, `loadLogic`, `lobbyLogic`, `localSlots`, `optionsLogic`.
- **Unterordner:**

| Ordner | Aufgabe |
| --- | --- |
| [render/](render/codemap.md) | Zeichnet die `World` je Stufe: `WorldRenderer`, `StageView`, Grafik-Zuordnung, Reiter, Bürger, Schrift-Katalog |
| [hud/](hud/codemap.md) | Bildschirmfeste HUD-Anzeigen: Layout, Meldungen, Gold, Radar, Skill-Menü, Wirtschafts-/Bürger-Texte, Pause-Hinweise |
| [hints/](hints/codemap.md) | Tasten-Glyphen, Aktions-Overlay und geführte erste Nacht |
| [effects/](effects/codemap.md) | Feedback: Ring-/Partikel-Effekte, Kamera-Shake, Controller-Rumble |
| [debug/](debug/codemap.md) | Diagnose-Overlay, Cheat-Dialog, Dev-Aktionen und Gesten |

## Flow

1. `main.ts` registriert die Szenen; `LoadScene` lädt den Atlas und die Grafiken (`preloadSprites`, `preloadWorld`, `preloadBuildings` aus `render/`; danach `createSpriteAnims`, `prepareWorld`, `prepareBuildings`) und startet `LobbyScene`.
2. `LobbyScene` führt `LobbyFlow`/`applyCommand` gegen `RoomClient`; bei Beitritt startet `GameScene` (`GameSceneData`).
3. `GameScene.update()`: Eingaben aus Keyboard/Gamepad/Touch → `LocalSlots` → `SkillMenus.route` (`hud/`; Menü fängt Befehle ab) → Kommandos an `RoomClient`; `takeFrames()` schiebt Frames in die `Timeline` und löst `GameEvent`s sofort je Frame aus; `draw()` zeichnet `timeline.sample(now)`, ersetzt das x der lokalen Monarchen durch die Vorhersage (`Predictor` aus den gesendeten Eingaben) und schreibt über `applyState` (Verzögerung `delayMs` im Debug-Overlay).
4. `spawnEffects()` je Event: `feedback()` (Shake/Rumble), `effectFor` → `StageEffects.spawn` (`effects/`), `audioCore().onEvent(...)` mit `listeners()` (Kamera-Ausschnitte).
5. `StageView`/`WorldRenderer` (`render/`) zeichnen die `World` je Stufe in die Layer der Zellen; `cellStages` und `showOnly` ordnen Zellen und Kameras zu.
6. `HudScene` liest `GameScene.hudCells()` und `pendingEvents`, ordnet die Elemente über `hudLayout` und zeichnet die Anzeigen aus `hud/`, `hints/` (`ActionOverlay`, `GuideOverlay`) und `debug/` (`DebugOverlay`).
7. Menu kurz bzw. Touch-`PauseButton` (`hud/`) → `scene.launch('options')`; Änderungen gehen über `saveSettings` zurück; wird der Schalter `guide` wieder eingeschaltet, setzt `guideSeen.reset()` die Hinweise zurück.

## Integration

- Konsument: `src/main.ts` (Szenen-Registrierung, `VERSION_KEY` aus `debug/debugOverlay`); `src/tools/testScenarios.test.ts` (`lobbyLogic`); Asset-Studio `tools/k3c-studio/frontend` nutzt Dateien aus `render/` und `effects/` (siehe deren Codemaps).
- Abhängigkeiten: `src/online/` (`RoomClient`, `clientWorld`, `clientTimeline`, `clientPredict`, `clientProtocol`), `src/model/` (`types`, `data`, `biome`), `src/input/` (`playerInput`, `touchInput`, `botInput`, `slotBindings`), `src/audio/` (`audioCore`, `events`, `music`), `src/core/` (`constants`, `texts`, `settings`, `saveStore`, `fullscreen`, `clientLog`, `guideSeen`), Phaser; Unterordner nur über die eigenen Codemaps.
