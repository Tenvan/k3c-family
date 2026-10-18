# tools/k3c-studio/frontend/src/gegner/

## Responsibility
Seite Gegner (B-464, WZ11): alle Gegnertypen und Bosse nach Biom und Tiefe mit den Animationen des Spiels, mit Merkmalen, Lücken und Grafik-Auswahl je Gegner-ID.

## Design
- `enemyLogic.ts`: reine Logik; Daten aus `@data/enemies.json`, `@data/sprites.json`, Biome aus `@game/model/biome`. `groups()` gruppiert nach Portal/Nacht, Biom und Tiefe, `cardOf` baut Karten mit Merkmalen (`traits`, `extras`) und Lücken, `ENEMY_ACTIONS` (Getroffen, Sterben) und `enemyItems` erzeugen `FigureItem`s für die Bühne (inkl. Monarch-Vergleich auf derselben Bodenlinie).
- `bossLogic.ts`: Bosse aus `@data/bosses.json` (5 Minibosse je Tiefe, Endboss `crystalHeartWarden`); `bossCards`, `phaseTexts`, `displaysOf` mit Lücken für Darstellungen, die das Spiel nicht zeichnet (Warnkreis, Boss-Leiste, Banner).
- `GegnerPage.tsx`: Seite mit Gruppenwahl, Kartenliste `CardRow`, Aktions- und Animationswahl, Wiedergabe über `abspiel/`; liest das Slot-Register nur lesend (`useSlots`) für die Auswahl der Grafik, ohne es zu ändern.
- Tests: `enemyLogic.test.ts`, `bossLogic.test.ts`, `GegnerPage.test.tsx`.

## Flow
1. Seite lädt die Slots, baut die Gruppen und zeigt je Gruppe Karten.
2. Auswahl von Gruppe, Aktion und Grafik → `enemyItems(...)` → `FigureStage` zeichnet mit den Spiel-Animationen.
3. Lücken (Merkmal, Boss-Darstellung, fehlende Grafik) erscheinen als `GapBadge`.

## Integration
- Abhängigkeiten: `../abspiel/` (Bühne, Steuerung, `Gap`), `../monarch/monarchLogic` (`slotOptions`), `../api` (`assetsList`), `@data/*`, `@game/model/biome`.
- Eingehängt in `../App.tsx` (Seite `gegner`).
