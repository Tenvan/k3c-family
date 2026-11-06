import { useState } from 'react';
import { Flex, Text } from '@webkit/atoms';
import type { AssetEintrag, Spec, Vorlage } from '../api';
import { GameOpen } from '../game/GameOpen';
import { zustandVon } from '../prompts/fill';
import { PromptButtons } from '../prompts/PromptButtons';
import { TeilLayer } from '../prompts/TeilLayer';
import { AssetList } from './AssetList';
import { Preview } from './Preview';
import { useCatalog } from './useCatalog';

/** Seite Katalog im Bereich Assets (WB1.2, WB1.3): Liste aller Specs, Vorschau des gewählten Assets mit Teil-Auswahl per
 *  Klick und Prompt-Buttons. Nur Ansehen; Änderungen laufen über die MCP-Tools assets_* (Entscheidung 005 › 4, 5). */
export function CatalogPage() {
  const { list, inDiff, prompts, error } = useCatalog();
  const [selected, setSelected] = useState('');
  const [teil, setTeil] = useState('');
  const choose = (id: string) => {
    setSelected(id);
    setTeil('');
  };
  const eintrag = list.find((a) => a.id === selected);
  return (
    <Flex gap="4" p="4" className="as-page">
      <AssetList list={list} inDiff={inDiff} selected={selected} onSelect={choose} />
      <Flex direction="column" gap="2" className="as-detail">
        <GameOpen />
        {error && <Text color="red">{error}</Text>}
        {selected ? (
          <Preview key={selected} id={selected}
            overlay={(frame, spec) => <TeilLayer teile={frame.teile} reihenfolge={spec.teile.map((t) => t.id)} selected={teil} onSelect={setTeil} />}
            footer={(spec) => <Prompts spec={spec} eintrag={eintrag} prompts={prompts} teil={teil} />} />
        ) : (
          <Text color="gray">Asset links wählen</Text>
        )}
      </Flex>
    </Flex>
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
