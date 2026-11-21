import { Badge, Flex, Text } from '@webkit/atoms';
import type { WorldDetail } from '../api';
import { PromptRow } from './PromptRow';
import { storyGaps, storyPrompt } from './StoryState';
import { ENTRY_NAMES, linesOf, partPrompt, type StoryPart } from './storyPrompts';

// Reiter „Geschichte“ des Welten-Editors (B-1216): die Geschichte als Baum (Start → Etappen → Inseln mit Ankunft,
// Endboss und Sieg → Ende → Flaschenpost → Wortschatz) mit Symbol und Name je Eintrag, den Zeilen und einem Prompt je
// Knoten. Anzeigen ist hier, Ändern geht per Prompt an Claude (world_story_set).

const nodeKey = (part: StoryPart) =>
  part.kind === 'island' ? `${part.island}.${part.entry}` : part.kind === 'etappe' ? `etappe.${part.n}` : part.kind;

function Node({ detail, part, icon, name }: { detail: WorldDetail; part: StoryPart; icon: string; name: string }) {
  const lines = linesOf(detail, part);
  return (
    <li className="st-node" data-story-node={nodeKey(part)}>
      <Flex gap="2" align="baseline" wrap="wrap">
        <span aria-hidden="true" className="st-icon">{icon}</span>
        <Text size="2" weight="bold">{name}</Text>
        {lines.length === 0 && <Badge color="orange" size="1">fehlt</Badge>}
        <PromptRow prompt={partPrompt(detail, part)} title={`${name} in Claude ändern`} />
      </Flex>
      {part.kind === 'lexicon'
        ? lines.length > 0 && <Text as="p" size="1" color="gray" className="st-lines">{lines.join(' · ')}</Text>
        : lines.map((l, i) => <Text key={i} as="p" size="2" className="st-lines">„{l}“</Text>)}
    </li>
  );
}

/** Etappen-Nummern der Welt: die der Inseln (Etappe oder Listenplatz + 1), aufsteigend und ohne Doppelte. */
export function etappenOf(detail: WorldDetail): string[] {
  const set = new Set(detail.islands.map((isl, n) => String(isl.etappe ?? n + 1)));
  return [...set].sort((a, b) => Number(a) - Number(b));
}

const ENTRY_ICONS = { arrival: '⚓', boss: '👹', done: '🏆' } as const;

export function StoryTab({ detail }: { detail: WorldDetail }) {
  return (
    <section className="wd-card" aria-label="Geschichte der Welt">
      <Flex gap="3" align="center" wrap="wrap">
        <Text size="2" weight="bold">Geschichte</Text>
        <Text size="1" color="gray">{storyGaps(detail).length === 0 ? 'vollständig' : `${storyGaps(detail).length} Einträge fehlen`}</Text>
      </Flex>
      <ol className="st-tree">
        <Node detail={detail} part={{ kind: 'start' }} icon="🚩" name="Start" />
        {etappenOf(detail).map((n) => (
          <li key={n} className="st-group">
            <ol className="st-tree">
              {detail.islands.map((isl, i) => String(isl.etappe ?? i + 1) === n && (
                <li key={isl.id} className="st-group" data-island={isl.id}>
                  <Text size="2" weight="bold" className="st-island">🏝 {isl.name} <Text size="1" color="gray" weight="regular">{isl.id}</Text></Text>
                  <ol className="st-tree">
                    {(['arrival', 'boss', 'done'] as const).map((entry) => (
                      <Node key={entry} detail={detail} part={{ kind: 'island', island: isl.id, entry }} icon={ENTRY_ICONS[entry]} name={ENTRY_NAMES[entry]} />
                    ))}
                  </ol>
                </li>
              ))}
            </ol>
            <Node detail={detail} part={{ kind: 'etappe', n }} icon="🏁" name={`Etappe ${n} abgeschlossen`} />
          </li>
        ))}
        <Node detail={detail} part={{ kind: 'end' }} icon="🎉" name="Ende" />
        <Node detail={detail} part={{ kind: 'bottles' }} icon="🍾" name="Flaschenpost" />
        <Node detail={detail} part={{ kind: 'lexicon' }} icon="📖" name="Wortschatz" />
      </ol>
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="1" color="gray">Ganze Geschichte neu schreiben lassen:</Text>
        <PromptRow prompt={storyPrompt(detail)} label="Geschichte schreiben" title="Geschichte in Claude schreiben" />
      </Flex>
    </section>
  );
}
