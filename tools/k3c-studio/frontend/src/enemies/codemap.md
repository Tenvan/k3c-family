# tools/k3c-studio/frontend/src/enemies/

## Responsibility
Seite Gegner (B-464, WZ11): alle Gegnertypen und Bosse nach Biom und Tiefe mit den Animationen des Spiels, mit Merkmalen, Lücken und Grafik-Auswahl je Gegner-ID.

## Design
- `enemyLogic.ts`: reine Logik; Daten aus `@data/enemies.json`, `@data/sprites.json`, Biome aus `@game/model/biome`. `groups()` gruppiert nach Portal/Nacht, Biom und Tiefe, `cardOf` baut Karten mit Merkmalen (`traits`, `extras`) und Lücken, `ENEMY_ACTIONS` (Getroffen, Sterben) und `enemyItems` erzeugen `FigureItem`s für die Bühne (inkl. Monarch-Vergleich auf derselben Bodenlinie).
- `bossLogic.ts`: Bosse aus `@data/bosses.json` (5 Minibosse je Tiefe, Endboss `crystalHeartWarden`); `bossCards`, `phaseTexts`, `displaysOf` mit Lücken für Darstellungen, die das Spiel nicht zeichnet (Warnkreis, Boss-Leiste, Banner).
- `EnemiesPage.tsx`: eine Leiste (Gruppe, Modus-Segment, Monarch, Wiedergabe, Lücken-Chip), Bühne, darunter `EnemyTable.tsx` (Lückenzahl je Zeile) und `EnemyInspector.tsx` (Merkmale, Lückenzeilen, Phasen mit ▶, Grafik-Auswahl); Zustand in `useEnemies.ts` (Slot-Register nur lesend). Klick auf Zeile oder Figur wählt (B-672).
- `abilityLogic.ts`: Angriff (Treffer, Geschoss der Fernkämpfer) und Boss-Fähigkeiten (`aoe` Warnkreis, `summon` Figuren, `shardThrow` Geschoss; `flameTrail` und Unbekanntes als Lücke) als Zeitachsen-Effekte.
- Tests: `enemyLogic.test.ts`, `bossLogic.test.ts`, `EnemiesPage.test.tsx`.

## Flow
1. Seite lädt die Slots, baut die Gruppen und zeigt je Gruppe Karten.
2. Auswahl von Gruppe, Aktion und Grafik → `enemyItems(...)` → `FigureStage` zeichnet mit den Spiel-Animationen.
3. Lücken (Merkmal, Boss-Darstellung, fehlende Grafik) erscheinen als `GapBadge`.

## Integration
- Abhängigkeiten: `../playback/` (Bühne, Steuerung, `Gap`), `../monarch/monarchLogic` (`slotOptions`), `../api` (`assetsList`), `@data/*`, `@game/model/biome`.
- Eingehängt in `../App.tsx` (Seite `gegner`, Komponente `EnemiesPage`).
