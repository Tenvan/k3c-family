import { useCallback, useEffect, useRef, useState } from 'react';
import { backend, isStale, STALE, type AssetChoice, type AssetSaved, type AssetsView, type AssetTarget } from '../api';
import { errorText } from '@webkit/lib/errors';
import { applySaved } from './assets';

/** Slot-Register einer Seite: lädt frisch, lädt bei `assets:changed` neu (nur `view` wird ersetzt, Auswahl, Filter und
 *  Eingaben der Komponenten bleiben), speichert mit dem Stand des letzten Ladens (Schreibregel, B-459/AC-07). */
export function useAssets() {
  const [view, setView] = useState<AssetsView | null>(null);
  const [error, setError] = useState('');
  const stand = useRef('');
  const valid = useRef(false); // schon einmal ein gültiges Register geladen

  /** Lädt neu. Ist das Register gerade ungültig (z. B. halb geschrieben) und gab es schon gültige Daten, bleiben diese
   *  stehen und der Fehler erscheint als Meldung, damit offene Eingaben der Seite nicht verloren gehen. */
  const reload = useCallback(async () => {
    try {
      const next = await backend.assetsList();
      if (next.error && valid.current) {
        setError(next.error);
        return false;
      }
      valid.current = !next.error;
      stand.current = next.stand;
      setView(next);
      setError('');
      return true;
    } catch (e) {
      setError(errorText(e));
      return false;
    }
  }, []);

  useEffect(() => {
    void reload();
    return backend.on('assets:changed', () => void reload());
  }, [reload]);

  /** Schreibt einen Slot (Zuordnungen oder Zielmaß). Ablehnung aus Go steht in error; auf veraltetem Stand wird neu geladen und
   *  „Register geändert, bitte erneut speichern“ gemeldet, die Eingaben der Seite bleiben. */
  const write = useCallback(async (id: string, call: (stand: string) => Promise<AssetSaved>) => {
    try {
      const saved = await call(stand.current);
      stand.current = saved.stand;
      setView((v) => (v ? applySaved(v, id, saved) : v));
      setError('');
      return true;
    } catch (e) {
      if (!isStale(e)) {
        setError(errorText(e));
        return false;
      }
      if (await reload()) setError(STALE);
      return false;
    }
  }, [reload]);
  const save = useCallback((id: string, wahl: AssetChoice[]) => write(id, (s) => backend.assetsSet(id, wahl, s)), [write]);
  /** Zielmaß eines Grafik-Slots (null = heutiges Maß), gleiche Schreibregel wie `save` (B-685) */
  const saveTarget = useCallback((id: string, ziel: AssetTarget | null) => write(id, (s) => backend.assetsTargetSet(id, ziel, s)), [write]);

  return { view, error, reload, save, saveTarget };
}
