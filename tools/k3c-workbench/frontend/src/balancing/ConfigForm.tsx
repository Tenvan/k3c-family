import { useState } from 'react';
import { ActionButton, Button, Flex, SelectField, Text, TextArea, TextField } from '@webkit/atoms';
import type { BalanceSpec } from '../api';
import { TuneConfig } from './TuneConfig';
import { dropPreset, EMPTY_FORM, savePreset, specFromForm, type BalanceForm, type Preset } from './spec';

interface Props {
  form: BalanceForm;
  onForm: (f: BalanceForm) => void;
  presets: Preset[];
  onPresets: (p: Preset[]) => void;
  onStart: (spec: BalanceSpec) => Promise<void>;
}

const NUMBER_FIELDS = [
  ['seeds', 'Seeds', 'alle'], ['depth', 'Tiefe', 'Ziel (0 = Wald)'], ['players', 'Spieler', 'Ziel'], ['days', 'Tage', 'Ziel'],
] as const;

/** Konfiguration eines Laufs: leere Felder nehmen den Standard der Ziele (data/balance-targets.json). */
export function ConfigForm({ form, onForm, presets, onPresets, onStart }: Props) {
  const [error, setError] = useState('');
  const [presetName, setPresetName] = useState('');
  const set = (key: keyof BalanceForm) => (v: string) => onForm({ ...form, [key]: v });
  const start = async () => {
    const { spec, error: err } = specFromForm(form);
    setError(err ?? '');
    if (spec) await onStart(spec);
  };
  return (
    <section className="bal-card" aria-label="Konfiguration">
      <Text weight="bold">Lauf konfigurieren</Text>
      <div className="bal-grid">
        {NUMBER_FIELDS.map(([key, label, hint]) => (
          <label key={key} className="bal-field">
            <Text size="1" color="gray">{label}</Text>
            <TextField.Root aria-label={label} placeholder={hint} value={form[key]} onChange={(e) => set(key)(e.target.value)} />
          </label>
        ))}
        <label className="bal-field">
          <Text size="1" color="gray">Bot</Text>
          <TextField.Root aria-label="Bot" placeholder="Ziel" value={form.bot} onChange={(e) => set('bot')(e.target.value)} />
        </label>
        <label className="bal-field">
          <Text size="1" color="gray">Grad</Text>
          <TextField.Root aria-label="Grad" placeholder="Ziel" value={form.grade} onChange={(e) => set('grade')(e.target.value)} />
        </label>
      </div>
      <label className="bal-field">
        <Text size="1" color="gray">Varianten (eine je Zeile: datei.json:pfad=wert)</Text>
        <TextArea aria-label="Varianten" rows={3} value={form.vary} onChange={(e) => set('vary')(e.target.value)} />
      </label>
      <TuneConfig tune={form.tune} onTune={(tune) => onForm({ ...form, tune })} />
      <label className="bal-field">
        <Text size="1" color="gray">Label</Text>
        <TextField.Root aria-label="Label" value={form.label} onChange={(e) => set('label')(e.target.value)} />
      </label>
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
