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
| `hub:1` | `docs/rules/materialien-gebaeude.md` § 2 | `grafik/gebaeude` | `burg-1-32.png` (Holz) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB4 | eigen | zugeordnet |
| `hub:2` | `docs/rules/materialien-gebaeude.md` § 2 | `grafik/gebaeude` | `burg-2-32.png` (Stein) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB4 | eigen | zugeordnet |
| `hub:3` | `docs/rules/materialien-gebaeude.md` § 2 | `grafik/gebaeude` | `burg-3-32.png` (Kupfer) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB4 | eigen | zugeordnet |
| `hub:4` | `docs/rules/materialien-gebaeude.md` § 2 | `grafik/gebaeude` | `burg-4-32.png` (Eisen) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB4 | eigen | zugeordnet |
| `hub:5` | `docs/rules/materialien-gebaeude.md` § 2 | `grafik/gebaeude` | `burg-5-32.png` (Kristall) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB4 | eigen | zugeordnet |

## Mauer-Stufen

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `wall:1` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/gebaeude` | `mauer-1-32.png` (Holz) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB3 | eigen | zugeordnet |
| `wall:2` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/gebaeude` | `mauer-2-32.png` (Stein) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB3 | eigen | zugeordnet |
| `wall:3` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/gebaeude` | `mauer-3-32.png` (Kupfer) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB3 | eigen | zugeordnet |
| `wall:4` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/gebaeude` | `mauer-4-32.png` (Eisen) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB3 | eigen | zugeordnet |
| `wall:5` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/gebaeude` | `mauer-5-32.png` (Kristall) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB3 | eigen | zugeordnet |

## Turm-Stufen

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `tower:1` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/gebaeude` | `turm-1-32.png` (Holz) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB3 | eigen | zugeordnet |
| `tower:2` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/gebaeude` | `turm-2-32.png` (Stein) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB3 | eigen | zugeordnet |
| `tower:3` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/gebaeude` | `turm-3-32.png` (Kupfer) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB3 | eigen | zugeordnet |
| `tower:4` | `docs/rules/materialien-gebaeude.md` § 3.1 | `grafik/gebaeude` | `turm-4-32.png` (Eisen) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB3 | eigen | zugeordnet |
| `tower:5` | `docs/rules/materialien-gebaeude.md` § 3.1 (Zaubertum) | `grafik/gebaeude` | `turm-5-32.png` (Kristall, Zaubertum) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB3 | eigen | zugeordnet |
| `wall:back` | B-442 (Hintergrundmauer, Typ B) | `grafik/gebaeude` | `hintergrundmauer-<n>-32.png` (1 × 4 Kacheln, kachelbar; Stufe der äußeren Mauer) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1, Tiefe 1 hinter den Bauplätzen; GEB3 | eigen | zugeordnet |

## Materialien und Adern

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `material:wood` | `docs/rules/materialien-gebaeude.md` § 1 | `grafik/resource-icons` | `ressourcen/resources-icons.png` (50×91), Zeile 1 Holzstapel, links mit Kontur | ca. 24×13 px, Kutejnikov (Braun), ×2; Vermerk: kein 16-px-Raster; Entscheidung 🧑 GR2.2 | CC0 1.0 | zugeordnet |
| `material:stone` | `data/economy.json` › `gatherables.rock` | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 1, Spalte 6 (Brocken) und 7 (Block) | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `material:copper` | `data/economy.json` › `gatherables.copperOre` | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 2, Spalte 6 (Brocken) und 7 (Barren) | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `material:iron` | `docs/rules/materialien-gebaeude.md` § 1 | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 3, Spalte 6 (Brocken) und 7 (Barren), helles Erz als Eisen gedeutet | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `material:crystal` | `docs/rules/materialien-gebaeude.md` § 1 | `grafik/various-stones-and-oregem-veins-16x16` | Erz-Sheet Zeile 9, Spalte 6 (türkiser Edelstein) | 16 px, vico (hell, gesättigt), ×2 | CC0 1.0 | zugeordnet |
| `vein:stone` | `docs/rules/materialien-gebaeude.md` § 1 (Adern) | `grafik/gebaeude` | `ader-1-32.png` (2 × 2 Kacheln, Stufe 1 Stein) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `vein:copper` | `docs/rules/materialien-gebaeude.md` § 1 (Adern) | `grafik/gebaeude` | `ader-2-32.png` (Stufe 2 Kupfer) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `vein:iron` | `docs/rules/materialien-gebaeude.md` § 1 (Adern) | `grafik/gebaeude` | `ader-3-32.png` (Stufe 3 Eisen) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `vein:crystal` | `docs/rules/materialien-gebaeude.md` § 1 (Adern) | `grafik/gebaeude` | `ader-4-32.png` (Stufe 4 Kristall) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `plantation` | `docs/rules/materialien-gebaeude.md` § 1 (Farm-Plantage) | `grafik/gebaeude` | `setzling-32.png` (1 × 1 Kachel, leerer Platz; gewachsen: `pappel-32.png`) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB5 | eigen | zugeordnet |

## Truhen, Portale, Münzen, Ausgang

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `pickup:chest` | `data/economy.json` › `chestGold`, Pickup `chest` | `grafik/gebaeude` | `truhe-32.png` (1 × 1 Kachel, offene Truhe mit Gold) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `portal` | `data/biomes/*.json` › `portals` | `grafik/gebaeude` | `portal-32.png` (2 × 3 Kacheln, statisch) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `coin` | `data/economy.json` › `purse` | `grafik/16x16-small-and-medium-coin-animation` | `muenzen/sCoins_1.png`, `muenzen/sCoins_2.png` (Drehung, je 8 Frames 16×16) | 16 px, WolfTech (gelb), ×2 | CC0 1.0 | zugeordnet |
| `guide:coin` | `src/scenes/guideHints.ts` › `guideImage('coin')` (Hinweis „Münze aufheben“, B-294) | `grafik/gold-treasure-icons-16x16` | `icons/2.png` (Münze von vorn, Textur `grafik:coinIcon`); die Münze aus `sCoins_1.png` ist als Symbol zu klein und hat Markierungspixel | 16 px, Bonsaiheldin (gelb/braun), ×4 (Münze 6 px → 24 px, etwa Texthöhe) | CC0 1.0 | zugeordnet |
| `exit` | `src/scenes/worldRenderer.ts` › `drawStatic` (Level-Objekt `exit`) | `grafik/gebaeude` | `ausgang-32.png` (2 × 3 Kacheln, dunkle Öffnung, Beschriftung darüber) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `pickup:skillPoint` | `src/model/types.ts` › `Pickup` (`skillPoint`) | `grafik/item-ruby-banana-star` | `props/part-star.png` (Stern, 4 Frames 16×16) | 16 px, mieki256 (Gelb), ×2; Entscheidung 🧑 GR2.2 | CC0 1.0 | zugeordnet |

`guide:moon` (Nacht-Hinweis, B-370): kein Pack-Bild. Das Emoji 🌙 wird zur Laufzeit in die Canvas-Textur `guide:moon` gezeichnet (`src/scenes/guideOverlay.ts` › `ensureMoon`, Systemfont, 28 px, ×1, kein Asset, keine Lizenz). Test: `guideHints.test.ts`.

## Ressourcen und Rekrutierungslager

Level-Objekte, die `src/scenes/worldRenderer.ts` zeichnet (`createNode`, `drawStatic`); ab GEB6 eigene Grafik aus `tools/gebaeude` (Looks je Biom: Regeldatei § 6).

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `node:tree` | `data/economy.json` › `gatherables.tree` | `grafik/gebaeude` | `tanne`, `pappel`, `birke` und je zwei weitere Formen (`<art>-2`, `<art>-3`); Form aus der Position, Look Wald oder Höhle aus dem Biom (`src/scenes/natureSprites.ts`) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `node:rock` | `data/economy.json` › `gatherables.rock` | `grafik/gebaeude` | `felsen-1-32.png` (Wald), `felsen-2-32.png` (Höhle), `felsen-3-32.png` (Mine, auch Eisenstollen und Kristallhöhle) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `node:copperOre` | `data/economy.json` › `gatherables.copperOre` | `grafik/gebaeude` | `erzbrocken-32.png` (1 × 1 Kachel) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `node:bush` | `data/biomes/forest.json` › `resourcesPerChunk` (Deko) | `grafik/gebaeude` | `busch-32.png` (1 × 1 Kachel, nur Wald) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |
| `camp:recruit` | `data/economy.json` › `recruitCamp` | `grafik/gebaeude` | `rekrutierungslager-32.png` (3 × 2 Kacheln, Zelt und Feuerstelle in einem Bild, statisch) | eigene Grafik (`tools/gebaeude`), 32 px je Kachel, ×1; GEB6 | eigen | zugeordnet |

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
| `bg:mine` | `data/biomes/mine.json` | `grafik/warped-super-grotto-escape-pack` | `ebenen/back.png`, `ebenen/far.png`, `ebenen/middle.png` (3 Parallax-Ebenen, 240 px hoch), `ebenen/tileset.png` | Hintergrund, ansimuz (Dunkelviolett), ×2; Palettenbruch bei Hintergründen erlaubt; Entscheidung 🧑 GR2.2 | CC0 1.0 | zugeordnet |
| `bg:ironhold` | `data/biomes/ironhold.json` | – | – (neues Biom, W2.2) | Ziel: Hintergrund ×2, Palettenbruch erlaubt (Q13) | Ziel: CC0 oder CC-BY | **Lücke: neues Biom, Platzhalter-Palette, B-010** |
| `bg:crystal` | `data/biomes/crystal.json` | – | – (neues Biom, W2.2) | Ziel: Hintergrund ×2, Palettenbruch erlaubt (Q13) | Ziel: CC0 oder CC-BY | **Lücke: neues Biom, Platzhalter-Palette, B-010** |
