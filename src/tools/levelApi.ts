import type { LevelResponse } from './levelView';
import { t } from './texts';

/** Ergebnis eines Abrufs: das Level oder ein Hinweis für die Seite (nie eine Ausnahme). */
export type LevelResult = { level: LevelResponse } | { error: string };

/** Das Wenige, was `fetchLevel` von `fetch` braucht (Tests geben eine eigene Funktion mit). */
export type FetchLike = (url: string) => Promise<{ ok: boolean; status: number; json(): Promise<unknown> }>;

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
