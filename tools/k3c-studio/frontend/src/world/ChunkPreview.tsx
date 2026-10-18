import { BIOMES } from '@game/model/biome';
import { Button, Select, TextField } from '@radix-ui/themes';
import { useCallback, useEffect, useRef, useState, type CSSProperties, type ReactNode } from 'react';
import { backend } from '../api';
import type { WorldSource } from '../api';
import { BIG_KINDS, SPECIAL_CHUNKS, colorOf, levelModel, seedFromBytes, type LevelModel } from './levelView';
import type { View } from './viewState';

const DEFAULT_SEED = 'k3c';
const DEFAULT_BIOME = 'forest';
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
function useWorldLevel(onLoaded: (seed: string, biome: string, model: LevelModel) => void) {
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
    loaded.current(seed, source.biome ?? '', next);
    const name = BIOMES.find((b) => b.id === next.biomeId)?.name ?? next.biomeId;
    setNote(`Seed „${next.seed}“ · ${name} · ${next.widthUnits} Units · ${next.chunks.length} Abschnitte · ${next.objects.length} Objekte`);
  }, []);

  useEffect(() => { void load(DEFAULT_SEED, { biome: DEFAULT_BIOME }); }, [load]);
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
 *  ansehen, mit Legende und Warnungen der Prüfung. Das Level rechnet das Studio selbst, ohne Spielserver. Position und
 *  Zoom kommen von außen (`view`, B-477): der Streifen hat keine eigene Scrollbar, das Mausrad geht an `onWheel`.
 *  `children` stehen unter der Zeile mit der Zusammenfassung (Leiste für Zoom, Sprünge und Position). */
export function ChunkPreview({ maximized, onToggle, onLoaded, view, onWheel, children }: {
  maximized: boolean; onToggle: () => void; onLoaded: (seed: string, biome: string, model: LevelModel) => void;
  view: View; onWheel: (e: WheelEvent) => void; children?: ReactNode;
}) {
  const [seed, setSeed] = useState(DEFAULT_SEED);
  const [biome, setBiome] = useState(DEFAULT_BIOME);
  const { model, note, load } = useWorldLevel(onLoaded);
  const viewport = useRef<HTMLDivElement>(null);

  useEffect(() => { if (viewport.current) viewport.current.scrollLeft = view.left * view.scale; }, [view, model]);

  // Ein React-Handler für das Rad wäre passiv (kein preventDefault), daher am Element; die Seite entscheidet und bricht ab.
  const wheel = useRef(onWheel);
  wheel.current = onWheel;
  useEffect(() => {
    const el = viewport.current;
    if (!el) return;
    const handler = (e: WheelEvent) => wheel.current(e);
    el.addEventListener('wheel', handler, { passive: false });
    return () => el.removeEventListener('wheel', handler);
  }, []);

  const random = () => {
    const next = seedFromBytes(crypto.getRandomValues(new Uint8Array(8)));
    setSeed(next);
    void load(next, { biome });
  };

  return (
    <div className="wd-chunks" data-maximized={maximized}>
      <div className="wd-bar">
        <TextField.Root className="wd-seed" aria-label="Seed" value={seed} onChange={(e) => setSeed(e.target.value)}
          onKeyDown={(e) => { if (e.key === 'Enter') void load(seed, { biome }); }} />
        <Select.Root value={biome} onValueChange={(v) => { setBiome(v); void load(seed, { biome: v }); }}>
          <Select.Trigger aria-label="Biom" />
          <Select.Content>{BIOMES.map((b) => <Select.Item key={b.id} value={b.id}>{b.name}</Select.Item>)}</Select.Content>
        </Select.Root>
        <Button onClick={() => void load(seed, { biome })}>Laden</Button>
        <Button variant="soft" onClick={random}>Zufälliger Seed</Button>
        <span className="wd-grow" />
        <Button variant="outline" onClick={onToggle}>{maximized ? 'Verkleinern' : 'Maximieren'}</Button>
      </div>
      <p className="wd-note" role="status">{note}</p>
      {children}
      <div className="wd-viewport" ref={viewport}>{model && <Strip model={model} scale={view.scale} />}</div>
      {model && <Legend model={model} />}
    </div>
  );
}
