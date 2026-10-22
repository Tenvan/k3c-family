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

/** Schichten je Biom (GEB7): Namensteil der Dateien `gebaeude/hg-<teil>-<fern|mitte|nah>-32.png` und Scroll-Faktoren hinten → vorn */
const BACKGROUND_LAYERS: Record<string, { teil: string; scroll: [number, number, number] }> = {
  forest: { teil: 'wald', scroll: [0.15, 0.35, 0.5] },
  cave: { teil: 'hoehle', scroll: [0.1, 0.2, 0.4] },
  mine: { teil: 'mine', scroll: [0.1, 0.25, 0.45] },
  ironhold: { teil: 'eisenstollen', scroll: [0.1, 0.25, 0.45] },
  crystal: { teil: 'kristall', scroll: [0.1, 0.25, 0.45] },
};
const LAYER_NAMES = ['fern', 'mitte', 'nah'];

/** Texturen `bg:<biom>:<n>` (n = 1 hinten … 3 vorn) aus den Hintergrund-Schichten von `tools/buildings` */
function backgroundFiles(): Record<string, WorldFile> {
  return Object.fromEntries(
    Object.entries(BACKGROUND_LAYERS).flatMap(([biome, b]) => LAYER_NAMES.map((name, i) => [`bg:${biome}:${i + 1}`, { file: `gebaeude/hg-${b.teil}-${name}-32.png` }])),
  );
}

export const WORLD_TEXTURES: Record<string, WorldFile> = {
  ...natureFiles(),
  ...backgroundFiles(),
  'grafik:coinIcon': { file: 'gold-treasure-icons-16x16/icons/2.png' }, // Münze von vorn, Hinweis der ersten Nacht (B-294)
  'grafik:star': { file: 'item-ruby-banana-star/props/part-star.png', frameWidth: 16, frameHeight: 16 },
  'grafik:coins': { file: '16x16-small-and-medium-coin-animation/muenzen/sCoins_1.png', frameWidth: 16, frameHeight: 16 },
};

/** Bild eines Objekts: Frame im Sheet (Nummer oder Ausschnitt-Name), Anzahl Frames für die Animation (1 = still), Zoom (Q13 ganzzahlig) */
export interface ObjectSprite {
  key: string;
  frame: number | string;
  frames: number;
  zoom: number;
}

const sprite = (key: string, frame: number | string = 0, frames = 1, zoom = 2): ObjectSprite => ({ key, frame, frames, zoom });

/** Objekt-IDs wie in der Zuordnung (Knoten: `natureSprites.ts` › `nodeSprite`); Truhe, Portal, Ausgang, Lager aus `tools/buildings` im Maßstab 1 (GEB6) */
const OBJECTS: Record<string, ObjectSprite> = {
  portal: sprite('grafik:portal', 0, 1, 1),
  exit: sprite('grafik:ausgang', 0, 1, 1),
  // Eine liegende Truhe ist ungeöffnet: geschlossen; die offene zeigt der Renderer kurz beim Aufheben (GEB10)
  'pickup:chest': sprite('grafik:truhe-geschlossen', 0, 1, 1),
  'pickup:skillPoint': sprite('grafik:star', 0, 4),
  coin: sprite('grafik:coins', 0, 8),
  'camp:recruit': sprite('grafik:lager', 0, 1, 1),
};

/** Geschlossene Truhe → offene: der Renderer zeigt sie kurz, wenn die Truhe aufgehoben wird (GEB10, B-455) */
export const CHEST_OPEN: Record<string, string> = { 'grafik:truhe-geschlossen': 'grafik:truhe' };

export function objectSprite(id: string): ObjectSprite | null {
  return OBJECTS[id] ?? null;
}

/** Hintergrund-Ebene: Textur und Scroll-Faktor (hinten langsam, vorne schneller) */
export interface BackgroundLayer {
  key: string;
  scroll: number;
}

/** Maßstab 1 wie die Gebäude (Regeldatei § 10) */
const BACKGROUNDS: Record<string, { zoom: number; layers: BackgroundLayer[] }> = Object.fromEntries(
  Object.entries(BACKGROUND_LAYERS).map(([biome, b]) => [biome, { zoom: 1, layers: b.scroll.map((scroll, i) => ({ key: `bg:${biome}:${i + 1}`, scroll })) }]),
);

/** Ebenen eines Bioms; `null` = keine Zuordnung, dann bleiben die Silhouetten */
export function biomeBackground(biome: string): { zoom: number; layers: BackgroundLayer[] } | null {
  return BACKGROUNDS[biome] ?? null;
}
