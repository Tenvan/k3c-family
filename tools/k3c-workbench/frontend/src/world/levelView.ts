import type { PlannedChunk, WorldLevel } from '../api';

// Chunk-Vorschau des Welten-Editors (B-879, Port aus dem Asset-Studio, world/levelView.ts): reine Funktionen. Aus dem
// Level wird das Zeichenmodell; dazu Zufalls-Seed, Platz eines Abschnitts und seine Objekte. Kein DOM, kein Netz.

export interface ModelChunk {
  index: number;
  kind: string;
  /** Units */
  from: number;
  to: number;
  /** Platz relativ zu den Hub-Chunks (−n links, +n rechts, 0 = Hub), wie `slot` der geplanten Chunks */
  slot: number;
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
  planned: PlannedChunk[];
  warnings: string[];
}

export const INCOMPLETE = 'Antwort unvollständig';

/** Platz je Chunk-Index: Hub-Chunks sind 0, links davon −1, −2 …, rechts +1, +2 … (Go: gen.slotIndex umgekehrt). */
export function slotsOf(kinds: readonly string[]): number[] {
  const first = kinds.indexOf('hub');
  const last = kinds.lastIndexOf('hub');
  if (first < 0) return kinds.map(() => 0);
  return kinds.map((_, i) => (i < first ? i - first : i > last ? i - last : 0));
}

/** Zeichenmodell eines Levels; ein unvollständiges ergibt ein leeres Modell mit einer Warnung, nie einen Fehler. */
export function levelModel(res: Partial<WorldLevel> | null | undefined): LevelModel {
  const r = res ?? {};
  if (!Array.isArray(r.chunks) || !Array.isArray(r.entities) || typeof r.widthUnits !== 'number' || typeof r.chunkWidthUnits !== 'number') {
    return { seed: r.seed ?? '', biomeId: r.biomeId ?? '', widthUnits: 0, hubCenterUnits: 0, chunks: [], objects: [], counts: [], planned: [], warnings: [...(r.warnings ?? []), INCOMPLETE] };
  }
  const width = r.chunkWidthUnits;
  const slots = slotsOf(r.chunks.map((c) => c.kind));
  const objects = r.entities.map((e) => ({ kind: e.kind, x: e.x })).sort((a, b) => a.x - b.x);
  const perKind = new Map<string, number>();
  for (const o of objects) perKind.set(o.kind, (perKind.get(o.kind) ?? 0) + 1);
  return {
    seed: r.seed ?? '',
    biomeId: r.biomeId ?? '',
    widthUnits: r.widthUnits,
    hubCenterUnits: r.hubCenterUnits ?? 0,
    chunks: r.chunks.map((c, i) => ({ index: c.index, kind: c.kind, from: c.startUnits, to: c.startUnits + width, slot: slots[i] ?? 0 })),
    objects,
    counts: [...perKind].sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0])),
    planned: r.planned ?? [],
    warnings: r.warnings ?? [],
  };
}

/** Objekte im Abschnitt (from ≤ x < to). */
export const objectsIn = (model: LevelModel, chunk: ModelChunk): ModelObject[] => model.objects.filter((o) => o.x >= chunk.from && o.x < chunk.to);

/** Planung des Abschnitts (gleicher Platz), sonst undefined; der Hub ist nie geplant. */
export const plannedAt = (model: LevelModel, chunk: ModelChunk): PlannedChunk | undefined =>
  chunk.slot === 0 ? undefined : model.planned.find((p) => p.slot === chunk.slot);

/** Platz als Text: „−3“, „+2“, „Hub“. */
export const slotLabel = (slot: number): string => (slot === 0 ? 'Hub' : slot > 0 ? `+${slot}` : `−${-slot}`);

const ALPHABET = 'abcdefghijklmnopqrstuvwxyz0123456789';
const SEED_LENGTH = 8;

/** Zufalls-Seed (Kleinbuchstaben und Ziffern) aus Zufallsbytes; gleiche Bytes ergeben denselben Seed. */
export function seedFromBytes(bytes: ArrayLike<number>): string {
  return Array.from({ length: SEED_LENGTH }, (_, i) => ALPHABET[(bytes[i] ?? 0) % ALPHABET.length]).join('');
}

const COLORS: Record<string, string> = {
  castle: '#ffd166', portal: '#b388ff', exit: '#6bd96b', tree: '#3d6b3d', bush: '#5f8f4f', rock: '#8a97aa',
  chest: '#e8a33d', skillPoint: '#4dd0e1', recruitCamp: '#ff8fa3', harbor: '#4fa3e0',
};

/** Farbe eines Objekts im Streifen; unbekannte Arten bekommen eine neutrale Farbe. */
export const colorOf = (kind: string): string => COLORS[kind] ?? '#c0c8d4';

/** Große Marker: Burg, Portal und Ausgang. */
export const BIG_KINDS: ReadonlySet<string> = new Set(['castle', 'portal', 'exit']);
/** Abschnitte mit einem besonderen Objekt werden hervorgehoben. */
export const SPECIAL_CHUNKS: ReadonlySet<string> = new Set(['portal', 'exit', 'edge', 'chest', 'recruitCamp', 'harbor']);
