import { SelectField } from '@webkit/atoms';
import type { PickKey, PicksView } from '../api';

// Wetter, Reittier und Einsiedler je Welt (SI7.1, B-1003): drei Auswahlfelder mit „keins“. Die Zuordnung steht in den
// Katalog-Dateien; ein Bestandswert, den der Katalog nicht kennt, bleibt sichtbar und ist markiert.

const LABEL: Record<PickKey, string> = { weather: 'Wetter', mount: 'Reittier', hermit: 'Einsiedler' };
const ORDER: PickKey[] = ['weather', 'mount', 'hermit'];

export function PickSelects({ picks, onChange }: { picks: PicksView; onChange: (key: PickKey, value: string) => void }) {
  return (
    <>
      {ORDER.map((key) => {
        const value = picks.values[key] ?? '';
        const known = picks.options[key] ?? [];
        const options = [{ value: '', label: 'keins' }, ...known.map((v) => ({ value: v, label: v }))];
        if (value && !known.includes(value)) options.push({ value, label: `${value} (unbekannt)` });
        return <SelectField key={key} size="1" aria-label={`${LABEL[key]} wählen`} value={value} onChange={(v) => onChange(key, v)} options={options} />;
      })}
    </>
  );
}
