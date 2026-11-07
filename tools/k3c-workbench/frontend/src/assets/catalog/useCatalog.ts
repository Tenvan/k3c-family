import { useEffect, useState } from 'react';
import { errorText } from '@webkit/lib/errors';
import { assetsFrame, assetsGet, type Frame, type Spec } from '../api';
import { currentGeneration, frameCache, specCache, useCatalogStore } from './catalogStore';

/** Katalog aus dem Cache (B-732, catalogStore.ts): alle Specs, Prompt-Vorlagen und die IDs im aktuellen Diff (Review gegen
 *  HEAD, Entscheidung 005 › 6). Ein Fehler des Reviews lässt die Liste stehen, nur ohne Abzeichen. */
export const useCatalog = useCatalogStore;

/** Spec des gewählten Assets; `error` statt Abbruch (005 › 7: die übrigen Assets bleiben bedienbar). */
export function useSpec(id: string) {
  const [state, setState] = useState<{ id: string; spec: Spec | null; error: string }>(() => ({ id, spec: specCache.get(id) ?? null, error: '' }));
  useEffect(() => {
    const hit = specCache.get(id);
    if (hit) {
      setState({ id, spec: hit, error: '' });
      return;
    }
    let live = true;
    const gen = currentGeneration();
    assetsGet(id).then(
      (spec) => {
        if (gen === currentGeneration()) specCache.set(id, spec);
        if (live) setState({ id, spec, error: '' });
      },
      (e) => live && setState({ id, spec: null, error: errorText(e) }),
    );
    return () => {
      live = false;
    };
  }, [id]);
  return state.id === id ? state : { id, spec: null, error: '' };
}

/** Frames einer Vorschau: je Asset, Animation und Frame ein Promise (ein zweiter Durchlauf lädt nichts nach, WB2). */
export type FrameCache = Map<string, Promise<Frame>>;

/** Frame aus dem Cache, sonst vom Backend (Arbeitsstand, ohne Variante). Ein Fehler bleibt bis zum Asset-Wechsel im Cache,
 *  damit das Abspielen einen defekten Frame nicht jede Runde neu anfragt (Review-Befund 4); erfolgreiche Frames merkt
 *  sich `frameCache` über den Tab-Wechsel hinaus (B-732). */
export function cachedFrame(cache: FrameCache, id: string, animation: string, frame: number): Promise<Frame> {
  const key = `${id}|${animation}|${frame}`;
  let p = cache.get(key);
  if (!p) {
    const hit = frameCache.get(key);
    const gen = currentGeneration();
    p = hit
      ? Promise.resolve(hit)
      : assetsFrame(id, animation, frame).then((f) => {
          if (gen === currentGeneration()) frameCache.set(key, f);
          return f;
        });
    cache.set(key, p);
  }
  return p;
}

interface Shown {
  key: string;
  frame: Frame | null;
  /** Animation und Frame des gezeigten Bilds (kann beim Laden hinter der Wahl zurückliegen). */
  anim: string;
  nr: number;
  error: string;
}

/** Bild eines Frames; das alte Bild bleibt stehen, bis das neue da ist (`loading`), eine neuere Wahl verwirft ältere Antworten. */
export function useFrame(cache: FrameCache, id: string, animation: string, frame: number) {
  const key = `${id}|${animation}|${frame}`;
  // ein Frame aus dem Cache steht sofort da, ohne Lade-Anzeige (B-732)
  const [state, setState] = useState<Shown>(() => {
    const hit = frameCache.get(key);
    return { key: hit ? key : '', frame: hit ?? null, anim: animation, nr: frame, error: '' };
  });
  useEffect(() => {
    if (!animation) return;
    let live = true;
    cachedFrame(cache, id, animation, frame).then(
      (f) => live && setState({ key, frame: f, anim: animation, nr: frame, error: '' }),
      // das letzte Bild bleibt stehen, der Fehler liegt darüber
      (e) => live && setState((s) => ({ ...s, key, error: errorText(e) })),
    );
    return () => {
      live = false;
    };
  }, [cache, id, animation, frame, key]);
  return { ...state, loading: Boolean(animation) && state.key !== key };
}

/** Abspielen mit den fps der Animation (Standard 8). */
export function usePlayback(frames: number, fps: number, onFrame: (next: (f: number) => number) => void) {
  const [playing, setPlaying] = useState(false);
  useEffect(() => {
    if (!playing || frames < 2) return;
    const t = setInterval(() => onFrame((f) => (f + 1) % frames), 1000 / Math.max(1, fps));
    return () => clearInterval(t);
  }, [playing, frames, fps, onFrame]);
  return { playing, setPlaying };
}
