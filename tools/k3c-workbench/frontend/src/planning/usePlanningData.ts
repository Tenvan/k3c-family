import { useEffect, useRef, useState } from 'react';
import { backend, type PlanningChanged, type PlanningData } from '../api';
import { errorText } from '@webkit/lib/errors';
import { PLAN_RELOAD_MS, useLiveReload } from '../lib/useLiveReload';
import { applyItems, collect, FIRST, NOTHING, type Pending } from './planningState';
import { findSession } from './planning';

const DOCS = new Set(['plan', 'fragen', 'glossar']); // Dokumente aus docs/: kein Planungseintrag, die Seite lädt sie selbst

/** Fehler beim Laden von Session-Texten (ID → Meldung) und Wiederholen eines Textes. */
export interface TextLoad {
  errors: Record<string, string>;
  retry: (id: string) => void;
}

export interface PlanningState {
  data: PlanningData | null;
  error: string;
  /** Fließtexte der Sessions, die geöffnet oder nachgeladen wurden (ID → Markdown, leer ohne Datei). */
  texts: Record<string, string>;
  textLoad: TextLoad;
  /** Ganze Liste neu laden (nach einer Änderung von Hand). */
  load: () => Promise<void>;
}

/** Planung der Seite (B-909): Listen ohne Fließtexte; nach `planning:changed` nur die genannten IDs nachladen, ohne IDs
 *  oder bei Strukturänderung die Liste neu. Den Text der ausgewählten Session holt es beim Öffnen. */
export function usePlanningData(sel: string): PlanningState {
  const [data, setData] = useState<PlanningData | null>(null);
  const [error, setError] = useState('');
  const [texts, setTexts] = useState<Record<string, string>>({});
  const pending = useRef<Pending>(FIRST);
  const current = useRef<PlanningData | null>(null);
  const [textErrors, setTextErrors] = useState<Record<string, string>>({});
  const asked = useRef(new Set<string>());
  const seq = useRef(0); // Nummer des letzten Listen-Aufrufs: eine ältere Antwort überschreibt keine neuere
  const gen = useRef(0); // zählt jeden Stand, der die Liste ersetzt: eine Antwort auf veralteter Grundlage wird verworfen
  const keep = (d: PlanningData) => { gen.current++; current.current = d; setData(d); setError(''); };
  const load = () => {
    const n = ++seq.current;
    return backend.planningList().then((d) => { if (n === seq.current) keep(d); }, (e) => { if (n === seq.current) setError(errorText(e)); });
  };
  const full = () => { setTexts({}); setTextErrors({}); asked.current.clear(); return load(); };

  const run = async () => {
    const p = pending.current;
    pending.current = NOTHING;
    const ids = [...p.ids].filter((id) => !DOCS.has(id));
    if (p.all || !current.current) return full();
    if (ids.length === 0) return;
    try {
      const g = gen.current;
      const got = await backend.planningItems(ids);
      const base = current.current;
      if (g !== gen.current || !base) return void (await load()); // währenddessen kam eine neuere Liste: nicht darüber schreiben
      const done = applyItems(base, got);
      if (done.data !== base) keep(done.data);
      setTexts((t) => ({ ...t, ...done.texts }));
      if (done.reload) await load();
    } catch {
      await full(); // Nachladen gescheitert: einmal alles laden, scheitert auch das, bleibt der alte Stand mit Fehlerhinweis
    }
  };
  useLiveReload(['planning:changed'], () => void run(), PLAN_RELOAD_MS, (d: PlanningChanged | null) => { pending.current = collect(pending.current, d); });

  // Text der gewählten Session beim Öffnen holen (Entwürfe ohne Datei ergeben einen leeren Text).
  const sessionSel = data && findSession(data, sel) ? sel : '';
  useEffect(() => {
    if (!sessionSel || texts[sessionSel] !== undefined || asked.current.has(sessionSel)) return;
    asked.current.add(sessionSel);
    backend.planningItems([sessionSel]).then((items) => {
      const it = items[0];
      const text = it && !it.error ? it.text : '';
      setTexts((t) => (sessionSel in t ? t : { ...t, [sessionSel]: text })); // ein neuerer Text aus `planning:changed` gewinnt
    }, (e) => {
      setTextErrors((m) => ({ ...m, [sessionSel]: errorText(e) }));
    });
  }, [sessionSel, texts, textErrors]);

  const retry = (id: string) => {
    asked.current.delete(id);
    setTextErrors((m) => { const { [id]: _gone, ...rest } = m; return rest; }); // löst den Effekt neu aus
  };

  return { data, error, texts, textLoad: { errors: textErrors, retry }, load };
}
