import { useState } from 'react';
import { Flex, Text } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import { ListDetail } from '@webkit/templates';
import type { AssetEintrag, Spec, Vorlage } from '../api';
import { GameOpen } from '../game/GameOpen';
import { zustandVon } from '../prompts/fill';
import { PromptButtons } from '../prompts/PromptButtons';
import { TeilLayer } from '../prompts/TeilLayer';
import { AssetList } from './AssetList';
import { SizeCompare } from '../size/SizeCompare';
import { Preview } from './Preview';
import { TeilSicht } from './TeilSicht';
import { useCatalog } from './useCatalog';

type Ansicht = 'vorschau' | 'groesse';
/** Vorschau mit Teil-Auswahl oder Größenvergleich mit dem Monarchen vor einem Biom (B-733). */
const ANSICHTEN = [{ value: 'vorschau', label: 'Vorschau' }, { value: 'groesse', label: 'Größenvergleich' }] as const;

/** Seite Katalog im Bereich Assets (WB1.2, WB1.3): Liste aller Specs, Vorschau des gewählten Assets mit Teil-Auswahl per
 *  Klick und Prompt-Buttons. Nur Ansehen; Änderungen laufen über die MCP-Tools assets_* (Entscheidung 005 › 4, 5). */
export function CatalogPage() {
  const { list, inDiff, prompts, error, loading, open, reload, stand } = useCatalog();
  const [selected, setSelected] = useState('');
  const [teil, setTeil] = useState('');
  const [ansicht, setAnsicht] = useState<Ansicht>('vorschau');
  const [ohne, setOhne] = useState<string[]>([]);
  const choose = (id: string) => {
    setSelected(id);
    setTeil('');
    setOhne([]);
  };
  const eintrag = list.find((a) => a.id === selected);
  // Liste und Vorschau scrollen je für sich (WB2/AC-04)
  return (
    <ListDetail listWidth={300} list={<AssetList list={list} loading={loading} inDiff={inDiff} open={open} onReload={reload} selected={selected} onSelect={choose} />}>
      <Flex direction="column" gap="2">
        <GameOpen />
        {error && <Text color="red">{error}</Text>}
        {selected && <ChipSwitch label="Ansicht" options={ANSICHTEN} value={ansicht} onChange={setAnsicht} />}
        {selected && ansicht === 'groesse' && <SizeCompare key={`${selected}|${stand}`} id={selected} />}
        {selected && ansicht === 'vorschau' && (
          <Preview key={`${selected}|${stand}`} id={selected} ohne={ohne}
            overlay={(frame, spec) => <TeilLayer teile={frame.teile} reihenfolge={spec.teile.map((t) => t.id)} selected={teil} onSelect={setTeil} />}
            footer={(spec) => (
              <>
                <TeilSicht teil={teil} ohne={ohne} onOhne={setOhne} />
                <Prompts spec={spec} eintrag={eintrag} prompts={prompts} teil={teil} />
              </>
            )} />
        )}
        {!selected && <Text color="gray">Asset links wählen</Text>}
      </Flex>
    </ListDetail>
  );
}

function Prompts({ spec, eintrag, prompts, teil }: { spec: Spec; eintrag?: AssetEintrag; prompts: Vorlage[]; teil: string }) {
  const werte = { asset: spec.id, rig: spec.rig, pfad: eintrag?.pfad, zustand: zustandVon(eintrag, spec), teil };
  return (
    <Flex direction="column" gap="3">
      <PromptButtons caption="Prompt fürs Asset" vorlagen={prompts.filter((p) => p.ort === 'asset')} werte={werte} />
      {teil ? (
        <PromptButtons caption={`Teil: ${teil}`} vorlagen={prompts.filter((p) => p.ort === 'teil')} werte={werte} />
      ) : (
        <Text size="1" color="gray">Klick in die Vorschau wählt ein Teil</Text>
      )}
    </Flex>
  );
}
