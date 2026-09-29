import { isSaveData, type SaveData } from '../world/sim/save';

/**
 * Spielstand ablegen: Heimnetz-Server (GET/PUT api/save, siehe server/saves.mjs) plus localStorage als Fallback.
 * Der Browser-Speicher auf der Xbox gilt als unzuverlässig, deshalb zählt der Server, wenn er einen Stand hat.
 * Liegen beide vor, gewinnt der neuere (savedAt).
 */

const URL_ = 'api/save';
const LOCAL_KEY = 'k3c:save';

export async function loadSave(): Promise<SaveData | null> {
  const [remote, local] = await Promise.all([loadRemote(), Promise.resolve(loadLocal())]);
  if (remote && local) return local.savedAt > remote.savedAt ? local : remote;
  return remote ?? local;
}

/** Schreibt lokal (sofort) und auf den Server. `keepalive`, damit das auch beim Verlassen der Seite noch rausgeht. */
export function storeSave(save: SaveData): void {
  const json = JSON.stringify(save);
  try {
    localStorage.setItem(LOCAL_KEY, json);
  } catch {
    // Speicher voll oder gesperrt: dann eben nur der Server.
  }
  fetch(URL_, { method: 'PUT', body: json, headers: { 'Content-Type': 'application/json' }, keepalive: json.length < 60_000 }).catch(
    () => undefined, // kein Server (z.B. GitHub Pages): localStorage reicht
  );
}

async function loadRemote(): Promise<SaveData | null> {
  try {
    const res = await fetch(URL_, { cache: 'no-store' });
    if (res.status !== 200) return null;
    const data: unknown = await res.json();
    return isSaveData(data) ? data : null;
  } catch {
    return null;
  }
}

function loadLocal(): SaveData | null {
  try {
    const data: unknown = JSON.parse(localStorage.getItem(LOCAL_KEY) ?? 'null');
    return isSaveData(data) ? data : null;
  } catch {
    return null;
  }
}
