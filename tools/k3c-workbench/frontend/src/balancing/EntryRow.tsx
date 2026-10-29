import { Button, Checkbox, Progress, StatusBadge, Text, type Tone } from '@webkit/atoms';
import { entryKind, entryLabel, entryLine2, entryMeta, entryRating, entryStatus, type EntryStatus, type ListEntry } from './entries';
import { progressOf } from './spec';

const TONES: Record<EntryStatus, Tone> = { wartet: 'neutral', läuft: 'info', Pass: 'ok', Fail: 'error', abgebrochen: 'warn', fehler: 'error' };
const RATING_TEXT = { gut: 'gut', schlecht: 'schlecht', unklar: 'unklar', referenz: 'Referenz', '': '' } as const;

interface Props {
  entry: ListEntry;
  now: Date;
  selected: boolean;
  checked: boolean;
  onSelect: (id: string) => void;
  onCheck: (id: string, on: boolean) => void;
}

/** Eine Zeile der Liste (B-660): Checkbox und Name sind die einzigen Bedienelemente. */
export function EntryRow({ entry, now, selected, checked, onSelect, onCheck }: Props) {
  const label = entryLabel(entry, now);
  const status = entryStatus(entry);
  const rating = entryRating(entry);
  const run = entry.type === 'run' ? entry.run : null;
  return (
    <div className="bal-item" data-id={entry.id} data-on={selected ? 'true' : undefined} data-state={status} data-kind={entryKind(entry)} data-rating={rating || undefined}>
      <Checkbox size="1" aria-label={`Auswählen: ${label}`} checked={checked} disabled={run !== null} onCheckedChange={(v) => onCheck(entry.id, v === true)} />
      <Button size="1" variant="ghost" color="gray" highContrast className="bal-item-name" title={label} onClick={() => onSelect(entry.id)}>{label}</Button>
      <StatusBadge tone={TONES[status]}>{status}</StatusBadge>
      <Text size="1" color="gray" className="bal-item-meta">{entryMeta(entry)}</Text>
      {run && status === 'läuft' && (
        <div className="bal-item-progress">
          <Progress size="1" value={progressOf(run)} aria-label={`Fortschritt ${label}`} />
          <Text size="1" color="gray">{run.phase} · {run.done}/{run.total}</Text>
        </div>
      )}
      <Text size="1" color="gray" className="bal-item-meta">
        {entryLine2(entry, now)}
        {rating && <span className="bal-rating"><span className="bal-dot" aria-hidden />{RATING_TEXT[rating]}</span>}
      </Text>
    </div>
  );
}
