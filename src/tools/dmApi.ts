/** Zugriff der Dungeon-Master-Seite auf `/api/dev` (B-232, docs/protocol.md › HTTP: Dungeon-Master-Seite). */
import { nameOf } from '../core/texts';
import { t } from './texts';

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

/** Knöpfe der Seite (Beschriftung beim Laden in der gewählten Sprache); `slot` setzt die Seite für Aktionen, die einen Monarchen brauchen. */
export const DM_ACTIONS: { label: string; action: DmAction; needsSlot?: boolean }[] = [
  { label: t('dm.gold'), action: { action: 'gold', amount: 50 }, needsSlot: true },
  ...['wood', 'stone', 'copper', 'iron', 'crystal'].map((resource) => ({
    label: t('dm.material', { resource: nameOf('res', resource, resource) }),
    action: { action: 'material', amount: 20, resource },
    needsSlot: true,
  })),
  ...[1, 2, 4, 8].map((factor) => ({ label: t('dm.timescale', { factor }), action: { action: 'timescale', factor } })),
  { label: t('dm.pause'), action: { action: 'pause', paused: true } },
  { label: t('dm.resume'), action: { action: 'pause', paused: false } },
  { label: t('dm.wave'), action: { action: 'wave' }, needsSlot: true },
  { label: t('dm.day'), action: { action: 'phase', phase: 'day' } },
  { label: t('dm.dusk'), action: { action: 'phase', phase: 'dusk' } },
  { label: t('dm.night'), action: { action: 'phase', phase: 'night' } },
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
    return body?.error ?? t('level.httpError', { status: res.status });
  } catch {
    return t('dm.unreachable');
  }
}
