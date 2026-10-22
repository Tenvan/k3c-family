import { useCallback, useEffect, useMemo, useState } from 'react';
import { backend } from '../api';
import type { PlanCheckView, PlanIsland, PlanView, PlannedChunk } from '../api';
import type { PlanSource } from '../stage/stageLogic';
import { isDirty, normalized, savedCount, savedPlan } from './plan';
import { useWorlds } from './useWorlds';
import { savePlan, type Notice } from './planSave';

export type { Notice } from './planSave';

const OK: PlanCheckView = { ok: true, slot: 0, error: '' };
const CHECK_DELAY = 200; // ms nach der letzten Änderung

const reason = (e: unknown) => (e instanceof Error ? e.message : String(e));

/** Zustand des Editors für geplante Chunks (B-461, WZ8.5): geladene Inseln, Auswahl von Insel und Stufe, der Entwurf der
 *  gewählten Stufe (Planung und Chunk-Anzahl, 0 = Biom-Länge, B-485/AC-04), die Prüfung des Entwurfs (verzögert) und
 *  Speichern. Der Entwurf bleibt im Speicher, bis gespeichert, verworfen oder neu geladen wird; jede Änderung ruft die
 *  Prüfung auf, die Vorschauen zeigen den letzten gültigen Stand (`source`). */
export function usePlan() {
  const [view, setView] = useState<PlanView | null>(null);
  const [islandId, setIslandId] = useState('');
  const [stage, setStageState] = useState(0);
  const [draft, setDraft] = useState<PlannedChunk[]>([]);
  const [count, setCount] = useState(0);
  const [check, setCheck] = useState<PlanCheckView>(OK);
  const [shown, setShown] = useState<PlanSource | null>(null); // letzter gültiger Entwurf; null = gespeicherter Stand
  const [notice, setNotice] = useState<Notice>(null);

  const island: PlanIsland | undefined = view?.islands.find((i) => i.id === islandId) ?? view?.islands[0];
  const depth = island?.depths[stage];
  const saved = useMemo(() => savedPlan(island, stage), [island, stage]);
  const savedSize = savedCount(island, stage);

  const load = useCallback(async () => {
    try {
      const next = await backend.planLoad();
      if (next.error) { setNotice({ kind: 'error', text: next.error }); return; }
      setView(next);
      setNotice(null);
    } catch (e) {
      setNotice({ kind: 'error', text: `Laden fehlgeschlagen: ${reason(e)}` }); // B-487: nie stumm
    }
  }, []);
  useEffect(() => { void load(); }, [load]);

  // Auswahl oder gespeicherter Stand wechseln (Laden, Speichern): der Entwurf beginnt beim gespeicherten Stand.
  const stand = view?.stand ?? '';
  useEffect(() => { setDraft(saved); setCount(savedSize); setShown(null); setCheck(OK); }, [island?.id, stage, stand, saved, savedSize]);

  useEffect(() => {
    if (!island || depth === undefined) return;
    let current = true;
    const chunks = normalized(draft);
    const timer = setTimeout(() => {
      // abgelehnte Prüfung: Fehler am Entwurf, Speichern gesperrt, das letzte gültige Bild bleibt (B-487)
      const failed = (e: unknown): PlanCheckView => ({ ok: false, slot: 0, error: `Prüfung fehlgeschlagen: ${reason(e)}` });
      void backend.planCheck(island.id, depth, chunks, count).catch(failed).then((result) => {
        if (!current) return;
        setCheck(result);
        if (result.ok) setShown({ island: island.id, stage, planned: chunks, chunkCount: count });
      });
    }, CHECK_DELAY);
    return () => { current = false; clearTimeout(timer); };
  }, [draft, count, island, depth, stage]);

  const dirty = isDirty(draft, saved) || count !== savedSize;
  const edit = (next: PlannedChunk[]) => { setDraft(next); setNotice(null); };
  const setSize = (next: number) => { setCount(next); setNotice(null); };

  const save = useCallback(async () => {
    if (!island || depth === undefined || !view) return;
    const result = await savePlan(view, island.id, depth, normalized(draft), count);
    if (result.view) setView(result.view);
    setNotice(result.notice);
  }, [draft, count, island, depth, view]);

  const worlds = useWorlds(async () => { await load(); setIslandId(''); setStageState(0); }, setNotice);
  const discard = () => { setDraft(saved); setCount(savedSize); setNotice(null); };
  const fallback = useMemo<PlanSource | undefined>(() => (island ? { island: island.id, stage, planned: saved, chunkCount: savedSize } : undefined), [island, stage, saved, savedSize]);
  const source = island ? (shown ?? fallback) : undefined;
  return {
    view, island, stage, draft, count, sizes: view?.sizes ?? [], check, notice, dirty, source,
    selectIsland: (id: string) => { setIslandId(id); setStageState(0); },
    selectStage: setStageState,
    edit, setSize, save, discard, reload: load, ...worlds,
  };
}

export type PlanState = ReturnType<typeof usePlan>;

