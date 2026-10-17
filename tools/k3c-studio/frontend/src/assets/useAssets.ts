import { useCallback, useEffect, useRef, useState } from 'react';
import { backend, isStale, STALE, type AssetChoice, type AssetsView } from '../api';
import { errorText } from '../lib/errors';
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

  /** Speichert die Zuordnungen eines Slots. Ablehnung aus Go steht in error; auf veraltetem Stand wird neu geladen und
   *  „Register geändert, bitte erneut speichern“ gemeldet, die Eingaben der Seite bleiben. */
  const save = useCallback(async (id: string, wahl: AssetChoice[]) => {
    try {
      const saved = await backend.assetsSet(id, wahl, stand.current);
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

  return { view, error, reload, save };
}
