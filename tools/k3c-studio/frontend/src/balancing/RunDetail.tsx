import { ActionButton, Code, Heading, Progress, StatusBadge, Text, type Tone } from '@webkit/atoms';
import type { BalanceRun } from '../api';
import { SpecList } from './SpecList';
import { isActive, progressOf } from './spec';

const TONES: Record<BalanceRun['state'], Tone> = { wartet: 'neutral', läuft: 'info', fertig: 'ok', abgebrochen: 'warn', fehler: 'error' };
const LAST_LINES = 12;

/** Detail eines laufenden, wartenden oder ohne Bericht beendeten Laufs (B-660): Fortschritt, Phase, letzte Zeilen, Auftrag. */
export function RunDetail({ run, onStop }: { run: BalanceRun; onStop: (id: string) => Promise<void> }) {
  const name = run.label || run.id;
  return (
    <article className="bal-report" aria-label="Lauf">
      <header className="bal-dhead">
        <div className="bal-run-head">
          <Heading as="h2" size="4" className="bal-dtitle">{name}</Heading>
          <StatusBadge tone={TONES[run.state]}>{run.state}</StatusBadge>
          {isActive(run) && <ActionButton size="1" color="red" variant="soft" onClick={() => onStop(run.id)}>Abbrechen</ActionButton>}
        </div>
        <Text size="2" color="gray">Lauf {run.id} · {run.where}{run.verdict ? ` · ${run.verdict}` : ''}</Text>
      </header>
      {isActive(run) && (
        <section className="bal-card" aria-label="Fortschritt">
          <div className="bal-run-head">
            <Text weight="bold" size="2">Fortschritt</Text>
            <Text size="2" color="gray">{run.phase} · {run.done}/{run.total}</Text>
          </div>
          <Progress value={progressOf(run)} aria-label={`Fortschritt ${name}`} />
          <Text size="1" color="gray">Der Bericht öffnet sich hier, sobald der Lauf fertig ist.</Text>
        </section>
      )}
      <section className="bal-card" aria-label="Letzte Zeilen">
        <Text weight="bold" size="2">Letzte Zeilen</Text>
        {run.lines.length === 0 ? <Text size="2" color="gray">Noch keine Ausgabe.</Text> : <Code size="1" className="bal-insight-code">{run.lines.slice(-LAST_LINES).join('\n')}</Code>}
      </section>
      {run.spec && (
        <section className="bal-card" aria-label="Auftrag">
          <Text weight="bold" size="2">Auftrag</Text>
          <SpecList spec={run.spec} />
        </section>
      )}
    </article>
  );
}
