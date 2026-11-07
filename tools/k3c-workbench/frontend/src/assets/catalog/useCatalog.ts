import { useEffect, useState } from 'react';
import { errorText } from '@webkit/lib/errors';
import { assetsFrame, assetsGet, assetsList, assetsPrompts, assetsReview, type AssetEintrag, type Frame, type Spec, type Vorlage } from '../api';

/** Katalog: alle Specs, Prompt-Vorlagen und die IDs im aktuellen Diff (Review gegen HEAD, Entscheidung 005 › 6). Ein Fehler des Reviews
 *  lässt die Liste stehen, nur ohne Abzeichen. */
export function useCatalog() {
  const [list, setList] = useState<AssetEintrag[]>([]);
  const [inDiff, setInDiff] = useState<Set<string>>(new Set());
  const [prompts, setPrompts] = useState<Vorlage[]>([]);
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    let live = true;
    assetsList()
      .then((l) => live && setList(l), (e) => live && setError(errorText(e)))
      .finally(() => live && setLoading(false));
    assetsPrompts().then((p) => live && setPrompts(p), (e) => live && setError(errorText(e)));
    assetsReview('HEAD').then((r) => live && setInDiff(new Set(r.assets.map((a) => a.id))), () => undefined);
    return () => {
      live = false;
    };
  }, []);
  return { list, inDiff, prompts, error, loading };
}

/** Spec des gewählten Assets; `error` statt Abbruch (005 › 7: die übrigen Assets bleiben bedienbar). */
export function useSpec(id: string) {
  const [state, setState] = useState<{ id: string; spec: Spec | null; error: string }>({ id: '', spec: null, error: '' });
  useEffect(() => {
    let live = true;
    assetsGet(id).then(
      (spec) => live && setState({ id, spec, error: '' }),
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

/** Frame aus dem Cache, sonst vom Backend (Arbeitsstand, ohne Variante); ein Fehler bleibt nicht im Cache. */
export function cachedFrame(cache: FrameCache, id: string, animation: string, frame: number): Promise<Frame> {
  const key = `${id}|${animation}|${frame}`;
  let p = cache.get(key);
  if (!p) {
    p = assetsFrame(id, animation, frame);
    cache.set(key, p);
    p.catch(() => cache.delete(key));
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
  const [state, setState] = useState<Shown>({ key: '', frame: null, anim: '', nr: 0, error: '' });
  const key = `${id}|${animation}|${frame}`;
  useEffect(() => {
    if (!animation) return;
    let live = true;
    cachedFrame(cache, id, animation, frame).then(
      (f) => live && setState({ key, frame: f, anim: animation, nr: frame, error: '' }),
      (e) => live && setState({ key, frame: null, anim: animation, nr: frame, error: errorText(e) }),
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
