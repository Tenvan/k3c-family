import { useEffect, useRef, useState } from 'react';
import { Badge, Box, Flex, Heading, SelectField, Text } from '@webkit/atoms';
import { Tabs } from '@webkit/organisms';
import { ListDetail } from '@webkit/templates';
import type { WorldDetail, WorldInfo } from '../api';
import { loadText, savePref } from '../lib/prefs';
import { CatalogLinks } from './CatalogLinks';
import { CatalogTab } from './CatalogTab';
import { DeleteWorld } from './DeleteWorld';
import { ChunkInspector, type ChunkPlace } from './ChunkInspector';
import { ChunkPreview, type Scale } from './ChunkPreview';
import { KindBadge, KindSelect, ParentSelect } from './WorldKind';
import { NatureValues } from './NatureValues';
import { NewWorld } from './NewWorld';
import { Overview } from './Overview';
import { StoryState } from './StoryState';
import { StoryTab } from './StoryTab';
import { kindOf } from './statsCount';
import { WorldStats } from './WorldStats';
import { WorldValues } from './WorldValues';
import { WorldPrompts } from './WorldPromptBar';
import { useWorld, type StageChoice, type WorldState } from './useWorld';

// Seite Welten-Editor (B-879, Bereich Development): links die Welten unter data/worlds/, rechts Kopf mit Insel und
// Stufe, Chunk-Vorschau, Inspektor und Werte. Port des Welten-Editors aus dem früheren Asset-Studio (Commit 616b5afcb).

function WorldList({ worlds, id, onChoose, onCreate }: { worlds: WorldInfo[]; id: string; onChoose: (id: string) => void; onCreate: WorldState['create'] }) {
  return (
    <Flex direction="column" gap="1" role="list" aria-label="Welten">
      <Heading size="2" color="gray">Welten</Heading>
      {worlds.map((w) => (
        w.missing ? (
          <Box key={w.id} role="button" tabIndex={0} data-world={w.id} data-missing="true" className="wd-entry"
            title="Datei fehlt: ein Klick legt die Welt aus der Standard-Welt an und öffnet sie"
            onClick={() => void onCreate(w.id, w.name, 'standard')} onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && void onCreate(w.id, w.name, 'standard')}>
            <Text size="2" weight="bold" color="gray">{w.name}</Text> <Text size="1" color="gray">{w.id} · noch nicht umgesetzt (Klick legt an)</Text>
          </Box>
        ) : (
          <Box key={w.id} role="button" tabIndex={0} data-world={w.id} aria-pressed={w.id === id}
            className={`wd-entry${w.id === id ? ' wd-entry-active' : ''}`}
            onClick={() => onChoose(w.id)} onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && onChoose(w.id)}>
            <Text size="2" weight="bold">{w.name}</Text> <Text size="1" color="gray" title={`data/worlds/${w.id}.json`}>{w.gewaesser || w.id}</Text>{w.id !== 'standard' && <> <KindBadge kind={w.kind} /></>}
          </Box>
        )
      ))}
      {worlds.length === 0 && <Text size="1" color="gray">Keine Welten unter data/worlds/.</Text>}
      <NewWorld worlds={worlds} onCreate={onCreate} />
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
    <div className="wd-head">
    <Flex direction="column" gap="1" className="wd-head-main">
      <Flex gap="2" align="center" wrap="wrap">
        <Heading size="4">{detail.name}</Heading>
        {detail.id !== 'standard' && !detail.error && <><KindBadge kind={kindOf(detail)} /><KindSelect kind={kindOf(detail)} onChange={(k) => void w.setKind(k)} />{kindOf(detail) === 'spinoff' && <ParentSelect id={detail.id} parent={detail.extends} worlds={w.worlds} onChange={(p) => void w.setParent(p)} />}</>}
        <Badge color="gray">data/worlds/{detail.id}.json</Badge>
        {detail.islands.length > 0 && (
          <SelectField size="1" aria-label="Insel und Stufe" value={stageKey(w.stage)} onChange={pick} options={stageOptions(detail)} />
        )}
      </Flex>
      {!detail.error && <CatalogLinks welt={detail.id} />}
      <DeleteWorld key={detail.id} id={detail.id} name={detail.name} onDelete={w.remove} />
      {detail.error && <Text size="1" color="red" role="alert">{detail.error}</Text>}
      {detail.notes && <Text size="1" color="gray" className="wd-notes">{detail.notes}</Text>}
      {!detail.error && <StoryState detail={detail} />}
      {w.notice && <Text size="1" role="status" color={w.notice.tone === 'error' ? 'red' : w.notice.tone === 'warn' ? 'orange' : 'green'}>{w.notice.text}</Text>}
    </Flex>
    {!detail.error && <WorldStats detail={detail} />}
    </div>
  );
}

/** Ort des gewählten Abschnitts für den Prompt, solange Welt und Stufe geladen sind. */
function placeOf(w: WorldState): ChunkPlace | null {
  const isl = w.detail?.islands.find((i) => i.id === w.stage?.island);
  if (!isl || !w.stage) return null;
  return { world: w.id, island: isl.id, stage: w.stage.stage, depth: isl.depths[w.stage.stage] ?? 0, seed: w.model?.seed ?? w.seed };
}

const TABS = ['inseln', 'geschichte', 'katalog', 'werte'] as const;
type TabId = (typeof TABS)[number];
const loadTab = (): TabId => { const t = loadText('world.tab', 'inseln'); return (TABS as readonly string[]).includes(t) ? t as TabId : 'inseln'; };

export function WorldPage() {
  const w = useWorld();
  const [chunk, setChunk] = useState<number | null>(null);
  const [scale, setScale] = useState<Scale>('2');
  const [tab, setTab] = useState<TabId>(loadTab);
  const editor = useRef<HTMLDivElement>(null);
  useEffect(() => setChunk(null), [w.id, w.stage]);
  const pick = (t: string) => { setTab(t as TabId); savePref('world.tab', t); };
  return (
    <ListDetail listWidth={240} className="wd-page" list={<WorldList worlds={w.worlds} id={w.id} onChoose={w.choose} onCreate={w.create} />}>
      <Flex direction="column" gap="3" className="wd-main">
        <WorldHead w={w} />
        <Tabs.Root value={tab} onValueChange={pick} className="wd-tabs">
          <Tabs.List aria-label="Ansicht der Welt">
            <Tabs.Trigger value="inseln">🏝 Inseln</Tabs.Trigger>
            <Tabs.Trigger value="geschichte">📖 Geschichte</Tabs.Trigger>
            <Tabs.Trigger value="katalog">🗂 Katalog</Tabs.Trigger>
            <Tabs.Trigger value="werte">🔧 Werte</Tabs.Trigger>
          </Tabs.List>
          <Tabs.Content value="inseln">
            <Flex direction="column" gap="3" pt="3">
              {w.detail && <WorldPrompts detail={w.detail} />}
              {(() => {
                const editorParts = (
                  <>
                    <div ref={editor} className="wd-editor" />
                    <ChunkPreview model={w.model} note={w.note} seed={w.seed} onSeed={w.setSeed} onLoad={w.reload} scale={scale} onScale={setScale}
                      selected={chunk} onChunk={setChunk} />
                    <ChunkInspector model={w.model} index={chunk} place={placeOf(w)} />
                    <NatureValues level={w.level} onSet={(key, value) => void w.setNature(key, value)} />
                  </>
                );
                return w.detail && w.detail.islands.length > 0 ? (
                  <Overview detail={w.detail} seed={w.seed} choice={w.stage} onEtappe={w.setEtappe} refresh={w.crossRefresh}
                    selected={w.stage?.island ?? w.detail.islands[0]!.id} onSelect={(island) => w.setStage({ world: w.id, island, stage: 0 })}
                    onPick={(island, stage) => { w.setStage({ world: w.id, island, stage }); editor.current?.scrollIntoView?.({ behavior: 'smooth', block: 'start' }); }}>
                    {editorParts}
                  </Overview>
                ) : editorParts;
              })()}
            </Flex>
          </Tabs.Content>
          <Tabs.Content value="geschichte">
            <Flex direction="column" gap="3" pt="3" className="wd-fill">{w.detail && !w.detail.error && <StoryTab detail={w.detail} />}</Flex>
          </Tabs.Content>
          <Tabs.Content value="katalog">
            <Flex direction="column" gap="3" pt="3" className="wd-fill">{w.detail && !w.detail.error && <CatalogTab key={w.id} detail={w.detail} />}</Flex>
          </Tabs.Content>
          <Tabs.Content value="werte">
            <Flex direction="column" gap="3" pt="3" className="wd-fill"><WorldValues key={w.id} values={w.values} onSet={(file, path, value) => void w.setValue(file, path, value)} /></Flex>
          </Tabs.Content>
        </Tabs.Root>
      </Flex>
    </ListDetail>
  );
}
