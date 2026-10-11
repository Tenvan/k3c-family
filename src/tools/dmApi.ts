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
export interface DmEntry {
  label: string;
  action: DmAction;
  needsSlot?: boolean;
  /** Umschalter: die Seite merkt den Zustand und sendet `on` (true/false). */
  toggle?: boolean;
  /** Stufenwahl 1–5: die Seite sendet die gewählte Stufe als `level`. */
  leveled?: boolean;
}

/** Größte wählbare Ausbau-Stufe von `buildAll`. */
export const DM_MAX_LEVEL = 5;

export const DM_ACTIONS: DmEntry[] = [
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
  { label: t('dm.nextPhase'), action: { action: 'nextPhase' } },
  { label: t('dm.maxStock'), action: { action: 'maxStock', amount: 100 } },
  { label: t('dm.fillStock'), action: { action: 'fillStock' } },
  { label: t('dm.costOne'), action: { action: 'cheapBuild' }, toggle: true },
  { label: t('dm.resetOnPhase'), action: { action: 'timescaleAuto' }, toggle: true },
  { label: t('dm.buildAll'), action: { action: 'buildAll' }, leveled: true },
];

/** Körper einer Aktion: Monarch (`slot`), Umschalter (`on`) und Ausbau-Stufe (`level`, auf 1–5 begrenzt) ergänzt. */
export function actionBody(e: DmEntry, ctx: { slot: number; on?: boolean; level?: number }): DmAction {
  const body: DmAction = { ...e.action };
  if (e.needsSlot) body.slot = ctx.slot;
  if (e.toggle) body.on = ctx.on ?? true;
  if (e.leveled) body.level = Math.min(DM_MAX_LEVEL, Math.max(1, Math.round(ctx.level ?? 1)));
  return body;
}

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
