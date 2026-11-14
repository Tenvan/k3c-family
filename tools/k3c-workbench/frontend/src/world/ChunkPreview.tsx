import type { CSSProperties } from 'react';
import { Button, Flex, Text, TextField } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import { BIG_KINDS, SPECIAL_CHUNKS, colorOf, plannedAt, seedFromBytes, slotLabel, type LevelModel, type ModelChunk } from './levelView';
import { iconOf, nameOf } from './kindIcons';

// Chunk-Vorschau des Welten-Editors (B-879, Port aus dem Asset-Studio, world/ChunkPreview.tsx): das Level der gewählten
// Insel-Stufe als Streifen mit Abschnitten, Objekten und Hub-Mitte; ein Klick wählt den Abschnitt für den Inspektor.

/** Pixel je Unit, wählbar über den Zoom */
export const SCALES = ['1', '2', '4'] as const;
export type Scale = (typeof SCALES)[number];

const chunkClass = (c: ModelChunk, model: LevelModel, selected: boolean) =>
  ['wd-chunk', c.kind === 'hub' && 'wd-hub', SPECIAL_CHUNKS.has(c.kind) && 'wd-special', plannedAt(model, c) && 'wd-planned', selected && 'wd-selected']
    .filter(Boolean).join(' ');

/** Streifen im Maßstab scale (Pixel je Unit). */
export function Strip({ model, scale, selected, onChunk }: { model: LevelModel; scale: number; selected: number | null; onChunk: (index: number) => void }) {
  return (
    <div className="wd-strip" style={{ width: model.widthUnits * scale }}>
      {model.chunks.map((c) => (
        <div key={c.index} role="button" tabIndex={0} data-chunk={c.index} aria-pressed={c.index === selected}
          className={chunkClass(c, model, c.index === selected)} style={{ left: c.from * scale, width: (c.to - c.from) * scale }}
          title={`Abschnitt ${c.index} (Platz ${slotLabel(c.slot)}): ${nameOf(c.kind)}, ${c.from}–${c.to} Units`}
          onClick={() => onChunk(c.index)} onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && onChunk(c.index)}>
          {c.index} {iconOf(c.kind)}
        </div>
      ))}
      {model.objects.map((o, i) => {
        const size = BIG_KINDS.has(o.kind) ? 18 : 8;
        const style: CSSProperties = { left: o.x * scale - size / 2, width: size, height: size, background: colorOf(o.kind) };
        return <div key={i} className={`wd-marker${BIG_KINDS.has(o.kind) ? ' wd-marker-big' : ''}`} style={style} title={`${nameOf(o.kind)} bei ${o.x} Units`} />;
      })}
      <div className="wd-hubline" style={{ left: model.hubCenterUnits * scale }} title={`Hub-Mitte bei ${model.hubCenterUnits} Units`} />
    </div>
  );
}

function Legend({ model }: { model: LevelModel }) {
  return (
    <>
      <ul className="wd-legend" aria-label="Objekte je Art">
        {model.counts.map(([kind, count]) => (
          <li key={kind} style={{ '--dot': colorOf(kind) } as CSSProperties}>{nameOf(kind)} {count}</li>
        ))}
      </ul>
      {model.warnings.length > 0 && (
        <ul className="wd-warnings" aria-label="Warnungen">{model.warnings.map((w, i) => <li key={i}>{w}</li>)}</ul>
      )}
    </>
  );
}

interface Props {
  model: LevelModel | null;
  note: string;
  seed: string;
  onSeed: (seed: string) => void;
  onLoad: (seed: string) => void;
  scale: Scale;
  onScale: (s: Scale) => void;
  selected: number | null;
  onChunk: (index: number) => void;
}

/** Chunk-Vorschau mit Seed, Zoom, Streifen, Legende und Warnungen der Spielbarkeits-Prüfung. */
export function ChunkPreview({ model, note, seed, onSeed, onLoad, scale, onScale, selected, onChunk }: Props) {
  const random = () => {
    const next = seedFromBytes(crypto.getRandomValues(new Uint8Array(8)));
    onSeed(next);
    onLoad(next);
  };
  return (
    <section className="wd-card" aria-label="Chunk-Vorschau">
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold">Chunk-Vorschau</Text>
        <TextField.Root size="1" className="wd-seed" aria-label="Seed" value={seed} onChange={(e) => onSeed(e.target.value)}
          onKeyDown={(e) => { if (e.key === 'Enter') onLoad(seed); }} />
        <Button size="1" onClick={() => onLoad(seed)}>Laden</Button>
        <Button size="1" variant="soft" aria-label="Zufälliger Seed" title="Zufälliger Seed" onClick={random}>🎲</Button>
        <ChipSwitch label="Zoom" options={SCALES.map((s) => ({ value: s, label: `${s}×` }))} value={scale} onChange={onScale} />
      </Flex>
      <Text as="p" size="1" color="gray" role="status">{note}</Text>
      <div className="wd-viewport">
        {model && <Strip model={model} scale={Number(scale)} selected={selected} onChunk={onChunk} />}
      </div>
      {model && <Legend model={model} />}
    </section>
  );
}
