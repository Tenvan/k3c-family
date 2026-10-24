import { SelectField, Text, TextArea, TextField } from '@webkit/atoms';
import type { TuneMode } from '../api';
import { HelpField } from './HelpButton';
import type { TuneForm } from './tune';

const MODES = [
  { value: '', label: 'Aus' },
  { value: 'sweep', label: 'Sweep (volles Raster)' },
  { value: 'search', label: 'Suche (Budget)' },
];

/** Bereich „Abstimmen“: 1–2 Parameter `datei.json:pfad=min..max`, Modus, Schritte, Budget (leer = Vorgabe der CLI: 5 und 40). */
export function TuneConfig({ tune, onTune }: { tune: TuneForm; onTune: (t: TuneForm) => void }) {
  const set = (key: 'params' | 'steps' | 'budget') => (v: string) => onTune({ ...tune, [key]: v });
  return (
    <fieldset className="bal-tune" aria-label="Abstimmen">
      <Text size="2" weight="medium">Abstimmen</Text>
      <HelpField name="mode" hint={false}>
        <SelectField aria-label="Abstimm-Modus" value={tune.mode} options={MODES} onChange={(v) => onTune({ ...tune, mode: v as '' | TuneMode })} />
      </HelpField>
      {tune.mode !== '' && (
        <>
          <HelpField name="params">
            <TextArea aria-label="Parameter" rows={2} placeholder="buildings.json:wall.cost.gold=2..6" value={tune.params} onChange={(e) => set('params')(e.target.value)} />
          </HelpField>
          <div className="bal-grid">
            <HelpField name="steps">
              <TextField.Root aria-label="Schritte" placeholder="5" value={tune.steps} onChange={(e) => set('steps')(e.target.value)} />
            </HelpField>
            <HelpField name="budget">
              <TextField.Root aria-label="Budget" placeholder="40" value={tune.budget} onChange={(e) => set('budget')(e.target.value)} />
            </HelpField>
          </div>
        </>
      )}
    </fieldset>
  );
}
