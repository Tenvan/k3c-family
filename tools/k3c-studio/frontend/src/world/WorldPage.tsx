import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState } from 'react';
import { LivePreview } from '../stage/LivePreview';
import { ChunkPreview } from './ChunkPreview';
import { GameLauncher } from './GameLauncher';
import { PlanEditor } from './PlanEditor';
import { usePlan } from './usePlan';
import type { LevelModel } from './levelView';
import { ViewBar } from './ViewBar';
import { DEFAULT_SCALE, centerRange, centerViews, jumpPoints, panViews, paneView, wheelAction, zoomPanes, type Pane, type Views } from './viewState';

const KEY_STEP = 120; // Pixel je Pfeiltaste
const FALLBACK_PX = 1000; // Breite der Ansicht, solange sie nicht gemessen ist
const BOTH: readonly Pane[] = ['chunk', 'live'];

/** Breite der Ansicht in Pixeln, folgt dem Element. */
function useWidth(el: React.RefObject<HTMLElement | null>): number {
  const [px, setPx] = useState(FALLBACK_PX);
  useLayoutEffect(() => { if (el.current && el.current.clientWidth > 0) setPx(el.current.clientWidth); }, [el]); // vor dem ersten Laden des Levels
  useEffect(() => {
    const target = el.current;
    if (!target || typeof ResizeObserver === 'undefined') return;
    const watch = new ResizeObserver(() => { if (target.clientWidth > 0) setPx(target.clientWidth); });
    watch.observe(target);
    return () => watch.disconnect();
  }, [el]);
  return px;
}

/** Ansicht beider Vorschauen (B-485/AC-05): Zoom je Vorschau, gemeinsame Mitte. Der Zustand lebt nur in diesem Fenster und
 *  bleibt beim Wechsel von Insel und Stufe (ein neues Level setzt nur die Mitte). */
function useViews(widthUnits: number, viewPx: number) {
  const [views, setViews] = useState<Views>({ center: 0, chunk: DEFAULT_SCALE, live: DEFAULT_SCALE });
  const chunkView = useMemo(() => paneView(views, 'chunk', widthUnits, viewPx), [views, widthUnits, viewPx]);
  const liveView = useMemo(() => paneView(views, 'live', widthUnits, viewPx), [views, widthUnits, viewPx]);

  // Bildlauf in Pixeln der Vorschau `pane`: beide wandern um dieselbe Welt-Strecke.
  const panBy = useCallback((px: number, pane: Pane = 'chunk') => setViews((v) => panViews(v, px / v[pane], widthUnits, viewPx)), [widthUnits, viewPx]);
  const panLive = useCallback((px: number) => panBy(px, 'live'), [panBy]);
  const zoom = (panes: readonly Pane[], to: (scale: number) => number) => setViews((v) => zoomPanes(v, panes, to, widthUnits, viewPx));
  const zoomBy = (factor: number) => zoom(BOTH, (s) => s * factor);
  const setCenter = (x: number) => setViews((v) => centerViews(v, x, widthUnits, viewPx));

  // Strg + Rad zoomt beide, Alt + Rad nur die Vorschau unter dem Zeiger, Umschalt + Rad scrollt (das gewöhnliche Rad scrollt die Seite).
  const onWheel = (e: WheelEvent, pane: Pane) => {
    const a = wheelAction(e);
    if (!a) return;
    e.preventDefault();
    if (a.kind === 'pan') panBy(a.px, pane); else zoom(a.own ? [pane] : BOTH, (s) => s * a.factor);
  };
  return { views, setViews, chunkView, liveView, panBy, panLive, zoom, zoomBy, setCenter, onWheel };
}

/** Seite Welt-Betrachter des Asset-Studios (B-460, B-461, B-477): Chunk-Streifen und Live-Vorschau in einer Ansicht, dazwischen
 *  der Editor für geplante Chunks. Beide zeigen dieselbe Weltposition in der Bildmitte (eine Positions-Leiste, Sprungpunkte);
 *  der Zoom gilt je Vorschau (Alt + Rad), die Leiste und Strg + Rad zoomen beide (B-485/AC-05). Beide lesen dieselbe
 *  Quelle: Insel, Stufe und der Entwurf der Planung (B-461/AC-11).
 *  „Verkleinern“ und „Maximieren“ ändern die Höhe der beiden Vorschauen, nie ihre Sichtbarkeit. */
export function WorldPage() {
  const [maximized, setMaximized] = useState(true);
  const [source, setSource] = useState({ seed: 'k3c', biome: 'forest' });
  const plan = usePlan();
  const [model, setModel] = useState<LevelModel | null>(null);
  const [gates, setGates] = useState<number[]>([]);
  const frame = useRef<HTMLDivElement>(null);
  const viewPx = useWidth(frame);
  const widthUnits = model?.widthUnits ?? 0;
  const { views, setViews, chunkView, liveView, panBy, panLive, zoom, zoomBy, setCenter, onWheel } = useViews(widthUnits, viewPx);

  // Pfeile scrollen, + und − zoomen, M verkleinert und maximiert (nicht, solange im Eingabefeld getippt wird).
  const keys = useRef({ panBy, zoomBy });
  keys.current = { panBy, zoomBy };
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLSelectElement || e.ctrlKey || e.metaKey || e.altKey) return;
      const k = keys.current;
      if (e.key === 'ArrowLeft') k.panBy(-KEY_STEP);
      else if (e.key === 'ArrowRight') k.panBy(KEY_STEP);
      else if (e.key === '+' || e.key === '=') k.zoomBy(1.25);
      else if (e.key === '-') k.zoomBy(0.8);
      else if (e.key === 'm' || e.key === 'M') setMaximized((m) => !m);
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, []);

  const onLoaded = (seed: string, biome: string, next: LevelModel) => {
    setSource((s) => (s.seed === seed && s.biome === biome ? s : { seed, biome }));
    setModel(next);
    setViews((v) => centerViews(v, next.hubCenterUnits, next.widthUnits, viewPx)); // ein neues Level beginnt beim Hub, der Zoom bleibt
  };
  const onSites = useCallback((sites: readonly { kind: string; x: number }[]) => {
    setGates(sites.filter((s) => s.kind === 'gate').map((s) => s.x));
  }, []);

  return (
    <div className="wd-page" data-maximized={maximized} ref={frame}>
      <ChunkPreview maximized={maximized} onToggle={() => setMaximized((m) => !m)} onLoaded={onLoaded} source={plan.source} view={chunkView} onWheel={(e) => onWheel(e, 'chunk')}>
        <ViewBar views={views} panes={{ chunk: chunkView, live: liveView }} viewPx={viewPx} widthUnits={widthUnits}
          range={centerRange([views.chunk, views.live], widthUnits, viewPx)} jumps={jumpPoints(model, gates)}
          onScale={(scale) => zoom(BOTH, () => scale)} onZoom={zoomBy} onCenter={setCenter} />
      </ChunkPreview>
      <PlanEditor plan={plan} />
      <section className="wd-live" aria-label="Live-Vorschau">
        <LivePreview seed={source.seed} biome={source.biome} position={liveView} onPan={panLive} onWheel={(e) => onWheel(e, 'live')} onSites={onSites} plan={plan.source} />
      </section>
      <GameLauncher seed={source.seed} biome={source.biome} />
    </div>
  );
}
