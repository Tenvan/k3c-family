import { useEffect, useMemo, useRef, useState } from 'react';
import { backend, type SourceProposal } from '../api';
import type { SourcesState } from '../sources/useSources';
import { running, summarize } from '../sources/sourcesLogic';
import { pickedProposals, proposalIds } from './pageLogic';

export type DownloadMode = 'laden' | 'zuordnen';

/** Auswahl, Bestätigung und Download der Netz-Dateien (Sicht Dateien): „Nur laden“ legt sie in den Vorrat, „Laden und zuordnen“
 *  übernimmt jede fertig geladene Datei danach als letzte Präferenz ihres Slots (Übernehmen ordnet zu). */
export function useDownloads(sources: SourcesState, proposals: readonly SourceProposal[], onAdopted: () => void) {
  const [picked, setPicked] = useState<ReadonlySet<string>>(new Set());
  const [confirm, setConfirm] = useState<DownloadMode | null>(null);
  const [error, setError] = useState('');
  const pending = useRef<string[]>([]);
  const busy = running([...sources.jobs.values()]);
  const selection = useMemo(() => summarize(pickedProposals(proposals, picked), new Set(proposalIds(picked))), [proposals, picked]);
  const toggle = (keys: string[], on: boolean) => setPicked((cur) => { const n = new Set(cur); keys.forEach((k) => (on ? n.add(k) : n.delete(k))); return n; });

  const start = async () => {
    const mode = confirm;
    const ids = proposalIds(picked);
    setConfirm(null);
    setPicked(new Set());
    pending.current = mode === 'zuordnen' ? ids : [];
    await sources.download(ids);
  };

  // Übernimmt fertige Downloads, sobald ihr Fortschritt „fertig“ meldet; ein Fehler gibt den Eintrag frei, die Liste zeigt ihn.
  // `adopting` verhindert, dass ein weiterer Lauf des Effekts denselben Eintrag noch einmal übernimmt (zwei Kopien im Slot).
  const { jobs, reload } = sources;
  const adopting = useRef(false);
  useEffect(() => {
    if (adopting.current || pending.current.length === 0) return;
    const finished = pending.current.filter((id) => jobs.get(id)?.status === 'fertig');
    const failed = pending.current.filter((id) => jobs.get(id)?.status === 'fehler');
    if (finished.length === 0 && failed.length === 0) return;
    adopting.current = true;
    pending.current = pending.current.filter((id) => !finished.includes(id) && !failed.includes(id));
    void (async () => {
      for (const id of finished) {
        try {
          await backend.sourceAdopt(id);
        } catch (e) {
          if (/nicht geladen/.test((e as Error).message)) pending.current.push(id);
          else setError((e as Error).message);
        }
      }
      await reload();
      adopting.current = false;
      onAdopted();
    })();
  }, [jobs, reload, onAdopted]);

  return { picked, toggle, clear: () => setPicked(new Set()), selection, confirm, ask: setConfirm, start, busy, error };
}
