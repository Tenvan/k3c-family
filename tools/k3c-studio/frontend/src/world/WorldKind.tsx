import { Badge, SelectField } from '@webkit/atoms';
import type { WorldInfo, WorldKind } from '../api';

// Art der Welt (B-1217): Ableger oder Episode einer Eltern-Welt, oder komplett neu. Die Marke steht neben dem Namen in
// der Liste und im Kopf; der Kopf bietet die Auswahl. Dieselbe Änderung geht per MCP (world_set, Feld kind).

const LABEL: Record<WorldKind, string> = { spinoff: 'Ableger', new: 'Neu' };
const HINT: Record<WorldKind, string> = {
  spinoff: 'Ableger oder Episode: erwartet die Assets und Werte der Eltern-Welt',
  new: 'Komplett neue Welt: erwartet eigenen Look, Einheiten und Werte',
};

export function KindBadge({ kind }: { kind?: WorldKind }) {
  if (!kind) return null;
  return <Badge size="1" color={kind === 'new' ? 'green' : 'blue'} title={HINT[kind]} data-kind={kind}>{LABEL[kind]}</Badge>;
}

/** Eltern-Welt eines Ableger: die anderen Welten mit Datei (die Engine lehnt Zyklen ab). */
export function ParentSelect({ id, parent, worlds, onChange }: { id: string; parent: string; worlds: readonly WorldInfo[]; onChange: (p: string) => void }) {
  const options = worlds.filter((x) => x.id !== id && !x.missing).map((x) => ({ value: x.id, label: x.name }));
  return <SelectField size="1" aria-label="Eltern-Welt wählen" value={parent} onChange={onChange} options={options} />;
}

export function KindSelect({ kind, onChange }: { kind: WorldKind; onChange: (k: WorldKind) => void }) {
  return (
    <SelectField size="1" aria-label="Art der Welt" value={kind} onChange={(v) => onChange(v as WorldKind)}
      options={[{ value: 'spinoff', label: 'Ableger / Episode' }, { value: 'new', label: 'Komplett neu' }]} />
  );
}
