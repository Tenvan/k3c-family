import { useCallback, useLayoutEffect, useMemo, useState, type RefObject } from 'react';
import { DEFAULT_SCALE, centerViews, panViews, paneView, wheelAction, zoomPanes, type Pane, type PanePx, type Views } from './viewState';

// Zustand der Ansicht im Welt-Betrachter (B-477, B-485/AC-05): gemessene Größen der beiden Vorschauen und die Ansicht selbst.

const FALLBACK_PX = 1000; // Breite einer Vorschau, solange sie nicht gemessen ist
export const BOTH: readonly Pane[] = ['chunk', 'live'];

/** Gemessenes Element: Breite und Höhe in Pixeln (Inhalt ohne Rahmen) */
export interface Box {
  el: HTMLElement | null;
  w: number;
  h: number;
}

/** Größe des ersten Elements unter `root`, das zu `selector` passt; folgt ihm bei jeder Größenänderung. Die Ansicht rechnet
 *  mit der echten Breite jeder Vorschau (Streifen und Bühne sind verschieden breit), sonst passen ihre Bildmitten nicht. */
export function useBox(root: RefObject<HTMLElement | null>, selector: string): Box {
  const [box, setBox] = useState<Box>({ el: null, w: FALLBACK_PX, h: 0 });
  useLayoutEffect(() => {
    const el = root.current?.querySelector<HTMLElement>(selector) ?? null;
    if (!el) return;
    const read = () => setBox((b) => {
      const w = el.clientWidth > 0 ? el.clientWidth : b.w;
      return b.el === el && b.w === w && b.h === el.clientHeight ? b : { el, w, h: el.clientHeight };
    });
    read();
    if (typeof ResizeObserver === 'undefined') return;
    const watch = new ResizeObserver(read);
    watch.observe(el);
    return () => watch.disconnect();
  }, [root, selector]);
  return box;
}

/** Ansicht beider Vorschauen (B-485/AC-05): Zoom je Vorschau, gemeinsame Mitte, `px` = sichtbare Breite je Vorschau. Der
 *  Zustand lebt nur in diesem Fenster und bleibt beim Wechsel von Insel und Stufe (ein neues Level setzt nur die Mitte). */
export function useViews(widthUnits: number, px: PanePx) {
  const [views, setViews] = useState<Views>({ center: 0, chunk: DEFAULT_SCALE, live: DEFAULT_SCALE });
  const chunkView = useMemo(() => paneView(views, 'chunk', widthUnits, px), [views, widthUnits, px]);
  const liveView = useMemo(() => paneView(views, 'live', widthUnits, px), [views, widthUnits, px]);

  // Bildlauf in Pixeln der Vorschau `pane`: beide wandern um dieselbe Welt-Strecke.
  const panBy = useCallback((by: number, pane: Pane = 'chunk') => setViews((v) => panViews(v, by / v[pane], widthUnits, px)), [widthUnits, px]);
  const panLive = useCallback((by: number) => panBy(by, 'live'), [panBy]);
  const zoom = (panes: readonly Pane[], to: (scale: number) => number) => setViews((v) => zoomPanes(v, panes, to, widthUnits, px));
  const zoomBy = (factor: number) => zoom(BOTH, (s) => s * factor);
  const setCenter = (x: number) => setViews((v) => centerViews(v, x, widthUnits, px));

  // Strg + Rad zoomt beide, Alt + Rad nur die Vorschau unter dem Zeiger, Umschalt + Rad scrollt (das gewöhnliche Rad scrollt die Seite).
  const onWheel = (e: WheelEvent, pane: Pane) => {
    const a = wheelAction(e);
    if (!a) return;
    e.preventDefault();
    if (a.kind === 'pan') panBy(a.px, pane); else zoom(a.own ? [pane] : BOTH, (s) => s * a.factor);
  };
  return { views, setViews, chunkView, liveView, panBy, panLive, zoom, zoomBy, setCenter, onWheel };
}
