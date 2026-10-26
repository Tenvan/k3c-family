import { useState, type ReactNode } from 'react';
import { ActionButton, Button, Flex, SelectField, Text, TextArea, TextField } from '@webkit/atoms';
import type { BalanceSpec } from '../api';
import { BotField } from './BotField';
import { CHOICES, FIELD_HELP, isMix, mixCount, resizeMix } from './help';
import { HelpField } from './HelpButton';
import { TuneConfig } from './TuneConfig';
import { dropPreset, EMPTY_FORM, savePreset, specFromForm, type BalanceForm, type Preset } from './spec';

interface Props {
  form: BalanceForm;
  onForm: (f: BalanceForm) => void;
  presets: Preset[];
  onPresets: (p: Preset[]) => void;
  onStart: (spec: BalanceSpec) => Promise<void>;
  /** Neben der Überschrift, z. B. der Knopf „Konfigurationsdateien“. */
  children?: ReactNode;
}

const TEXT_FIELDS = [['seeds', 'alle'], ['days', 'Ziel']] as const;
const SELECT_FIELDS = ['depth', 'players', 'bot', 'grade', 'world'] as const;
type SelectKey = Exclude<(typeof SELECT_FIELDS)[number], 'bot'>;

/** Auswahl eines Felds; ein fremder Wert (z. B. aus einem alten Lauf) bleibt als eigener Eintrag sichtbar. */
function choices(key: SelectKey, value: string) {
  const list = CHOICES[key];
  return list.some((o) => o.value === value) ? list : [...list, { value, label: value }];
}

/** Konfiguration eines Laufs: leere Felder nehmen den Standard der Ziele (data/balance-targets.json). */
export function ConfigForm({ form, onForm, presets, onPresets, onStart, children }: Props) {
  const [error, setError] = useState('');
  const [presetName, setPresetName] = useState('');
  const set = (key: keyof BalanceForm) => (v: string) => onForm({ ...form, [key]: v });
  // Bot je Spieler folgt der Spielerzahl (WZ28.12).
  const setPlayers = (v: string) => onForm({ ...form, players: v, bot: isMix(form.bot) ? resizeMix(form.bot, mixCount(v)) : form.bot });
  const start = async () => {
    const { spec, error: err } = specFromForm(form);
    setError(err ?? '');
    if (spec) await onStart(spec);
  };
  return (
    <section className="bal-card" aria-label="Konfiguration">
      <Flex justify="between" align="center" gap="2" wrap="wrap">
        <Text weight="bold">Lauf konfigurieren</Text>
        <Flex gap="2">{children}</Flex>
      </Flex>
      <div className="bal-grid">
        {TEXT_FIELDS.map(([key, hint]) => (
          <HelpField key={key} name={key}>
            <TextField.Root aria-label={FIELD_HELP[key].label} placeholder={hint} value={form[key]} onChange={(e) => set(key)(e.target.value)} />
          </HelpField>
        ))}
        {SELECT_FIELDS.map((key) => key === 'bot' ? <BotField key={key} form={form} onForm={onForm} /> : (
          <HelpField key={key} name={key} hint={false}>
            <SelectField aria-label={FIELD_HELP[key].label} value={form[key]} options={choices(key, form[key])} onChange={key === 'players' ? setPlayers : set(key)} />
          </HelpField>
        ))}
      </div>
      <HelpField name="vary">
        <TextArea aria-label="Varianten" rows={3} placeholder="economy.json:purse.startGold=120" value={form.vary} onChange={(e) => set('vary')(e.target.value)} />
      </HelpField>
      <TuneConfig tune={form.tune} onTune={(tune) => onForm({ ...form, tune })} />
      <HelpField name="label">
        <TextField.Root aria-label="Label" value={form.label} onChange={(e) => set('label')(e.target.value)} />
      </HelpField>
      {error && <p className="svc-error" role="alert">{error}</p>}
      <Flex gap="2" align="center" wrap="wrap">
        <ActionButton onClick={start}>Lauf starten</ActionButton>
        <Button variant="soft" color="gray" onClick={() => { onForm(EMPTY_FORM); setError(''); }}>Zurücksetzen</Button>
      </Flex>
      <Presets presets={presets} form={form} name={presetName} onName={setPresetName} onForm={onForm} onPresets={onPresets} />
    </section>
  );
}

function Presets(p: { presets: Preset[]; form: BalanceForm; name: string; onName: (n: string) => void; onForm: (f: BalanceForm) => void; onPresets: (l: Preset[]) => void }) {
  const [chosen, setChosen] = useState('');
  const options = [{ value: '', label: 'Voreinstellung …' }, ...p.presets.map((x) => ({ value: x.name, label: x.name }))];
  const pick = (name: string) => {
    setChosen(name);
    const hit = p.presets.find((x) => x.name === name);
    if (hit) p.onForm(hit.form);
  };
  return (
    <Flex gap="2" align="center" wrap="wrap" className="bal-presets">
      <SelectField aria-label="Voreinstellung" value={chosen} options={options} onChange={pick} />
      <Button variant="soft" color="gray" disabled={!chosen} onClick={() => { p.onPresets(dropPreset(p.presets, chosen)); setChosen(''); }}>Löschen</Button>
      <TextField.Root aria-label="Name der Voreinstellung" placeholder="Name" value={p.name} onChange={(e) => p.onName(e.target.value)} />
      <Button variant="soft" disabled={p.name.trim() === ''} onClick={() => { p.onPresets(savePreset(p.presets, p.name, p.form)); setChosen(p.name.trim()); p.onName(''); }}>
        Speichern
      </Button>
    </Flex>
  );
}
