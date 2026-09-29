import { createRng, type Rng } from '../core/rng';
import type { BiomeConfig, EventChunkKind } from './biome';

/**
 * Prozeduraler Level-Generator (K2C-Stil, 1D Side-Scroller).
 *
 * Aufbau:  [Rand/Ausgang] ... [Portal] ... [Chunks] [HUB] [Chunks] ... [Portal] ... [Ausgang/Rand]
 * - Der Hub liegt immer in der Mitte.
 * - Der Tiefen-Eingang (exit) liegt am Ende der Seite `exitSide`, die andere Seite endet am Rand (edge).
 * - Portale halten Mindestabstand zum Hub und verteilen sich auf beide Seiten.
 * - Alles in "Units" (1 Unit = UNIT_PX Pixel im Rendering).
 *
 * Reine Funktion ohne Phaser-Abhängigkeit => per Vitest testbar.
 */

export type ChunkKind = string; // 'hub' | 'edge' | 'exit' | 'portal' | EventChunkKind | Biom-Chunk-Typ

export interface Chunk {
  index: number;
  kind: ChunkKind;
  startUnits: number;
}

export interface LevelEntity {
  kind: string; // 'castle' | 'portal' | 'exit' | 'tree' | 'rock' | 'chest' | 'skillPoint' | 'recruitCamp' | ...
  x: number; // Units
}

export interface LevelLayout {
  seed: string | number;
  biomeId: string;
  widthUnits: number;
  chunkWidthUnits: number;
  hubCenterUnits: number;
  chunks: Chunk[];
  entities: LevelEntity[];
}

const RESERVED = new Set(['hub', 'edge', 'exit', 'portal']);
const EVENT_KINDS: EventChunkKind[] = ['chest', 'recruitCamp'];

export function generateLevel(biome: BiomeConfig, seed: string | number): LevelLayout {
  const rng = createRng(`${biome.id}:${seed}`);
  const cw = biome.chunkWidthUnits;

  // Gerade Anzahl Chunks, damit der Hub exakt in der Mitte liegt.
  const hubChunks = Math.max(2, Math.round(biome.hubWidthUnits / cw / 2) * 2);
  let chunkCount = Math.round(rng.int(biome.lengthUnits.min, biome.lengthUnits.max) / cw);
  if (chunkCount % 2 !== 0) chunkCount += 1;

  const kinds: (ChunkKind | null)[] = new Array(chunkCount).fill(null);
  const hubStart = chunkCount / 2 - hubChunks / 2;
  for (let i = 0; i < hubChunks; i++) kinds[hubStart + i] = 'hub';
  kinds[biome.exitSide === 'right' ? chunkCount - 1 : 0] = 'exit';
  kinds[biome.exitSide === 'right' ? 0 : chunkCount - 1] = 'edge';

  const hubCenterUnits = (chunkCount / 2) * cw;
  const chunkCenter = (i: number) => i * cw + cw / 2;
  const free = () => kinds.flatMap((k, i) => (k === null ? [i] : []));

  placePortals(biome, rng, kinds, hubCenterUnits, chunkCenter);

  for (const kind of EVENT_KINDS) {
    const { min, max } = biome.eventChunks[kind];
    const slots = rng.shuffle(free());
    const count = rng.int(min, max);
    if (slots.length < count) throw new Error(`Level zu kurz für ${count}x ${kind} (Seed ${String(seed)})`);
    // Rekrutierungs-Camps sollen erreichbar nah am Hub liegen: nächstgelegene freie Slots bevorzugen.
    if (kind === 'recruitCamp') slots.sort((a, b) => Math.abs(chunkCenter(a) - hubCenterUnits) - Math.abs(chunkCenter(b) - hubCenterUnits));
    for (let n = 0; n < count; n++) kinds[slots[n]] = kind;
  }

  for (const i of free()) kinds[i] = rng.weighted(biome.chunkWeights);

  const chunks: Chunk[] = kinds.map((kind, index) => ({ index, kind: kind!, startUnits: index * cw }));
  const entities: LevelEntity[] = [{ kind: 'castle', x: hubCenterUnits }];

  for (const chunk of chunks) {
    const center = chunk.startUnits + cw / 2;
    if (chunk.kind === 'hub' || chunk.kind === 'edge') continue;
    if (RESERVED.has(chunk.kind) || (EVENT_KINDS as string[]).includes(chunk.kind)) {
      entities.push({ kind: chunk.kind, x: center });
      continue;
    }
    const resources = biome.resourcesPerChunk[chunk.kind] ?? {};
    for (const [resource, [min, max]] of Object.entries(resources)) {
      const count = rng.int(min, max);
      for (let n = 0; n < count; n++) {
        // 2 Units Abstand zu Chunk-Grenzen, damit nichts an Nahtstellen klebt.
        entities.push({ kind: resource, x: chunk.startUnits + 2 + rng.next() * (cw - 4) });
      }
    }
  }

  // Skill-Punkte: versteckt in beliebigen Chunks außerhalb von Hub und Rand.
  const hideouts = chunks.filter((c) => c.kind !== 'hub' && c.kind !== 'edge');
  const skillPoints = rng.int(biome.skillPoints.min, biome.skillPoints.max);
  for (let n = 0; n < skillPoints; n++) {
    const chunk = rng.pick(hideouts);
    entities.push({ kind: 'skillPoint', x: chunk.startUnits + 2 + rng.next() * (cw - 4) });
  }

  entities.sort((a, b) => a.x - b.x);
  return { seed, biomeId: biome.id, widthUnits: chunkCount * cw, chunkWidthUnits: cw, hubCenterUnits, chunks, entities };
}

function placePortals(
  biome: BiomeConfig,
  rng: Rng,
  kinds: (ChunkKind | null)[],
  hubCenter: number,
  chunkCenter: (i: number) => number,
): void {
  const { count, minDistanceFromHubUnits } = biome.portals;
  const eligible = (side: 'left' | 'right') =>
    kinds.flatMap((k, i) => {
      const dx = chunkCenter(i) - hubCenter;
      const onSide = side === 'left' ? dx < 0 : dx > 0;
      return k === null && onSide && Math.abs(dx) >= minDistanceFromHubUnits ? [i] : [];
    });

  // Abwechselnd links/rechts, Startseite zufällig.
  let side: 'left' | 'right' = rng.next() < 0.5 ? 'left' : 'right';
  for (let n = 0; n < count; n++) {
    let options = eligible(side);
    if (options.length === 0) options = eligible(side === 'left' ? 'right' : 'left');
    if (options.length === 0) throw new Error(`Kein Platz für Portal ${n + 1}/${count} in ${biome.id}`);
    kinds[rng.pick(options)] = 'portal';
    side = side === 'left' ? 'right' : 'left';
  }
}

/** Prüft die Spielbarkeits-Regeln. Leere Liste = Level ist gültig. */
export function validateLevel(level: LevelLayout, biome: BiomeConfig): string[] {
  const errors: string[] = [];
  const count = (kind: string) => level.chunks.filter((c) => c.kind === kind).length;

  // Toleranz: Runden auf ganze Chunks (±0.5) plus Auffüllen auf gerade Chunk-Anzahl (+1).
  const tolerance = level.chunkWidthUnits * 1.5;
  if (level.widthUnits < biome.lengthUnits.min - tolerance) errors.push('Level zu kurz');
  if (level.widthUnits > biome.lengthUnits.max + tolerance) errors.push('Level zu lang');
  if (count('exit') !== 1) errors.push('Genau ein Tiefen-Eingang erwartet');
  if (count('portal') !== biome.portals.count) errors.push(`${biome.portals.count} Portale erwartet, ${count('portal')} gefunden`);

  const exitChunk = level.chunks.find((c) => c.kind === 'exit');
  const expectedExitIndex = biome.exitSide === 'right' ? level.chunks.length - 1 : 0;
  if (exitChunk && exitChunk.index !== expectedExitIndex) errors.push('Tiefen-Eingang nicht am Rand');

  for (const c of level.chunks.filter((c) => c.kind === 'portal')) {
    const dist = Math.abs(c.startUnits + level.chunkWidthUnits / 2 - level.hubCenterUnits);
    if (dist < biome.portals.minDistanceFromHubUnits) errors.push(`Portal zu nah am Hub (${dist} Units)`);
  }

  for (const kind of EVENT_KINDS) {
    const { min, max } = biome.eventChunks[kind];
    const n = count(kind);
    if (n < min || n > max) errors.push(`${kind}: ${n} (erwartet ${min}-${max})`);
  }

  const skillPoints = level.entities.filter((e) => e.kind === 'skillPoint').length;
  if (skillPoints < biome.skillPoints.min || skillPoints > biome.skillPoints.max) errors.push(`skillPoints: ${skillPoints}`);

  if (level.entities.filter((e) => e.kind === 'castle').length !== 1) errors.push('Genau eine Burg erwartet');
  if (level.entities.some((e) => e.x < 0 || e.x > level.widthUnits)) errors.push('Entity außerhalb des Levels');
  return errors;
}
