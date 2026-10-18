import { useCallback, useEffect, useMemo, useState } from 'react';
import { backend } from '../api';
import type { PlanCheckView, PlanIsland, PlanSaved, PlanView, PlannedChunk } from '../api';
import type { PlanSource } from '../stage/stageLogic';
import { isDirty, normalized, savedPlan } from './plan';

const OK: PlanCheckView = { ok: true, slot: 0, error: '' };
const CHECK_DELAY = 200; // ms nach der letzten Änderung

export type Notice = { kind: 'hint' | 'error' | 'stale'; text: string } | null;

/** Speichern; lehnt der Aufruf ab (Wails mit Text), wird die Ablehnung zum Fehler, sonst bliebe sie ohne Meldung (B-486). */
async function saveOrError(...args: Parameters<typeof backend.planSave>): Promise<PlanSaved> {
  try {
    return await backend.planSave(...args);
  } catch (e) {
    return { stand: '', hint: '', stale: false, error: `Speichern fehlgeschlagen: ${e instanceof Error ? e.message : String(e)}` };
  }
}

/** Zustand des Editors für geplante Chunks (B-461, WZ8.5): geladene Inseln, Auswahl von Insel und Stufe, der Entwurf der
 *  gewählten Stufe, die Prüfung des Entwurfs (verzögert) und Speichern. Der Entwurf bleibt im Speicher, bis gespeichert,
 *  verworfen oder neu geladen wird; jede Änderung ruft die Prüfung auf, die Vorschauen zeigen den letzten gültigen Stand. */
export function usePlan() {
  const [view, setView] = useState<PlanView | null>(null);
  const [islandId, setIslandId] = useState('');
  const [stage, setStageState] = useState(0);
  const [draft, setDraft] = useState<PlannedChunk[]>([]);
  const [check, setCheck] = useState<PlanCheckView>(OK);
  const [shown, setShown] = useState<PlannedChunk[] | null>(null);
  const [notice, setNotice] = useState<Notice>(null);

  const island: PlanIsland | undefined = view?.islands.find((i) => i.id === islandId) ?? view?.islands[0];
  const depth = island?.depths[stage];
  const saved = useMemo(() => savedPlan(island, stage), [island, stage]);

  const load = useCallback(async () => {
    const next = await backend.planLoad();
    if (next.error) { setNotice({ kind: 'error', text: next.error }); return; }
    setView(next);
    setNotice(null);
  }, []);
  useEffect(() => { void load(); }, [load]);

  // Auswahl oder gespeicherte Planung wechseln (Laden, Speichern): der Entwurf beginnt bei der gespeicherten Planung.
  const stand = view?.stand ?? '';
  useEffect(() => { setDraft(saved); setShown(saved); setCheck(OK); }, [island?.id, stage, stand, saved]);

  useEffect(() => {
    if (!island || depth === undefined) return;
    let current = true;
    const chunks = normalized(draft);
    const timer = setTimeout(() => {
      void backend.planCheck(island.id, depth, chunks).then((result) => {
        if (!current) return;
        setCheck(result);
        if (result.ok) setShown(chunks);
      });
    }, CHECK_DELAY);
    return () => { current = false; clearTimeout(timer); };
  }, [draft, island, depth]);

  const dirty = isDirty(draft, saved);
  const edit = (next: PlannedChunk[]) => { setDraft(next); setNotice(null); };

  const save = useCallback(async () => {
    if (!island || depth === undefined || !view) return;
    const chunks = normalized(draft);
    const result = await saveOrError(island.id, depth, chunks, view.stand);
    if (result.error) { setNotice({ kind: result.stale ? 'stale' : 'error', text: result.error }); return; }
    setView({
      ...view, stand: result.stand,
      islands: view.islands.map((i) => (i.id === island.id ? { ...i, planned: { ...i.planned, [String(depth)]: chunks } } : i)),
    });
    setNotice({ kind: 'hint', text: result.hint });
  }, [draft, island, depth, view]);

  const discard = () => { setDraft(saved); setNotice(null); };
  const source = useMemo<PlanSource | undefined>(() => (island ? { island: island.id, stage, planned: shown ?? saved } : undefined), [island, stage, shown, saved]);
  return {
    view, island, stage, draft, check, notice, dirty, source,
    selectIsland: (id: string) => { setIslandId(id); setStageState(0); },
    selectStage: setStageState,
    edit, save, discard, reload: load,
  };
}

export type PlanState = ReturnType<typeof usePlan>;
