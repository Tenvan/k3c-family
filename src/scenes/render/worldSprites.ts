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

/** Schichten je Biom (GEB7): Namensteil der Dateien `gebaeude/hg-<teil>-<fern|mitte|nah>-v<n>-64.png` und Scroll-Faktoren hinten → vorn */
const BACKGROUND_LAYERS: Record<string, { teil: string; scroll: [number, number, number] }> = {
  forest: { teil: 'wald', scroll: [0.15, 0.35, 0.5] },
  cave: { teil: 'hoehle', scroll: [0.1, 0.2, 0.4] },
  mine: { teil: 'mine', scroll: [0.1, 0.25, 0.45] },
  ironhold: { teil: 'eisenstollen', scroll: [0.1, 0.25, 0.45] },
  crystal: { teil: 'kristall', scroll: [0.1, 0.25, 0.45] },
};
const LAYER_NAMES = ['fern', 'mitte', 'nah'];

/** Dichte der Hintergrund-Dateien, die das Spiel lädt (GEB18, Entscheidung Ralf: 64 statt 128, auf 1080p voll sichtbar) */
export const BACKGROUND_DENSITY = 64;
/** Maßstab der Hintergrund-Bilder: 64 px je Kachel → 32 px je Unit */
export const BACKGROUND_ZOOM = 32 / BACKGROUND_DENSITY;
/** Breite einer Hintergrund-Kachel in Spiel-Pixeln (24 Units, Regeldatei § 10; Server: `level.BackdropTileUnits`) */
export const BACKGROUND_TILE_PX = 24 * 32;

/** Textur-Schlüssel einer Schicht (n = 1 hinten … 3 vorn) in Variante v */
export const backgroundKey = (biome: string, n: number, v: number): string => `bg:${biome}:${n}:v${v}`;

/** Datei zu einem Schlüssel aus `backgroundKey`; `null` = kein Hintergrund-Schlüssel oder unbekanntes Biom */
export function backgroundFile(key: string): WorldFile | null {
  const m = /^bg:([a-z]+):([1-3]):v([1-9][0-9]*)$/.exec(key);
  const b = m && BACKGROUND_LAYERS[m[1]!];
  return b ? { file: `gebaeude/hg-${b.teil}-${LAYER_NAMES[Number(m[2]) - 1]}-v${m[3]}-${BACKGROUND_DENSITY}.png` } : null;
}

/** Bilder einer Hintergrund-Schicht: je Kachel k ein Bild bei x = k · BACKGROUND_TILE_PX in Variante `backdrops[k]` */
export interface BackgroundLayerPlan {
  scroll: number;
  parts: { key: string; x: number }[];
}

/**
 * Hintergrund einer Stufe (GEB18): je Schicht so viele Kacheln, wie die Welt beim Scroll-Faktor braucht, Variante je Kachel
 * aus `level.backdrops` (Server, nach Seed); fehlt die Folge, überall Variante 1, ist sie kürzer, wiederholt sie sich.
 * `null` = Biom ohne Hintergrund. Rein, ohne Phaser; geladen werden nur die Schlüssel des Plans.
 */
export function backgroundPlan(biome: string, widthPx: number, viewPx: number, backdrops?: readonly number[]): BackgroundLayerPlan[] | null {
  const b = BACKGROUND_LAYERS[biome];
  if (!b) return null;
  return b.scroll.map((scroll, i) => {
    const count = Math.ceil((widthPx * scroll + viewPx * 2) / BACKGROUND_TILE_PX);
    const parts = Array.from({ length: count }, (_, k) => {
      const v = backdrops?.length ? backdrops[k % backdrops.length]! : 1;
      return { key: backgroundKey(biome, i + 1, v), x: k * BACKGROUND_TILE_PX };
    });
    return { scroll, parts };
  });
}

/** Schichten eines Bioms fern → nah mit Name `bg:<biom>:<n>` (Name der Container in der Stufe) und Scroll-Faktor; `null` = kein Hintergrund */
export function biomeLayers(biome: string): { key: string; scroll: number }[] | null {
  return BACKGROUND_LAYERS[biome]?.scroll.map((scroll, i) => ({ key: `bg:${biome}:${i + 1}`, scroll })) ?? null;
}

/** Alle Schlüssel eines Plans, je einmal (die Stufe lädt genau diese) */
export const planKeys = (plan: readonly BackgroundLayerPlan[]): string[] => [...new Set(plan.flatMap((l) => l.parts.map((p) => p.key)))];

export const WORLD_TEXTURES: Record<string, WorldFile> = {
  ...natureFiles(),
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

