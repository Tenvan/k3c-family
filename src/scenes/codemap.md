# src/scenes/

## Responsibility

Presentation Layer des Browser-Clients (Phaser 4): Szenen, Kamera-Layout, Rendering und Feedback. Der Ordner enthält keine Spielregeln; er zeichnet die Server-Snapshots und reicht Eingaben weiter. Logik, die ohne Phaser testbar sein soll, liegt in `*Logic.ts`/`*Rules.ts`.

## Design

- **Scene + reine Logik (Humble Object):** `LobbyScene.ts`/`lobbyLogic.ts`, `OptionsScene.ts`/`optionsLogic.ts`, `LoadScene.ts`/`loadLogic.ts`; die Szene zeichnet nur, die Logik-Datei hält Auswahl, Befehle, Texte.
- **Szenen:** `LoadScene` (Atlas laden) → `LobbyScene` (Raumwahl) → `GameScene` (Eingabe, Frames, Kameras) + parallel `HudScene` (bildschirmfeste Anzeigen); `OptionsScene` als Overlay.
- **Rendering:** `worldRenderer.ts` (`WorldRenderer`, ein Phaser-Objekt je Entity-id), `stageView.ts` (`StageView` je Stufe, Platzhalter-Layer für ungeladene Zellen), `sprites.ts` (Sheets aus `data/sprites.json`), `viewRules.ts` (Darstellungsregeln ohne Sim-Import: `canAfford`, `isOnTower`, `daylight`).
- **Split-Screen:** `layout.ts` (`computeLayout`, 1–4 Zellen, `sharedAnchor`), `cellStages.ts` (Stufe je Zelle), `localSlots.ts` (`LocalSlots`: Eingabegerät ↔ Slot, `addSlot`/`removeSlot`/`leave`).
- **Radar:** `radar.ts` (reine Marker-Berechnung, `radarRect`) + `radarView.ts` (`RadarLayer`).
- **Feedback/Juice:** `effects.ts` (`effectFor`, `EFFECT_CONFIG`, Limits), `effectRules.ts` (Blitz-Sperrfenster, Shake/Rumble-Auswahl je Seat), `effectsView.ts` (`StageEffects`, Ring + Partikel).
- **Schrift:** `fontRules.ts` (`FONTS`-Katalog, Mindestgrößen, `contrastRatio`).
- **Debug (`?dev`):** `debugOverlay.ts` (Textzeilen, u. a. `Puffer … ms` und `Latenz … ms (p95 … ms)`), `debugOverlayView.ts` (`DebugOverlay`), `debugOverlayPanel.ts` (`DevActionPanel`, Controller-Fokus), `debugActions.ts` (`DEV_ACTIONS` → `DevMessage`).
- `pauseButton.ts`: Touch-Schaltfläche „Optionen“, ein Knopf pro Seite.

## Flow

1. `main.ts` registriert die Szenen; `LoadScene` lädt den Atlas (`preloadSprites`, `createSpriteAnims`) und startet `LobbyScene`.
2. `LobbyScene` führt `LobbyFlow`/`applyCommand` gegen `RoomClient`; bei Beitritt startet `GameScene` (`GameSceneData`).
3. `GameScene.update()`: Eingaben aus Keyboard/Gamepad/Touch → `LocalSlots` → Kommandos an `RoomClient`; `takeFrames()` schiebt Frames in die `Timeline` und löst `GameEvent`s sofort je Frame aus; `draw()` zeichnet `timeline.sample(now)`, ersetzt das x der lokalen Monarchen durch die Vorhersage (`Predictor` aus den gesendeten Eingaben) und schreibt über `applyState` (Verzögerung `delayMs` im Debug-Overlay).
4. `spawnEffects()` je Event: `feedback()` (Shake/Rumble), `effectFor` → `StageEffects.spawn`, `audioCore().onEvent(...)` mit `listeners()` (Kamera-Ausschnitte).
5. `StageView`/`WorldRenderer` zeichnen die `World` je Stufe in die Layer der Zellen; `showOnly` ordnet Kameras zu.
6. `HudScene` liest `GameScene.hudCells()` und `pendingEvents`, zeichnet Gold, Vorrat, Meldungen, `RadarLayer` und `DebugOverlay`.
7. Menu kurz bzw. Touch-`PauseButton` → `scene.launch('options')`; Änderungen gehen über `saveSettings` zurück.

## Integration

- Konsument: `src/main.ts` (Szenen-Registrierung, `VERSION_KEY`); `src/tools/testScenarios.test.ts`.
- Abhängigkeiten: `src/online/` (`RoomClient`, `clientWorld`, `clientTimeline`, `clientPredict`, `clientProtocol`), `src/model/` (`types`, `data`, `biome`), `src/input/` (`playerInput`, `touchInput`), `src/audio/audioCore`, `src/core/` (`constants`, `texts`, `settings`, `fullscreen`, `clientLog`), `data/sprites.json`, Phaser.
