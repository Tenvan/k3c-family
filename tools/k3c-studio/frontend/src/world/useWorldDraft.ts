import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { backend, type PlanSaved, type WorldCheckView, type WorldDraft } from '../api';
import type { Notice } from './planSave';
import type { PlanState } from './usePlan';
import { draftOf, sameDraft } from './worldDraft';

const OK: WorldCheckView = { ok: true, error: '', island: -1, field: '', preview: '' };
const CHECK_DELAY = 200; // ms nach der letzten Änderung

async function saveOrError(draft: WorldDraft, stand: string): Promise<PlanSaved> {
  try {
    return await backend.planWorldSave(draft, stand);
  } catch (e) {
    return { stand: '', hint: '', stale: false, error: `Speichern fehlgeschlagen: ${e instanceof Error ? e.message : String(e)}` };
  }
}

/** Entwurf der gewählten Welt (WZ18.6, B-488/AC-05): Name, Notizen und Inseln. Jede Änderung wird verzögert geprüft
 *  (planWorldCheck, Fehler am Feld, Vorschau der Datei); gespeichert wird nur Gültiges auf dem Stand der geladenen Sicht.
 *  Bei Stale bleibt der Entwurf; ein neuer Stand ersetzt ihn nur, solange er unverändert ist. */
export function useWorldDraft(plan: PlanState) {
  const { view, reload, refreshWorlds } = plan;
  const saved = useMemo(() => (view ? draftOf(view) : null), [view]);
  const [own, setDraft] = useState<WorldDraft | null>(null); // null = der gespeicherte Stand
  const draft = own ?? saved;
  const [check, setCheck] = useState<WorldCheckView>(OK);
  const [notice, setNotice] = useState<Notice>(null);
  const prev = useRef(saved);
  const dirty = draft !== null && !sameDraft(draft, saved);

  useEffect(() => {
    setDraft((d) => (d === null || sameDraft(d, prev.current) ? null : d));
    prev.current = saved;
  }, [saved]);

  useEffect(() => {
    if (!draft || !dirty) { setCheck(OK); return; }
    let current = true;
    const timer = setTimeout(() => {
      void backend.planWorldCheck(draft).then((result) => { if (current) setCheck(result); });
    }, CHECK_DELAY);
    return () => { current = false; clearTimeout(timer); };
  }, [draft, dirty]);

  const save = useCallback(async () => {
    if (!draft || !view) return;
    const result = await saveOrError(draft, view.stand);
    if (result.error) { setNotice({ kind: result.stale ? 'stale' : 'error', text: result.error }); return; }
    // gespeichert: Herkunft der Inseln = ihre neue id, Planfelder aus dem Chat sind jetzt in der Datei (sonst bliebe der Entwurf offen und schriebe sie später zurück); Eingaben während des Speicherns bleiben stehen
    setDraft((d) => (sameDraft(d, draft) ? { ...draft, islands: draft.islands.map(({ plannedChunks: _p, chunkCounts: _c, ...i }) => ({ ...i, from: i.id })) } : d));
    await reload();
    await refreshWorlds();
    setNotice({ kind: 'hint', text: result.hint });
  }, [draft, view, reload, refreshWorlds]);

  return {
    draft, check, notice, dirty, save,
    /** Grund, warum Speichern gesperrt ist: ein offener Entwurf der Planung ginge beim Neuladen verloren */
    lock: plan.dirty ? 'Erst die Planung speichern oder verwerfen' : undefined,
    edit: (next: WorldDraft) => { setDraft(next); setNotice(null); },
    /** Entwurf aus dem Chat (WZ19.3): ersetzt den Entwurf, nichts wird gespeichert; ein Prüffehler steht als Meldung am Entwurf */
    load: (next: WorldDraft, error: string) => {
      setDraft(next);
      setNotice(error ? { kind: 'error', text: error } : { kind: 'hint', text: 'Antwort aus dem Chat als Entwurf geladen, nichts gespeichert' });
    },
    /** Meldung am Entwurf, ohne ihn zu ändern (z. B. „Antwort nicht lesbar“) */
    report: (text: string) => setNotice({ kind: 'error', text }),
    discard: () => { setDraft(null); setNotice(null); },
  };
}

export type WorldDraftState = ReturnType<typeof useWorldDraft>;
