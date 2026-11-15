import { useEffect, useState } from 'react';
import { backend, type WorldDetail } from '../api';
import { stageId } from './crossModel';
import { levelModel, type LevelModel } from './levelView';

// Alle Stufen der Welt für den Querschnitt (B-963): je Insel und Stufe ein Level zum Seed über die bestehende API
// `worldLevel`. Ein Fehler lässt die Stufe aus (sie fehlt im Streifen), nie die ganze Ansicht.

type Entry = readonly [string, LevelModel];

/** Level-Modelle aller Stufen der Welt, Schlüssel `<insel>:<stufe>`; leer, solange geladen wird. */
export function useCross(detail: WorldDetail | null, seed: string): ReadonlyMap<string, LevelModel> {
  const [models, setModels] = useState<ReadonlyMap<string, LevelModel>>(new Map());
  useEffect(() => {
    let live = true;
    setModels(new Map());
    if (!detail) return;
    const jobs = detail.islands.flatMap((isl) => isl.depths.map((_, stage) =>
      backend.worldLevel(detail.id, isl.id, stage, seed).then((lv): Entry | null => (lv.error ? null : [stageId(isl.id, stage), levelModel(lv)]), () => null)));
    void Promise.all(jobs).then((all) => {
      if (live) setModels(new Map(all.filter((e): e is Entry => e !== null)));
    });
    return () => { live = false; };
  }, [detail, seed]);
  return models;
}
