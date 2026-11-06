import { useEffect, useState } from 'react';
import { errorText } from '@webkit/lib/errors';
import { assetsFrame, assetsGet, assetsList, assetsReview, type AssetEintrag, type Frame, type Spec } from '../api';

/** Katalog: alle Specs und die IDs im aktuellen Diff (Review gegen HEAD, Entscheidung 005 › 6). Ein Fehler des Reviews
 *  lässt die Liste stehen, nur ohne Abzeichen. */
export function useCatalog() {
  const [list, setList] = useState<AssetEintrag[]>([]);
  const [inDiff, setInDiff] = useState<Set<string>>(new Set());
  const [error, setError] = useState('');
  useEffect(() => {
    let live = true;
    assetsList().then((l) => live && setList(l), (e) => live && setError(errorText(e)));
    assetsReview('HEAD').then((r) => live && setInDiff(new Set(r.assets.map((a) => a.id))), () => undefined);
    return () => {
      live = false;
    };
  }, []);
  return { list, inDiff, error };
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

/** Bild eines Frames (Arbeitsstand, ohne Variante); neue Wahl verwirft eine ältere Antwort. */
export function useFrame(id: string, animation: string, frame: number) {
  const [state, setState] = useState<{ key: string; frame: Frame | null; error: string }>({ key: '', frame: null, error: '' });
  const key = `${id}|${animation}|${frame}`;
  useEffect(() => {
    if (!animation) return;
    let live = true;
    assetsFrame(id, animation, frame).then(
      (f) => live && setState({ key, frame: f, error: '' }),
      (e) => live && setState({ key, frame: null, error: errorText(e) }),
    );
    return () => {
      live = false;
    };
  }, [id, animation, frame, key]);
  return state.key === key ? state : { key, frame: null, error: '' };
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
