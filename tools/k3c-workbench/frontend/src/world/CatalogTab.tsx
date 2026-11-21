import { useEffect, useMemo, useState } from 'react';
import { Badge, Flex, LinkButton, SelectField, Text } from '@webkit/atoms';
import { zuordnung, type WeltModus } from '@webkit/lib/weltFilter';
import { backend, type WorldDetail } from '../api';
import type { AssetEintrag } from '../api/typesAssets';
import { useWeltSicht } from '../assets/useWeltSicht';
import { openCatalog } from './CatalogLinks';
import { belongsTo, biomesOf, groupIcon } from './catalogView';
import { PromptRow } from './PromptRow';

// Reiter „Katalog“ des Welten-Editors (B-1216): die Katalog-Einträge (Assets), die die Welt sieht, nach Insel gefiltert, nach
// Gruppe geordnet, mit Herkunft (exklusiv oder geerbt) und einem Prompt für neue oder geänderte Assets der Insel. Assets
// ändern nur die MCP-Tools assets_* (Entscheidung 005); der Reiter zeigt und beauftragt nur.

const ALL = '';
const keyOf = (a: AssetEintrag) => `${a.gruppe}/${a.id}`;

/** Auftrag an Claude: neue oder geänderte Assets einer Insel; Ist-Zustand und Weg über assets_*. */
export function catalogPrompt(detail: WorldDetail, island: string, shown: readonly AssetEintrag[]): string {
  const isl = detail.islands.find((i) => i.id === island);
  const where = isl ? `Insel ${isl.id} („${isl.name}“, Biome ${isl.biomes.join('/')})` : `alle Inseln (${detail.islands.map((i) => i.id).join(', ')})`;
  const groups = [...new Set(shown.map((a) => a.gruppe))].sort().map((g) => `${g} ${shown.filter((a) => a.gruppe === g).length}`).join(', ');
  return [
    `Welten-Editor, Katalog der Welt „${detail.name}“ (Welt-ID ${detail.id}), ${where}.`,
    `Ist: ${shown.length} Katalog-Einträge (${groups || 'keine'}).`,
    'Lies zuerst assets_guide und assets_list; lege neue Assets nur mit assets_create an und ändere sie nur mit assets_set (Entscheidung 005, Papierschnitt-Specs). Weise Assets der Welt zu, wenn sie nur dort gelten sollen.',
    'Was soll sich ändern: ',
  ].join('\n');
}

export function CatalogTab({ detail }: { detail: WorldDetail }) {
  const [assets, setAssets] = useState<AssetEintrag[] | null>(null);
  const [error, setError] = useState('');
  const [island, setIsland] = useState(ALL);
  const [modus, setModus] = useState<WeltModus>('gesamt');
  const { sicht, laedt } = useWeltSicht(detail.id);
  useEffect(() => {
    let live = true;
    backend.assetsList().then((l) => live && setAssets(l), (e: unknown) => live && setError(e instanceof Error ? e.message : String(e)));
    return () => { live = false; };
  }, []);
  const chosen = detail.islands.some((i) => i.id === island) ? island : ALL;
  const shown = useMemo(
    () => (assets ?? []).filter((a) => zuordnung(sicht, modus, keyOf(a)).sichtbar && belongsTo(a, detail, chosen)),
    [assets, sicht, modus, detail, chosen],
  );
  const groups = [...new Set(shown.map((a) => a.gruppe))].sort();
  return (
    <section className="wd-card wd-fillcard" aria-label="Katalog der Welt">
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold">Katalog</Text>
        <SelectField size="1" aria-label="Insel des Katalogs" value={chosen} onChange={setIsland}
          options={[{ value: ALL, label: 'Alle Inseln' }, ...detail.islands.map((i) => ({ value: i.id, label: `${i.name} (${i.biomes.join(' → ')})` }))]} />
        <SelectField size="1" aria-label="Sicht auf den Katalog" value={modus} onChange={(v) => setModus(v as WeltModus)}
          options={[{ value: 'gesamt', label: 'mit geerbten (gesamt)' }, { value: 'exklusiv', label: 'nur diese Welt (exklusiv)' }]} />
        <Text size="1" color="gray" role="status">{assets === null ? 'Lade …' : `${shown.length} Einträge`}</Text>
        <LinkButton size="1" data-katalog="oeffnen" onClick={() => openCatalog(detail.id, modus)}>im Katalog öffnen</LinkButton>
        <PromptRow prompt={catalogPrompt(detail, chosen, shown)} label="Asset-Prompt kopieren" title="Assets der Insel in Claude ändern" />
      </Flex>
      {error && <Text as="p" size="1" color="red" role="alert">{error}</Text>}
      <div className="ct-list" role="list" aria-label="Katalog-Einträge">
        {(laedt || assets === null) && !error && <Text size="1" color="gray">Lade Katalog …</Text>}
        {groups.map((g) => (
          <div key={g} className="ct-group" role="group" aria-label={g} data-group={g}>
            <Text size="2" weight="bold">{groupIcon(g)} {g} <Text size="1" color="gray" weight="regular">{shown.filter((a) => a.gruppe === g).length}</Text></Text>
            <div className="ct-entries">
              {shown.filter((a) => a.gruppe === g).map((a) => {
                const z = zuordnung(sicht, modus, keyOf(a));
                const biomes = biomesOf(a);
                return (
                  <div key={keyOf(a)} role="listitem" className="ct-entry" data-asset={keyOf(a)}>
                    <Text size="2">{a.id}</Text>
                    <Text size="1" color="gray">{a.rig}</Text>
                    {biomes.length > 0 && <Badge size="1" color="blue">{biomes.join(' · ')}</Badge>}
                    {sicht && <Badge size="1" color={z.exklusiv ? 'orange' : 'gray'}>{z.exklusiv ? 'exklusiv' : `geerbt: ${z.herkunft}`}</Badge>}
                    {!a.gueltig && <Badge size="1" color="red">ungültig</Badge>}
                  </div>
                );
              })}
            </div>
          </div>
        ))}
        {assets !== null && !laedt && shown.length === 0 && <Text size="1" color="gray">Keine Einträge für diese Auswahl.</Text>}
      </div>
    </section>
  );
}
