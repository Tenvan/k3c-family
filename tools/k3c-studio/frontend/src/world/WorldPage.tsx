import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { LivePreview } from '../stage/LivePreview';
import { ChunkPreview } from './ChunkPreview';
import { slotHints } from './chunkMarks';
import { GameLauncher } from './GameLauncher';
import { PlanEditor } from './PlanEditor';
import { usePlan } from './usePlan';
import type { LevelModel } from './levelView';
import { ViewBar } from './ViewBar';
import { BOTH, useBox, useViews } from './viewHooks';
import { centerRange, centerViews, jumpPoints, paneUnits, type View } from './viewState';

const KEY_STEP = 120; // Pixel je Pfeiltaste
/** Sichtbarer Bereich einer Vorschau für Schieber und Streifen */
const spanOf = (v: View, units: number) => ({ left: v.left, units });

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
  const strip = useBox(frame, '.wd-viewport');
  const stage = useBox(frame, '.wd-stage');
  const px = useMemo(() => ({ chunk: strip.w, live: stage.w }), [strip.w, stage.w]);
  const widthUnits = model?.widthUnits ?? 0;
  const { views, setViews, chunkView, liveView, panBy, panLive, zoom, zoomBy, setCenter, onWheel } = useViews(widthUnits, px);
  const spans = { chunk: spanOf(chunkView, paneUnits(views, 'chunk', px)), live: spanOf(liveView, paneUnits(views, 'live', px)) };
  const ownLive = views.live !== views.chunk;

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
    setViews((v) => centerViews(v, next.hubCenterUnits, next.widthUnits, px)); // ein neues Level beginnt beim Hub, der Zoom bleibt
  };
  const onSites = useCallback((sites: readonly { kind: string; x: number }[]) => {
    setGates(sites.filter((s) => s.kind === 'gate').map((s) => s.x));
  }, []);

  return (
    <div className="wd-page" data-maximized={maximized} ref={frame}>
      <ChunkPreview maximized={maximized} onToggle={() => setMaximized((m) => !m)} onLoaded={onLoaded} source={plan.source} view={chunkView} onWheel={(e) => onWheel(e, 'chunk')}
        hints={slotHints(plan.draft, plan.check)} liveSpan={ownLive ? spans.live : undefined}>
        <ViewBar views={views} spans={spans} widthUnits={widthUnits} range={centerRange(views, widthUnits, px)} jumps={jumpPoints(model, gates)}
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
