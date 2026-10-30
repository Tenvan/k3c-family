# tools/k3c-studio/frontend/src/stage/

## Responsibility
Wiederverwendbare Phaser-Bühne des Studios: Live-Vorschau einer Spielwelt (Welten Editor, Gebäude-Planer) und Figuren-Bühne (Monarch, Gegner) mit den Renderern des Spiels.

## Design
- `PreviewStage.ts`: Phaser-Spiel im Element; lädt die Grafiken wie die Lade-Szene des Spiels (Basis-URL eine Ebene über `/studio/`), zeichnet eine `World` mit `StageView`/`WorldRenderer` aus `@game` (`show`), Figuren über `showFigures`/`setControl`, Kamera von außen (`setPosition`: Zoom und linke Kante, Boden unten, oben die HUD-Leiste `hudPx`), Ziehen und Mausrad melden nur `onPan`/`onWheel`, `setLook` für Tageszeit und Hintergrund-Schichten; zwei `ResizeObserver` (Element: `refreshToParent`, `document.body`: `settle`); Fassade `PreviewStage` mit `destroy`.
- `figureLayer.ts`: Figuren-Ebene der Bühne (`FigureLayer`, Host = Phaser-Szene): Figuren, Klick-Treffer (`click`), Markierung, Kamera-Anpassung, Zoom und Wiedergabe; `PreviewStage` delegiert `setFigureClick`/`setFigureMark`/`setFigureZoom`/`setControl` dorthin.
- `hudBar.ts`: Höhen der Bühne wie im Spiel: `hudPx` (Leiste des globalen HUDs), `worldPx` (Weltausschnitt), `stagePx` (Summe) je Maßstab (Pixel je Unit).
- `stageResize.ts`: `refreshToParent` (Elterngröße neu messen, dann skalieren) und `settleToParent` (nur bei geänderter Größe oder Lage); Typ `ParentScale` für Phasers ScaleManager.
- `stageLogic.ts`: reine Logik: Hub-Stufen (`hubLevelChoices`), Gebäude-Liste (`buildingToggles`, `buildingGroups` nach Art mit Gruppenzustand), `previewArgsOf` (Optionen inkl. Plan-Quelle `PlanSource`, Planer-Kandidat, Platz-Zustände `SiteLook`), Hintergrund-Schalter (`backgroundToggles`), `worldOf`.
- `timeOfDay.ts`: vier Tageszeit-Einträge auf drei Sim-Phasen (Morgengrauen = Nacht mit Fortschritt 0,9 + 0,1·p), `brightnessOf`.
- `LivePreview.tsx`: Komponente; Hub-Stufe (eigen oder aus `PlanerSource`) und Tageszeit als `SelectField`, Fortschritt als Radix `Slider`, Hintergrund-Schalter (`CheckField`); ohne `treeHost` die Chip-Leiste `BuildingChips` über der Bühne, mit `treeHost` der `BuildingInspector`; mit `planer.onPick` gehen Wahlen in Bühne und Chips an die Seite, die Zustands-Karte der gewählten Plätze per `lookHost` (B-670); `backend.worldPreview` (nur die letzte Antwort zählt, `usePreviewWorld`); lädt Phaser beim ersten Rendern per `import()` (`useStage`); `stageStyle` setzt die Höhe über `stagePx`; Steuerung und Gebäude-Baum optional per Portal in `controlsHost`/`treeHost` der Seite; `position`, `onPan`, `onWheel`, `onSites` kommen von der Seite.
- `BuildingChips.tsx`: Arten-Chips mit Farbe je Art (Klick blendet aus/ein) und Lageleiste mit einer farbigen Marke je Platz an seiner Stelle (Klick wählt; B-670). `BuildingInspector.tsx`: Reiter Gebäude des Welten Editors (B-666) und `SelectionCard` (Baustufe, Zustand der gewählten Plätze).
- Phaser stammt aus der Wurzel-`node_modules` (eine Instanz für Studio und `src/scenes/`).
- Tests: `stageLogic.test.ts`, `timeOfDay.test.ts`, `hudBar.test.ts`, `stageResize.test.ts`, `LivePreview.test.tsx`, `BuildingInspector.test.tsx`, `LookControls.test.tsx`.

## Flow
1. `LivePreview` fragt `worldPreview(previewArgsOf(...))` → `worldOf` → `PreviewStage.show(world)`; bis die neue Antwort da ist, bleibt die letzte Welt stehen.
2. Position und Zoom setzt die Seite per `setPosition`; Ziehen meldet `onPan` zurück, das Rad `onWheel`.
3. Optionen (Tageszeit, Schichten, Gebäude, Platz-Zustände) ändern die Anfrage oder `setLook`.
4. Nach jedem Rendern der Seite prüft `settle` Größe und Lage; Layout-Änderungen fängt der Beobachter, die Bühne skaliert nach.

## Integration
- Konsumenten: `../world/WorldPage`, `../planner/PlannerPage`, `../playback/FigureStage`.
- Abhängigkeiten: `../api`, `../playback/*` (Figuren), `@webkit/useDebounced`, `@game/scenes/*`, `@game/core/constants`, `@game/model/*`, `@game/online/*`, `@data/hub.json`, `@data/buildings.json`.
