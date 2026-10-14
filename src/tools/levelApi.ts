import type { LevelResponse } from './levelView';
import { t } from './texts';

/** Ergebnis eines Abrufs: das Level oder ein Hinweis für die Seite (nie eine Ausnahme). */
export type LevelResult = { level: LevelResponse } | { error: string };

/** Das Wenige, was `fetchLevel` von `fetch` braucht (Tests geben eine eigene Funktion mit). */
export type FetchLike = (url: string, init?: RequestInit) => Promise<{ ok: boolean; status: number; json(): Promise<unknown> }>;

/** Anfrage-URL: Seed und Biom werden kodiert, nie in die URL eingeschleust. */
export function levelUrl(seed: string, biome: string): string {
  return `/api/level?seed=${encodeURIComponent(seed)}&biome=${encodeURIComponent(biome)}`;
}

function messageOf(body: unknown): string | null {
  if (typeof body === 'object' && body !== null && 'error' in body && typeof body.error === 'string') return body.error;
  return null;
}

/** `GET /api/level` (B-091): 200 mit dem Level, sonst die Meldung des Servers; ohne Verbindung der Hinweis zum Starten. */
export async function fetchLevel(seed: string, biome: string, fetchFn: FetchLike = (url) => fetch(url)): Promise<LevelResult> {
  let res: Awaited<ReturnType<FetchLike>>;
  try {
    res = await fetchFn(levelUrl(seed, biome));
  } catch {
    return { error: t('level.unreachable') };
  }
  let body: unknown;
  try {
    body = await res.json();
  } catch {
    return { error: res.ok ? t('level.invalid') : t('level.httpError', { status: res.status }) };
  }
  if (!res.ok) return { error: messageOf(body) ?? t('level.httpError', { status: res.status }) };
  if (typeof body !== 'object' || body === null) return { error: t('level.invalid') };
  return { level: body as LevelResponse };
}

/** Ergebnis von `createFullSave`: der Name des angelegten Spielstands oder ein Hinweis für die Seite. */
export type FullSaveResult = { save: string } | { error: string };

const FULL_PREFIX = 'voll-';
/** Der Name `voll-<seed>` muss ins Namensformat der Lobby passen (Kleinbuchstaben, Ziffern, -, höchstens 32 Zeichen). */
const FULL_SEED = /^[a-z0-9-]{1,27}$/;

/**
 * `POST /api/dev/fullsave` (B-315, SV1.3): legt den Spielstand „voll ausgebaut“ zum Seed an. 403 (kein Dev-Mode) und
 * jeder andere Fehler werden ein Hinweis, nie eine Ausnahme.
 */
export async function createFullSave(seed: string, fetchFn: FetchLike = (url, init) => fetch(url, init)): Promise<FullSaveResult> {
  if (!FULL_SEED.test(seed)) return { error: t('level.fullBadSeed') };
  let res: Awaited<ReturnType<FetchLike>>;
  try {
    res = await fetchFn('/api/dev/fullsave', {
      method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ seed }),
    });
  } catch {
    return { error: t('level.unreachable') };
  }
  if (res.status === 403) return { error: t('level.fullForbidden') };
  const body: unknown = await res.json().catch(() => null);
  if (!res.ok) return { error: messageOf(body) ?? t('level.httpError', { status: res.status }) };
  if (typeof body === 'object' && body !== null && 'save' in body && typeof body.save === 'string' && body.save.startsWith(FULL_PREFIX)) {
    return { save: body.save };
  }
  return { error: t('level.fullInvalid') };
}

/** Ziel zum Laden eines vorhandenen Spielstands: Autostart ohne `fresh`, Start in Tiefe 0 (B-095). */
export function fullSaveTarget(save: string): string {
  return `game.html?${new URLSearchParams({ autostart: '1', save }).toString()}`;
}
