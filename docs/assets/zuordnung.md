# Grafik-Zuordnung: Packs und Stil

Diese Datei hält fest, welche Grafik-Packs zum gesetzten Grafik-Stil passen und welche Lücken bleiben (B-161, Sprint GR1). Die Pack-Tabelle ist die Grundlage für die Zuordnung der Spielobjekte.

- **Stilbeschluss Q13** (`docs/fragenkatalog.md` › Beschlüsse vom 2026-10-03): „Grundraster 16/32 px, Skalierung ×2 bis ×3; nicht passende Packs sind Lücken, Palettenbruch nur bei Hintergründen.“
- **Datum:** 2026-10-04
- **Bestätigt von 🧑 am 2026-10-04** (Workshop GR1.1 im Chat). Abweichungen vom Vorschlag: Figuren gelten trotz nicht ganzzahliger Skalierung als „passt“ (Vermerk B-251); `blue-cave-background` passt als Hintergrund; 32-px-Packs nativ ×1 passen.
- Die Zeilen je Spielobjekt stehen in weiteren Dateien `docs/assets/zuordnung-*.md` (GR1.2, GR1.3).

Regeln für den Vorschlag (Anwendung von Q13): Raster 16 px (×2 auf `UNIT_PX` = 32) oder 32 px (nativ ×1) und ganzzahlige Skalierung ×2 oder ×3 → „passt“. Anderes Raster oder nicht ganzzahlige Skalierung → „passt nicht (Lücke)“. Eine abweichende Palette ist nur bei Hintergründen erlaubt. Die Spalte „Entscheidung 🧑“ füllt allein der Nutzer. Q13 gilt streng für Umgebung; Figuren übernehmen die Skalierung aus `data/sprites.json` (Entscheidung 🧑, Vermerk B-251).

## Umgebungs-Packs (`public/grafik/`)

| Pack (Ordner-ID) | Quelle/Urheber | Lizenz | Raster | Skalierung auf UNIT_PX=32 | Palette | Vorschlag | Grund | Entscheidung 🧑 |
|---|---|---|---|---|---|---|---|---|
| gothicvania-town | GothicVania Town, Luis Zuno (ansimuz) | CC0 1.0 | 16 px (Kacheln 16/32/48) | ×2 (16 → 32) | dunkel, violett/rosa, mittel gesättigt | passt | Kachelraster 16 px, ganzzahlig ×2. | passt |
| gothicvania-church-pack | GothicVania Church Pack, Luis Zuno (ansimuz) | CC0 1.0 | 16 px (Tileset 336×224 = 21×14) | ×2 | sehr dunkel, blauviolett, entsättigt | passt | 16-px-Raster, ganzzahlig ×2; Palette passt zu Innenräumen. | passt |
| various-stones-and-oregem-veins-16x16 | Various stones and ore/gem veins 16x16, vico | CC0 1.0 | 16 px (Sheet 112×176) | ×2 | hell, gesättigt (Erze, Edelsteine) | passt | 16-px-Raster, ganzzahlig ×2. | passt |
| portals-32-x-48 | Portals 32 x 48, Rusher_go | CC0 1.0 | 32×48 (Sheet 128×192, 4×4 Frames) | ×1 (32 nativ), Höhe 48 = 1,5 Units | mittel, gesättigt (blau, rot, orange, grau) | passt | Breite 32 px = Grundraster; Höhe 48 ist ein Vielfaches von 16. | passt |
| 16x16-small-and-medium-coin-animation | 16x16 Small and Medium Coin Animation, WolfTech | CC0 1.0 | 16 px (Sheet 128×64) | ×2 | hell, gelb, gesättigt | passt | 16-px-Raster, ganzzahlig ×2. | passt |
| gold-treasure-icons-16x16 | Gold treasure icons 16x16, Bonsaiheldin | CC0 1.0 | 16 px (10 Icons 16×16, 1 Sheet 64×64) | ×2 | hell, gelb/braun, gesättigt | passt | 16-px-Raster, ganzzahlig ×2. | passt |
| sunnyland-tall-forest-environment | SunnyLand Tall Forest Environment, Luis Zuno (ansimuz) | CC0 1.0 | 16 px (Ebenen 144–192×240; Props frei 27–42 px) | ×2 | hell, grün, gesättigt | passt | Ebenen sind Vielfache von 16 px, ×2; Props liegen nicht auf dem Raster, sind aber frei platzierbar. | passt |
| gotthicvania-swamp | GothicVania Swamp, Luis Zuno (ansimuz) | CC0 1.0 | 16 px (Ebenen 96/208/336×…) | ×2 | dunkel, grün, entsättigt | passt | Größen sind Vielfache von 16 px, ×2; als Hintergrund-Pack ohne Kachelsatz. | passt |
| forest-background | Forest Background, Luis Zuno (ansimuz) | CC0 1.0 | kein Kachelraster (4 Parallax-Ebenen 272×160) | ×2 | mittel, braun/orange, einfarbig-flach | passt | Reine Hintergrund-Ebenen, 272×160 sind Vielfache von 16; Palette weicht ab, bei Hintergründen erlaubt. | passt |
| blue-cave-background | Blue Cave Background, Urheber unbekannt | CC0 1.0 | kein Pixelraster (Einzelbild 400×225) | ×2 ergibt 800×450, kein Vielfaches von 32 | dunkel, blau, weich/verwaschen | passt nicht (Lücke) | 225 px Höhe liegt nicht im 16-px-Raster, Stil ist malerisch statt Pixel-Art, Urheber ungeklärt. | passt |
| sunnyland-fort-of-illusion | SunnyLand Fort of Illusion, Luis Zuno (ansimuz) | CC0 1.0 | 16 px (Ebenen 64–335×272/128; Props 16–96 px) | ×2 | dunkel, blaugrün/violett, gesättigt | passt | 16-px-Raster, ganzzahlig ×2. | passt |
| warped-caves-pixel-art-pack | Warped Caves Pixel Art Pack, Luis Zuno (ansimuz) | CC BY 3.0 (Namensnennung Pflicht) | 16 px (Ebenen 160–384×128–192; Props 48×48) | ×2 | dunkel, Neon-Pink/Violett, stark gesättigt | passt | 16-px-Raster, ×2; die Palette ist ein Bruch, bei Höhlen-Hintergründen erlaubt, für Objekte davor nicht. | passt |

## Figuren-Packs (`public/sprites/`)

Frame = Leinwandgröße je Bild aus `data/sprites.json` (`frameWidth`×`frameHeight`), Figur = Figurhöhe in Frame-Pixeln. Figuren haben kein Kachelraster; die Skalierung ist der Faktor `scale` aus `data/sprites.json` (aktuell auf gleiche Figurgröße abgestimmt, nicht auf das Raster).

| Pack (Ordner-ID) | Quelle/Urheber | Lizenz | Raster | Skalierung auf UNIT_PX=32 | Palette | Vorschlag | Grund | Entscheidung 🧑 |
|---|---|---|---|---|---|---|---|---|
| medieval-king | Medieval King Pack, LuizMelo | CC0 1.0 | Frame 155×155, Figur 81 px | ×1,5 | mittel, rot/blau, handgemalt | passt nicht (Lücke) | ×1,5 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| medieval-king-2 | Medieval King Pack 2, LuizMelo | CC0 1.0 | Frame 160×111, Figur 54 px | ×2,25 | mittel, entsättigt | passt nicht (Lücke) | ×2,25 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| martial-hero-3 | Martial Hero 3, LuizMelo | CC0 1.0 | Frame 126×126, Figur 41 px | ×2,5 | mittel, entsättigt | passt nicht (Lücke) | ×2,5 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| huntress-2 | Huntress 2, LuizMelo | CC0 1.0 | Frame 100×100, Figur 36 px | ×2,75 | mittel, entsättigt | passt nicht (Lücke) | ×2,75 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| medieval-warrior | Medieval Warrior Pack, LuizMelo | CC0 1.0 | Frame 184×137, Figur 81 px | ×1,25 | mittel, entsättigt | passt nicht (Lücke) | ×1,25 liegt unter ×2 und ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| medieval-warrior-2 | Medieval Warrior Pack 2, LuizMelo | CC0 1.0 | Frame 150×150, Figur 41 px | ×2,5 | mittel, entsättigt | passt nicht (Lücke) | ×2,5 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| goblin, skeleton, mushroom, flying-eye | Monsters Creatures Fantasy, LuizMelo | CC0 1.0 | Frame 150×150, Figuren 33–51 px | ×2 (Skelett, Pilz, Auge), ×2,25 (Goblin) | mittel, gesättigt (Grün, Weiß, Braun) | passt nicht (Lücke) | Drei Figuren stehen auf ×2, der Goblin auf ×2,25; die Gruppe ist nicht einheitlich. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| medieval-warrior-3 | Medieval Warrior Pack 3, LuizMelo | CC0 1.0 | Frame 108×51 (zugeschnitten), Figur 38 px | ×2,75 | mittel, entsättigt | passt nicht (Lücke) | ×2,75 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| fantasy-warrior | Fantasy Warrior, LuizMelo | CC0 1.0 | Frame 116×61 (zugeschnitten), Figur 45 px | ×2,5 | mittel, entsättigt | passt nicht (Lücke) | ×2,5 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| martial-hero, martial-hero-2 | Martial Hero 1–2, LuizMelo | CC0 1.0 | Frame 122×71 / 114×68, Figur 52 / 56 px | ×2 | mittel, entsättigt | passt | Beide Figuren stehen auf ganzzahlig ×2. | passt |
| huntress | Huntress, LuizMelo | CC0 1.0 | Frame 89×67, Figur 42 px | ×2,5 | mittel, entsättigt | passt nicht (Lücke) | ×2,5 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| wizard | Wizard Pack, LuizMelo | CC0 1.0 | Frame 169×138, Figur 86 px | ×1,25 | mittel, entsättigt | passt nicht (Lücke) | ×1,25 liegt unter ×2 und ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| evil-wizard, evil-wizard-2, evil-wizard-3 | Evil Wizard 1–3, LuizMelo | CC0 1.0 | Frame 96×71 / 164×143 / 90×68, Figuren 53–95 px | ×2,25 / ×1,25 / ×2 | mittel, dunkel, rote Akzente | passt nicht (Lücke) | Gruppe uneinheitlich, zwei von drei nicht auf ×2 oder ×3. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| fire-worm | Fire Worm, LuizMelo | CC0 1.0 | Frame 74×56, Figur 41 px | ×1,75 | mittel, orange/rot, gesättigt | passt nicht (Lücke) | ×1,75 liegt unter ×2 und ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| hell-hound, ghost, fire-skull, demon, nightmare, gothic-hero, wolf | Gothicvania Patreon's Collection, ansimuz | CC0 1.0 | Frames 50×44 … 223×172, Figuren 18–124 px | ×3 / ×2,5 / ×1 / ×2 / ×1,75 / ×2,75 / ×3,25 | dunkel, gesättigte Akzente | passt nicht (Lücke) | Nur Höllenhund (×3) und Dämon (×2) ganzzahlig, die übrigen nicht. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| cemetery-skeleton, cemetery-skeleton-clothed, hell-gato, cemetery-hero, cemetery-ghost | Gothicvania Cemetery, ansimuz | CC0 1.0 | Frames 34×46 … 92×46, Figuren 35–46 px | ×2,5 / ×2,5 / ×3,25 / ×2,5 / ×2 | dunkel, entsättigt | passt nicht (Lücke) | Nur Geist (×2) ganzzahlig, die übrigen ×2,5 und ×3,25. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| horse, horse-walk | [LPC] Horses, bluecarrot16 | CC-BY 3.0 (auch OGA-BY 3.0 / GPL) | Frame 100×65 / 67×64 (LPC-Raster 32/64), Figur 61 px | ×2,2 | hell, braun, natürlich entsättigt | passt nicht (Lücke) | ×2,2 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| white-horse, unicorn, pegasus | Horses Rework, bluecarrot16, AntumDeluge, lemmling | CC-BY 3.0 | Frame 64×62–64 (LPC-Raster), Figur 60–62 px | ×2,4 | hell, weiß/pastell, entsättigt | passt nicht (Lücke) | ×2,4 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| elephant | Elephant Rework, lawnjelly, AntumDeluge | CC-BY 3.0 | Frame 94×59 (LPC-Raster), Figur 55 px | ×2 | hell, grau, entsättigt | passt | Ganzzahlig ×2. | passt |
| stag | Deer Rework, Calciumtrice, AntumDeluge | CC-BY 3.0 | Frame 64×61 (LPC-Raster), Figur 56 px | ×2,4 | hell, braun, entsättigt | passt nicht (Lücke) | ×2,4 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |
| lpc-wolf, lpc-hellhound | Wolf Rework, AntumDeluge | CC-BY 3.0 | Frame 64×33 (LPC-Raster), Figur 32 px | ×2,6 | hell bzw. dunkelrot, entsättigt | passt nicht (Lücke) | ×2,6 ist nicht ganzzahlig. | passt (Vermerk: Skalierung nicht ganzzahlig, B-251) |

## Ergebnis Workshop GR1.1

- Umgebungs-Packs: alle 12 passen (`blue-cave-background` entgegen dem Vorschlag als Hintergrund bestätigt; Warped Caves nur als Höhlen-Hintergrund).
- Figuren-Packs: alle 21 passen; 19 davon mit Vermerk „Skalierung nicht ganzzahlig“ (B-251).
- Keine Pack-Lücken; Lücken entstehen nur bei Objekten ohne Grafik (`zuordnung-objekte.md`).
- Gebäude: Burg, Turm und Tor aus SunnyLand Fort bleiben zugeordnet, Palette wird an Gothicvania Town angeglichen (Entscheidung 🧑). Häuser und Treppen aus Gothicvania Town kommen als Bestands-Kandidaten in GR2.1, die Objekte bleiben bis GR2.2 Lücken.
