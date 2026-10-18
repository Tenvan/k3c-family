# tools/k3c-studio/frontend/src/stage/

## Responsibility
Wiederverwendbare Phaser-Bühne des Studios: Live-Vorschau einer Spielwelt (Welten Editor, Gebäude-Planer) und Figuren-Bühne (Monarch, Gegner) mit den Renderern des Spiels.

## Design
- `PreviewStage.ts`: Phaser-Spiel im Element; lädt die Grafiken wie die Lade-Szene des Spiels, zeichnet eine `World` mit `StageView`/`WorldRenderer` aus `@game` (`show`), Figuren über `showFigures`/`setControl`, Kamera von außen (`setPosition`: Zoom und linke Kante, Boden unten), Ziehen, Mausrad und Pfeile melden nur `onPan`, Größe per `ResizeObserver`, `setLook` für Tageszeit und Hintergrund-Schichten; Fassade `PreviewStage` mit `destroy`.
- `stageLogic.ts`: reine Logik: Hub-Stufen (`hubLevelChoices`), Gebäude-Liste (`buildingToggles`, `buildingGroups` nach Art), `previewArgsOf`, Hintergrund-Schalter (`backgroundToggles`), `worldOf`.
- `timeOfDay.ts`: vier Tageszeit-Einträge auf drei Sim-Phasen (Morgengrauen = Nacht mit Fortschritt 0,9 + 0,1·p), `brightnessOf`.
- `LivePreview.tsx`: Komponente; Hub-Stufe, Tageszeit, Hintergrund-Schalter, Gebäude-Filter `BuildingFilter.tsx`; `backend.worldPreview` (nur die letzte Antwort zählt); lädt Phaser beim ersten Rendern per `import()`; `position`, `onPan`, `onSites` kommen von der Seite.
- Phaser stammt aus der Wurzel-`node_modules` (eine Instanz für Studio und `src/scenes/`).
- Tests: `stageLogic.test.ts`, `timeOfDay.test.ts`, `LivePreview.test.tsx`, `BuildingFilter.test.tsx`, `LookControls.test.tsx`.

## Flow
1. `LivePreview` fragt `worldPreview(args)` → `worldOf` → `PreviewStage.show(world)`.
2. Position und Zoom setzt die Seite per `setPosition`; Ziehen meldet `onPan` zurück.
3. Optionen (Tageszeit, Schichten, Gebäude) ändern die Anfrage oder `setLook`.

## Integration
- Konsumenten: `../world/WorldPage`, `../planer/PlanerPage`, `../abspiel/FigureStage`.
- Abhängigkeiten: `../api`, `@game/scenes/*`, `@game/model/*`, `@data/hub.json`.
