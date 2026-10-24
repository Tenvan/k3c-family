# Grafik-Zuordnung: Gebäude, Gegner, Truppen

Teil der Zuordnungstabelle aus B-161 (Sprint GR1, Session GR1.2). Stilbeschluss (Q13) und Pack-Tabelle stehen im Kopf von
`docs/assets/zuordnung.md`; hier steht je Objekt-ID aus `data/` eine Zeile. Grundstil nach Q13: Raster 16 oder 32 px,
Skalierung ×2 bis ×3, Nachbearbeitung nur Skalieren und Palette.

- **Pack:** `grafik/<ordner>` = Umgebungs-Pack unter `public/grafik/` (Credits `public/grafik/CREDITS.md`),
  `sprites/<ordner>` = Figur unter `public/sprites/` (Credits `public/sprites/CREDITS.md`), Zuordnung über `data/sprites.json`.
- **Stil bei Figuren:** Frame-Größe, Figurhöhe in Frame-Pixeln und Skalierung aus `data/sprites.json`
  (`scale` des Sheets, mal `scale` der Figur, falls gesetzt).
- **Hub-Gebäude (GEB9):** Werkstatt, Farm, Kaserne, Lager, Taverne, Schmiede, Heilplatz und Rüstkammer wechseln mit der Hub-Stufe; das Register
  führt je Gebäude die Slots `<id>:1` bis `<id>:5` (z. B. `workshop:3`), der Slot ohne Stufe zeigt die Grundform (Stufe 1).
- **Lücken** sind fett und nennen das Ticket für die Suche (B-162, GR2). `src/tools/zuordnung.test.ts` prüft Vollständigkeit,
  Credits und Lücken-Tickets.

## Gebäude

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `castle` | `data/buildings.json` | `grafik/gebaeude` | `burg-<n>-32.png` (8 × 9 Kacheln, Hub-Stufen 1–5 in `burg.json`) | eigene Grafik (`tools/buildings`), 32 px je Kachel, ×1; GEB4; Stufe = `World.hubLevel`, siehe zuordnung-welt.md | eigen | zugeordnet |
| `wall` | `data/buildings.json` | `grafik/gebaeude` | `mauer-<n>-32.png` (1 × 5 Kacheln, Stufen 1–5 in `mauer.json`) | eigene Grafik (`tools/buildings`), 32 px je Kachel, ×1; GEB3; Stufen siehe zuordnung-welt.md | eigen | zugeordnet |
| `tower` | `data/buildings.json` | `grafik/gebaeude` | `turm-<n>-32.png` (3 × 8 Kacheln, Stufen 1–5 in `turm.json`) | eigene Grafik (`tools/buildings`), 32 px je Kachel, ×1; GEB3; Stufen siehe zuordnung-welt.md | eigen | zugeordnet |
| `gate` | `data/buildings.json` | `grafik/gebaeude` | `tor-<n>-32.png` (2 × 4 Kacheln, Stufen 1–5 in `tor.json`) | eigene Grafik (`tools/buildings`), 32 px je Kachel, ×1; GEB4; Stufe = Stufe der gebauten Mauer seiner Linie, sonst 1 | eigen | zugeordnet |
| `workshop` | `data/buildings.json` | `grafik/gebaeude` | `werkstatt-<n>-32.png` (n = Hub-Stufe 1–5) | 32 px je Kachel, eigene Grafik (`tools/buildings`), ×1; GEB1/GEB2, GEB9 | eigen | zugeordnet |
| `tavern` | `data/buildings.json` | `grafik/gebaeude` | `taverne-<n>-32.png` (n = Hub-Stufe 1–5) | 32 px je Kachel, eigene Grafik (`tools/buildings`), ×1; GEB2, GEB9 | eigen | zugeordnet |
| `healer` | `data/buildings.json` | `grafik/gebaeude` | `heilplatz-<n>-32.png` (n = Hub-Stufe 1–5) | 32 px je Kachel, eigene Grafik (`tools/buildings`), ×1; GEB2, GEB9 | eigen | zugeordnet |
| `smithy` | `data/buildings.json` | `grafik/gebaeude` | `schmiede-<n>-32.png` (n = Hub-Stufe 1–5) | 32 px je Kachel, eigene Grafik (`tools/buildings`), ×1; GEB2, GEB9 | eigen | zugeordnet |
| `armory` | `data/buildings.json` | `grafik/gebaeude` | `ruestkammer-<n>-32.png` (n = Hub-Stufe 1–5) | 32 px je Kachel, eigene Grafik (`tools/buildings`), ×1; GEB2, GEB9 | eigen | zugeordnet |
| `storage` | `data/buildings.json` | `grafik/gebaeude` | `lager-<n>-32.png` (n = Hub-Stufe 1–5) (6 × 4 Kacheln) | 32 px je Kachel, eigene Grafik (`tools/buildings`), ×1; GEB5, GEB9 | eigen | zugeordnet |
| `farm` | `data/buildings.json` | `grafik/gebaeude` | `farm-<n>-32.png` (n = Hub-Stufe 1–5) (4 × 3 Kacheln) | 32 px je Kachel, eigene Grafik (`tools/buildings`), ×1; GEB5, GEB9 | eigen | zugeordnet |
| `barracks` | `data/buildings.json` | `grafik/gebaeude` | `kaserne-<n>-32.png` (n = Hub-Stufe 1–5) (6 × 4 Kacheln) | 32 px je Kachel, eigene Grafik (`tools/buildings`), ×1; GEB5, GEB9 | eigen | zugeordnet |
| `stairsUp` | `data/buildings.json` | `grafik/gebaeude` | `treppe-hoch-32.png` (4 × 4 Kacheln, ein Bild) | 32 px je Kachel, eigene Grafik (`tools/buildings`), ×1; GEB5 | eigen | zugeordnet |
| `stairsDown` | `data/buildings.json` | `grafik/gebaeude` | `treppe-runter-32.png` (4 × 3 Kacheln) | 32 px je Kachel, eigene Grafik (`tools/buildings`), ×1; GEB5 | eigen | zugeordnet |

## Gegner

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `greed` | `data/enemies.json` | `sprites/goblin` | `idle/run/attack.png`, Frame 150×150, Tönung `#c77dff` | Figur 36 px, LuizMelo, ×2,25 | CC0 1.0 | zugeordnet |
| `wolf` | `data/enemies.json` | `sprites/hell-hound` | `idle/run/attack.png`, Frame 50×44 | Figur 24 px, Gothicvania, ×3 | CC0 1.0 | zugeordnet |
| `alphaWolf` | `data/enemies.json` | `sprites/hell-hound` | `idle/run/attack.png`, Frame 50×44, Tönung `#e0e0ff` (vorläufig wie Wolf, B-010) | Figur 24 px, Gothicvania, ×3 | CC0 1.0 | zugeordnet |
| `goblin` | `data/enemies.json` | `sprites/goblin` | `idle/run/attack.png`, Frame 150×150 | Figur 36 px, LuizMelo, ×2,25 | CC0 1.0 | zugeordnet |
| `goblinArcher` | `data/enemies.json` | `sprites/goblin` | `idle/run/attack.png`, Frame 150×150, Tönung `#74c69d` | Figur 36 px, LuizMelo, ×2,25 | CC0 1.0 | zugeordnet |
| `skeleton` | `data/enemies.json` | `sprites/cemetery-skeleton` | `idle/run/attack.png`, Frame 34×46 | Figur 40 px, Gothicvania, ×2,5 | CC0 1.0 | zugeordnet |
| `bat` | `data/enemies.json` | `sprites/fire-skull` | `idle/run/attack.png`, Frame 91×102, 130 px über Boden | Figur 54 px, Gothicvania, ×1; Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |
| `caveTroll` | `data/enemies.json` | `sprites/hell-gato` | `idle/run/attack.png`, Frame 87×37 | Figur 35 px, Gothicvania, ×4,2 (3,25 × 1,3); Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |
| `zombie` | `data/enemies.json` | `sprites/cemetery-skeleton-clothed` | `idle/run/attack.png`, Frame 34×46 | Figur 40 px, Gothicvania, ×2,5 | CC0 1.0 | zugeordnet |
| `ratSwarm` | `data/enemies.json` | `sprites/mushroom` | `idle/run/attack.png`, Frame 150×150, Tönung `#b08968` | Figur 37 px, LuizMelo, ×1,4 (2 × 0,7); Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |
| `mineGhost` | `data/enemies.json` | `sprites/ghost` | `idle/run/attack.png`, Frame 48×54, Deckkraft 0,85 | Figur 42 px, Gothicvania, ×2,5 | CC0 1.0 | zugeordnet |
| `lavaSlime` | `data/enemies.json` | `sprites/fire-slime` | `idle/run/attack.png`, Frame 24×30 (je 3 Frames, Zeile 0 der Quelle) | Figur 28 px, ImogiaGames (DB32), ×2 | CC0 1.0 | zugeordnet |
| `ironBeetle` | `data/enemies.json` | `sprites/lpc-beetle` | `idle/run/attack.png`, Frame 40×22 | Figur 20 px, Redshrike (LPC Beetle), ×3; Namensnennung Pflicht (CC-BY 3.0 / OGA-BY 3.0) | CC-BY 3.0 / OGA-BY 3.0 | zugeordnet |
| `fireSpirit` | `data/enemies.json` | `sprites/cemetery-ghost` | `idle/attack.png`, Frame 37×60, Tönung `#f4a261` | Figur 46 px, Gothicvania, ×2 | CC0 1.0 | zugeordnet |
| `crystalSpider` | `data/enemies.json` | `sprites/crystal-spider` | `idle/run/attack.png`, Frame 32×24 | Figur 20 px, eigene Arbeit (`tools/spritegen`), ×3 | CC0 1.0 | zugeordnet |
| `shardling` | `data/enemies.json` | `sprites/eye-rock` | `idle/run/attack.png`, Frame 66×66 | Figur 64 px, Night Blade, ×1 | CC0 1.0 | zugeordnet |
| `crystalGuardian` | `data/enemies.json` | `sprites/crystal-golem` | `idle/run/attack.png`, Frame 160×112 (idle = run) | Figur 110 px, irmirx (Golem animations, 30 % verkleinert), ×1; Namensnennung Pflicht (CC-BY 3.0) | CC-BY 3.0 | zugeordnet |

## Truppen

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `vagrant` | `data/troops.json` | `sprites/martial-hero-3` | `idle/run/attack.png`, Frame 126×126, Tönung `#8f8f8f` | Figur 41 px, LuizMelo, ×2,5 | CC0 1.0 | zugeordnet |
| `peasant` | `data/troops.json` | `sprites/martial-hero-3` | `idle/run/attack.png`, Frame 126×126 | Figur 41 px, LuizMelo, ×2,5 | CC0 1.0 | zugeordnet |
| `archer` | `data/troops.json` | `sprites/huntress-2` | `idle/run/attack.png`, Frame 100×100 | Figur 36 px, LuizMelo, ×2,75 | CC0 1.0 | zugeordnet |
| `warrior` | `data/troops.json` | `sprites/medieval-warrior-2` | `idle/run/attack.png`, Frame 150×150 | Figur 41 px, LuizMelo, ×2,5 | CC0 1.0 | zugeordnet |
| `eliteArcher` | `data/troops.json` | `sprites/huntress-2` | `idle/run/attack.png`, Frame 100×100, Tönung `#ffd166` | Figur 36 px, LuizMelo, ×2,75 | CC0 1.0 | zugeordnet |
| `eliteWarrior` | `data/troops.json` | `sprites/medieval-warrior` | `idle/run/attack.png`, Frame 184×137 | Figur 81 px, LuizMelo, ×1,25; Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |
