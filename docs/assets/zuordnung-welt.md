# Grafik-Zuordnung: Ausbau, Materialien, Welt, Reittiere, Icons

Teil der Zuordnungstabelle aus B-161 (Sprint GR1, Session GR1.3, Kriterium AC-02). Stilbeschluss (Q13) und Pack-Tabelle
stehen im Kopf von `docs/assets/zuordnung.md`; Spalten, Pack-Schreibweise und Lücken wie in `docs/assets/zuordnung-objekte.md`.
Viele dieser Objekte gibt es noch nicht im Code (B-112, B-114); dann ist die Herkunft die Regel in `docs/rules/`.

- **Objekt-IDs** (eindeutig, von `src/tools/zuordnung.test.ts` geprüft):
  `hub:<1–5>` Hub-Stufe, `wall:<1–5>` und `tower:<1–5>` Mauer- und Turm-Stufe (`tower:5` = Zaubertum),
  `material:<wood|stone|copper|iron|crystal>`, `vein:<stone|copper|iron|crystal>` Ader je Material,
  `plantation`, `portal`, `coin`, `mount:<ID aus data/sprites.json › mounts>`,
  `node:<Art aus data/economy.json › gatherables bzw. data/biomes/*.json › resourcesPerChunk>`, `camp:recruit`, `exit`,
  `pickup:<chest|skillPoint>` (Pickup-Arten aus `src/model/types.ts` › `Pickup`),
  `icon:<Dateiname ohne .png der Gruppe icons in public/grafik/index.json>`, `skill-icon`, `boss`,
  `bg:<id aus data/biomes/*.json>`.
- **Frames im Erz-Sheet** `ressourcen/Stones_ores_gems_without_grass.png` (112×144, 7 Spalten × 9 Zeilen à 16×16):
  Zeile 1 Stein, 2 Kupfer, 3 helles Erz (als Eisen gedeutet), 9 Kristall (türkis); Spalte 1–5 Ader im Fels,
  Spalte 6 Brocken/Edelstein, Spalte 7 Barren (nur Zeile 1–5).
- **Gebäude-Palette:** Gebäude orientieren sich an `gothicvania-town` (Dunkelviolett); Teile aus `sunnyland-fort-of-illusion`
  tragen den Vermerk „Palette an gothicvania-town angleichen“ (Entscheidung 🧑 GR1.1).
- **Lücken „später“** gehören zu Mechaniken, die es noch nicht gibt (B-161 › Nicht-Ziele), und nennen das Mechanik-Ticket.

## Hub-Stufen

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `hub:1` | `docs/rules/materialien-gebaeude.md` § 2 | `grafik/sunnyland-fort-of-illusion` | wie `castle` (GR1.2): `ebenen/tileset.png` (Burgmauer mit Zinnen), `props/banner.png` | 16 px, SunnyLand-Fort (Blaugrau), ×2; Vermerk: Palette an gothicvania-town angleichen, Entscheidung 🧑 GR1.1 | CC0 1.0 | zugeordnet |
| `hub:2` | `docs/rules/materialien-gebaeude.md` § 2 | – | – (Burg-Ausbau Stein fehlt) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Ausbau-Grafik, B-162** |
| `hub:3` | `docs/rules/materialien-gebaeude.md` § 2 | – | – (Burg-Ausbau Kupfer fehlt) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Ausbau-Grafik, B-162** |
| `hub:4` | `docs/rules/materialien-gebaeude.md` § 2 | – | – (Burg-Ausbau Eisen fehlt) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Ausbau-Grafik, B-162** |
| `hub:5` | `docs/rules/materialien-gebaeude.md` § 2 | – | – (Burg-Ausbau Kristall fehlt) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Ausbau-Grafik, B-162** |

## Mauer-Stufen

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `wall:1` | `docs/rules/materialien-gebaeude.md` § 3.1 | – | – (keine Holzmauer im Bestand; `wall` aus GR1.2 zeigt eine Putz-/Steinmauer) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Holzmauer, B-162** |
| `wall:2` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/gothicvania-town` | wie `wall` (GR1.2): `tileset-einzeln/wall.png`, `tileset-einzeln/wall-b.png` (16×16) | 16 px, Gothicvania (Dunkelviolett), ×2 | CC0 1.0 | zugeordnet |
| `wall:3` | `docs/rules/materialien-gebaeude.md` § 3.1 | – | – (Kandidat: Palettentausch von `wall:2`, Q13 erlaubt Palette) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Kupfermauer, B-162** |
| `wall:4` | `docs/rules/materialien-gebaeude.md` § 3.1 | – | – (Kandidat: Palettentausch von `wall:2`) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Eisenmauer, B-162** |
| `wall:5` | `docs/rules/materialien-gebaeude.md` § 3.1 | – | – (Warped Caves nur als Hintergrund, GR1.1) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Kristallmauer, B-162** |

## Turm-Stufen

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `tower:1` | `docs/rules/materialien-gebaeude.md` § 3.1 | – | – (kein Holzturm im Bestand; `tower` aus GR1.2 ist ein Steinturm) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: kein Holzturm, B-162** |
| `tower:2` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/sunnyland-fort-of-illusion` | wie `tower` (GR1.2): `ebenen/front.png` (Turm mit Kegeldach, 112×128) | 16 px, SunnyLand-Fort (Blaugrau), ×2; Vermerk: Palette an gothicvania-town angleichen, Entscheidung 🧑 GR1.1 | CC0 1.0 | zugeordnet |
| `tower:3` | `docs/rules/materialien-gebaeude.md` § 3.1 | – | – (Kandidat: Palettentausch von `tower:2`) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: kein Kupferturm, B-162** |
| `tower:4` | `docs/rules/materialien-gebaeude.md` § 3.1 | – | – (Kandidat: Palettentausch von `tower:2`) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: kein Eisenturm, B-162** |
| `tower:5` | `docs/rules/materialien-gebaeude.md` § 3.1 (Zaubertum) | – | – | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: kein Zaubertum, B-162** |

## Materialien und Adern

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `material:wood` | `docs/rules/materialien-gebaeude.md` § 1 | – | – (kein Holz-Symbol; Kiste und Fass aus gothicvania-town sind keine Stämme) | Ziel: 16 px, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: kein Holz-Symbol, B-162** |
| `material:stone` | `data/economy.json` › `gatherables.rock` | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 1, Spalte 6 (Brocken) und 7 (Block) | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `material:copper` | `data/economy.json` › `gatherables.copperOre` | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 2, Spalte 6 (Brocken) und 7 (Barren) | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `material:iron` | `docs/rules/materialien-gebaeude.md` § 1 | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 3, Spalte 6 (Brocken) und 7 (Barren), helles Erz als Eisen gedeutet | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `material:crystal` | `docs/rules/materialien-gebaeude.md` § 1 | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 9, Spalte 6 (türkiser Edelstein) | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `vein:stone` | `docs/rules/materialien-gebaeude.md` § 1 (Adern) | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 1, Spalte 1–5 | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `vein:copper` | `docs/rules/materialien-gebaeude.md` § 1 (Adern) | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 2, Spalte 1–5 | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `vein:iron` | `docs/rules/materialien-gebaeude.md` § 1 (Adern) | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 3, Spalte 1–5 (helles Erz als Eisen gedeutet) | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `vein:crystal` | `docs/rules/materialien-gebaeude.md` § 1 (Adern) | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 9, Spalte 1–5 | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `plantation` | `docs/rules/materialien-gebaeude.md` § 1 (Farm-Plantage) | – | – (keine Wachstumsstufen; Kandidaten `gotthicvania-swamp/umgebung/trees.png`, `sunnyland-tall-forest-environment/props/Plant.png`) | Ziel: 16 px, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Setzlinge und Wachstumsstufen, B-162** |

## Truhen, Portale, Münzen, Ausgang

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `pickup:chest` | `data/economy.json` › `chestGold`, Pickup `chest` | `grafik/gold-treasure-icons-16x16` | `icons/8.png` (offene Truhe mit Gold, 16×16); geschlossene Truhe fehlt | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `portal` | `data/biomes/*.json` › `portals` | `grafik/portals-32-x-48` | `portale/portalsSpriteSheet.png`, Frame 32×48, je Zeile eine Farbe (blau, rot, orange, grau) mit 4 Frames | 32 px nativ, Rusher_go (gesättigt), ×1 | CC0 1.0 | zugeordnet |
| `coin` | `data/economy.json` › `purse` | `grafik/16x16-small-and-medium-coin-animation` | `muenzen/sCoins_1.png`, `muenzen/sCoins_2.png` (Drehung, je 8 Frames 16×16) | 16 px, WolfTech (gelb), ×2 | CC0 1.0 | zugeordnet |
| `exit` | `src/scenes/worldRenderer.ts` › `drawStatic` (Level-Objekt `exit`) | `grafik/sunnyland-fort-of-illusion` | `props/door.png` (offener Torbogen, 96×80); dieselbe Datei wie `gate` (offen) | 16 px, SunnyLand-Fort (Blaugrau), ×2; Vermerk: Palette an gothicvania-town angleichen, Entscheidung 🧑 GR1.1 | CC0 1.0 | zugeordnet |
| `pickup:skillPoint` | `src/model/types.ts` › `Pickup` (`skillPoint`) | – | – (kein Symbol für Skillpunkte) | Ziel: 16 px, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: kein Skillpunkt-Symbol, B-162** |

## Ressourcen und Rekrutierungslager

Endliche Level-Objekte, die `src/scenes/worldRenderer.ts` heute als Formen zeichnet (`createNode`, `drawStatic`).

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `node:tree` | `data/economy.json` › `gatherables.tree` | `grafik/gotthicvania-swamp` | `umgebung/trees.png` (knorriger Baum, 288×208) | 16 px, Gothicvania Swamp (Dunkelgrün, entsättigt), ×2 | CC0 1.0 | zugeordnet |
| `node:rock` | `data/economy.json` › `gatherables.rock` | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 1, Spalte 2 (Fels ohne Erz; Ader `vein:stone` nutzt dieselbe Zeile) | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `node:copperOre` | `data/economy.json` › `gatherables.copperOre` | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 2, Spalte 1 (Ader `vein:copper` nutzt dieselbe Zeile) | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `node:bush` | `data/biomes/forest.json` › `resourcesPerChunk` (Deko) | `grafik/sunnyland-tall-forest-environment` | `props/Plant.png` (Farn, 42×27) | 16 px, SunnyLand (Grün, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `camp:recruit` | `data/economy.json` › `recruitCamp` | – | – (Zelt und Lagerfeuer; laut B-161 kein Treffer im Bestand) | Ziel: 16 px, Gothicvania, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: kein Zelt und Lagerfeuer, B-162** |

## Reittiere

Stil wie bei Figuren in `zuordnung-objekte.md`; Skalierung ist `scale` des Reittiers in `data/sprites.json` › `mounts`.

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `mount:horse` | `data/sprites.json` › `mounts` | `sprites/horse` | `idle/run.png`, Frame 100×65 | Figur 61 px, LPC, ×2,2; Vermerk: Skalierung nicht ganzzahlig, B-251 | CC-BY 3.0 | zugeordnet |
| `mount:horseWalk` | `data/sprites.json` › `mounts` | `sprites/horse-walk` | `idle/run.png`, Frame 67×64 | Figur 61 px, LPC, ×2,2; Vermerk: Skalierung nicht ganzzahlig, B-251 | CC-BY 3.0 | zugeordnet |
| `mount:blackHorse` | `data/sprites.json` › `mounts` | `sprites/horse` | `idle/run.png`, Frame 100×65, Tönung `#5a4f55` | Figur 61 px, LPC, ×2,2; Vermerk: Skalierung nicht ganzzahlig, B-251 | CC-BY 3.0 | zugeordnet |
| `mount:whiteHorse` | `data/sprites.json` › `mounts` | `sprites/white-horse` | `idle/run.png`, Frame 64×62 | Figur 60 px, LPC, ×2,4; Vermerk: Skalierung nicht ganzzahlig, B-251 | CC-BY 3.0 | zugeordnet |
| `mount:unicorn` | `data/sprites.json` › `mounts` | `sprites/unicorn` | `idle/run.png`, Frame 64×64 | Figur 62 px, LPC, ×2,4; Vermerk: Skalierung nicht ganzzahlig, B-251 | CC-BY 3.0 | zugeordnet |
| `mount:pegasus` | `data/sprites.json` › `mounts` | `sprites/pegasus` | `idle/run.png`, Frame 64×62 | Figur 60 px, LPC, ×2,4; Vermerk: Skalierung nicht ganzzahlig, B-251 | CC-BY 3.0 | zugeordnet |
| `mount:elephant` | `data/sprites.json` › `mounts` | `sprites/elephant` | `idle/run.png`, Frame 94×59 | Figur 55 px, LPC, ×2 | CC-BY 3.0 | zugeordnet |
| `mount:stag` | `data/sprites.json` › `mounts` | `sprites/stag` | `idle/run.png`, Frame 64×61 | Figur 56 px, LPC, ×2,4; Vermerk: Skalierung nicht ganzzahlig, B-251 | CC-BY 3.0 | zugeordnet |
| `mount:wolf` | `data/sprites.json` › `mounts` | `sprites/lpc-wolf` | `idle/run.png`, Frame 64×33 | Figur 32 px, LPC, ×2,6; Vermerk: Skalierung nicht ganzzahlig, B-251 | CC-BY 3.0 | zugeordnet |
| `mount:hellhound` | `data/sprites.json` › `mounts` | `sprites/lpc-hellhound` | `idle/run.png`, Frame 64×33 | Figur 32 px, LPC, ×2,6; Vermerk: Skalierung nicht ganzzahlig, B-251 | CC-BY 3.0 | zugeordnet |
| `mount:nightmare` | `data/sprites.json` › `mounts` | `sprites/nightmare` | `idle/run.png`, Frame 121×90 | Figur 81 px, Gothicvania, ×1,75; Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |
| `mount:hellHound` | `data/sprites.json` › `mounts` | `sprites/hell-hound` | `idle/run.png`, Frame 50×44 | Figur 24 px, Gothicvania, ×3,5; Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |
| `mount:whiteWolf` | `data/sprites.json` › `mounts` | `sprites/wolf` | `idle/run.png`, Frame 48×22 | Figur 18 px, Gothicvania, ×3,8; Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |

## Icons

Je Bild der Gruppe `icons` in `public/grafik/index.json` eine Zeile; die Verwendung ist ein Vorschlag für den Einbau (B-010).

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `icon:1` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/1.png` (eine flache Münze), Vorschlag: Gold-Anzeige im HUD | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `icon:2` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/2.png` (Münze von vorn) | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `icon:3` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/3.png` (kleiner Münzstapel) | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `icon:4` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/4.png` (zwei Münzstapel) | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `icon:5` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/5.png` (Münzhaufen), Vorschlag: Beutel voll | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `icon:6` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/6.png` (Pokal) | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `icon:7` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/7.png` (Goldbarren) | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `icon:8` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/8.png` (offene Truhe), siehe `pickup:chest` | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `icon:9` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/9.png` (Krone), Vorschlag: Monarch bzw. Krone im HUD | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `icon:10` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/10.png` (gekreuzte Schwerter mit Münzen) | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `icon:sheet` | `public/grafik/index.json` › `icons` | `grafik/gold-treasure-icons-16x16` | `icons/sheet.png` (64×64, dieselben 10 Icons als Sheet) | 16 px, Bonsaiheldin (gelb/braun), ×2 | CC0 1.0 | zugeordnet |
| `skill-icon` | `docs/backlog/B-124-skill-menue-tasten.md` (Skill-Slots) | – | – (Skills haben noch keine Icons) | Ziel: 16 px, ×2 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: später, B-124** |

## Bosse

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `boss` | `docs/backlog/B-130-bosse.md` (Bosse fehlen in `data/`) | – | – | Ziel: Figur wie Gegner, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: später, B-130** |

## Hintergründe je Biom

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `bg:forest` | `data/biomes/forest.json` | `grafik/forest-background`, `grafik/sunnyland-tall-forest-environment` | `ebenen/parallax-forest-*.png` (4 Ebenen 272×160); Alternative `ebenen/back.png`, `far.png`, `middle.png` (Tall Forest) | Hintergrund, ansimuz (Braun/Orange bzw. Grün), ×2; Palettenbruch bei Hintergründen erlaubt (Q13) | CC0 1.0 | zugeordnet |
| `bg:cave` | `data/biomes/cave.json` | `grafik/blue-cave-background`, `grafik/warped-caves-pixel-art-pack` | `ebenen/startcavebg.png` (400×225); Alternative `ebenen/background.png`, `ebenen/middleground.png` (Warped Caves, nur als Höhlen-Hintergrund, GR1.1) | Hintergrund, Blau malerisch bzw. Neon-Violett, ×2; Palettenbruch bei Hintergründen erlaubt (Q13) | CC0 1.0 bzw. CC BY 3.0 | zugeordnet |
| `bg:mine` | `data/biomes/mine.json` | – | – (laut B-161 kein Treffer im Bestand) | Ziel: Hintergrund ×2, Braun (`palette` in `mine.json`) | Ziel: CC0 oder CC-BY | **Lücke: kein Mine-Hintergrund, B-162** |
