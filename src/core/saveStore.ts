import { isSaveGame, type SaveGame } from '../model/types';

/**
 * Spielstände: primär auf dem Heimnetz-Server (/api/save, Browser-Speicher auf der Xbox gilt als unzuverlässig),
 * zusätzlich immer in localStorage als Rückfall. Beim Laden gewinnt der neuere Stand.
 */

export const DEFAULT_SLOT = 'autosave';
const localKey = (slot: string) => `k3c-save-${slot}`;
const TIMEOUT_MS = 3000;

export type SaveTarget = 'server' | 'local' | 'none';

function writeLocal(slot: string, save: SaveGame): boolean {
  try {
    localStorage.setItem(localKey(slot), JSON.stringify(save));
    return true;
  } catch {
    return false;
  }
}

function readLocal(slot: string): SaveGame | null {
  try {
    const data: unknown = JSON.parse(localStorage.getItem(localKey(slot)) ?? 'null');
    return isSaveGame(data) ? data : null;
  } catch {
    return null;
  }
}

/** `keepalive`: auch beim Verlassen der Seite (pagehide) noch zustellen. */
export async function storeSave(save: SaveGame, slot = DEFAULT_SLOT, keepalive = false): Promise<SaveTarget> {
  const local = writeLocal(slot, save);
  try {
    const res = await fetch(`api/save?slot=${encodeURIComponent(slot)}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(save),
      keepalive,
      signal: keepalive ? undefined : AbortSignal.timeout(TIMEOUT_MS),
    });
    if (res.ok) return 'server';
  } catch {
    // Server nicht erreichbar (z.B. GitHub Pages): localStorage reicht dann
  }
  return local ? 'local' : 'none';
}

export async function fetchSave(slot = DEFAULT_SLOT): Promise<SaveGame | null> {
  let remote: SaveGame | null = null;
  try {
    const res = await fetch(`api/save?slot=${encodeURIComponent(slot)}`, { signal: AbortSignal.timeout(TIMEOUT_MS), cache: 'no-store' });
    const data: unknown = res.ok ? await res.json() : null;
    remote = isSaveGame(data) ? data : null;
  } catch {
    remote = null;
  }
  const local = readLocal(slot);
  if (!remote || !local) return remote ?? local;
  return local.savedAt > remote.savedAt ? local : remote;
}
