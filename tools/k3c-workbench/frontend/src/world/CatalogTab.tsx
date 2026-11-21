import { useMemo, useState, type CSSProperties } from 'react';
import { Badge, Flex, LinkButton, SelectField, Splitter, Text } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import { zuordnung } from '@webkit/lib/weltFilter';
import type { WorldDetail } from '../api';
import type { AssetEintrag } from '../api/typesAssets';
import { AssetDetail } from '../assets/catalog/AssetDetail';
import { AssetTree } from '../assets/catalog/AssetTree';
import { useCatalog } from '../assets/catalog/useCatalog';
import { useWeltSicht } from '../assets/useWeltSicht';
import { prefsStore } from '../lib/prefs';
import { openCatalog } from './CatalogLinks';
import { belongsTo, biomesOf } from './catalogView';
import { PromptRow } from './PromptRow';

// Reiter „Katalog“ des Welten-Editors (B-1216): die eigenen, exklusiven Assets der Welt (nicht die geerbten) als Baum aus
// Gruppen und Assets mit Symbolen, nach Insel filterbar; ein Klick zeigt rechts denselben Asset-Editor wie die Seite
// Katalog (`AssetDetail`: Vorschau, Teile, Größenvergleich, Prompts). Assets ändern nur die MCP-Tools assets_*
// (Entscheidung 005); dieser Reiter zeigt und beauftragt nur.

const ALL = '';
const keyOf = (a: AssetEintrag) => `${a.gruppe}/${a.id}`;

/** Auftrag an Claude: neue oder geänderte Assets einer Insel; Ist-Zustand und Weg über assets_*. */
export function catalogPrompt(detail: WorldDetail, island: string, shown: readonly AssetEintrag[]): string {
  const isl = detail.islands.find((i) => i.id === island);
  const where = isl ? `Insel ${isl.id} („${isl.name}“, Biome ${isl.biomes.join('/')})` : `alle Inseln (${detail.islands.map((i) => i.id).join(', ')})`;
  const groups = [...new Set(shown.map((a) => a.gruppe))].sort().map((g) => `${g} ${shown.filter((a) => a.gruppe === g).length}`).join(', ');
  return [
    `Welten-Editor, Katalog der Welt „${detail.name}“ (Welt-ID ${detail.id}), ${where}.`,
    `Ist: ${shown.length} eigene Katalog-Einträge der Welt (${groups || 'keine'}).`,
    'Lies zuerst assets_guide und assets_list; lege neue Assets nur mit assets_create an und ändere sie nur mit assets_set (Entscheidung 005, Papierschnitt-Specs). Weise Assets der Welt zu, wenn sie nur dort gelten sollen.',
    'Was soll sich ändern: ',
  ].join('\n');
}

export function CatalogTab({ detail }: { detail: WorldDetail }) {
  const { list, prompts, error, loading, stand } = useCatalog();
  const { sicht, laedt } = useWeltSicht(detail.id);
  const [island, setIsland] = useState(ALL);
  const [selected, setSelected] = useState('');
  const [split, setSplit, resetSplit] = useSplit(prefsStore, 'world.split.catalog', 32);
  const chosen = detail.islands.some((i) => i.id === island) ? island : ALL;
  // nur die eigenen (exklusiven) Assets der Welt, nach Insel gefiltert
  const shown = useMemo(
    () => list.filter((a) => zuordnung(sicht, 'exklusiv', keyOf(a)).sichtbar && belongsTo(a, detail, chosen)),
    [list, sicht, detail, chosen],
  );
  const current = shown.some((a) => a.id === selected) ? selected : '';
  return (
    <section className="wd-card wd-fillcard" aria-label="Katalog der Welt">
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold">Katalog</Text>
        <SelectField size="1" aria-label="Insel des Katalogs" value={chosen} onChange={setIsland}
          options={[{ value: ALL, label: 'Alle Inseln' }, ...detail.islands.map((i) => ({ value: i.id, label: `${i.name} (${i.biomes.join(' → ')})` }))]} />
        <Text size="1" color="gray" role="status">{loading || laedt ? 'Lade …' : `${shown.length} eigene Assets`}</Text>
        <LinkButton size="1" data-katalog="oeffnen" onClick={() => openCatalog(detail.id, 'exklusiv')}>im Katalog öffnen</LinkButton>
        <PromptRow prompt={catalogPrompt(detail, chosen, shown)} label="Asset-Prompt kopieren" title="Assets der Insel in Claude ändern" />
      </Flex>
      {error && <Text as="p" size="1" color="red" role="alert">{error}</Text>}
      <div className="st-layout" style={{ '--st-split': `${split}%` } as CSSProperties}>
        <div className="st-tree" aria-label="Eigene Assets der Welt">
          <AssetTree assets={shown} selected={current} onSelect={setSelected}
            badges={(a) => {
              const biomes = biomesOf(a);
              return biomes.length > 0 ? <Badge size="1" color="blue">{biomes.join(' · ')}</Badge> : null;
            }} />
          {!loading && !laedt && shown.length === 0 && <Text size="1" color="gray">Diese Welt hat keine eigenen Assets für diese Auswahl.</Text>}
        </div>
        <Splitter className="st-split" label="Breite des Baums" value={split} onChange={setSplit} onReset={resetSplit} />
        <div className="st-detail" aria-label="Asset-Editor">
          <AssetDetail key={current} selected={current} list={list} prompts={prompts} stand={stand} empty="Asset links wählen" />
        </div>
      </div>
    </section>
  );
}
