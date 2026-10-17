import { BIOMES } from '@game/model/biome';
import { Button, TextField } from '@radix-ui/themes';
import { useCallback, useEffect, useRef, useState, type CSSProperties } from 'react';
import { backend } from '../api';
import type { WorldSource } from '../api';
import type { PlanSource } from '../stage/stageLogic';
import { BIG_KINDS, DEFAULT_SCALE, SPECIAL_CHUNKS, colorOf, levelModel, seedFromBytes, zoomScale, type LevelModel } from './levelView';

const DEFAULT_SEED = 'k3c';
const DEFAULT_BIOME = 'forest';

/** Quelle der Anfrage: Insel + Stufe mit dem Entwurf (gleiche Quelle wie die Live-Vorschau), bis eine Insel geladen ist das Biom. */
const toSource = (plan?: PlanSource): WorldSource => (plan ? { island: plan.island, stage: plan.stage, planned: plan.planned } : { biome: DEFAULT_BIOME });
const SCROLL_STEP = 120; // Pixel je Pfeiltaste

const chunkClass = (kind: string) => `wd-chunk${kind === 'hub' ? ' wd-hub' : SPECIAL_CHUNKS.has(kind) ? ' wd-special' : ''}`;

/** Streifen mit Abschnitten, Objekten und Hub-Mitte im Maßstab scale (Pixel je Unit). */
export function Strip({ model, scale }: { model: LevelModel; scale: number }) {
  return (
    <div className="wd-strip" style={{ width: model.widthUnits * scale }}>
      {model.chunks.map((c) => (
        <div key={c.index} className={chunkClass(c.kind)} style={{ left: c.from * scale, width: (c.to - c.from) * scale }}
          title={`Abschnitt ${c.index}: ${c.kind} (${c.from}–${c.to} Units)`}>
          {c.index} {c.kind}
        </div>
      ))}
      {model.objects.map((o, i) => {
        const big = BIG_KINDS.has(o.kind);
        const size = big ? 22 : 10;
        const style: CSSProperties = { left: o.x * scale - size / 2, top: big ? '0.5rem' : '3.25rem', width: size, height: size, background: colorOf(o.kind) };
        return <div key={i} className="wd-marker" style={style} title={`${o.kind} bei ${o.x} Units`} />;
      })}
      <div className="wd-hubline" style={{ left: model.hubCenterUnits * scale }} title={`Hub-Mitte bei ${model.hubCenterUnits} Units`} />
    </div>
  );
}

/** Lädt das Level über das Backend; eine neuere Anfrage macht die ältere ungültig. note ist die Zeile unter der Leiste. */
function useWorldLevel(onLoaded: (seed: string, biome: string) => void) {
  const [model, setModel] = useState<LevelModel | null>(null);
  const [note, setNote] = useState('Lade …');
  const request = useRef(0);
  const loaded = useRef(onLoaded);
  loaded.current = onLoaded;

  const load = useCallback(async (seed: string, source: WorldSource) => {
    const mine = ++request.current;
    setNote('Lade …');
    let view;
    try {
      view = await backend.worldLevel(seed, source);
    } catch (e) {
      if (mine === request.current) { setModel(null); setNote(e instanceof Error ? e.message : String(e)); }
      return;
    }
    if (mine !== request.current) return; // eine neuere Anfrage läuft schon
    if (view.error) {
      setModel(null);
      setNote(view.error);
      return;
    }
    const next = levelModel(view);
    setModel(next);
    loaded.current(seed, next.biomeId);
    const name = BIOMES.find((b) => b.id === next.biomeId)?.name ?? next.biomeId;
    setNote(`Seed „${next.seed}“ · ${name} · ${next.widthUnits} Units · ${next.chunks.length} Abschnitte · ${next.objects.length} Objekte`);
  }, []);

  return { model, note, load };
}

function Legend({ model }: { model: LevelModel }) {
  return (
    <>
      <ul className="wd-legend">
        {model.counts.map(([kind, count]) => (
          <li key={kind} style={{ '--dot': colorOf(kind) } as CSSProperties}>{kind} {count}</li>
        ))}
      </ul>
      <ul className="wd-warnings">{model.warnings.map((w, i) => <li key={i}>{w}</li>)}</ul>
    </>
  );
}

/** Chunk-Vorschau des Welt-Betrachters (B-460/AC-01, AC-02): Seed und Biom wählen, das Level des Spiels als Streifen
 *  ansehen, mit Legende und Warnungen der Prüfung. Das Level rechnet das Studio selbst, ohne Spielserver. */
export function ChunkPreview({ maximized, onToggle, onLoaded, source }: { maximized: boolean; onToggle: () => void; onLoaded: (seed: string, biome: string) => void; source?: PlanSource }) {
  const [seed, setSeed] = useState(DEFAULT_SEED);
  const [scale, setScale] = useState(DEFAULT_SCALE);
  const { model, note, load } = useWorldLevel(onLoaded);
  const viewport = useRef<HTMLDivElement>(null);

  // Jede Änderung der Quelle (Insel, Stufe, Entwurf) lädt das Level neu, mit dem Seed, der zuletzt eingegeben wurde.
  const latest = useRef({ seed, source });
  latest.current = { seed, source };
  const sourceKey = JSON.stringify(source ?? null);
  useEffect(() => { void load(latest.current.seed, toSource(latest.current.source)); }, [sourceKey, load]);

  const zoom = (factor: number) => {
    const el = viewport.current;
    const next = zoomScale(scale, factor);
    if (next === scale) return;
    const center = el ? (el.scrollLeft + el.clientWidth / 2) / scale : 0;
    setScale(next);
    if (el) requestAnimationFrame(() => { el.scrollLeft = center * next - el.clientWidth / 2; });
  };

  // Tastatur wie auf der früheren Seite: Pfeile scrollen, + und − zoomen; dazu M zum Verkleinern und Maximieren
  // (nicht, solange im Eingabefeld getippt wird).
  const keys = useRef({ zoom, onToggle });
  keys.current = { zoom, onToggle };
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLSelectElement || e.ctrlKey || e.metaKey || e.altKey) return;
      const el = viewport.current;
      if (e.key === 'ArrowLeft' && el) el.scrollLeft -= SCROLL_STEP;
      else if (e.key === 'ArrowRight' && el) el.scrollLeft += SCROLL_STEP;
      else if (e.key === '+' || e.key === '=') keys.current.zoom(1.25);
      else if (e.key === '-') keys.current.zoom(0.8);
      else if (e.key === 'm' || e.key === 'M') keys.current.onToggle();
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, []);

  const random = () => {
    const next = seedFromBytes(crypto.getRandomValues(new Uint8Array(8)));
    setSeed(next);
    void load(next, toSource(source));
  };

  return (
    <div className={`wd-chunks${maximized ? ' wd-max' : ''}`} data-maximized={maximized}>
      <div className="wd-bar">
        <TextField.Root className="wd-seed" aria-label="Seed" value={seed} onChange={(e) => setSeed(e.target.value)}
          onKeyDown={(e) => { if (e.key === 'Enter') void load(seed, toSource(source)); }} />
        <Button onClick={() => void load(seed, toSource(source))}>Laden</Button>
        <Button variant="soft" onClick={random}>Zufälliger Seed</Button>
        <Button variant="soft" aria-label="Herauszoomen" onClick={() => zoom(0.8)}>−</Button>
        <Button variant="soft" aria-label="Hineinzoomen" onClick={() => zoom(1.25)}>+</Button>
        <span className="wd-grow" />
        <Button variant="outline" onClick={onToggle}>{maximized ? 'Verkleinern' : 'Maximieren'}</Button>
      </div>
      <p className="wd-note" role="status">{note}</p>
      <div className="wd-viewport" ref={viewport}>{model && <Strip model={model} scale={scale} />}</div>
      {model && <Legend model={model} />}
    </div>
  );
}
