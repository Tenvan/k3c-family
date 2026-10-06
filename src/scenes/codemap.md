# src/scenes/

## Responsibility

Presentation Layer des Browser-Clients (Phaser 4): Szenen, Kamera-Layout, Rendering, Hinweis-Overlays (Aktionen, Skill-Menü, geführte erste Nacht) und Feedback. Der Ordner enthält keine Spielregeln; er zeichnet die Server-Snapshots und reicht Eingaben weiter. Logik, die ohne Phaser testbar sein soll, liegt in `*Logic.ts`/`*Rules.ts`.

## Design

- **Scene + reine Logik (Humble Object):** `LobbyScene.ts`/`lobbyLogic.ts`, `OptionsScene.ts`/`optionsLogic.ts`, `LoadScene.ts`/`loadLogic.ts`; die Szene zeichnet nur, die Logik-Datei hält Auswahl, Befehle, Texte.
- **Szenen:** `LoadScene` (Atlas laden) → `LobbyScene` (Raumwahl) → `GameScene` (Eingabe, Frames, Kameras) + parallel `HudScene` (bildschirmfeste Anzeigen); `OptionsScene` als Overlay.
- **Rendering:** `worldRenderer.ts` (`WorldRenderer`, ein Phaser-Objekt je Entity-id), `stageView.ts` (`StageView` je Stufe, Platzhalter-Layer für ungeladene Zellen, Hintergrund über `backgroundLayers`), `sprites.ts` (Sheets, Reittiere `MOUNTS`, `RIDER_WAIST` aus `data/sprites.json`), `viewRules.ts` (Darstellungsregeln ohne Sim-Import: `canAfford`, `hasDepth`, `isOnTower`, `daylight`, `PRICE_TAG_RANGE`).
- **Grafik-Zuordnung (rein, `null` = Platzhalter):** `buildingSprites.ts` (`siteTexture`, `hubTexture`, `BUILDING_TEXTURES`, `STAIRS_UP_TILES`), `worldSprites.ts` (`objectSprite`, `biomeBackground`, `WORLD_TEXTURES`/`WORLD_FRAMES`); gezeichnet von `siteView.ts` (`createSiteView`/`updateSiteView`, `drawCastle`, `preloadBuildings`/`prepareBuildings`, Preisschild) und `objectView.ts` (`preloadWorld`/`prepareWorld`, `objectImage`, `backgroundLayers`).
- **Reittier:** `mountPose.ts` (`mountPose`: reine Pose aus `vx`, `facing`, Frame und `MountContext`, `timeScale` begrenzt) + `mountView.ts` (`createRider`/`updateRider`, Container je Monarch, vom `WorldRenderer` genutzt).
- **Hinweise mit Tasten-Glyphen:** `glyphs.ts` (`glyphOf`: Aktion + Gerät → Glyph-Schlüssel `pad:`/`key:`/`touch:`, B und View nie als Bild) und `glyphView.ts` (`drawGlyph`, `GlyphRow`: Zeile aus Text und Glyphen); `actionHints.ts` (`playerHints`/`hintView`: Aktionen aus `actions` des Servers, Bauen/Zahlen wie das Preisschild) + `actionOverlay.ts` (`ActionOverlay`, `screenAt`); `guideHints.ts` (`nextGuide`/`stepGuide`/`guideDone`: geführte erste Nacht `coin`→`pay`→`dusk`→`recruit` aus dem Snapshot) + `guideOverlay.ts` (`GuideOverlay`; gesehen-Stand über `core/guideSeen`).
- **Skill-Menü:** `skillMenuLogic.ts` (`menuEntries`, `slotViews`, `SkillMenus.route`: lernen/Respec aus `points`/`actions` des Servers, Befehle statt Spielerkommandos) + `skillMenuView.ts` (`SkillMenuLayer`).
- **Split-Screen:** `layout.ts` (`computeLayout`, 1–4 Zellen, `sharedAnchor`), `cellStages.ts` (Stufe je Zelle), `localSlots.ts` (`LocalSlots`: Eingabegerät ↔ Slot, `addSlot`/`removeSlot`/`leave`).
- **Radar:** `radar.ts` (reine Marker-Berechnung, `radarRect`) + `radarView.ts` (`RadarLayer`).
- **Feedback/Juice:** `effects.ts` (`effectFor`, `EFFECT_CONFIG`, Limits), `effectRules.ts` (Blitz-Sperrfenster, Shake/Rumble-Auswahl je Seat), `effectsView.ts` (`StageEffects`, Ring + Partikel).
- **Schrift:** `fontRules.ts` (`FONTS`-Katalog, Mindestgrößen, `contrastRatio`).
- **Debug (`?dev`):** `debugOverlay.ts` (Textzeilen, u. a. `Puffer … ms` und `Latenz … ms (p95 … ms)`), `debugOverlayView.ts` (`DebugOverlay`), `debugOverlayPanel.ts` (`CheatDialog`, `focusStep`/`muteFocused`: Controller-Fokus sperrt Spielkommandos), `debugActions.ts` (`DEV_ACTIONS` → `DevMessage`, `pauseMessage`, `roomDevMode`), `debugGestures.ts` (`holdStep`: LB+RB 3 s = Cheat-Dialog, RB 3 s = Diagnose; `tapStep`/`listenTaps`: Doppeltap mit zwei bzw. einem Finger).
- `pauseButton.ts`: Touch-Schaltfläche „Optionen“, ein Knopf pro Seite.

## Flow

1. `main.ts` registriert die Szenen; `LoadScene` lädt den Atlas und die Grafiken (`preloadSprites`, `preloadWorld`, `preloadBuildings`; danach `createSpriteAnims`, `prepareWorld`, `prepareBuildings`) und startet `LobbyScene`.
2. `LobbyScene` führt `LobbyFlow`/`applyCommand` gegen `RoomClient`; bei Beitritt startet `GameScene` (`GameSceneData`).
3. `GameScene.update()`: Eingaben aus Keyboard/Gamepad/Touch → `LocalSlots` → `SkillMenus.route` (Menü fängt Befehle ab) → Kommandos an `RoomClient`; `takeFrames()` schiebt Frames in die `Timeline` und löst `GameEvent`s sofort je Frame aus; `draw()` zeichnet `timeline.sample(now)`, ersetzt das x der lokalen Monarchen durch die Vorhersage (`Predictor` aus den gesendeten Eingaben) und schreibt über `applyState` (Verzögerung `delayMs` im Debug-Overlay).
4. `spawnEffects()` je Event: `feedback()` (Shake/Rumble), `effectFor` → `StageEffects.spawn`, `audioCore().onEvent(...)` mit `listeners()` (Kamera-Ausschnitte).
5. `StageView`/`WorldRenderer` zeichnen die `World` je Stufe in die Layer der Zellen (Gebäude über `siteView`, Welt-Objekte über `objectView`, Reiter über `mountView`); `showOnly` ordnet Kameras zu.
6. `HudScene` liest `GameScene.hudCells()` und `pendingEvents`, zeichnet Gold, Vorrat, Meldungen, `RadarLayer`, `SkillMenuLayer` (Zustand aus `GameScene.skillMenus`), `ActionOverlay`, `GuideOverlay` und `DebugOverlay`.
7. Menu kurz bzw. Touch-`PauseButton` → `scene.launch('options')`; Änderungen gehen über `saveSettings` zurück; wird der Schalter `guide` wieder eingeschaltet, setzt `guideSeen.reset()` die Hinweise zurück.

## Integration

- Konsument: `src/main.ts` (Szenen-Registrierung, `VERSION_KEY`); `src/tools/testScenarios.test.ts`.
- Abhängigkeiten: `src/online/` (`RoomClient`, `clientWorld`, `clientTimeline`, `clientPredict`, `clientProtocol`), `src/model/` (`types`, `data`, `biome`), `src/input/` (`playerInput`, `touchInput`), `src/audio/audioCore`, `src/input/slotBindings` (Tasten-Belegung, `Device`), `src/core/` (`constants`, `texts`, `settings`, `fullscreen`, `clientLog`, `guideSeen`), `data/sprites.json`, Grafiken unter `public/grafik/`, Phaser.
