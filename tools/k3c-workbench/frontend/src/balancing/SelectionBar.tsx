import { useState } from 'react';
import { Button, SelectField, Text } from '@webkit/atoms';
import type { ReportRating } from '../api';
import { Confirm, type ConfirmRequest } from '../git/Confirm';
import type { ListEntry } from './entries';

export const MIN_COMPARE = 2;
export const MAX_COMPARE = 4;

type BulkRating = ReportRating | 'keine';
const RATINGS: { value: BulkRating; label: string }[] = [
  { value: 'gut', label: 'gut' }, { value: 'schlecht', label: 'schlecht' }, { value: 'unklar', label: 'unklar' },
  { value: 'referenz', label: 'Referenz' }, { value: 'keine', label: 'keine Bewertung' },
];

interface Props {
  /** Pfade der gewählten Berichte in der Reihenfolge der Wahl; der erste ist A. */
  checked: string[];
  entries: readonly ListEntry[];
  onCompare: (paths: string[]) => void;
  onRate: (paths: string[], rating: ReportRating) => void;
  onDelete: (paths: string[]) => void;
  onClear: () => void;
}

/** Warum „Vergleichen“ nicht geht; leer = geht. */
export function compareBlock(checked: string[], entries: readonly ListEntry[]): string {
  if (checked.length < MIN_COMPARE) return 'Ab zwei Berichten';
  if (checked.length > MAX_COMPARE) return `Höchstens ${MAX_COMPARE} Berichte`;
  const tuning = entries.some((e) => checked.includes(e.id) && e.type === 'report' && e.report.kind === 'tuning');
  return tuning ? 'Abstimmungen lassen sich nicht vergleichen' : '';
}

/** Leiste unten in der Liste (B-660): Vergleichen, Bewerten, Löschen (eine Rückfrage für alle), Aufheben. */
export function SelectionBar({ checked, entries, onCompare, onRate, onDelete, onClear }: Props) {
  const [confirm, setConfirm] = useState<ConfirmRequest | null>(null);
  if (checked.length === 0) return null;
  const block = compareBlock(checked, entries);
  const n = checked.length;
  const ask = () =>
    setConfirm({ title: n === 1 ? 'Bericht löschen?' : `${n} Berichte löschen?`, text: `${checked.join(', ')} ${n === 1 ? 'wird' : 'werden'} gelöscht (samt Markdown und Bewertung).`, action: 'Löschen', danger: true, run: () => onDelete(checked) });
  return (
    <div className="bal-bulk" role="region" aria-label="Auswahl">
      <Text size="2" weight="bold">{n} gewählt</Text>
      <Button size="1" disabled={block !== ''} title={block || 'A ist der zuerst gewählte Bericht'} onClick={() => onCompare(checked)}>Vergleichen</Button>
      <SelectField size="1" aria-label="Auswahl bewerten" placeholder="Bewerten" value={'' as BulkRating} options={RATINGS} onChange={(v) => onRate(checked, v === 'keine' ? '' : v)} />
      <Button size="1" color="red" variant="soft" onClick={ask}>Löschen</Button>
      <Button size="1" variant="ghost" color="gray" onClick={onClear}>Aufheben</Button>
      <Confirm request={confirm} onClose={() => setConfirm(null)} />
    </div>
  );
}
