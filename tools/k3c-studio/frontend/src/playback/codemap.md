# tools/k3c-studio/frontend/src/playback/

## Responsibility
Wiederverwendbare Abspiel-Bausteine für Figuren-Seiten (Monarch, Gegner; B-463, WZ10.1): Entscheidung, welche Animation, Pose und Größe eine Figur bekommt, Wiedergabe-Steuerung und Zeichnen mit den Definitionen des Spiels.

## Design
- `figure.ts`: reine Logik ohne Phaser und DOM. `FigureSpec` (Sheet, Animation, tint, scale, alpha, y), `animInfo`/`resolveAnim` (Bildzahl, Bildrate, Ursprung aus `@data/sprites.json`), `Gap` (Lücke = das Spiel stellt es nicht dar, mit Grund), Wiedergabe-Zustand `Control` (Schleife, Einzelbild, Tempo `TEMPO_STEPS`) mit `setTempo`, `toggleLoop`, `stepFrame`, `resume`, `effectiveFps`; Reittier-Zuordnung (`riderContext`, `mountsWithSheet`, `mountPoseOf` über `@game/scenes/render/mountPose`), Einheiten (`unitsToPx`, `figureTop`, `unitTicks`).
- `figureView.ts`: Phaser-Anteil. `FigureView` zeichnet `FigureItem`s mit `makeSprite`/`playAnim`/`face` aus `@game/scenes/render/sprites` und den Effekten aus `@game/scenes/effects/effects`; `loadFigureSheets` lädt die Sheets, `drawScale` bestimmt den Maßstab. Nicht headless prüfbar.
- `FigureStage.tsx`: React-Hülle um `PreviewStage` (aus `../stage/`); Phaser wird beim ersten Zeichnen per `import()` geladen, eine Instanz je Seite; `items` ersetzt die Figuren, `control` steuert die Wiedergabe, `onInfo` meldet Lücke und Bildzahl je Figur, veraltete Antworten werden verworfen.
- `PlaybackControls.tsx`: Schaltflächen Schleife, Bild zurück/vor, Weiter, Tempo; `GapBadge` zeigt die Marke „Lücke“.
- Tests: `figure.test.ts`, `playback.test.tsx`.

## Flow
1. Eine Seite baut `FigureItem`s aus ihrer Logik (`monarchItems`, `enemyItems`) und reicht sie mit `Control` an `FigureStage`.
2. `FigureStage` lädt die Bühne und ruft `showFigures(items)`; `FigureView` zeichnet und spielt die Animation.
3. `PlaybackControls` ändert `Control` über die reinen Funktionen aus `figure.ts`, die Bühne übernimmt es mit `setControl`.

## Integration
- Konsumenten: `../monarch/`, `../enemies/`.
- Abhängigkeiten: `../stage/PreviewStage`, `@game/scenes/*`, `@game/core/constants`, `@data/sprites.json`, `@webkit/atoms` (Radix Themes: `Button`, `SelectField`, `Badge` für `GapBadge`).
