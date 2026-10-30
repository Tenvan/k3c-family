import { Flex, Text, TextField } from '@webkit/atoms';
import type { TuneMode } from '../../api';
import { HelpField } from '../HelpButton';
import type { TuneForm } from '../tune';
import { Choice } from './Choice';

interface Props {
  tune: TuneForm;
  onTune: (t: TuneForm) => void;
  /** Zahl der Bereiche (1 oder 2). */
  ranges: number;
}

const MODES = [
  { value: 'search', label: 'Suche (Budget)' },
  { value: 'sweep', label: 'Sweep (Raster)' },
];

/** Abstimm-Kasten (B-661): erscheint ab einem Bereich; Modus Suche oder Sweep, dazu Budget bzw. Schritte. */
export function TuneBox({ tune, onTune, ranges }: Props) {
  const mode: TuneMode = tune.mode || 'search';
  return (
    <div className="bal-ed-tune" role="group" aria-label="Abstimmung">
      <Text size="2">
        <strong>Abstimmung:</strong> {ranges} {ranges === 1 ? 'Bereich' : 'Bereiche'}. Der Lauf sucht Werte, die alle Korridore treffen, und schlägt einen Diff vor; data/ bleibt unverändert.
      </Text>
      <Flex gap="4" wrap="wrap" align="end">
        <HelpField name="mode" hint={false}>
          <Choice label="Abstimm-Modus" value={mode} options={MODES} onChange={(v) => onTune({ ...tune, mode: v as TuneMode })} />
        </HelpField>
        {mode === 'search' ? (
          <HelpField name="budget" hint={false}>
            <TextField.Root aria-label="Budget" placeholder="40" value={tune.budget} onChange={(e) => onTune({ ...tune, budget: e.target.value })} />
          </HelpField>
        ) : (
          <HelpField name="steps" hint={false}>
            <TextField.Root aria-label="Schritte" placeholder="5" value={tune.steps} onChange={(e) => onTune({ ...tune, steps: e.target.value })} />
          </HelpField>
        )}
      </Flex>
    </div>
  );
}
