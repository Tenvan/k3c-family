import type { BalanceBaseline, BalanceSpec } from '../api';
import { ConfigForm } from './ConfigForm';
import { DataFiles } from './DataFiles';
import { Examples } from './Examples';
import { appendLine, formFromExample } from './help';
import { EMPTY_FORM, formFromBaseline, type BalanceForm, type Preset } from './spec';

interface Props {
  form: BalanceForm;
  onForm: (f: BalanceForm | ((f: BalanceForm) => BalanceForm)) => void;
  presets: Preset[];
  onPresets: (p: Preset[]) => void;
  baselines: BalanceBaseline[];
  onReloadBaselines: () => void;
  onStart: (spec: BalanceSpec) => Promise<void>;
}

/** Das bisherige Formular mit Beispielen und Konfigurationsdateien; bis zum Lauf-Editor (WZ32.4) das Ziel von „Bearbeiten und starten“. */
export function FormEditor({ form, onForm, presets, onPresets, baselines, onReloadBaselines, onStart }: Props) {
  return (
    <ConfigForm form={form} onForm={onForm} presets={presets} onPresets={onPresets} onStart={onStart}>
      <Examples
        baselines={baselines} onOpen={onReloadBaselines}
        onPick={(e) => onForm(formFromExample(e, EMPTY_FORM))} onPickBaseline={(b) => onForm(formFromBaseline(b))}
      />
      <DataFiles
        onVary={(line) => onForm((f) => ({ ...f, vary: appendLine(f.vary, line) }))}
        onParam={(line) => onForm((f) => ({ ...f, tune: { ...f.tune, mode: f.tune.mode || 'search', params: appendLine(f.tune.params, line) } }))}
      />
    </ConfigForm>
  );
}
