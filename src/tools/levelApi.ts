import type { LevelResponse } from './levelView';

/** Ergebnis eines Abrufs: das Level oder ein Hinweis für die Seite (nie eine Ausnahme). */
export type LevelResult = { level: LevelResponse } | { error: string };

/** Das Wenige, was `fetchLevel` von `fetch` braucht (Tests geben eine eigene Funktion mit). */
export type FetchLike = (url: string) => Promise<{ ok: boolean; status: number; json(): Promise<unknown> }>;

export const SERVER_UNREACHABLE = 'Server nicht erreichbar. Starte ihn mit `task start`.';
export const INVALID_ANSWER = 'Die Antwort des Servers ist kein gültiges Level.';

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
    return { error: SERVER_UNREACHABLE };
  }
  let body: unknown;
  try {
    body = await res.json();
  } catch {
    return { error: res.ok ? INVALID_ANSWER : `Fehler ${res.status}` };
  }
  if (!res.ok) return { error: messageOf(body) ?? `Fehler ${res.status}` };
  if (typeof body !== 'object' || body === null) return { error: INVALID_ANSWER };
  return { level: body as LevelResponse };
}
