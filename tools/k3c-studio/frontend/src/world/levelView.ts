import type { LevelLayout } from '@game/model/types';

// Chunk-Vorschau des Welt-Betrachters (B-460, WZ7.2): reine Funktionen, aus der früheren Seite leveltest.html
// (B-092, src/tools/levelView.ts) hierher umgezogen. Aus dem Level wird das Zeichenmodell; dazu Zoom und Zufalls-Seed.
// Kein DOM, kein Netz. „Im Spiel starten“ und „Voll ausgebaut“ folgen in WZ7.4.

/** Das Level wie im Spiel plus die Warnungen der Spielbarkeits-Prüfung (Go: WorldLevelView ohne Error). */
export type LevelResponse = LevelLayout & { warnings: string[] };

export interface ModelChunk {
  index: number;
  kind: string;
  /** Units */
  from: number;
  to: number;
}

export interface ModelObject {
  kind: string;
  /** Units */
  x: number;
}

export interface LevelModel {
  seed: string;
  biomeId: string;
  widthUnits: number;
  hubCenterUnits: number;
  chunks: ModelChunk[];
  /** nach x sortiert */
  objects: ModelObject[];
  /** Objekte je Art, die häufigste zuerst */
  counts: [kind: string, count: number][];
  warnings: string[];
}

export const INCOMPLETE = 'Antwort unvollständig';

function emptyModel(res: Partial<LevelResponse>): LevelModel {
  return {
    seed: String(res.seed ?? ''),
    biomeId: res.biomeId ?? '',
    widthUnits: 0,
    hubCenterUnits: 0,
    chunks: [],
    objects: [],
    counts: [],
    warnings: [...(res.warnings ?? []), INCOMPLETE],
  };
}

/** Zeichenmodell eines Levels; ein unvollständiges ergibt ein leeres Modell mit einer Warnung, nie einen Fehler. */
export function levelModel(res: Partial<LevelResponse> | null | undefined): LevelModel {
  if (!res || !Array.isArray(res.chunks) || !Array.isArray(res.entities) || typeof res.widthUnits !== 'number' || typeof res.chunkWidthUnits !== 'number') {
    return emptyModel(res ?? {});
  }
  const width = res.chunkWidthUnits;
  const objects = res.entities.map((e) => ({ kind: e.kind, x: e.x })).sort((a, b) => a.x - b.x);
  const perKind = new Map<string, number>();
  for (const o of objects) perKind.set(o.kind, (perKind.get(o.kind) ?? 0) + 1);
  return {
    seed: String(res.seed ?? ''),
    biomeId: res.biomeId ?? '',
    widthUnits: res.widthUnits,
    hubCenterUnits: res.hubCenterUnits ?? 0,
    chunks: res.chunks.map((c) => ({ index: c.index, kind: c.kind, from: c.startUnits, to: c.startUnits + width })),
    objects,
    counts: [...perKind].sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0])),
    warnings: res.warnings ?? [],
  };
}

/** Namensformat eines Spielstands, wie `SAVE_FORMAT` in `src/scenes/lobbyLogic.ts` (der Seed eines neuen Spiels ist dieser Name). */
const SAVE_NAME = /^[a-z0-9-]{1,32}$/;
/** Für „voll ausgebaut“ muss `voll-<seed>` ins Namensformat passen (Go: fullSeed). */
const FULL_SEED = /^[a-z0-9-]{1,27}$/;

/** Grund, warum „Im Spiel öffnen“ nicht geht, oder null. Ein neues Spiel gibt es nur für den Wald und Seeds im Namensformat,
 *  weil der Server den Seed aus dem Namen des Spielstands nimmt und beim Autostart in Tiefe 0 beginnt (B-095); „voll
 *  ausgebaut“ hat jedes Biom, der Seed muss kürzer sein. Go prüft dasselbe (`WorldOpenGame`). */
export function openGameReason(mode: 'new' | 'full', seed: string, biomeId: string): string | null {
  if (mode === 'new' && biomeId !== 'forest') return 'Start nur für den Wald (Tiefe 0)';
  if (mode === 'new' && !SAVE_NAME.test(seed)) return 'Seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 32 Zeichen)';
  if (mode === 'full' && !FULL_SEED.test(seed)) return 'Für „voll ausgebaut“ muss der Seed aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 27 Zeichen)';
  return null;
}

const ALPHABET = 'abcdefghijklmnopqrstuvwxyz0123456789';
const SEED_LENGTH = 8;

/** Startbarer Zufalls-Seed (Kleinbuchstaben und Ziffern) aus Zufallsbytes; gleiche Bytes ergeben denselben Seed. */
export function seedFromBytes(bytes: ArrayLike<number>): string {
  return Array.from({ length: SEED_LENGTH }, (_, i) => ALPHABET[(bytes[i] ?? 0) % ALPHABET.length]).join('');
}

export const MIN_SCALE = 2;
export const MAX_SCALE = 24;
export const DEFAULT_SCALE = 6;

/** Neuer Maßstab (Pixel je Unit) nach einem Zoomschritt, begrenzt auf MIN_SCALE bis MAX_SCALE. */
export function zoomScale(scale: number, factor: number): number {
  return Math.min(MAX_SCALE, Math.max(MIN_SCALE, scale * factor));
}

const COLORS: Record<string, string> = {
  castle: '#ffd166',
  portal: '#b388ff',
  exit: '#6bd96b',
  tree: '#3d6b3d',
  bush: '#5f8f4f',
  rock: '#8a97aa',
  chest: '#e8a33d',
  skillPoint: '#4dd0e1',
  recruitCamp: '#ff8fa3',
};

/** Farbe eines Objekts im Streifen; unbekannte Arten bekommen eine neutrale Farbe. */
export const colorOf = (kind: string): string => COLORS[kind] ?? '#c0c8d4';

/** Große Marker: Burg, Portal und Ausgang. */
export const BIG_KINDS: ReadonlySet<string> = new Set(['castle', 'portal', 'exit']);
/** Abschnitte mit einem besonderen Objekt werden hervorgehoben. */
export const SPECIAL_CHUNKS: ReadonlySet<string> = new Set(['portal', 'exit', 'edge', 'chest', 'recruitCamp']);
