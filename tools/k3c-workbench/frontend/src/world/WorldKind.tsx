import { Badge, SelectField } from '@webkit/atoms';
import type { WorldKind } from '../api';

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

export function KindSelect({ kind, onChange }: { kind: WorldKind; onChange: (k: WorldKind) => void }) {
  return (
    <SelectField size="1" aria-label="Art der Welt" value={kind} onChange={(v) => onChange(v as WorldKind)}
      options={[{ value: 'spinoff', label: 'Ableger / Episode' }, { value: 'new', label: 'Komplett neu' }]} />
  );
}
