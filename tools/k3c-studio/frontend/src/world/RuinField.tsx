import { Button, Text, TextField } from '@webkit/atoms';
import type { PlannedChunk } from '../api';
import { KindPicker } from './KindPicker';
import { ruinOf, slotLabel } from './plan';

const number = (value: string, fallback: number) => (value === '' || Number.isNaN(Number(value)) ? fallback : Number(value));

/** Ruine eines geplanten Chunks (WZ14.7, B-468): ohne Ruine ein Knopf „Ruine“ (erste erlaubte Art, Lage 20), sonst Art aus
 *  der Liste des Backends (data/ruins.json), Lage `at` in Units ab Chunk-Anfang und Entfernen. Ob Art und Lage gelten,
 *  prüft das Backend; der Fehler steht am Chunk. */
export function RuinField({ chunk, kinds, onChange }: {
  chunk: PlannedChunk; kinds: readonly string[]; onChange: (ruin: { kind: string; at: number } | null) => void;
}) {
  const ruin = ruinOf(chunk);
  const where = `Platz ${slotLabel(chunk.slot)}`;
  if (!ruin) {
    return (
      <div className="wd-object">
        <Button variant="soft" disabled={kinds.length === 0} onClick={() => onChange({ kind: kinds[0] ?? 'storage', at: 20 })}>Ruine</Button>
      </div>
    );
  }
  return (
    <div className="wd-object" data-ruin={ruin.kind}>
      <Text size="2">Ruine</Text>
      <KindPicker label={`Ruine ${where}`} value={ruin.kind} options={kinds} onChange={(kind) => onChange({ ...ruin, kind })} />
      <TextField.Root className="wd-slot" type="number" aria-label={`Lage Ruine ${where}`} value={ruin.at} onChange={(e) => onChange({ ...ruin, at: number(e.target.value, ruin.at) })} />
      <Button variant="outline" color="red" aria-label={`Ruine ${where} entfernen`} onClick={() => onChange(null)}>×</Button>
    </div>
  );
}
