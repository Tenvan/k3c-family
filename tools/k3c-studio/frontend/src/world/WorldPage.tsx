import { useCallback, useEffect, useLayoutEffect, useRef, useState } from 'react';
import { LivePreview } from '../stage/LivePreview';
import { ChunkPreview } from './ChunkPreview';
import { GameLauncher } from './GameLauncher';
import type { LevelModel } from './levelView';
import { ViewBar } from './ViewBar';
import { DEFAULT_SCALE, clampLeft, jumpPoints, leftFor, wheelAction, zoomView, type View } from './viewState';

const KEY_STEP = 120; // Pixel je Pfeiltaste
const FALLBACK_PX = 1000; // Breite der Ansicht, solange sie nicht gemessen ist

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

/** Seite Welt-Betrachter des Asset-Studios (B-460, B-477): Chunk-Streifen und Live-Vorschau in einer Ansicht. Position und
 *  Zoom gelten für beide gleich (eine Positions-Leiste, Sprungpunkte, Zoom stufenlos und in festen Stufen); „Verkleinern“
 *  und „Maximieren“ ändern die Höhe der beiden Vorschauen, nie ihre Sichtbarkeit. */
export function WorldPage() {
  const [maximized, setMaximized] = useState(true);
  const [source, setSource] = useState({ seed: 'k3c', biome: 'forest' });
  const [model, setModel] = useState<LevelModel | null>(null);
  const [gates, setGates] = useState<number[]>([]);
  const [view, setView] = useState<View>({ left: 0, scale: DEFAULT_SCALE });
  const frame = useRef<HTMLDivElement>(null);
  const viewPx = useWidth(frame);
  const widthUnits = model?.widthUnits ?? 0;
  const viewUnits = viewPx / view.scale;

  const panBy = useCallback((px: number) => setView((v) => ({ ...v, left: clampLeft(v.left + px / v.scale, widthUnits, viewPx / v.scale) })), [widthUnits, viewPx]);
  const setScale = (scale: number) => setView((v) => zoomView(v, scale, widthUnits, viewPx));
  const setLeft = (left: number) => setView((v) => ({ ...v, left: clampLeft(left, widthUnits, viewPx / v.scale) }));
  const jump = (x: number) => setView((v) => ({ ...v, left: leftFor(x, widthUnits, viewPx / v.scale) }));

  // Strg + Rad zoomt, Umschalt + Rad scrollt (das gewöhnliche Rad scrollt die Seite); gilt über Streifen und Bühne.
  const onWheel = (e: WheelEvent) => {
    const a = wheelAction(e);
    if (!a) return;
    e.preventDefault();
    if (a.kind === 'pan') panBy(a.px); else setView((v) => zoomView(v, v.scale * a.factor, widthUnits, viewPx));
  };

  // Pfeile scrollen, + und − zoomen, M verkleinert und maximiert (nicht, solange im Eingabefeld getippt wird).
  const keys = useRef({ panBy, setScale, scale: view.scale });
  keys.current = { panBy, setScale, scale: view.scale };
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLSelectElement || e.ctrlKey || e.metaKey || e.altKey) return;
      const k = keys.current;
      if (e.key === 'ArrowLeft') k.panBy(-KEY_STEP);
      else if (e.key === 'ArrowRight') k.panBy(KEY_STEP);
      else if (e.key === '+' || e.key === '=') k.setScale(k.scale * 1.25);
      else if (e.key === '-') k.setScale(k.scale * 0.8);
      else if (e.key === 'm' || e.key === 'M') setMaximized((m) => !m);
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, []);

  const onLoaded = (seed: string, biome: string, next: LevelModel) => {
    setSource((s) => (s.seed === seed && s.biome === biome ? s : { seed, biome }));
    setModel(next);
    setView((v) => ({ ...v, left: leftFor(next.hubCenterUnits, next.widthUnits, viewPx / v.scale) })); // ein neues Level beginnt beim Hub
  };
  const onSites = useCallback((sites: readonly { kind: string; x: number }[]) => {
    setGates(sites.filter((s) => s.kind === 'gate').map((s) => s.x));
  }, []);

  return (
    <div className="wd-page" data-maximized={maximized} ref={frame}>
      <ChunkPreview maximized={maximized} onToggle={() => setMaximized((m) => !m)} onLoaded={onLoaded} view={view} onWheel={onWheel}>
        <ViewBar view={view} widthUnits={widthUnits} viewUnits={viewUnits} jumps={jumpPoints(model, gates)} onScale={setScale} onLeft={setLeft} onJump={jump} />
      </ChunkPreview>
      <section className="wd-live" aria-label="Live-Vorschau">
        <LivePreview seed={source.seed} biome={source.biome} position={view} onPan={panBy} onWheel={onWheel} onSites={onSites} />
      </section>
      <GameLauncher seed={source.seed} biome={source.biome} />
    </div>
  );
}
