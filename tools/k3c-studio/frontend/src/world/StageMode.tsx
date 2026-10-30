import { Button } from '@webkit/atoms';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { LivePreview } from '../stage/LivePreview';
import { ChunkInspector } from './ChunkInspector';
import { ChunkPreview } from './ChunkPreview';
import { slotOfIndex } from './bandLayout';
import { chunkAtSlot, slotHints } from './chunkMarks';
import { Inspector, type Tab } from './Inspector';
import { LiveChunkBand } from './LiveChunkBand';
import { usePlan } from './usePlan';
import type { LevelModel, ModelChunk } from './levelView';
import { PositionBar, ViewBar } from './ViewBar';
import { BOTH, useBox, useViews } from './viewHooks';
import { centerRange, centerViews, jumpPoints, paneUnits, type View } from './viewState';

const KEY_STEP = 120; // Pixel je Pfeiltaste
/** Sichtbarer Bereich einer Vorschau für Schieber und Streifen */
const spanOf = (v: View, units: number) => ({ left: v.left, units });

/** Bedienelemente, die ihre Tasten selbst brauchen: Eingabefelder, Auswahllisten (auch offen), Regler */
const OWN_KEYS = 'input, textarea, [role="combobox"], [role="listbox"], [role="slider"], [role="checkbox"]';

/** Pfeile scrollen, + und − zoomen, M füllt die Fläche mit der Bühne (nicht, solange im Eingabefeld getippt, eine Auswahl
 *  bedient oder ein Regler geschoben wird). */
function useKeys(panBy: (px: number) => void, zoomBy: (factor: number) => void, toggle: () => void) {
  const keys = useRef({ panBy, zoomBy, toggle });
  keys.current = { panBy, zoomBy, toggle };
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.target instanceof Element && e.target.closest(OWN_KEYS)) || e.ctrlKey || e.metaKey || e.altKey) return;
      const k = keys.current;
      if (e.key === 'ArrowLeft') k.panBy(-KEY_STEP);
      else if (e.key === 'ArrowRight') k.panBy(KEY_STEP);
      else if (e.key === '+' || e.key === '=') k.zoomBy(1.25);
      else if (e.key === '-') k.zoomBy(0.8);
      else if (e.key === 'm' || e.key === 'M') k.toggle();
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, []);
}

/** Modus Stufe bearbeiten (B-460, B-461, B-477, B-665): links der Navigator (Chunk-Vorschau), die Werkzeugleiste und die
 *  Live-Vorschau mit dem Band der Abschnitte, rechts der Inspektor. Beide Vorschauen zeigen dieselbe Weltposition in der
 *  Bildmitte (eine Positions-Leiste, Sprungpunkte); der Zoom gilt je Vorschau (Alt + Rad), Zoomleiste und Strg + Rad zoomen
 *  beide (B-485/AC-05). Beide lesen dieselbe Quelle: Insel, Stufe und der Entwurf der Planung (B-461/AC-11). Ein Klick auf
 *  einen Chunk wählt seinen Chunk-Platz im Reiter Chunk des Inspektors. */
export function StageMode({ plan, source, onSource, maximized, onToggleMax }: {
  plan: ReturnType<typeof usePlan>; source: { seed: string; biome: string }; onSource: (s: { seed: string; biome: string }) => void;
  maximized: boolean; onToggleMax: () => void;
}) {
  const [model, setModel] = useState<LevelModel | null>(null);
  const [walls, setWalls] = useState<number[]>([]);
  const [tab, setTab] = useState<Tab>('chunk');
  const [slot, setSlot] = useState<number | null>(null);
  const [band, setBand] = useState(true);
  const frame = useRef<HTMLDivElement>(null);
  const [controlsHost, setControlsHost] = useState<HTMLElement | null>(null); // Hub-Stufe, Tageszeit: in der Werkzeugleiste
  const [treeHost, setTreeHost] = useState<HTMLElement | null>(null); // Gebäude: Reiter Gebäude
  const [layersHost, setLayersHost] = useState<HTMLElement | null>(null); // Hintergrund-Ebenen: Reiter Ansicht
  const strip = useBox(frame, '.wd-viewport');
  const stage = useBox(frame, '.wd-stage', true);
  const px = useMemo(() => ({ chunk: strip.w, live: stage.w }), [strip.w, stage.w]);
  const widthUnits = model?.widthUnits ?? 0;
  const { views, setViews, chunkView, liveView, panBy, panLive, zoom, zoomBy, setCenter, onWheel } = useViews(widthUnits, px);
  const spans = { chunk: spanOf(chunkView, paneUnits(views, 'chunk', px)), live: spanOf(liveView, paneUnits(views, 'live', px)) };
  const ownLive = views.live !== views.chunk;
  const hints = slotHints(plan.draft, plan.check);
  const jumpTo = (c: ModelChunk) => setCenter((c.from + c.to) / 2); // Bildmitte = Chunk-Mitte, Zoom bleibt
  /** Chunk gewählt (Navigator oder Band): Platz im Inspektor, Reiter Chunk, Sprung dorthin */
  const pick = (c: ModelChunk) => { if (model) setSlot(slotOfIndex(model.chunks, c.index)); setTab('chunk'); jumpTo(c); };
  /** Chunk-Platz gewählt (Chip oder Änderung im Inspektor): die Vorschauen zentrieren den Chunk */
  const pickSlot = (s: number) => { setSlot(s); const c = model && chunkAtSlot(model.chunks, s); if (c) jumpTo(c); };
  const chunk = slot === null || !model ? undefined : model.chunks.find((c) => slotOfIndex(model.chunks, c.index) === slot);

  useKeys(panBy, zoomBy, onToggleMax);

  // Ein neues Level (Seed, Biom, Insel, Stufe, Inselgröße) beginnt beim Hub, der Zoom bleibt; lädt nur der Entwurf der
  // Planung neu, bleibt die Mitte (sonst spränge die Ansicht nach jeder Änderung eines Chunks zurück, WZ17.5).
  const levelKey = useRef('');
  const onLoaded = (seed: string, biome: string, next: LevelModel) => {
    if (seed !== source.seed || biome !== source.biome) onSource({ seed, biome });
    setModel(next);
    const key = [seed, biome, plan.source?.island, plan.source?.stage, plan.source?.chunkCount].join('|');
    if (key !== levelKey.current) { setViews((v) => centerViews(v, next.hubCenterUnits, next.widthUnits, px)); setSlot(null); } // der gewählte Platz gehört zu seiner Stufe
    levelKey.current = key;
  };
  const onSites = useCallback((sites: readonly { kind: string; x: number }[]) => {
    setWalls(sites.filter((s) => s.kind === 'wall').map((s) => s.x));
  }, []);

  return (
    <div className="wd-stage-mode" ref={frame}>
      <div className="wd-stage-main">
        <ChunkPreview onLoaded={onLoaded} source={plan.source} view={chunkView} onWheel={(e) => onWheel(e, 'chunk')} hints={hints} liveSpan={ownLive ? spans.live : undefined} onChunk={pick}>
          <PositionBar views={views} spans={spans} widthUnits={widthUnits} range={centerRange(views, widthUnits, px)} onCenter={setCenter} />
        </ChunkPreview>
        <div className="wd-bar wd-toolbar" role="toolbar" aria-label="Ansicht der Bühne">
          <div ref={setControlsHost} className="wd-controls-host" />
          <span className="wd-grow" />
          <ViewBar views={views} jumps={jumpPoints(model, walls)} onScale={(scale) => zoom(BOTH, () => scale)} onZoom={zoomBy} onCenter={setCenter} />
          <Button variant="soft" aria-label="Bühne auf ganze Fläche" aria-pressed={maximized} title="Bühne auf ganze Fläche (M)" onClick={onToggleMax}>⛶</Button>
        </div>
        <section className="wd-live" aria-label="Live-Vorschau">
          <LivePreview seed={source.seed} biome={source.biome} position={liveView} onPan={panLive} onWheel={(e) => onWheel(e, 'live')} onSites={onSites} plan={plan.source}
            controlsHost={controlsHost} treeHost={treeHost} layersHost={layersHost} onPickSite={() => setTab('gebaeude')} />
          {band && <LiveChunkBand stage={stage} view={liveView} model={model} draft={plan.draft} hints={hints} onChunk={pick} />}
        </section>
      </div>
      <Inspector tab={tab} onTab={setTab} setTreeHost={setTreeHost} setLayersHost={setLayersHost} band={band} onBand={setBand}
        chunk={<ChunkInspector plan={plan} seed={source.seed} slot={slot} kind={chunk?.kind} onSlot={pickSlot} />} />
    </div>
  );
}
