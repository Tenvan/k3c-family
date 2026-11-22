/** Cache des Größenvergleichs (B-733) auf Modul-Ebene, ohne Backend-Import: Katalog- und Review-Store leeren ihn bei
 *  „Neu laden“, ohne die API zu laden (reine Node-Tests importieren den Review-Store). */
const cache = new Map<string, Promise<unknown>>();

/** Promise aus dem Cache, sonst `load()`; ein Fehler bleibt nicht im Cache, der nächste Blick fragt neu. */
export function cached<T>(key: string, load: () => Promise<T>): Promise<T> {
  let p = cache.get(key) as Promise<T> | undefined;
  if (!p) {
    p = load();
    cache.set(key, p);
    p.catch(() => cache.get(key) === p && cache.delete(key));
  }
  return p;
}

export function clearSizeCache() {
  cache.clear();
}
