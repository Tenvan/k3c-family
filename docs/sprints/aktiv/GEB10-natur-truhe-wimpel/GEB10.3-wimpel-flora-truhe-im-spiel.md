# GEB10.3 · Wimpel in Spielerfarbe, Höhlen-Flora und Truhe im Spiel

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb10/3-wimpel-flora-truhe-im-spiel
- **Abhängig von:** GEB10.1, GEB10.2
- **Tickets:** B-454, B-455, B-456
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Im Spiel trägt ein markierter Knoten einen Wimpel in der Farbe des markierenden Spielers; Bäume sind hoch, außerhalb des Walds stehen Wurzeln und Höhlenbüsche; eine liegende Truhe ist geschlossen und öffnet sich kurz beim Aufheben. Die alten Höhle-Look-PNGs der Bäume sind gelöscht.

## Kontext

- **Spec:** GEB10 AC-01, AC-02, AC-03 (B-454/AC-02, B-455/AC-02, B-456/AC-01), Entscheidungen 3–6 der Sprint-README.
- `src/scenes/render/natureSprites.ts` (`NATURE_TEXTURES`, `treeTexture`, `nodeSprite`, `natureLook`), `worldSprites.ts` (`pickup:chest`), `buildingSprites.ts` (`grafik:pappel` der Plantage), `worldRenderer.ts` (`createNode`: Fahne als Rechteck in Rot über der Bildhöhe, `updateNode`; `createPickup`; `Layer` entfernt Views per `destroy`).
- Spielerfarbe: `PLAYER_COLORS` in `src/core/constants.ts` (wie der Punkt unter dem Monarchen). Der Snapshot kennt nur `ResourceNode.marked`; das Ereignis `coinGive` (`to: 'mark'`, `player`, `x` = Ort des Spielers, Reichweite `data/economy.json` › `payRangeUnits` = 2) nennt den Spieler. Ereignisse kommen in `GameScene.spawnEffects` je Stufe an.
- Sichtbare Welt über dem Boden: 480 px; Wurzeln (15 Kacheln) reichen bis an den oberen Rand, der Wimpel sitzt deshalb höchstens 11,25 Kacheln hoch.

## Erlaubte Dateien

- `src/scenes/render/` (`natureSprites.ts`, `worldSprites.ts`, `buildingSprites.ts`, `worldRenderer.ts`, neue Datei für Wimpel-Farbe mit Test, zugehörige Tests, `codemap.md`)
- `src/scenes/GameScene.ts` (nur Weitergabe der Ereignisse an den Renderer)
- `data/assets.json` (nur `node:tree`, `node:bush`, `pickup:chest`)
- alte PNGs `public/grafik/gebaeude/{tanne,pappel,birke}-{2,3}-{1,2}-*.png`, `docs/assets/zuordnung-welt.md`, Tests, die genau diese Dateien nennen
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Feld `markedBy` im Snapshot (SIM/Protokoll, Ticket); mehr Bäume oder Höhlenbüsche im Level-Generator (REG/SIM, Ticket); Sichtprüfung im Spiel (GEB10.5).

## Schritte

1. Reine Wimpel-Logik mit Vitest: Farbe je Spieler aus `PLAYER_COLORS`, neutrale Farbe für unbekannt, Zuordnung `coinGive`-Ereignis → nächster Knoten in Reichweite.
2. Renderer: Wimpel (Dreieck) statt Rechteck, Farbe aus der Zuordnung, Wimpel höchstens 11,25 Kacheln hoch; `GameScene` reicht die Ereignisse der Stufe an den Renderer.
3. `natureSprites.ts`: Bäume nur Wald-Look, außerhalb des Walds Wurzel-Formen und Höhlenbusch; Plantage-Pappel hinter der Farm.
4. Truhe: `pickup:chest` zeigt `truhe-geschlossen`, beim Verschwinden kurz `truhe` mit Ausblenden.
5. Alte PNGs löschen (per Suche unbenutzt), Register `data/assets.json` und `docs/assets/zuordnung-welt.md` nachziehen.

## Fertig, wenn

- [ ] AC-01: `natureSprites.test.ts` prüft Wald → Bäume, Höhle und Mine → Wurzeln und Höhlenbusch; alle Texturen existieren als Datei.
- [ ] AC-02: `pickup:chest` → `truhe-geschlossen-32.png`, offene Truhe beim Aufheben (Test der Zuordnung).
- [ ] AC-03: Vitest für die Farbwahl (Spieler 1 und 2 verschieden, unbekannt neutral, Zuordnung zum nächsten Knoten).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
