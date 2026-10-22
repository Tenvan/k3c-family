import { useCallback, useState } from 'react';
import { DEFAULT_SCALE, clampScale, wheelAction, type View } from '../world/viewState';

/** Position und Zoom der Live-Vorschau im Gebäude-Planer. Die Vorschau bekommt beides von außen (B-477); der Planer hat keinen
 *  Chunk-Streifen und keine Weltbreite, daher gilt nur die linke Grenze 0. Ziehen und Mausrad wie im Welt-Betrachter
 *  (Strg + Rad zoomt, Umschalt + Rad scrollt). `wheelAction` wird mit dem Welt-Betrachter geteilt: Alt + Rad zoomt hier ebenfalls
 *  (es gibt nur eine Vorschau, also zoomt Alt dasselbe wie Strg; B-491/7). */
export function usePlannerView(): { position: View; onPan: (px: number) => void; onWheel: (e: WheelEvent) => void } {
  const [position, setPosition] = useState<View>({ left: 0, scale: DEFAULT_SCALE });
  const onPan = useCallback((px: number) => setPosition((v) => ({ ...v, left: Math.max(0, v.left + px / v.scale) })), []);
  const onWheel = useCallback((e: WheelEvent) => {
    const a = wheelAction(e);
    if (!a) return;
    e.preventDefault();
    if (a.kind === 'pan') onPan(a.px); else setPosition((v) => ({ ...v, scale: clampScale(v.scale * a.factor) }));
  }, [onPan]);
  return { position, onPan, onWheel };
}
