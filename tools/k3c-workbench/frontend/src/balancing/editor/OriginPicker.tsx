import { useState } from 'react';
import { Button, Flex, SelectField, Text } from '@webkit/atoms';
import type { BalanceBaseline } from '../../api';
import { EXAMPLES } from '../help';
import { dropPreset, type Preset } from '../spec';
import { Card } from './Sections';
import { Choice, type ChoiceOption } from './Choice';
import { baselineForm, exampleForm, normalizeForm, type EditorForm, type Origin, type OriginKind } from './variantForm';

interface Props {
  origin: Origin;
  /** Formular des Berichts, mit dem der Editor geöffnet wurde (Ausgangspunkt „Bericht“). */
  report: EditorForm;
  baselines: BalanceBaseline[];
  presets: Preset[];
  onPresets: (p: Preset[]) => void;
  /** Der Editor übernimmt alle Felder dieses Formulars. */
  onLoad: (form: EditorForm | undefined, origin: Origin) => void;
}

const HINT: Record<OriginKind, string> = {
  leer: 'Alle Felder auf der Vorgabe der Ziele.',
  beispiel: 'Beispiel wählen:',
  basis: 'Basis-Lauf wählen:',
  voreinstellung: 'Voreinstellung wählen:',
  bericht: '',
};

function options(kind: OriginKind, baselines: BalanceBaseline[], presets: Preset[]): { value: string; label: string }[] {
  if (kind === 'beispiel') return EXAMPLES.map((e) => ({ value: e.name, label: e.name }));
  if (kind === 'basis') return baselines.map((b) => ({ value: b.name, label: `${b.name} · ${b.run}` }));
  if (kind === 'voreinstellung') return presets.map((p) => ({ value: p.name, label: p.name }));
  return [];
}

/** Zusatztext zum gewählten Eintrag: Beschreibung des Beispiels. */
const describe = (kind: OriginKind, name: string) => (kind === 'beispiel' ? EXAMPLES.find((e) => e.name === name)?.text.replace(/`/g, '') ?? '' : '');

/** Ausgangspunkt (B-661): Umschalter Leer, Beispiel, Basis-Lauf, Voreinstellung, Bericht; die Wahl übernimmt alle Felder. */
export function OriginPicker({ origin, report, baselines, presets, onPresets, onLoad }: Props) {
  const [kind, setKind] = useState<OriginKind>(origin.kind);
  const [picked, setPicked] = useState(origin.kind === 'beispiel' || origin.kind === 'basis' || origin.kind === 'voreinstellung' ? origin.label : '');
  const kinds: ChoiceOption[] = [
    { value: 'leer', label: 'Leer' }, { value: 'beispiel', label: 'Beispiel' }, { value: 'basis', label: 'Basis-Lauf' },
    { value: 'voreinstellung', label: 'Voreinstellung' },
    { value: 'bericht', label: 'Bericht', disabled: origin.kind !== 'bericht', title: origin.kind === 'bericht' ? undefined : 'Nur beim Öffnen aus einem Bericht' },
  ];
  const choose = (k: string) => {
    const next = k as OriginKind;
    setKind(next);
    setPicked('');
    if (next === 'leer') onLoad(undefined, { kind: 'leer', label: '' });
    if (next === 'bericht') onLoad(report, origin);
  };
  const pick = (name: string) => {
    setPicked(name);
    const form = kind === 'beispiel' ? exampleForm(name) : kind === 'basis' ? baselineFor(baselines, name) : normalizeForm(presets.find((p) => p.name === name)?.form);
    if (form) onLoad(form, { kind, label: name });
  };
  const list = options(kind, baselines, presets);
  return (
    <Card title="Ausgangspunkt" summary="übernimmt alle Felder, danach frei änderbar">
      <Flex gap="3" align="center" wrap="wrap">
        <Choice label="Ausgangspunkt" value={kind} options={kinds} onChange={choose} />
        {kind === 'bericht' && <Text size="2">{origin.label}</Text>}
        {list.length > 0 && <SelectField aria-label="Eintrag des Ausgangspunkts" placeholder="wählen …" value={picked} options={list} onChange={pick} />}
        {kind === 'voreinstellung' && picked !== '' && (
          <Button size="1" variant="soft" color="gray" onClick={() => { onPresets(dropPreset(presets, picked)); setPicked(''); }}>Löschen</Button>
        )}
        {kind !== 'bericht' && list.length === 0 && <Text size="2" color="gray">{kind === 'leer' ? HINT.leer : 'Noch nichts gespeichert.'}</Text>}
      </Flex>
      {describe(kind, picked) && <Text size="1" color="gray">{describe(kind, picked)}</Text>}
    </Card>
  );
}

const baselineFor = (baselines: BalanceBaseline[], name: string) => {
  const b = baselines.find((x) => x.name === name);
  return b ? baselineForm(b) : undefined;
};
