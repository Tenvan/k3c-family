import { useEffect, useState } from 'react';
import { Badge, Box, Flex, Heading, SelectField, Text } from '@webkit/atoms';
import { ListDetail } from '@webkit/templates';
import type { WorldDetail, WorldInfo } from '../api';
import { ChunkInspector, type ChunkPlace } from './ChunkInspector';
import { ChunkPreview, type Scale } from './ChunkPreview';
import { WorldValues } from './WorldValues';
import { useWorld, type StageChoice, type WorldState } from './useWorld';

// Seite Welten-Editor (B-879, Bereich Development): links die Welten unter data/worlds/, rechts Kopf mit Insel und
// Stufe, Chunk-Vorschau, Inspektor und Werte. Port des Welten-Editors aus dem früheren Asset-Studio (Commit 616b5afcb).

function WorldList({ worlds, id, onChoose }: { worlds: WorldInfo[]; id: string; onChoose: (id: string) => void }) {
  return (
    <Flex direction="column" gap="1" role="list" aria-label="Welten">
      <Heading size="2" color="gray">Welten</Heading>
      {worlds.map((w) => (
        <Box key={w.id} role="button" tabIndex={0} data-world={w.id} aria-pressed={w.id === id}
          className={`wd-entry${w.id === id ? ' wd-entry-active' : ''}`}
          onClick={() => onChoose(w.id)} onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && onChoose(w.id)}>
          <Text size="2" weight="bold">{w.name}</Text> <Text size="1" color="gray">{w.id}</Text>
        </Box>
      ))}
      {worlds.length === 0 && <Text size="1" color="gray">Keine Welten unter data/worlds/.</Text>}
    </Flex>
  );
}

/** Wert der Auswahl „Insel · Stufe“: `<insel>:<index>`. */
const stageKey = (s: StageChoice | null) => (s ? `${s.island}:${s.stage}` : '');

function stageOptions(detail: WorldDetail) {
  return detail.islands.flatMap((isl) =>
    isl.depths.map((d, i) => ({ value: `${isl.id}:${i}`, label: `${isl.name} · Stufe ${i} (Tiefe ${d}, ${isl.biomes[i] ?? '?'})` })));
}

function WorldHead({ w }: { w: WorldState }) {
  const { detail } = w;
  if (!detail) return <Text size="2" color="gray">Lade Welt …</Text>;
  const pick = (v: string) => {
    const [island = '', stage = '0'] = v.split(':');
    w.setStage({ world: w.id, island, stage: Number(stage) });
  };
  return (
    <Flex direction="column" gap="1">
      <Flex gap="2" align="center" wrap="wrap">
        <Heading size="4">{detail.name}</Heading>
        <Badge color="gray">data/worlds/{detail.id}.json</Badge>
        {detail.islands.length > 0 && (
          <SelectField size="1" aria-label="Insel und Stufe" value={stageKey(w.stage)} onChange={pick} options={stageOptions(detail)} />
        )}
      </Flex>
      {detail.error && <Text size="1" color="red" role="alert">{detail.error}</Text>}
      {detail.notes && <Text size="1" color="gray" className="wd-notes">{detail.notes}</Text>}
      {w.notice && <Text size="1" role="status" color={w.notice.tone === 'error' ? 'red' : w.notice.tone === 'warn' ? 'orange' : 'green'}>{w.notice.text}</Text>}
    </Flex>
  );
}

/** Ort des gewählten Abschnitts für den Prompt, solange Welt und Stufe geladen sind. */
function placeOf(w: WorldState): ChunkPlace | null {
  const isl = w.detail?.islands.find((i) => i.id === w.stage?.island);
  if (!isl || !w.stage) return null;
  return { world: w.id, island: isl.id, stage: w.stage.stage, depth: isl.depths[w.stage.stage] ?? 0, seed: w.model?.seed ?? w.seed };
}

export function WorldPage() {
  const w = useWorld();
  const [chunk, setChunk] = useState<number | null>(null);
  const [scale, setScale] = useState<Scale>('2');
  useEffect(() => setChunk(null), [w.id, w.stage]);
  return (
    <ListDetail listWidth={240} className="wd-page" list={<WorldList worlds={w.worlds} id={w.id} onChoose={w.choose} />}>
      <Flex direction="column" gap="3">
        <WorldHead w={w} />
        <ChunkPreview model={w.model} note={w.note} seed={w.seed} onSeed={w.setSeed} onLoad={w.reload} scale={scale} onScale={setScale}
          selected={chunk} onChunk={setChunk} />
        <ChunkInspector model={w.model} index={chunk} place={placeOf(w)} />
        <WorldValues values={w.values} onSet={(file, path, value) => void w.setValue(file, path, value)} />
      </Flex>
    </ListDetail>
  );
}
