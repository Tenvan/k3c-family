/**
 * Grafiken für Ressourcen, Portal, Ausgang, Truhe, Stern, Münze und Hintergründe je Biom (GR3.2, Zuordnung
 * `docs/assets/zuordnung-welt.md`). Reine Auswahl ohne Phaser: `null` = keine Grafik, dann zeichnet der Platzhalter.
 */

/** Datei unter `public/grafik/`; mit Frame-Größe als Spritesheet geladen */
export interface WorldFile {
  file: string;
  frameWidth?: number;
  frameHeight?: number;
}

export const WORLD_TEXTURES: Record<string, WorldFile> = {
  'grafik:trees': { file: 'gotthicvania-swamp/umgebung/trees.png' },
  'grafik:stones': { file: 'various-stones-and-oregem-veins-16x16/ressourcen/Stones_ores_gems_without_grass.png', frameWidth: 16, frameHeight: 16 },
  'grafik:plant': { file: 'sunnyland-tall-forest-environment/props/Plant.png' },
  'grafik:portals': { file: 'portals-32-x-48/portale/portalsSpriteSheet.png', frameWidth: 32, frameHeight: 48 },
  'grafik:exit': { file: 'k3c-paletten/props/fort-door.png' },
  'grafik:chest': { file: 'gold-treasure-icons-16x16/icons/8.png' },
  'grafik:star': { file: 'item-ruby-banana-star/props/part-star.png', frameWidth: 16, frameHeight: 16 },
  'grafik:tent': { file: 'tent-8/props/objs.png' },
  'grafik:campfire': { file: '16x16-animated-campfire/props/campfire-16x16.png', frameWidth: 16, frameHeight: 16 },
  'grafik:coins': { file: '16x16-small-and-medium-coin-animation/muenzen/sCoins_1.png', frameWidth: 16, frameHeight: 16 },
  'bg:forest-back': { file: 'forest-background/ebenen/parallax-forest-back-trees.png' },
  'bg:forest-lights': { file: 'forest-background/ebenen/parallax-forest-lights.png' },
  'bg:forest-middle': { file: 'forest-background/ebenen/parallax-forest-middle-trees.png' },
  'bg:forest-front': { file: 'forest-background/ebenen/parallax-forest-front-trees.png' },
  'bg:cave': { file: 'blue-cave-background/ebenen/startcavebg.png' },
  'bg:mine-back': { file: 'warped-super-grotto-escape-pack/ebenen/back.png' },
  'bg:mine-far': { file: 'warped-super-grotto-escape-pack/ebenen/far.png' },
  'bg:mine-middle': { file: 'warped-super-grotto-escape-pack/ebenen/middle.png' },
};

/** Ausschnitte (Pixel im Bild, per Alpha-Analyse): erstes Zelt aus dem Blatt tent-8 */
export const WORLD_FRAMES: Record<string, [string, number, number, number, number][]> = {
  'grafik:tent': [['tent', 4, 4, 63, 79]],
};

/** Bild eines Objekts: Frame im Sheet (Nummer oder Ausschnitt-Name), Anzahl Frames für die Animation (1 = still), Zoom (Q13 ganzzahlig) */
export interface ObjectSprite {
  key: string;
  frame: number | string;
  frames: number;
  zoom: number;
}

const sprite = (key: string, frame: number | string = 0, frames = 1, zoom = 2): ObjectSprite => ({ key, frame, frames, zoom });

/** Objekt-IDs wie in der Zuordnung; Erz-Sheet 7 Spalten: Zeile 1 Spalte 2 = Fels, Zeile 2 Spalte 1 = Kupfer */
const OBJECTS: Record<string, ObjectSprite> = {
  'node:tree': sprite('grafik:trees'),
  'node:rock': sprite('grafik:stones', 1),
  'node:copperOre': sprite('grafik:stones', 7),
  'node:bush': sprite('grafik:plant'),
  portal: sprite('grafik:portals', 0, 4, 1),
  exit: sprite('grafik:exit'),
  'pickup:chest': sprite('grafik:chest'),
  'pickup:skillPoint': sprite('grafik:star', 0, 4),
  coin: sprite('grafik:coins', 0, 8),
  'camp:recruit': sprite('grafik:tent', 'tent'),
  'camp:recruit:fire': sprite('grafik:campfire', 0, 4),
};

export function objectSprite(id: string): ObjectSprite | null {
  return OBJECTS[id] ?? null;
}

/** Hintergrund-Ebene: Textur und Scroll-Faktor (hinten langsam, vorne schneller) */
export interface BackgroundLayer {
  key: string;
  scroll: number;
}

const BACKGROUNDS: Record<string, { zoom: number; layers: BackgroundLayer[] }> = {
  forest: {
    zoom: 3,
    layers: [{ key: 'bg:forest-back', scroll: 0.15 }, { key: 'bg:forest-lights', scroll: 0.25 }, { key: 'bg:forest-middle', scroll: 0.35 }, { key: 'bg:forest-front', scroll: 0.5 }],
  },
  cave: { zoom: 2, layers: [{ key: 'bg:cave', scroll: 0.2 }] },
  mine: { zoom: 2, layers: [{ key: 'bg:mine-back', scroll: 0.1 }, { key: 'bg:mine-far', scroll: 0.25 }, { key: 'bg:mine-middle', scroll: 0.45 }] },
};

/** Ebenen eines Bioms; `null` = keine Zuordnung, dann bleiben die Silhouetten */
export function biomeBackground(biome: string): { zoom: number; layers: BackgroundLayer[] } | null {
  return BACKGROUNDS[biome] ?? null;
}
