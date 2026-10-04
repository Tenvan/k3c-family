/**
 * Die gewählten Grafik-Packs für B-010 (Auswahl von 🧑, Recherche in docs/funde/b010-grafik-funde.html).
 * Die Dateien liegen gefiltert unter public/grafik/<id>/ (nur PNG der Umgebung und die Lizenzdatei), die Bildliste in public/grafik/index.json.
 * Urheber, Lizenz und Quelle müssen mit public/grafik/CREDITS.md und lizenzen.html übereinstimmen (Test).
 */
export interface GrafikPack {
  /** Ordner unter public/grafik/ */
  id: string;
  name: string;
  /** Urheber laut Lizenzdatei des Packs; ohne Angabe der Uploader auf OpenGameArt */
  artist: string;
  /** Lizenz, mit der wir das Pack nutzen */
  license: 'CC0 1.0' | 'CC BY 3.0' | 'CC BY 4.0';
  /** Hinweis zur Lizenz, wenn Quellseite und Pack sich unterscheiden */
  licenseNote?: string;
  source: string;
  /** Wofür es im Spiel taugen könnte (B-010) */
  covers: string[];
  note: string;
}

const ZUNO = 'Luis Zuno (ansimuz)';

/** Nicht gewählter Kandidat aus GR2 (AC-06): im Bestand, im Spiel nicht zugeordnet, Bilder in der Gruppe `kandidaten`. */
function kandidat(id: string, name: string, artist: string, license: GrafikPack['license'], source: string, luecke: string, note: string, licenseNote?: string): GrafikPack {
  return { id, name, artist, license, source, covers: [`Kandidat (nicht gewählt): ${luecke}`], note, ...(licenseNote ? { licenseNote } : {}) };
}

export const GRAFIK_PACKS: readonly GrafikPack[] = [
  {
    id: 'gothicvania-town',
    name: 'GothicVania Town',
    artist: ZUNO,
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/gothicvania-town',
    covers: ['Gebäude (Häuser)', 'Props (Fässer, Kisten)', 'Hintergrund Stadt'],
    note: 'Häuser und Props als Streifen und einzeln, Hintergrund und Mittelgrund, Tileset. Das Original enthält auch Musik (nicht CC0) und Demo-Code, beides haben wir nicht übernommen.',
  },
  {
    id: 'gothicvania-church-pack',
    name: 'GothicVania Church Pack',
    artist: ZUNO,
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/gothicvania-church-pack',
    covers: ['Kirche, Säulen', 'Hintergrund'],
    note: 'Nur die Umgebung (Hintergründe, Säule, Tileset); Figuren und Musik nicht übernommen.',
  },
  {
    id: 'various-stones-and-oregem-veins-16x16',
    name: 'Various stones and ore/gem veins 16x16',
    artist: 'vico (OpenGameArt-Uploader, das Pack nennt keinen Urheber)',
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/various-stones-and-oregem-veins-16x16',
    covers: ['Stein', 'Kupfererz', 'Kristalle'],
    note: 'Je 5 Formen für Stein, Kupfer, Eisen, Gold, Kohle, Smaragde, Rubine, Saphire und Diamanten; ein Sheet mit Gras.',
  },
  {
    id: 'portals-32-x-48',
    name: 'Portals 32 x 48',
    artist: 'Rusher_go (OpenGameArt-Uploader)',
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/portals-32-x-48',
    covers: ['Portal'],
    note: 'Ein Sheet mit Portalen in 32x48 px.',
  },
  {
    id: '16x16-small-and-medium-coin-animation',
    name: '16x16 Small and Medium Coin Animation',
    artist: 'WolfTech (OpenGameArt-Uploader)',
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/16x16-small-and-medium-coin-animation',
    covers: ['Münzen'],
    note: 'Animierte Münze in klein und mittel.',
  },
  {
    id: 'gold-treasure-icons-16x16',
    name: 'Gold treasure icons 16x16',
    artist: 'Bonsaiheldin (laut Credits.txt des Packs)',
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/gold-treasure-icons-16x16',
    covers: ['Münzen, Münzstapel', 'Schatztruhe', 'Goldbarren'],
    note: '16x16-Symbole in DawnBringers 16-Farben-Palette.',
  },
  {
    id: 'sunnyland-tall-forest-environment',
    name: 'SunnyLand Tall Forest Environment',
    artist: ZUNO,
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/sunnyland-tall-forest-environment',
    covers: ['Hintergrund Wald (3 Ebenen)', 'Bäume, Felsen, Pflanzen'],
    note: '16-Bit-Wald im SNES-Stil, Höhe der Hintergründe 240 px, mit Tileset und Props.',
  },
  {
    id: 'gotthicvania-swamp',
    name: 'GothicVania Swamp',
    artist: ZUNO,
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/gotthicvania-swamp',
    covers: ['Hintergrund Sumpf/Wald', 'Bäume, Props'],
    note: 'Sumpf-Umgebung im Gothicvania-Stil; Figuren und Musik nicht übernommen.',
  },
  {
    id: 'forest-background',
    name: 'Forest Background',
    artist: ZUNO,
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/forest-background',
    covers: ['Hintergrund Wald (4 Ebenen)'],
    note: 'Nahtlos kachelbare Ebenen: hintere, mittlere, vordere Bäume und Lichtstrahlen.',
  },
  {
    id: 'blue-cave-background',
    name: 'Blue Cave Background',
    artist: 'unbekannt (die OpenGameArt-Seite nennt keinen Uploader)',
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/blue-cave-background',
    covers: ['Hintergrund Höhle'],
    note: 'Ein einzelnes Bild, 400x225.',
  },
  {
    id: 'warped-caves-pixel-art-pack',
    name: 'Warped Caves Pixel Art Pack',
    artist: ZUNO,
    license: 'CC BY 3.0',
    licenseNote:
      'Die OpenGameArt-Seite zeigt CC0, die Lizenzdatei im Pack sagt CC-BY 3.0 (Namensnennung, entfällt nur für Patreon-Unterstützer). Wir behandeln es vorsichtig als CC BY 3.0 und nennen den Urheber; B-010 verlangt sonst nur CC0.',
    source: 'https://opengameart.org/content/warped-caves-pixel-art-pack',
    covers: ['Hintergrund Höhle (Ebenen, Wände)', 'Props (Tore, Stalaktit, Pflanzen, Steine)'],
    note: 'Höhle im Metroid-Stil, 240x176-Ebenen, Tileset und Props. Figuren, Demo-Code und Musik nicht übernommen.',
  },
  {
    id: 'sunnyland-fort-of-illusion',
    name: 'SunnyLand Fort of Illusion',
    artist: ZUNO,
    license: 'CC0 1.0',
    source: 'https://opengameart.org/node/140912',
    covers: ['Hintergrund Wald mit Burg', 'Turm im Vordergrund', 'Props (Banner, Tür, Fenster, Flagge)'],
    note: 'Wald-Umgebung nach „Castle of Illusion“: Ebenen, Tileset, Turm als Dekoration, Props.',
  },
  // GR2.3 (B-162): Auswahl von 🧑 in GR2.2, Recherche in docs/funde/gr2-grafik-funde.html
  {
    id: 'phantasy-dungeon-entrance',
    name: 'Phantasy Dungeon Entrance',
    artist: ZUNO,
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/phantasy-dungeon-entrance',
    covers: ['Treppe runter (Verlies-Eingang)'],
    note: 'Ein Kampf-Hintergrund 368x208 als zwei Ebenen (mit und ohne Schädel); Aseprite-Quelle nicht übernommen.',
  },
  {
    id: 'tent-8',
    name: 'Tent',
    artist: 'CDmir (OpenGameArt-Uploader)',
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/tent-8',
    covers: ['Rekrutierungslager (Zelte, Hütte, Schilder)'],
    note: 'Ein Blatt 256x184 mit zwei Zelten, Strohhütte und Holzschildern, erstellt für CaveTaxi.',
  },
  {
    id: '16x16-animated-campfire',
    name: '16x16 Animated Campfire',
    artist: 'krial (OpenGameArt-Uploader)',
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/16x16-animated-campfire',
    covers: ['Lagerfeuer (Rekrutierungslager)'],
    note: 'Animiertes Lagerfeuer, 4 Frames zu 16x16, DB16-Palette.',
  },
  {
    id: 'warped-super-grotto-escape-pack',
    name: 'Warped: Super Grotto Escape Pack',
    artist: ZUNO,
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/warped-super-grotto-escape-pack',
    covers: ['Hintergrund Mine (3 Parallax-Ebenen)', 'Tileset, Pflanzen'],
    note: 'Höhle im 16-Bit-Stil, Ebenen 240 px hoch; die Lizenzdatei im Pack verlangt keine Namensnennung. Aseprite-Quelle nicht übernommen.',
  },
  {
    id: 'wooden-fortress-and-animated-doors',
    name: 'Wooden fortress and animated doors',
    artist: 'Tuomo Untinen (Reemax)',
    license: 'CC BY 3.0',
    licenseNote: 'Die OpenGameArt-Seite nennt nur CC-BY 3.0 und verlangt die Namensnennung „Wooden fortress and animated castle doors by Tuomo Untinen“.',
    source: 'https://opengameart.org/content/wooden-fortress-and-animated-doors',
    covers: ['Burg Stufe 2 (Holzpalisade)', 'Holztor, animiert'],
    note: 'Palisade 144x192 im 32er-Raster und ein Tor mit 6 Frames zu 64x64.',
  },
  {
    id: 'opp2017-castle-tiles',
    name: 'OPP2017 - Castle tiles',
    artist: 'Hapiel (Open Pixel Project)',
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/opp2017-castle-tiles',
    covers: ['Burg Stufe 3–5 (Mauern, Türme, Tore)', 'Treppen, Fenster'],
    note: 'Kachelblätter 480x480 im 32er-Raster in violett und grau, DB32-Palette, dazu zwei Beispielbilder. Die Quellseite bietet auch CC-BY-SA und GPL an, wir nutzen CC0.',
  },
  {
    id: 'resource-icons',
    name: 'Resource icons',
    artist: 'Kutejnikov (OpenGameArt-Uploader)',
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/resource-icons',
    covers: ['Holz-Symbol', 'Stein, Eisen, Gold, Edelsteine'],
    note: 'Ein Blatt 50x91 mit 7 Rohstoffen, je mit und ohne Kontur.',
  },
  {
    id: 'item-ruby-banana-star',
    name: 'Item Ruby, Banana, Star',
    artist: 'mieki256 (OpenGameArt-Uploader)',
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/item-ruby-banana-star',
    covers: ['Skillpunkt-Pickup (Stern)', 'Rubin'],
    note: 'Stern 4 Frames zu 16x16, Rubin 7 Frames zu 24x24; die Banane ist nicht übernommen.',
  },
  {
    id: 'k3c-paletten',
    name: 'K3C-Paletten (Farbvarianten aus GothicVania Town und SunnyLand Fort of Illusion)',
    artist: ZUNO,
    license: 'CC0 1.0',
    source: 'https://opengameart.org/content/gothicvania-town',
    covers: ['Mauer und Turm in Kupfer, Eisen, Kristall', 'Burg, Turm, Tor in der Palette von GothicVania Town'],
    note: 'Eigene Ableitung (GR2.3): nur Farben getauscht (Q13). Quellen gothicvania-town (wall.png, wall-b.png) und sunnyland-fort-of-illusion (front.png, tileset.png, banner.png, door.png, closed-door.png), beide CC0.',
  },
  // Nicht gewählte Kandidaten aus GR2 (AC-06), nicht zugeordnet
  kandidat('16x16-block-texture-set', '16x16 Block Texture Set', 'ARoachIFoundOnMyPillow (OpenGameArt-Uploader)', 'CC0 1.0', 'https://opengameart.org/content/16x16-block-texture-set', 'Mauer-/Turm-Materialstufen', 'Nur das Tilemap-Sheet (Blöcke, Ziegel, Stein, Holz) übernommen; Einzelkacheln, Pflanzen und Feldfrüchte weggelassen.'),
  kandidat('16x16-rpg-items-db32', '16x16 RPG Items (DB32)', 'ARoachIFoundOnMyPillow', 'CC0 1.0', 'https://opengameart.org/content/16x16-rpg-items-db32', 'Skillpunkt-Pickup', '15 von 186 Einzel-PNGs (Edelsteine, Orb, Schriftrolle, Buch, Tränke, Beutel) übernommen; Waffen, Rüstung, Essen weggelassen. Das vom Kommentator hochgeladene Sheet nicht übernommen (kein Pack-Bestandteil).'),
  kandidat('32-pixel-set-log-cabin', '32-pixel Set - Log Cabin', 'cyanowl', 'CC0 1.0', 'https://opengameart.org/content/32-pixel-set-log-cabin', 'Werkstatt', 'Übernommen: Blockhaus, Türen und Gras (Duplikate Cabin_0/doors_and_grass_2 weggelassen); Mehrfachlizenz, CC0 gewählt.'),
  kandidat('building-block-assets-stone-logs-bricks', 'Building block assets (stone, logs, bricks)', 'Skalman (OpenGameArt-Uploader)', 'CC0 1.0', 'https://opengameart.org/content/building-block-assets-stone-logs-bricks', 'Mauer-/Turm-Materialstufen', 'Stein-, Ziegel-, Holz- und Türbausteine übernommen; Ambos weggelassen.'),
  kandidat('castle-set', 'Castle Set', 'Nia Mi', 'CC0 1.0', 'https://opengameart.org/content/castle-set', 'Kaserne', 'Alle acht PNGs (Türme, Mauer, Tor, Flaggen) übernommen; XCF und Thumbs.db weggelassen.'),
  kandidat('castles', 'Castles', 'Blarumyrran (OpenGameArt-Uploader)', 'CC0 1.0', 'https://opengameart.org/content/castles', 'Burg/Hub-Stufen', 'Das Burg-Sheet (16x16) übernommen; die identische Kopie castles_0.png weggelassen.'),
  kandidat('cavernous-background', 'Cavernous Background', 'Spring Spring', 'CC0 1.0', 'https://opengameart.org/content/cavernous-background', 'Mine-Hintergrund', 'Höhlen-Hintergrund (512x288) übernommen; cavernous_0.png ist byte-identisch und weggelassen.'),
  kandidat('crops-cc0', 'Crops CC0', 'SnoopethDuckDuck', 'CC0 1.0', 'https://opengameart.org/content/crops-cc0', 'Farm', 'Drei Sammelbilder der Feldfrüchte (0/1/2 px Kontur) übernommen; Einzelbilder, Gold-/Grau-/Shader-Varianten, PDN weggelassen.'),
  kandidat('farming-crops-16x16', 'Farming crops 16x16', 'josehzz', 'CC0 1.0', 'https://opengameart.org/content/farming-crops-16x16', 'Farm', 'Übernommen: Crop-Spritesheet (20 Pflanzen, 16x16); weggelassen: nichts Relevantes.'),
  kandidat('gothicvania-bridge-expansion-pack-1', 'Gothicvania Bridge Expansion Pack 1', 'ansimuz (Luis Zuno)', 'CC0 1.0', 'https://opengameart.org/content/gothicvania-bridge-expansion-pack-1', 'Werkstatt/Kaserne', 'Übernommen: Props Burg, Haus, Baum; weggelassen: Preview, Aseprite, Vorschaubild.'),
  kandidat('kyrises-free-16x16-rpg-icon-pack', 'Kyrise\'s Free 16x16 RPG Icon Pack', 'Kyrise', 'CC BY 4.0', 'https://opengameart.org/content/kyrises-free-16x16-rpg-icon-pack', 'Skillpunkt-Pickup', 'Nur die Spritesheets 16x16 und 32x32 (V1.2) übernommen; Einzelbilder, 48er-Sheet und sample.png weggelassen.', 'Namensnennung: Kyrise\'s Free 16x16 RPG Icon Pack | Graphics made by Kyrise: https://kyrise.itch.io/ (CC BY 4.0).'),
  kandidat('materials-pack', 'Materials Pack', 'xvideosman (OpenGameArt-Uploader)', 'CC BY 3.0', 'https://opengameart.org/content/materials-pack', 'Holz-Symbol', 'Materials.png (Barren, Erze, Edelsteine) übernommen; Aseprite-Quelldatei und readme weggelassen.', 'Namensnennung: "Materials Pack by xvideosman" (laut readme.txt des Packs), https://opengameart.org/content/materials-pack, CC BY 3.0.'),
  kandidat('opp2017-cave-and-mine-cart', 'OPP2017 - Cave and mine cart', 'Hapiel (Open Pixel Project)', 'CC0 1.0', 'https://opengameart.org/content/opp2017-cave-and-mine-cart', 'Mine-Hintergrund', 'Höhlen-Hintergründe, Felsen/Kristall/Lava-Kacheln, Schiene, Objekte und Mockup übernommen; Wagen-GIFs, Palette und Beschreibungsdateien weggelassen.', 'Quellseite nennt Mehrfachlizenz (CC-BY 3.0, CC-BY-SA 3.0, GPL 2.0/3.0, OGA-BY 3.0, CC0); das Pack liegt als CC0 bei, genutzt wird CC0.'),
  kandidat('opp2017-village-and-room', 'OPP2017 - Village and room', 'Hapiel (Open Pixel Project)', 'CC0 1.0', 'https://opengameart.org/content/opp2017-village-and-room', 'Werkstatt', 'Übernommen: Dorf- und Raum-Kacheln (CC0 gewählt, Seite bietet auch CC-BY/SA/GPL); weggelassen: Wolken, Mockups, Palette.'),
  kandidat('pixel-platformer-farm-expansion', 'Pixel Platformer: Farm Expansion', 'Kenney', 'CC0 1.0', 'https://kenney.nl/assets/pixel-platformer-farm-expansion', 'Farm', 'Übernommen: die beiden Tilemap-Sheets (Standard und gepackt); weggelassen: Einzelkacheln, Construct-Projekt.'),
  kandidat('pixel-platformer-metal-expansion', 'Pixel Platformer Metal Expansion', 'Pien Krings (Pienkrings)', 'CC0 1.0', 'https://opengameart.org/content/pixel-platformer-metal-expansion', 'Mauer-/Turm-Materialstufen', 'Tile-Sheets (tiles, tiles-packed) und Sample übernommen; Figuren-Sheets und über 300 Einzelkacheln weggelassen.'),
  kandidat('pixel-platformer', 'Pixel Platformer', 'Kenney', 'CC0 1.0', 'https://kenney.nl/assets/pixel-platformer', 'Treppe (Leitern)', 'Gepackte Kachel- und Hintergrund-Tilemap übernommen; Charakter-Tilemap, Einzelkacheln, Tiled/Construct-Dateien weggelassen.'),
  kandidat('plants-and-flowers-pixel-art', 'Plants and Flowers - Pixel Art', 'peony', 'CC BY 4.0', 'https://opengameart.org/content/plants-and-flowers-pixel-art', 'Plantage', '13 von 36 PNGs (Bäume, Pflanzen, Gartenwerkzeug/Setzling/Samen) übernommen; Blumen-Einzelbilder weggelassen.', 'Namensnennung: "Plants and Flowers - Pixel Art" von peony, https://opengameart.org/content/plants-and-flowers-pixel-art (CC BY 4.0). Laut Urheber genügt Link und Benutzername.'),
  kandidat('quick-32px-sprites-bucket-nest-wood-seeds-leaf-sapling', 'Quick 32px Sprites: Bucket, Nest, Wood, Seeds, Leaf, Sapling', 'Boysano', 'CC0 1.0', 'https://opengameart.org/content/quick-32px-sprites-bucket-nest-wood-seeds-leaf-sapling', 'Plantage', 'Alle 6 Einzel-PNGs (Eimer, Nest, Holz, Samen, Blatt, Setzling) übernommen.'),
  kandidat('tent-6', 'Tent', 'shangri-la', 'CC0 1.0', 'https://opengameart.org/content/tent-6', 'Rekrutierungslager', 'Zelt-Sprite (48x48) übernommen; tent_3.png ist byte-identisch und weggelassen.'),
];
