import type { LevelLayout } from '../model/types';
import { t } from './texts';

/**
 * Level-Betrachter (B-092): reine Funktionen für die Seite `leveltest.html`. Aus der Antwort von `GET /api/level`
 * wird das Zeichenmodell; dazu die Start-URL für „Im Spiel starten“ und der Zufalls-Seed. Kein DOM, kein Netz.
 */

/** Antwort von `GET /api/level`: die Felder des Levels plus die Warnungen der Spielbarkeits-Prüfung. */
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

function emptyModel(res: Partial<LevelResponse>): LevelModel {
  return {
    seed: String(res.seed ?? ''),
    biomeId: res.biomeId ?? '',
    widthUnits: 0,
    hubCenterUnits: 0,
    chunks: [],
    objects: [],
    counts: [],
    warnings: [...(res.warnings ?? []), t('level.incomplete')],
  };
}

/** Zeichenmodell einer Antwort; eine unvollständige Antwort ergibt ein leeres Modell mit einer Warnung, nie einen Fehler. */
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

export type StartTarget = { url: string } | { reason: string };

/**
 * Ob und wohin „Im Spiel starten“ führt (ohne `fresh`: die Lobby lädt einen vorhandenen Spielstand mit diesem Namen oder legt ihn einmal neu an, B-096): nur für den Wald (Tiefe 0) und Seeds im Namensformat, weil der Server den Seed
 * eines neuen Spiels aus dem Namen des Spielstands nimmt und beim Autostart immer in Tiefe 0 beginnt (B-095).
 */
export function startTarget(seed: string, biomeId: string): StartTarget {
  if (biomeId !== 'forest') return { reason: t('level.forestOnly') };
  if (!SAVE_NAME.test(seed)) return { reason: t('level.badSeed') };
  const params = new URLSearchParams({ autostart: '1', save: seed });
  return { url: `game.html?${params.toString()}` };
}

const ALPHABET = 'abcdefghijklmnopqrstuvwxyz0123456789';
const SEED_LENGTH = 8;

/** Startbarer Zufalls-Seed (Kleinbuchstaben und Ziffern) aus Zufallsbytes; gleiche Bytes ergeben denselben Seed. */
export function seedFromBytes(bytes: ArrayLike<number>): string {
  return Array.from({ length: SEED_LENGTH }, (_, i) => ALPHABET[(bytes[i] ?? 0) % ALPHABET.length]).join('');
}
