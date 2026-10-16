import { natureFiles } from './natureSprites';

/**
 * Grafiken für Truhe, Portal, Ausgang, Lager, Stern, Münze und Hintergründe je Biom (GR3.2, Zuordnung
 * `docs/assets/zuordnung-welt.md`); Bäume, Fels, Erz, Busch und Adern wählt `natureSprites.ts` (GEB6). Reine Auswahl ohne
 * Phaser: `null` = keine Grafik, dann zeichnet der Platzhalter.
 */

/** Datei unter `public/grafik/`; mit Frame-Größe als Spritesheet geladen */
export interface WorldFile {
  file: string;
  frameWidth?: number;
  frameHeight?: number;
}

export const WORLD_TEXTURES: Record<string, WorldFile> = {
  ...natureFiles(),
  'grafik:coinIcon': { file: 'gold-treasure-icons-16x16/icons/2.png' }, // Münze von vorn, Hinweis der ersten Nacht (B-294)
  'grafik:star': { file: 'item-ruby-banana-star/props/part-star.png', frameWidth: 16, frameHeight: 16 },
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

/** Bild eines Objekts: Frame im Sheet (Nummer oder Ausschnitt-Name), Anzahl Frames für die Animation (1 = still), Zoom (Q13 ganzzahlig) */
export interface ObjectSprite {
  key: string;
  frame: number | string;
  frames: number;
  zoom: number;
}

const sprite = (key: string, frame: number | string = 0, frames = 1, zoom = 2): ObjectSprite => ({ key, frame, frames, zoom });

/** Objekt-IDs wie in der Zuordnung (Knoten: `natureSprites.ts` › `nodeSprite`); Truhe, Portal, Ausgang, Lager aus `tools/gebaeude` im Maßstab 1 (GEB6) */
const OBJECTS: Record<string, ObjectSprite> = {
  portal: sprite('grafik:portal', 0, 1, 1),
  exit: sprite('grafik:ausgang', 0, 1, 1),
  'pickup:chest': sprite('grafik:truhe', 0, 1, 1),
  'pickup:skillPoint': sprite('grafik:star', 0, 4),
  coin: sprite('grafik:coins', 0, 8),
  'camp:recruit': sprite('grafik:lager', 0, 1, 1),
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
