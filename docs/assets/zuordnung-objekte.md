# Grafik-Zuordnung: Gebäude, Gegner, Truppen

Teil der Zuordnungstabelle aus B-161 (Sprint GR1, Session GR1.2). Stilbeschluss (Q13) und Pack-Tabelle stehen im Kopf von
`docs/assets/zuordnung.md`; hier steht je Objekt-ID aus `data/` eine Zeile. Grundstil nach Q13: Raster 16 oder 32 px,
Skalierung ×2 bis ×3, Nachbearbeitung nur Skalieren und Palette.

- **Pack:** `grafik/<ordner>` = Umgebungs-Pack unter `public/grafik/` (Credits `public/grafik/CREDITS.md`),
  `sprites/<ordner>` = Figur unter `public/sprites/` (Credits `public/sprites/CREDITS.md`), Zuordnung über `data/sprites.json`.
- **Stil bei Figuren:** Frame-Größe, Figurhöhe in Frame-Pixeln und Skalierung aus `data/sprites.json`
  (`scale` des Sheets, mal `scale` der Figur, falls gesetzt).
- **Lücken** sind fett und nennen das Ticket für die Suche (B-162, GR2). `src/tools/zuordnung.test.ts` prüft Vollständigkeit,
  Credits und Lücken-Tickets.

## Gebäude

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `castle` | `data/buildings.json` | `grafik/sunnyland-fort-of-illusion` | `ebenen/tileset.png` (Burgmauer mit Zinnen), `props/banner.png` | 16 px, SunnyLand-Fort (Blaugrau), ×2; Vermerk: Palette an gothicvania-town angleichen, Entscheidung 🧑 GR1.1 | CC0 1.0 | zugeordnet |
| `wall` | `data/buildings.json` | `grafik/gothicvania-town` | `tileset-einzeln/wall.png`, `tileset-einzeln/wall-b.png` (16×16) | 16 px, Gothicvania (Dunkelviolett), ×2; Vermerk: zeigt Stufe 2 (Stein), Stufen siehe zuordnung-welt.md (`wall:1`/`tower:1` Lücke) | CC0 1.0 | zugeordnet |
| `tower` | `data/buildings.json` | `grafik/sunnyland-fort-of-illusion` | `ebenen/front.png` (Turm mit Kegeldach, 112×128) | 16 px, SunnyLand-Fort (Blaugrau), ×2; Vermerk: Palette an gothicvania-town angleichen, Entscheidung 🧑 GR1.1; Vermerk: zeigt Stufe 2 (Stein), Stufen siehe zuordnung-welt.md (`wall:1`/`tower:1` Lücke) | CC0 1.0 | zugeordnet |
| `gate` | `data/buildings.json` | `grafik/sunnyland-fort-of-illusion` | `props/door.png` (offen), `props/closed-door.png` (zu), je 96×80 | 16 px, SunnyLand-Fort (Blaugrau), ×2; Vermerk: Palette an gothicvania-town angleichen, Entscheidung 🧑 GR1.1 | CC0 1.0 | zugeordnet |
| `workshop` | `data/buildings.json` | – | – | Ziel: 16/32 px, Gothicvania, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Grafik, B-162** |
| `storage` | `data/buildings.json` | `grafik/gothicvania-town` | `props-einzeln/crate-stack.png` (73×68) | 16 px, Gothicvania (Dunkelviolett), ×2 | CC0 1.0 | zugeordnet |
| `farm` | `data/buildings.json` | – | – | Ziel: 16/32 px, Gothicvania, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Grafik, B-162** |
| `barracks` | `data/buildings.json` | – | – | Ziel: 16/32 px, Gothicvania, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Grafik, B-162** |
| `stairsUp` | `data/buildings.json` | – | – | Ziel: 16/32 px, Gothicvania, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Grafik, B-162** |
| `stairsDown` | `data/buildings.json` | – | – | Ziel: 16/32 px, Gothicvania, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Grafik, B-162** |
| `tavern` | `data/buildings.json` | – | – | Ziel: 16/32 px, Gothicvania, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Grafik, B-162** |
| `healer` | `data/buildings.json` | – | – | Ziel: 16/32 px, Gothicvania, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Grafik, B-162** |
| `smithy` | `data/buildings.json` | – | – | Ziel: 16/32 px, Gothicvania, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Grafik, B-162** |
| `armory` | `data/buildings.json` | – | – | Ziel: 16/32 px, Gothicvania, ×2–×3 (Q13) | Ziel: CC0 oder CC-BY | **Lücke: keine Grafik, B-162** |

## Gegner

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `greed` | `data/enemies.json` | `sprites/goblin` | `idle/run/attack.png`, Frame 150×150, Tönung `#c77dff` | Figur 36 px, LuizMelo, ×2,25 | CC0 1.0 | zugeordnet |
| `wolf` | `data/enemies.json` | `sprites/hell-hound` | `idle/run/attack.png`, Frame 50×44 | Figur 24 px, Gothicvania, ×3 | CC0 1.0 | zugeordnet |
| `goblin` | `data/enemies.json` | `sprites/goblin` | `idle/run/attack.png`, Frame 150×150 | Figur 36 px, LuizMelo, ×2,25 | CC0 1.0 | zugeordnet |
| `goblinArcher` | `data/enemies.json` | `sprites/goblin` | `idle/run/attack.png`, Frame 150×150, Tönung `#74c69d` | Figur 36 px, LuizMelo, ×2,25 | CC0 1.0 | zugeordnet |
| `skeleton` | `data/enemies.json` | `sprites/cemetery-skeleton` | `idle/run/attack.png`, Frame 34×46 | Figur 40 px, Gothicvania, ×2,5 | CC0 1.0 | zugeordnet |
| `bat` | `data/enemies.json` | `sprites/fire-skull` | `idle/run/attack.png`, Frame 91×102, 130 px über Boden | Figur 54 px, Gothicvania, ×1; Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |
| `caveTroll` | `data/enemies.json` | `sprites/hell-gato` | `idle/run/attack.png`, Frame 87×37 | Figur 35 px, Gothicvania, ×4,2 (3,25 × 1,3); Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |
| `zombie` | `data/enemies.json` | `sprites/cemetery-skeleton-clothed` | `idle/run/attack.png`, Frame 34×46 | Figur 40 px, Gothicvania, ×2,5 | CC0 1.0 | zugeordnet |
| `ratSwarm` | `data/enemies.json` | `sprites/mushroom` | `idle/run/attack.png`, Frame 150×150, Tönung `#b08968` | Figur 37 px, LuizMelo, ×1,4 (2 × 0,7); Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |
| `mineGhost` | `data/enemies.json` | `sprites/ghost` | `idle/run/attack.png`, Frame 48×54, Deckkraft 0,85 | Figur 42 px, Gothicvania, ×2,5 | CC0 1.0 | zugeordnet |

## Truppen

| Objekt | Herkunft | Pack | Datei/Frame | Stil (Raster, Palette, Skalierung) | Lizenz | Status |
|---|---|---|---|---|---|---|
| `vagrant` | `data/troops.json` | `sprites/martial-hero-3` | `idle/run/attack.png`, Frame 126×126, Tönung `#8f8f8f` | Figur 41 px, LuizMelo, ×2,5 | CC0 1.0 | zugeordnet |
| `peasant` | `data/troops.json` | `sprites/martial-hero-3` | `idle/run/attack.png`, Frame 126×126 | Figur 41 px, LuizMelo, ×2,5 | CC0 1.0 | zugeordnet |
| `archer` | `data/troops.json` | `sprites/huntress-2` | `idle/run/attack.png`, Frame 100×100 | Figur 36 px, LuizMelo, ×2,75 | CC0 1.0 | zugeordnet |
| `warrior` | `data/troops.json` | `sprites/medieval-warrior-2` | `idle/run/attack.png`, Frame 150×150 | Figur 41 px, LuizMelo, ×2,5 | CC0 1.0 | zugeordnet |
| `eliteArcher` | `data/troops.json` | `sprites/huntress-2` | `idle/run/attack.png`, Frame 100×100, Tönung `#ffd166` | Figur 36 px, LuizMelo, ×2,75 | CC0 1.0 | zugeordnet |
| `eliteWarrior` | `data/troops.json` | `sprites/medieval-warrior` | `idle/run/attack.png`, Frame 184×137 | Figur 81 px, LuizMelo, ×1,25; Vermerk: Skalierung nicht ganzzahlig bzw. außerhalb ×2–×3, B-251 | CC0 1.0 | zugeordnet |
