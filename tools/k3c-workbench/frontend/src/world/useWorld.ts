import { useCallback, useEffect, useRef, useState } from 'react';
import { backend, type WorldDetail, type WorldInfo, type WorldLevel, type WorldValuesView } from '../api';
import { loadText, savePref } from '../lib/prefs';
import { levelModel, type LevelModel } from './levelView';

// Daten des Welten-Editors (B-879): Welten-Liste, gewählte Welt (gemerkt `world.id`), Insel und Stufe, Level zum Seed
// und die Werte der Welt. Eine neuere Anfrage macht eine ältere ungültig (kein Flackern beim schnellen Wechsel).

export const DEFAULT_SEED = 'k3c';

/** Gewählte Insel-Stufe: Welt, Insel-ID und Index in ihren Tiefen (die Welt gehört dazu, damit ein Weltwechsel nie mit
 *  der Insel der alten Welt rechnet). */
export interface StageChoice {
  world: string;
  island: string;
  stage: number;
}

export interface WorldState {
  worlds: WorldInfo[];
  id: string;
  choose: (id: string) => void;
  detail: WorldDetail | null;
  stage: StageChoice | null;
  setStage: (s: StageChoice) => void;
  seed: string;
  setSeed: (seed: string) => void;
  /** Level zum Seed neu rechnen (ohne Seed: der aktuelle) */
  reload: (seed?: string) => void;
  model: LevelModel | null;
  /** Zeile unter der Vorschau: Seed, Biom, Größe oder der Fehler */
  note: string;
  values: WorldValuesView | null;
  setValue: (file: string, path: string[], value: unknown) => Promise<void>;
  /** Grund der letzten Ablehnung oder Warnungen nach dem Speichern */
  notice: { tone: 'error' | 'warn' | 'ok'; text: string } | null;
}

/** Erste Insel, erste Stufe der Welt, sonst null. */
const firstStage = (d: WorldDetail | null): StageChoice | null => (d?.islands[0] ? { world: d.id, island: d.islands[0].id, stage: 0 } : null);

function useLevel(stage: StageChoice | null, seed: string) {
  const [model, setModel] = useState<LevelModel | null>(null);
  const [note, setNote] = useState('Lade …');
  const request = useRef(0);
  const load = useCallback(async (s: string) => {
    const mine = ++request.current;
    if (!stage) {
      setModel(null);
      setNote('Keine Insel gewählt.');
      return;
    }
    setNote('Lade …');
    const view = await backend.worldLevel(stage.world, stage.island, stage.stage, s).catch((e: unknown) => ({ error: e instanceof Error ? e.message : String(e) }));
    if (mine !== request.current) return;
    if (view.error) {
      setModel(null);
      setNote(view.error);
      return;
    }
    const next = levelModel(view as WorldLevel);
    setModel(next);
    setNote(`Seed „${next.seed}“ · ${next.biomeId} · ${next.widthUnits} Units · ${next.chunks.length} Abschnitte · ${next.objects.length} Objekte`);
  }, [stage]);
  const latestSeed = useRef(seed);
  latestSeed.current = seed;
  useEffect(() => { void load(latestSeed.current); }, [load]);
  return { model, note, load };
}

export function useWorld(): WorldState {
  const [worlds, setWorlds] = useState<WorldInfo[]>([]);
  const [id, setId] = useState(() => loadText('world.id', 'standard'));
  const [detail, setDetail] = useState<WorldDetail | null>(null);
  const [stage, setStage] = useState<StageChoice | null>(null);
  const [seed, setSeed] = useState(DEFAULT_SEED);
  const [values, setValues] = useState<WorldValuesView | null>(null);
  const [notice, setNotice] = useState<WorldState['notice']>(null);
  const { model, note, load } = useLevel(stage, seed);
  const current = useRef(id);
  current.current = id;

  useEffect(() => { void backend.worldList().then(setWorlds, () => setWorlds([])); }, []);
  useEffect(() => {
    let live = true;
    setNotice(null);
    void Promise.all([backend.worldDetail(id), backend.worldValues(id)]).then(([d, v]) => {
      if (!live) return;
      setDetail(d);
      setStage(firstStage(d));
      setValues(v);
    }, (e: unknown) => {
      if (live) setNotice({ tone: 'error', text: e instanceof Error ? e.message : String(e) });
    });
    return () => { live = false; };
  }, [id]);

  const choose = (next: string) => {
    if (next === id) return;
    savePref('world.id', next);
    setDetail(null);
    setStage(null);
    setValues(null);
    setId(next);
  };
  const setValue = async (file: string, path: string[], value: unknown) => {
    const world = id;
    const v = await backend.worldOverrideSet(world, file, path, value).catch((e: unknown) => ({ error: e instanceof Error ? e.message : String(e) }));
    if (current.current !== world) return; // inzwischen eine andere Welt gewählt
    if (v.error || !('entries' in v)) {
      setNotice({ tone: 'error', text: v.error || 'keine Antwort' });
      return;
    }
    setValues(v);
    setNotice(v.warnings?.length ? { tone: 'warn', text: v.warnings.join(' · ') } : { tone: 'ok', text: `${file}.${path.join('.')} gespeichert` });
  };
  return { worlds, id, choose, detail, stage, setStage, seed, setSeed, reload: (s = seed) => void load(s), model, note, values, setValue, notice };
}
