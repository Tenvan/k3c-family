import { useEffect, useRef, useState } from 'react';
import { backend, type WorldDetail } from '../api';
import { stageId } from './crossModel';
import { levelModel, type LevelModel } from './levelView';

// Alle Stufen der Welt für den Querschnitt (B-963): je Insel und Stufe ein Level zum Seed über die bestehende API
// `worldLevel`. Ein Fehler lässt die Stufe aus (sie fehlt im Streifen), nie die ganze Ansicht. Geladen wird nur, was im
// Cache fehlt (Schlüssel: Welt, Seed, Inseln und Tiefen; die Etappe gehört nicht dazu), höchstens LIMIT Aufrufe gleichzeitig;
// bisherige Werte bleiben stehen, bis ein neuer da ist.

const LIMIT = 3;

interface Job {
  island: string;
  stage: number;
}

interface Cache {
  key: string;
  world: string;
  seed: string;
  map: Map<string, LevelModel>;
  inflight: Set<string>;
}

/** Level-Modelle aller Stufen der Welt, Schlüssel `<insel>:<stufe>`. refresh = `<insel>:<stufe>#<n>` verwirft diese Stufe und lädt sie neu. */
export function useCross(detail: WorldDetail | null, seed: string, refresh = ''): ReadonlyMap<string, LevelModel> {
  const [models, setModels] = useState<ReadonlyMap<string, LevelModel>>(new Map());
  const cache = useRef<Cache>({ key: '', world: '', seed: '', map: new Map(), inflight: new Set() });
  const lastRefresh = useRef(refresh);
  const key = detail ? JSON.stringify([detail.id, seed, detail.islands.map((i) => [i.id, i.depths])]) : '';
  useEffect(() => {
    const c = cache.current;
    if (!detail) {
      cache.current = { key: '', world: '', seed: '', map: new Map(), inflight: new Set() };
      setModels(new Map());
      return;
    }
    if (c.world !== detail.id || c.seed !== seed) {
      c.map = new Map();
      c.inflight = new Set();
    }
    c.key = key;
    c.world = detail.id;
    c.seed = seed;
    const jobs: Job[] = detail.islands.flatMap((isl) => isl.depths.map((_, stage) => ({ island: isl.id, stage })));
    const wanted = new Set(jobs.map((j) => stageId(j.island, j.stage)));
    for (const k of [...c.map.keys()]) if (!wanted.has(k)) c.map.delete(k);
    if (refresh !== lastRefresh.current) {
      lastRefresh.current = refresh;
      const id = refresh.split('#')[0] ?? '';
      c.map.delete(id);
      c.inflight.delete(id);
    }
    setModels(new Map(c.map));
    const queue = jobs.filter((j) => !c.map.has(stageId(j.island, j.stage)) && !c.inflight.has(stageId(j.island, j.stage)));
    const world = detail.id;
    const load = async (j: Job) => {
      const id = stageId(j.island, j.stage);
      c.inflight.add(id);
      const model = await backend.worldLevel(world, j.island, j.stage, seed).then((lv) => (lv.error ? null : levelModel(lv)), () => null);
      if (c.key !== key || !c.inflight.delete(id)) return; // inzwischen andere Welt, anderer Seed oder verworfen
      if (model) c.map.set(id, model);
      setModels(new Map(c.map));
    };
    const worker = async () => {
      for (let j = queue.shift(); j; j = queue.shift()) await load(j);
    };
    for (let i = 0; i < LIMIT; i++) void worker();
  }, [key, refresh]); // detail ist über key abgedeckt
  return models;
}
