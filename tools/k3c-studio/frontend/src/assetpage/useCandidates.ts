import { errorText } from '@webkit/lib/errors';
import { useCallback, useEffect, useRef, useState } from 'react';
import { backend, type AssetCandidatesView } from '../api';
import { researchNote } from './candidateLogic';

/** Zahl der Katalog-Treffer je Abfrage */
const LIMIT = 60;

export interface Candidates {
  view: AssetCandidatesView | null;
  loading: boolean;
  error: string;
  /** Hinweis der letzten Recherche („n neue Treffer · …“) */
  note: string;
  searching: boolean;
  reload: () => Promise<void>;
  /** Recherche im Netz zum Suchbegriff, danach Liste neu laden */
  research: (query: string) => Promise<void>;
}

/** Kandidaten eines Slots (RPC AssetCandidates): lädt bei Slot- und Query-Wechsel und bei `assets:changed`. Eine späte
 *  Antwort eines früheren Aufrufs überschreibt die aktuelle nicht. Der Aufrufer setzt pro Slot einen `key`, damit Liste
 *  und Hinweis nicht in den nächsten Slot wandern. */
export function useCandidates(slotId: string, query: string): Candidates {
  const [view, setView] = useState<AssetCandidatesView | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [note, setNote] = useState('');
  const [searching, setSearching] = useState(false);
  const seq = useRef(0);
  const live = useRef(true);
  useEffect(() => { live.current = true; return () => { live.current = false; }; }, []);

  const reload = useCallback(async () => {
    const mine = ++seq.current;
    try {
      const next = await backend.assetCandidates(slotId, query, LIMIT);
      if (live.current && mine === seq.current) { setView(next); setError(''); }
    } catch (e) {
      if (live.current && mine === seq.current) setError(errorText(e));
    } finally {
      if (live.current && mine === seq.current) setLoading(false);
    }
  }, [slotId, query]);

  useEffect(() => {
    setLoading(true);
    void reload();
    const off = backend.on('assets:changed', () => void reload());
    return off;
  }, [reload]);

  const research = useCallback(async (q: string) => {
    setSearching(true);
    try {
      const res = await backend.assetsResearch(slotId, q);
      if (live.current) { setNote(researchNote(res.neu, res.hinweise)); setError(''); }
      await reload();
    } catch (e) {
      if (live.current) setError(errorText(e));
    } finally {
      if (live.current) setSearching(false);
    }
  }, [slotId, reload]);

  return { view, loading, error, note, searching, reload, research };
}
