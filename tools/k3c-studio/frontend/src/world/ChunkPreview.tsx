import { BIOMES } from '@game/model/biome';
import { Button, Text, TextField } from '@webkit/atoms';
import { useCallback, useEffect, useRef, useState, type CSSProperties, type ReactNode } from 'react';
import { backend } from '../api';
import type { WorldSource } from '../api';
import type { PlanSource } from '../stage/stageLogic';
import { BIG_KINDS, SPECIAL_CHUNKS, colorOf, levelModel, seedFromBytes, type LevelModel, type ModelChunk } from './levelView';
import { chunkMarks, type SlotHint } from './chunkMarks';
import { iconOf, nameOf } from './kindIcons';
import type { JumpPoint, View } from './viewState';

const DEFAULT_SEED = 'k3c';
const DEFAULT_BIOME = 'forest';
const EDGE_JUMPS = new Set(['links', 'mitte', 'rechts']);

/** Quelle der Anfrage: Insel + Stufe mit dem Entwurf (gleiche Quelle wie die Live-Vorschau), bis eine Insel geladen ist das Biom. */
const toSource = (plan?: PlanSource): WorldSource => (plan ? { island: plan.island, stage: plan.stage, planned: plan.planned, chunkCount: plan.chunkCount } : { biome: DEFAULT_BIOME });

const chunkClass = (kind: string) => `wd-chunk${kind === 'hub' ? ' wd-hub' : SPECIAL_CHUNKS.has(kind) ? ' wd-special' : ''}`;

/** Streifen mit Abschnitten, Objekten und Hub-Mitte im Maßstab scale (Pixel je Unit). `hints` markieren Chunk-Plätze mit
 *  einem farbigen unteren Rand im Abschnitt (B-485); der Rand liegt über der Unterkante und ändert die Größe nicht.
 *  `frame` liegt über dem ganzen Streifen (gelber Rahmen der Live-Vorschau, B-665). Ein Klick auf einen Abschnitt meldet ihn
 *  an `onChunk` (Sprung dorthin). */
export function Strip({ model, scale, hints = [], frame, onChunk }: {
  model: LevelModel; scale: number; hints?: readonly SlotHint[]; frame?: ReactNode; onChunk?: (chunk: ModelChunk) => void;
}) {
  const marks = chunkMarks(model, hints);
  return (
    <div className="wd-strip" style={{ width: model.widthUnits * scale }}>
      {model.chunks.map((c) => {
        const mark = marks.get(c.index);
        const title = `Abschnitt ${c.index}: ${nameOf(c.kind)} (${c.kind}, ${c.from}–${c.to} Units)`;
        return (
          <div key={c.index} className={chunkClass(c.kind)} style={{ left: c.from * scale, width: (c.to - c.from) * scale }}
            data-mark={mark?.kind} title={mark ? `${title}
${mark.label}` : title} onClick={() => onChunk?.(c)}>
            {c.index} {iconOf(c.kind)} {nameOf(c.kind)}
            {mark && <span className={`wd-mark wd-mark-${mark.kind}`} aria-hidden="true" />}
          </div>
        );
      })}
      {model.objects.map((o, i) => {
        const big = BIG_KINDS.has(o.kind);
        const size = big ? 22 : 10;
        const style: CSSProperties = { left: o.x * scale - size / 2, top: big ? '0.5rem' : '3.25rem', width: size, height: size, background: colorOf(o.kind) };
        return <div key={i} className="wd-marker" style={style} title={`${o.kind} bei ${o.x} Units`} />;
      })}
      <div className="wd-hubline" style={{ left: model.hubCenterUnits * scale }} title={`Hub-Mitte bei ${model.hubCenterUnits} Units`} />
      {frame}
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
    loaded.current(seed, next.biomeId, next);
    const name = BIOMES.find((b) => b.id === next.biomeId)?.name ?? next.biomeId;
    setNote(`Seed „${next.seed}“ · ${name} · ${next.widthUnits} Units · ${next.chunks.length} Abschnitte · ${next.objects.length} Objekte`);
  }, []);

  return { model, note, load };
}

/** Sprungpunkte im Level (Hub, Mauer, Portale, Ausgang) als kleine Marken über dem Streifen, im selben Maßstab (B-665/AC-02);
 *  Anfang, Mitte und Ende stehen nur im Menü „Springe zu“. */
function JumpRow({ jumps, widthUnits, scale, onJump }: { jumps: readonly JumpPoint[]; widthUnits: number; scale: number; onJump: (x: number) => void }) {
  const marks = jumps.filter((j) => j.x !== null && !EDGE_JUMPS.has(j.id));
  return (
    <div className="wd-jumprow" role="group" aria-label="Sprungmarken" style={{ width: widthUnits * scale }}>
      {marks.map((j) => (
        <Button key={j.id} className="wd-jumpmark" variant="soft" size="1" title={`Springe zu ${j.label} (${Math.round(j.x!)} Units)`}
          style={{ left: j.x! * scale }} onClick={() => onJump(j.x!)}>{j.label}</Button>
      ))}
    </div>
  );
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
 *  `frame` liegt über dem Streifen (gelber Rahmen der Live-Vorschau), `jumps` stehen als Marken darüber (B-665/AC-02). Er ist
 *  der Navigator der Seite (B-665). Mit `source` zeigt er Insel und Stufe samt Entwurf der geplanten Chunks; `hints` markieren
 *  Chunk-Plätze (geplant, Prüffehler). */
export function ChunkPreview({ onLoaded, source, view, onWheel, hints, frame, jumps = [], onJump, onChunk }: {
  onLoaded: (seed: string, biome: string, model: LevelModel) => void;
  source?: PlanSource; view: View; onWheel: (e: WheelEvent) => void; hints?: readonly SlotHint[]; frame?: ReactNode;
  jumps?: readonly JumpPoint[]; onJump?: (x: number) => void; onChunk?: (chunk: ModelChunk) => void;
}) {
  const [seed, setSeed] = useState(DEFAULT_SEED);
  const { model, note, load } = useWorldLevel(onLoaded);
  const viewport = useRef<HTMLDivElement>(null);

  // Jede Änderung der Quelle (Insel, Stufe, Entwurf) lädt das Level neu, mit dem Seed, der zuletzt eingegeben wurde.
  const latest = useRef({ seed, source });
  latest.current = { seed, source };
  const sourceKey = JSON.stringify(source ?? null);
  useEffect(() => { void load(latest.current.seed, toSource(latest.current.source)); }, [sourceKey, load]);

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
    void load(next, toSource(source));
  };

  return (
    <section className="wd-chunks wd-card" aria-label="Chunk-Vorschau">
      <div className="wd-bar">
        <Text size="2" weight="bold">Navigator</Text>
        <TextField.Root className="wd-seed" aria-label="Seed" value={seed} onChange={(e) => setSeed(e.target.value)}
          onKeyDown={(e) => { if (e.key === 'Enter') void load(seed, toSource(source)); }} />
        <Button onClick={() => void load(seed, toSource(source))}>Laden</Button>
        <Button variant="soft" aria-label="Zufälliger Seed" title="Zufälliger Seed" onClick={random}>🎲</Button>
        <p className="wd-note" role="status">{note}</p>
      </div>
      <div className="wd-viewport" ref={viewport}>
        {model && onJump && <JumpRow jumps={jumps} widthUnits={model.widthUnits} scale={view.scale} onJump={onJump} />}
        {model && <Strip model={model} scale={view.scale} hints={hints} frame={frame} onChunk={onChunk} />}
      </div>
      {model && <div className="wd-legend-block"><Legend model={model} /></div>}
    </section>
  );
}
