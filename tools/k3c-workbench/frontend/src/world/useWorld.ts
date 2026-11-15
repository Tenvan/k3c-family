import { useCallback, useEffect, useRef, useState } from 'react';
import { backend, type WorldDetail, type WorldInfo, type WorldLevel, type WorldValuesView } from '../api';
import { loadText, savePref } from '../lib/prefs';
import { stageId } from './crossModel';
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
  /** Level, wie das Backend es liefert (Natur-Werte, B-891) */
  level: WorldLevel | null;
  /** Natur-Wert der Stufe festsetzen; null setzt auf „aus Seed“ zurück (B-891) */
  setNature: (key: string, value: number | null) => Promise<void>;
  /** Zeile unter der Vorschau: Seed, Biom, Größe oder der Fehler */
  note: string;
  values: WorldValuesView | null;
  setValue: (file: string, path: string[], value: unknown) => Promise<void>;
  /** Neue Welt anlegen und wählen (B-954); die Antwort ist der Grund der Ablehnung, leer bei Erfolg */
  create: (id: string, name: string, extendsId: string) => Promise<string>;
  /** Etappe einer Insel setzen, null = Listenplatz + 1 (B-963); lädt danach die Welt neu */
  setEtappe: (island: string, etappe: number | null) => Promise<boolean>;
  /** Token `<insel>:<stufe>#<n>`: diese Stufe im Querschnitt neu laden (nach Natur-Änderung) */
  crossRefresh: string;
  /** Grund der letzten Ablehnung oder Warnungen nach dem Speichern */
  notice: { tone: 'error' | 'warn' | 'ok'; text: string } | null;
}

/** Erste Insel, erste Stufe der Welt, sonst null. */
const firstStage = (d: WorldDetail | null): StageChoice | null => (d?.islands[0] ? { world: d.id, island: d.islands[0].id, stage: 0 } : null);

function useLevel(stage: StageChoice | null, seed: string) {
  const [model, setModel] = useState<LevelModel | null>(null);
  const [level, setLevel] = useState<WorldLevel | null>(null);
  const [note, setNote] = useState('Lade …');
  const request = useRef(0);
  /** Antwort des Backends anzeigen (Laden und Festsetzen) */
  const show = useCallback((view: Partial<WorldLevel> & { error?: string }) => {
    if (view.error) {
      setModel(null);
      setLevel(null);
      setNote(view.error);
      return;
    }
    const next = levelModel(view as WorldLevel);
    setModel(next);
    setLevel(view as WorldLevel);
    setNote(`Seed „${next.seed}“ · ${next.biomeId} · ${next.widthUnits} Units · ${next.chunks.length} Abschnitte · ${next.objects.length} Objekte`);
  }, []);
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
    show(view);
  }, [stage, show]);
  const latestSeed = useRef(seed);
  latestSeed.current = seed;
  useEffect(() => { void load(latestSeed.current); }, [load]);
  return { model, level, note, load, show, request };
}

type SetNotice = (n: WorldState['notice']) => void;

const warnOr = (warnings: string[] | undefined, ok: string): WorldState['notice'] => (warnings?.length ? { tone: 'warn', text: warnings.join(' · ') } : { tone: 'ok', text: ok });

function useEtappe(id: string, current: { current: string }, setDetail: (d: WorldDetail) => void, setNotice: SetNotice) {
  const setEtappe = async (island: string, etappe: number | null): Promise<boolean> => {
    const world = id;
    const v = await backend.worldIslandEtappe(world, island, etappe).catch((e: unknown) => ({ id: world, warnings: [] as string[], error: e instanceof Error ? e.message : String(e) }));
    if (current.current !== world) return false; // inzwischen eine andere Welt gewählt
    if (v.error) {
      setNotice({ tone: 'error', text: v.error });
      return false;
    }
    const d = await backend.worldDetail(world).catch(() => null);
    if (d && current.current === world) setDetail(d);
    setNotice(warnOr(v.warnings, `Etappe von ${island} gespeichert`));
    return true;
  };
  return { setEtappe };
}

function useNature(stage: StageChoice | null, level: WorldLevel | null, seed: string, request: { current: number }, show: (v: Partial<WorldLevel> & { error?: string }) => void, setNotice: SetNotice) {
  const [crossRefresh, setCrossRefresh] = useState('');
  const setNature = async (key: string, value: number | null) => {
    if (!stage) return;
    const mine = ++request.current;
    const v = await backend.worldNatureSet(stage.world, stage.island, stage.stage, key, value, level?.seed ?? seed)
      .catch((e: unknown) => ({ error: e instanceof Error ? e.message : String(e) }) as Partial<WorldLevel>);
    if (mine !== request.current) return; // inzwischen anderes Level geladen
    if (v.rejected || v.error) {
      setNotice({ tone: 'error', text: v.rejected || v.error || 'keine Antwort' });
      if (v.rejected) show(v);
      return;
    }
    show(v);
    setCrossRefresh(`${stageId(stage.island, stage.stage)}#${mine}`);
    setNotice(warnOr(v.warnings, `${key} ${value === null ? 'aus Seed' : `= ${value}`} gespeichert`));
  };
  return { setNature, crossRefresh };
}

export function useWorld(): WorldState {
  const [worlds, setWorlds] = useState<WorldInfo[]>([]);
  const [id, setId] = useState(() => loadText('world.id', 'standard'));
  const [detail, setDetail] = useState<WorldDetail | null>(null);
  const [stage, setStage] = useState<StageChoice | null>(null);
  const [seed, setSeed] = useState(DEFAULT_SEED);
  const [values, setValues] = useState<WorldValuesView | null>(null);
  const [notice, setNotice] = useState<WorldState['notice']>(null);
  const { model, level, note, load, show, request } = useLevel(stage, seed);
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
  const create = async (newId: string, name: string, extendsId: string) => {
    const v = await backend.worldCreate(newId, name, extendsId).catch((e: unknown) => ({ id: newId, warnings: [] as string[], error: e instanceof Error ? e.message : String(e) }));
    if (v.error) return v.error;
    setWorlds(await backend.worldList().catch(() => worlds));
    choose(newId);
    setNotice(v.warnings.length ? { tone: 'warn', text: v.warnings.join(' · ') } : { tone: 'ok', text: `Welt ${newId} angelegt` });
    return '';
  };
  const { setEtappe } = useEtappe(id, current, setDetail, setNotice);
  const { setNature, crossRefresh } = useNature(stage, level, seed, request, show, setNotice);
  return { worlds, id, choose, detail, stage, setStage, seed, setSeed, reload: (s = seed) => void load(s), model, level, setNature, note, values, setValue, create, setEtappe, crossRefresh, notice };
}
