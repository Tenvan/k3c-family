import { useState, type CSSProperties } from 'react';
import { Splitter } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import { prefsStore } from '../lib/prefs';
import { Badge, Button, Flex, Text } from '@webkit/atoms';
import type { WorldDetail } from '../api';
import { PromptRow } from './PromptRow';
import { storyGaps, storyPrompt } from './StoryState';
import { ENTRY_NAMES, linesOf, partPrompt, type StoryPart } from './storyPrompts';

// Reiter „Geschichte“ des Welten-Editors (B-1216): links ein kompakter Baum (Start → Inseln mit Ankunft, Endboss und
// Sieg, je Etappe → Ende → Flaschenpost → Wortschatz) mit Symbol und Name, rechts der Text des gewählten Knotens mit
// seinem Prompt. Anzeigen ist hier, Ändern geht per Prompt an Claude (world_story_set).

const nodeKey = (part: StoryPart) =>
  part.kind === 'island' ? `${part.island}.${part.entry}` : part.kind === 'islandAll' ? part.island : part.kind === 'etappe' ? `etappe.${part.n}` : part.kind;

interface Item {
  part: StoryPart;
  icon: string;
  name: string;
}

/** Etappen-Nummern der Welt: die der Inseln (Etappe oder Listenplatz + 1), aufsteigend und ohne Doppelte. */
export function etappenOf(detail: WorldDetail): string[] {
  const set = new Set(detail.islands.map((isl, n) => String(isl.etappe ?? n + 1)));
  return [...set].sort((a, b) => Number(a) - Number(b));
}

const ENTRY_ICONS = { arrival: '⚓', boss: '👹', done: '🏆' } as const;

function Leaf({ detail, item, sel, onPick }: { detail: WorldDetail; item: Item; sel: string; onPick: (key: string) => void }) {
  const key = nodeKey(item.part);
  const missing = linesOf(detail, item.part).length === 0;
  return (
    <Button type="button" variant="ghost" color="gray" className={`st-node${sel === key ? ' is-on' : ''}`} data-story-node={key} aria-pressed={sel === key} onClick={() => onPick(key)}>
      <span aria-hidden="true" className="st-icon">{item.icon}</span>
      <Text size="2">{item.name}</Text>
      {missing && <Badge color="orange" size="1" aria-label="fehlt">fehlt</Badge>}
    </Button>
  );
}

/** Alle Knoten in Baumreihenfolge, für die Suche nach dem gewählten. */
function itemsOf(detail: WorldDetail): Item[] {
  const items: Item[] = [{ part: { kind: 'start' }, icon: '🚩', name: 'Start' }];
  for (const n of etappenOf(detail)) {
    detail.islands.forEach((isl, i) => {
      if (String(isl.etappe ?? i + 1) !== n) return;
      items.push({ part: { kind: 'islandAll', island: isl.id }, icon: '🏝', name: isl.name });
      for (const entry of ['arrival', 'boss', 'done'] as const) {
        items.push({ part: { kind: 'island', island: isl.id, entry }, icon: ENTRY_ICONS[entry], name: `${isl.name}: ${ENTRY_NAMES[entry]}` });
      }
    });
    items.push({ part: { kind: 'etappe', n }, icon: '🏁', name: `Etappe ${n} abgeschlossen` });
  }
  items.push({ part: { kind: 'end' }, icon: '🎉', name: 'Ende' }, { part: { kind: 'bottles' }, icon: '🍾', name: 'Flaschenpost' }, { part: { kind: 'lexicon' }, icon: '📖', name: 'Wortschatz' });
  return items;
}

/** Alle drei Einträge einer Insel mit Überschrift (Ankunft, Endboss, Sieg). */
function IslandLines({ detail, island }: { detail: WorldDetail; island: string }) {
  return (
    <>
      {(['arrival', 'boss', 'done'] as const).map((entry) => {
        const lines = linesOf(detail, { kind: 'island', island, entry });
        return (
          <div key={entry} data-entry={entry}>
            <Text size="1" color="gray">{ENTRY_ICONS[entry]} {ENTRY_NAMES[entry]}</Text>
            {lines.length === 0 ? <Text as="p" size="2" color="orange">fehlt</Text> : lines.map((l, i) => <Text key={i} as="p" className="st-line">„{l}“</Text>)}
          </div>
        );
      })}
    </>
  );
}

function Detail({ detail, item }: { detail: WorldDetail; item: Item }) {
  const lines = linesOf(detail, item.part);
  return (
    <div className="st-detail" aria-label="Text des Knotens" aria-live="polite">
      <Flex gap="2" align="center" wrap="wrap">
        <span aria-hidden="true" className="st-icon">{item.icon}</span>
        <Text size="3" weight="bold">{item.name}</Text>
        {lines.length === 0 && <Badge color="orange" size="1">fehlt</Badge>}
      </Flex>
      {item.part.kind === 'islandAll' && <IslandLines detail={detail} island={item.part.island} />}
      {item.part.kind === 'lexicon'
        ? lines.length > 0 && <Text as="p" size="2" color="gray">{lines.join(' · ')}</Text>
        : item.part.kind !== 'islandAll' && lines.map((l, i) => <Text key={i} as="p" className="st-line">„{l}“</Text>)}
      <PromptRow prompt={partPrompt(detail, item.part)} title={`${item.name} in Claude ändern`} />
    </div>
  );
}

export function StoryTab({ detail }: { detail: WorldDetail }) {
  const [sel, setSel] = useState('start');
  const [split, setSplit, resetSplit] = useSplit(prefsStore, 'world.split.story', 30);
  const items = itemsOf(detail);
  const current = items.find((i) => nodeKey(i.part) === sel) ?? items[0]!;
  const gaps = storyGaps(detail).length;
  const leaf = (item: Item) => <Leaf key={nodeKey(item.part)} detail={detail} item={item} sel={nodeKey(current.part)} onPick={setSel} />;
  return (
    <section className="wd-card wd-fillcard" aria-label="Geschichte der Welt">
      <Flex gap="3" align="center" wrap="wrap">
        <Text size="2" weight="bold">Geschichte</Text>
        <Text size="1" color="gray">{gaps === 0 ? 'vollständig' : `${gaps} Einträge fehlen`}</Text>
        <PromptRow prompt={storyPrompt(detail)} label="Ganze Geschichte schreiben" title="Geschichte in Claude schreiben" />
      </Flex>
      <div className="st-layout" style={{ '--st-split': `${split}%` } as CSSProperties}>
        <div className="st-tree" role="group" aria-label="Knoten der Geschichte">
          {leaf(items[0]!)}
          {etappenOf(detail).map((n) => (
            <div key={n} className="st-branch" data-etappe={n}>
              {detail.islands.map((isl, i) => String(isl.etappe ?? i + 1) === n && (
                <div key={isl.id} data-island={isl.id}>
                  {leaf(items.find((it) => it.part.kind === 'islandAll' && it.part.island === isl.id)!)}
                  <div className="st-branch">{items.filter((it) => it.part.kind === 'island' && it.part.island === isl.id).map(leaf)}</div>
                </div>
              ))}
              {leaf(items.find((it) => it.part.kind === 'etappe' && it.part.n === n)!)}
            </div>
          ))}
          {items.filter((it) => ['end', 'bottles', 'lexicon'].includes(it.part.kind)).map(leaf)}
        </div>
        <Splitter className="st-split" label="Breite des Baums" value={split} onChange={setSplit} onReset={resetSplit} />
        <Detail detail={detail} item={current} />
      </div>
    </section>
  );
}
