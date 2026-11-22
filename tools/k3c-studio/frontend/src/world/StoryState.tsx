import { useState } from 'react';
import { ActionButton, Badge, Flex, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import type { WorldDetail } from '../api';

// Stand der Geschichte einer Welt (B-975/R5, docs/rules/welten.md § 2.2): vorhanden oder fehlt, dazu der Prompt-Button
// „Geschichte schreiben“. Der Editor bearbeitet die Texte nicht; geschrieben wird über das MCP-Tool world_story_set.

const filled = (lines?: string[]) => (lines?.length ?? 0) > 0;

/** Fehlende Einträge der Geschichte, wie die Hinweise von world_check (Go: world.Detail.StoryGaps). */
export function storyGaps(detail: WorldDetail): string[] {
  const gaps: string[] = [];
  const s = detail.story;
  if (!s || !Object.values(s).some((v) => Object.keys(v ?? {}).length > 0)) gaps.push('story');
  else {
    if (!filled(s.start)) gaps.push('story.start');
    if (!filled(s.end)) gaps.push('story.end');
    if ((s.bottles?.length ?? 0) === 0) gaps.push('story.bottles');
  }
  for (const isl of detail.islands) {
    const prefix = `Insel ${isl.id}: story`;
    if (!isl.story || !Object.values(isl.story).some(filled)) gaps.push(prefix);
    else {
      for (const key of ['arrival', 'boss', 'done'] as const) if (!filled(isl.story[key])) gaps.push(`${prefix}.${key}`);
    }
  }
  return gaps;
}

/** Auftrag an Claude: Welt-ID, Klang, Inseln, fehlende Einträge und der Weg über world_story_set. */
export function storyPrompt(detail: WorldDetail): string {
  const islands = detail.islands.map((isl, n) => `${isl.id} („${isl.name}“, Etappe ${isl.etappe ?? n + 1})`).join(', ');
  const gaps = storyGaps(detail);
  return [
    `Schreibe die Geschichte der Welt „${detail.name}“ (Welt-ID ${detail.id}, data/worlds/${detail.id}.json) in Kunstsprache nach docs/rules/welten.md § 2.2.`,
    'Gewässer: Der Untertitel (zweiter Text) einer Welt ist immer das Gewässer, an dem sie sich orientiert: Fluss, See, Küste oder Meer; Geschichte, Kunstwörter und Bilder der Welt beziehen sich darauf.',
    `Klang: eine eigene Kunstsprache, deren Klang zur Welt „${detail.name}“ passt${detail.notes ? ` (${detail.notes})` : ''}; kurze echte Wörter in GROSSBUCHSTABEN deuten die Handlung an.`,
    `Inseln: ${islands || 'keine'}.`,
    gaps.length > 0 ? `Es fehlt: ${gaps.join(', ')}.` : 'Alle Einträge sind vorhanden; überarbeite nur, was ich nenne.',
    `Lies zuerst world_get {id: "${detail.id}"}. Schreibe mit dem MCP-Tool world_story_set {id: "${detail.id}", lexicon, start, etappen, end, bottles, islands: {<insel-id>: {arrival, boss, done}}} und prüfe danach mit world_check {id: "${detail.id}"}.`,
    'Ändere die Welt-Datei nicht von Hand. Lege mir die Zeilen danach zur Abnahme vor.',
  ].join('\n');
}

/** Kurzform des Stands: „Geschichte fehlt“, „Geschichte unvollständig (n fehlen)“ oder „Geschichte vorhanden“. */
function stateOf(detail: WorldDetail, gaps: string[]): { text: string; color: 'green' | 'orange' } {
  if (gaps.length === 0) return { text: 'Geschichte vorhanden', color: 'green' };
  const none = !detail.story && detail.islands.every((isl) => !isl.story);
  return { text: none ? 'Geschichte fehlt' : `Geschichte unvollständig (${gaps.length} fehlen)`, color: 'orange' };
}

export function StoryState({ detail }: { detail: WorldDetail }) {
  const [note, setNote] = useState<{ ok: boolean; text: string } | null>(null);
  const gaps = storyGaps(detail);
  const state = stateOf(detail, gaps);
  const copy = async () => {
    try {
      await navigator.clipboard.writeText(storyPrompt(detail));
      setNote({ ok: true, text: 'Auftrag kopiert' });
    } catch (e) {
      setNote({ ok: false, text: errorText(e) });
    }
  };
  return (
    <Flex gap="2" align="center" wrap="wrap" role="group" aria-label="Geschichte">
      <Badge color={state.color} title={gaps.join(', ')} data-story={gaps.length === 0 ? 'vorhanden' : 'fehlt'}>{state.text}</Badge>
      <ActionButton size="1" variant="soft" aria-label="Geschichte schreiben" title="Auftrag für Claude kopieren (world_story_set)" onClick={copy}>
        Geschichte schreiben
      </ActionButton>
      {note && <Text size="1" role="status" color={note.ok ? 'green' : 'red'}>{note.text}</Text>}
    </Flex>
  );
}
