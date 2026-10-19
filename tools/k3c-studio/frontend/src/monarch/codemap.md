# tools/k3c-studio/frontend/src/monarch/

## Responsibility
Seite Monarch (B-463, WZ10): zeigt den Monarchen für beide Spielerfarben mit Reittier, Bewegung, Animation und Aktionen so, wie das Spiel sie zeichnet, mit Grafik-Auswahl aus den Slots und Skill-Liste.

## Design
- `monarchLogic.ts`: reine Logik; Daten aus `@data/monarch.json`, `@data/sprites.json`. `MOUNT_KEYS`, `MOVES`, `PLAYERS` (Spielerfarben), `MONARCH_ANIMS`, `monarchItems` (→ `FigureItem`s), `slotOptions`/`monarchSlotId`/`mountSlotId` (Präferenzen je Slot, Lücke bei fehlender Grafik).
- `actions.ts`: Aktionen Schlagen, Münzen, Getroffen, Tod, Respawn (`ACTIONS`, `actionOf`, `effectMs`) mit Effekten aus `@game/scenes/effects`; `skillRows` die Skill-Liste mit Lücken (was das Spiel nicht darstellt).
- `MonarchPage.tsx`: Hook `useMonarch` (Reittier, Bewegung, Animation, Aktion, Wiedergabe, Grafik-Auswahl; `items` ändern sich nur bei geänderter Eingabe), Bühne und Steuerung aus `playback/`; `SkillList.tsx` zeigt die Skills.
- Tests: `monarchLogic.test.ts`, `actions.test.ts`, `MonarchPage.test.tsx`.

## Flow
1. Seite lädt das Slot-Register (nur lesen) → `slotOptions` je Spieler und Reittier.
2. Eingaben → `monarchItems(...)` → `FigureStage`; Aktion → Effekt und Deckkraft aus dem Spiel.
3. Fehlende Darstellungen erscheinen als Lücke, nie als Ersatz.

## Integration
- Abhängigkeiten: `../playback/`, `../api`, `@game/core/constants`, `@game/core/texts`, `@game/scenes/effects`, `@data/*`.
- `slotOptions` nutzt auch `../enemies/`; eingehängt in `../App.tsx` (Seite `monarch`).
