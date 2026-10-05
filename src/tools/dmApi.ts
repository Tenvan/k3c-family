/** Zugriff der Dungeon-Master-Seite auf `/api/dev` (B-232, docs/protocol.md › HTTP: Dungeon-Master-Seite). */

export interface DmRoom {
  code: string;
  name: string;
  depth: number;
  taken: number;
  running: boolean;
}

export interface DmDiagnose {
  code: string;
  tick: number;
  phase: string;
  day: number;
  gold: number[];
  troops: Record<string, number>;
  enemies: number;
  castleHp: number;
  wave: number;
  devices: { id: string; connected: boolean; slots: number[] }[];
  stages: { depth: number; phase: string; day: number; players: number[] }[];
  timescale: number;
  paused: boolean;
}

/** Eine Dev-Aktion wie die Nachricht `dev` ohne `t`; `slot` ist der Index des Monarchen. */
export type DmAction = Record<string, string | number | boolean>;

export type FetchLike = (
  url: string,
  init?: { method: string; body: string },
) => Promise<{ ok: boolean; status: number; json(): Promise<unknown> }>;

const defaultFetch: FetchLike = (url, init) => fetch(url, init);

/** Knöpfe der Seite; `slot` setzt die Seite für Aktionen, die einen Monarchen brauchen. */
export const DM_ACTIONS: { label: string; action: DmAction; needsSlot?: boolean }[] = [
  { label: '🪙 +50 Gold', action: { action: 'gold', amount: 50 }, needsSlot: true },
  ...['wood', 'stone', 'copper', 'iron', 'crystal'].map((resource) => ({
    label: `📦 +20 ${resource}`,
    action: { action: 'material', amount: 20, resource },
    needsSlot: true,
  })),
  ...[1, 2, 4, 8].map((factor) => ({ label: `⏩ Zeit ${factor}×`, action: { action: 'timescale', factor } })),
  { label: '⏸ Pause', action: { action: 'pause', paused: true } },
  { label: '▶ Weiter', action: { action: 'pause', paused: false } },
  { label: '🌙 Welle', action: { action: 'wave' }, needsSlot: true },
  { label: '☀ Tag', action: { action: 'phase', phase: 'day' } },
  { label: '🌆 Dämmerung', action: { action: 'phase', phase: 'dusk' } },
  { label: '🌑 Nacht', action: { action: 'phase', phase: 'night' } },
];

export const devUrl = (room?: string): string => (room ? `/api/dev?room=${encodeURIComponent(room)}` : '/api/dev');

/** Raumliste und Dev-Mode; null ohne Server. */
export async function fetchRooms(f: FetchLike = defaultFetch): Promise<{ dev: boolean; rooms: DmRoom[] } | null> {
  try {
    const res = await f(devUrl());
    return res.ok ? ((await res.json()) as { dev: boolean; rooms: DmRoom[] }) : null;
  } catch {
    return null;
  }
}

/** Diagnose eines Raums; 'gone', wenn der Raum geschlossen ist, null ohne Server. */
export async function fetchDiagnose(
  room: string,
  f: FetchLike = defaultFetch,
): Promise<{ dev: boolean; room: DmDiagnose } | 'gone' | null> {
  try {
    const res = await f(devUrl(room));
    if (res.status === 404) return 'gone';
    return res.ok ? ((await res.json()) as { dev: boolean; room: DmDiagnose }) : null;
  } catch {
    return null;
  }
}

/** Schickt eine Aktion; liefert null bei Erfolg, sonst die Meldung. */
export async function sendAction(room: string, action: DmAction, f: FetchLike = defaultFetch): Promise<string | null> {
  try {
    const res = await f(devUrl(room), { method: 'POST', body: JSON.stringify(action) });
    if (res.ok) return null;
    const body = (await res.json().catch(() => null)) as { error?: string } | null;
    return body?.error ?? `Fehler ${res.status}`;
  } catch {
    return 'Server nicht erreichbar';
  }
}
