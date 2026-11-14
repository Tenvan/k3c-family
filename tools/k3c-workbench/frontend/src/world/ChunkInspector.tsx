import { useState } from 'react';
import { ActionButton, Badge, Flex, IconButton, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { backend, type PlannedChunk } from '../api';
import { iconOf, nameOf } from './kindIcons';
import { objectsIn, plannedAt, slotLabel, type LevelModel, type ModelChunk } from './levelView';

// Inspektor des Welten-Editors (B-879, Port aus dem Asset-Studio, world/ChunkInspector.tsx): der in der Chunk-Vorschau
// gewählte Abschnitt mit Art, Platz, Objekten und Planung. Ändern geht per Prompt an Claude (wie „Fix in Chat“ im
// Studio): die Planung liegt in der Welt-Datei, Claude ändert sie und prüft sie mit den Tests.

/** Wo der Abschnitt liegt: Welt, Insel, Stufe und Tiefe. */
export interface ChunkPlace {
  world: string;
  island: string;
  stage: number;
  depth: number;
  seed: string;
}

/** Prompt an Claude zum Abschnitt: Ort, Ist-Zustand, Datei und Regeln. */
export function chunkPrompt(place: ChunkPlace, chunk: ModelChunk, planned: PlannedChunk | undefined): string {
  const where = `Welt „${place.world}“, Insel ${place.island}, Stufe ${place.stage} (Tiefe ${place.depth}), Seed „${place.seed}“`;
  const now = planned ? `geplant: ${JSON.stringify(planned)}` : 'zufällig (ohne Planung)';
  return [
    `Welten-Editor: ${where}, Abschnitt ${chunk.index} auf Platz ${slotLabel(chunk.slot)} (${chunk.kind}), ${now}.`,
    `Ändere die Planung dieses Abschnitts in data/worlds/${place.world}.json › islands › ${place.island} › plannedChunks › "${place.depth}" nach docs/rules/geplante-chunks.md.`,
    'Was soll sich ändern: ',
  ].join('\n');
}

function PlannedFields({ p }: { p: PlannedChunk }) {
  return (
    <Flex direction="column" gap="1" data-planned={p.slot}>
      <Text size="2">Geplante Art: {iconOf(p.kind)} {nameOf(p.kind)}{p.seed ? ` · Chunk-Seed „${p.seed}“` : ''}</Text>
      {(p.objects ?? []).map((o, i) => <Text key={i} size="1" color="gray">fest: {nameOf(o.kind)} bei {o.at} Units</Text>)}
      {(p.buildings ?? []).map((b, i) => <Text key={i} size="1" color="gray">Ruine: {nameOf(b.kind)}</Text>)}
      {(p.enemies?.length ?? 0) > 0 && <Text size="1" color="gray">Gegnerlager: {p.enemies!.length} Wachen</Text>}
    </Flex>
  );
}

function PromptRow({ prompt }: { prompt: string }) {
  const [note, setNote] = useState<{ ok: boolean; text: string } | null>(null);
  const run = async (open: boolean) => {
    try {
      await (open ? backend.openInClaude(prompt) : navigator.clipboard.writeText(prompt));
      setNote({ ok: true, text: open ? 'In Claude geöffnet' : 'Prompt kopiert' });
    } catch (e) {
      setNote({ ok: false, text: errorText(e) });
    }
  };
  return (
    <Flex gap="2" align="center" wrap="wrap">
      <ActionButton size="1" variant="soft" onClick={() => run(false)}>Prompt kopieren</ActionButton>
      <IconButton size="1" variant="ghost" aria-label="Abschnitt in Claude ändern" title="In Claude öffnen" onClick={() => run(true)}>✦</IconButton>
      {note && <Text size="1" color={note.ok ? 'green' : 'red'}>{note.text}</Text>}
    </Flex>
  );
}

/** Inspektor: ohne Wahl ein Hinweis, sonst Kopf, Objekte im Abschnitt, Planung und Prompt an Claude. */
export function ChunkInspector({ model, index, place }: { model: LevelModel | null; index: number | null; place: ChunkPlace | null }) {
  const chunk = index === null ? undefined : model?.chunks.find((c) => c.index === index);
  if (!model || !chunk || !place) {
    return (
      <section className="wd-card" aria-label="Inspektor">
        <Text size="2" weight="bold">Inspektor</Text>
        <Text as="p" size="1" color="gray">Wähle einen Abschnitt in der Chunk-Vorschau.</Text>
      </section>
    );
  }
  const planned = plannedAt(model, chunk);
  const objects = objectsIn(model, chunk);
  return (
    <section className="wd-card" aria-label="Inspektor" data-chunk={chunk.index}>
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold">Abschnitt {chunk.index}</Text>
        <Badge color="gray">Platz {slotLabel(chunk.slot)}</Badge>
        <Text size="2">{iconOf(chunk.kind)} {nameOf(chunk.kind)}</Text>
        <Badge color={planned ? 'blue' : 'gray'}>{planned ? 'geplant' : 'zufällig'}</Badge>
      </Flex>
      <Text as="p" size="1" color="gray">{chunk.from}–{chunk.to} Units · {objects.length} Objekte</Text>
      {objects.length > 0 && (
        <ul className="wd-objects" aria-label="Objekte im Abschnitt">
          {objects.map((o, i) => <li key={i}>{iconOf(o.kind)} {nameOf(o.kind)} bei {Math.round(o.x - chunk.from)} Units</li>)}
        </ul>
      )}
      {planned && <PlannedFields p={planned} />}
      <PromptRow prompt={chunkPrompt(place, chunk, planned)} />
    </section>
  );
}
