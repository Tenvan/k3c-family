/** `delta` aus Protokoll v2 auf einen Zustand anwenden (docs/protocol.md › Zustand und Delta). */

type State = Record<string, unknown>;
type Entry = { id: number };

/** Listen, deren Einträge eine `id` haben: im Delta als `{ set, del }`. */
const ID_LISTS = new Set(['players', 'coins', 'troops', 'nodes', 'sites', 'enemies', 'projectiles', 'pickups']);

function mergeList(old: Entry[] | undefined, change: { set?: Entry[]; del?: number[] }): Entry[] {
  const removed = new Set(change.del ?? []);
  const updated = new Map((change.set ?? []).map((e) => [e.id, e]));
  const kept = (old ?? []).filter((e) => !removed.has(e.id)).map((e) => updated.get(e.id) ?? e);
  const known = new Set(kept.map((e) => e.id));
  return [...kept, ...(change.set ?? []).filter((e) => !known.has(e.id) && !removed.has(e.id))];
}

/**
 * Liefert den neuen vollen Zustand, `state` bleibt unverändert. Ein fehlendes Feld ist unverändert, `null` ist ein Wert,
 * `events` fehlt im Delta, wenn es im Tick keine gab (dann leer).
 */
export function applyDelta(state: State, delta: State): State {
  const next: State = { ...state, events: [] };
  for (const [key, value] of Object.entries(delta)) {
    next[key] = ID_LISTS.has(key) ? mergeList(state[key] as Entry[] | undefined, value as { set?: Entry[]; del?: number[] }) : value;
  }
  return next;
}
