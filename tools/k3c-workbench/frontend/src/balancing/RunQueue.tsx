import { ActionButton, Button, Progress, StatusBadge, Text, type Tone } from '@webkit/atoms';
import type { BalanceRun } from '../api';
import { isActive, progressOf } from './spec';

const TONES: Record<BalanceRun['state'], Tone> = { wartet: 'neutral', läuft: 'info', fertig: 'ok', abgebrochen: 'warn', fehler: 'error' };

interface Props {
  runs: BalanceRun[];
  onStop: (id: string) => Promise<void>;
  onOpen: (path: string) => void;
  onReuse: (run: BalanceRun) => void;
}

/** Warteschlange und Läufe mit Live-Status: Fortschritt `done/total`, Abbruch je wartendem oder laufendem Lauf. */
export function RunQueue({ runs, onStop, onOpen, onReuse }: Props) {
  return (
    <section className="bal-card" aria-label="Läufe">
      <Text weight="bold">Läufe</Text>
      {runs.length === 0 && <Text color="gray" size="2">Noch kein Lauf in dieser Sitzung.</Text>}
      {runs.map((r) => (
        <div key={r.id} className="bal-run" data-state={r.state}>
          <div className="bal-run-head">
            <Text weight="medium">{r.label || r.id}</Text>
            <StatusBadge tone={TONES[r.state] ?? 'neutral'}>{r.state}</StatusBadge>
            {r.verdict && <Text size="2" color="gray">{r.verdict}</Text>}
          </div>
          {isActive(r) && (
            <div className="bal-run-progress">
              <Progress value={progressOf(r)} aria-label={`Fortschritt ${r.label || r.id}`} />
              <Text size="1" color="gray">{r.phase} · {r.done}/{r.total}</Text>
            </div>
          )}
          <div className="bal-run-actions">
            {isActive(r) && <ActionButton size="1" color="red" variant="soft" onClick={() => onStop(r.id)}>Abbrechen</ActionButton>}
            {r.report && <Button size="1" variant="soft" onClick={() => onOpen(r.report)}>Bericht öffnen</Button>}
            {r.spec && <Button size="1" variant="ghost" onClick={() => onReuse(r)}>Konfiguration übernehmen</Button>}
          </div>
        </div>
      ))}
    </section>
  );
}
