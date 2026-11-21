import { useState } from 'react';
import { Flex, Text } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import type { AssetEintrag, Spec, Vorlage } from '../api';
import { zustandVon } from '../prompts/fill';
import { PromptButtons } from '../prompts/PromptButtons';
import { TeilLayer } from '../prompts/TeilLayer';
import { SizeCompare } from '../size/SizeCompare';
import { Preview } from './Preview';
import { TeilSicht } from './TeilSicht';

type Ansicht = 'vorschau' | 'groesse';
/** Vorschau mit Teil-Auswahl oder Größenvergleich mit dem Monarchen vor einem Biom (B-733). */
const ANSICHTEN = [{ value: 'vorschau', label: 'Vorschau' }, { value: 'groesse', label: 'Größenvergleich' }] as const;

interface Props {
  /** ID des gewählten Assets; leer = Hinweis */
  selected: string;
  list: readonly AssetEintrag[];
  prompts: readonly Vorlage[];
  /** Stand des Katalogs: ein neuer Stand lädt Vorschau und Größenvergleich neu */
  stand: number | string;
  /** Hinweis ohne Auswahl */
  empty?: string;
}

/** Der Asset-Editor des Katalogs (WB1.2, WB1.3): Vorschau mit Teil-Auswahl per Klick, Größenvergleich und Prompt-Buttons.
 *  Seite Katalog und Reiter Katalog des Welten-Editors (B-1216) zeigen dieselbe Ansicht. Nur Ansehen; Änderungen laufen
 *  über die MCP-Tools assets_* (Entscheidung 005 › 4, 5). Teil und ausgeblendete Teile gelten je Asset: der Aufrufer setzt
 *  `key={selected}`. */
export function AssetDetail({ selected, list, prompts, stand, empty = 'Asset links wählen' }: Props) {
  const [teil, setTeil] = useState('');
  const [ansicht, setAnsicht] = useState<Ansicht>('vorschau');
  const [ohne, setOhne] = useState<string[]>([]);
  const eintrag = list.find((a) => a.id === selected);
  if (!selected) return <Text color="gray">{empty}</Text>;
  return (
    <Flex direction="column" gap="2">
      <ChipSwitch label="Ansicht" options={ANSICHTEN} value={ansicht} onChange={setAnsicht} />
      {ansicht === 'groesse' && <SizeCompare key={`${selected}|${stand}`} id={selected} />}
      {ansicht === 'vorschau' && (
        <Preview key={`${selected}|${stand}`} id={selected} ohne={ohne}
          overlay={(frame, spec) => <TeilLayer teile={frame.teile} reihenfolge={spec.teile.map((t) => t.id)} selected={teil} onSelect={setTeil} />}
          footer={(spec) => (
            <>
              <TeilSicht teil={teil} ohne={ohne} onOhne={setOhne} />
              <Prompts spec={spec} eintrag={eintrag} prompts={prompts} teil={teil} />
            </>
          )} />
      )}
    </Flex>
  );
}

function Prompts({ spec, eintrag, prompts, teil }: { spec: Spec; eintrag?: AssetEintrag; prompts: readonly Vorlage[]; teil: string }) {
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
