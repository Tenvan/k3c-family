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
  license: 'CC0 1.0' | 'CC BY 3.0';
  /** Hinweis zur Lizenz, wenn Quellseite und Pack sich unterscheiden */
  licenseNote?: string;
  source: string;
  /** Wofür es im Spiel taugen könnte (B-010) */
  covers: string[];
  note: string;
}

const ZUNO = 'Luis Zuno (ansimuz)';

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
];
