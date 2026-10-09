import { Flex, Heading, Text, TextField } from '@radix-ui/themes';
import { useState } from 'react';
import { backend, type TaskInfo, type TaskRun } from '../api';
import { errorText } from '../lib/errors';
import { formatDuration } from '../lib/format';
import { ConsoleView } from '../logs/ConsoleView';
import { ActionButton, NoticeCard, StatusBadge } from '../ui/parts';
import { describeRun, describeState, splitArgs } from './tasks';

/** Ausgabe des gewählten Tasks: Kopf mit Zustand, Argumentfeld (Enter startet), Start/Stopp, Konsole, letzte Läufe. */
export function TaskDetail({ task, run, history }: { task: TaskInfo; run?: TaskRun; history: TaskRun[] }) {
  const [args, setArgs] = useState('');
  const [error, setError] = useState('');
  const running = run?.state === 'running';
  const badge = describeRun(run);
  const act = async (fn: () => Promise<unknown>) => {
    setError('');
    try {
      await fn();
    } catch (e) {
      setError(errorText(e));
    }
  };
  const start = () => act(() => backend.taskStart(task.name, splitArgs(args)));
  return (
    <div className="tk-detail-body">
      <Flex align="center" gap="3" wrap="wrap">
        <Heading size="4">{task.name}</Heading>
        <StatusBadge tone={badge.tone}>{badge.label}</StatusBadge>
        {run && !running && run.durationMs > 0 && <Text size="2" color="gray">{formatDuration(run.durationMs)}</Text>}
        {running && <Text size="2" color="gray">PID {run?.pid}</Text>}
      </Flex>
      {task.desc && <Text as="p" size="2" color="gray">{task.desc} · {task.file}:{task.line}</Text>}
      <Flex gap="2" align="center">
        <TextField.Root className="tk-args" size="2" placeholder="Argumente nach -- (z. B. -run Foo), Enter startet" value={args}
          disabled={running} onChange={(e) => setArgs(e.target.value)}
          onKeyDown={(e) => { if (e.key === 'Enter' && !running) void start(); }} />
        {running
          ? <ActionButton color="red" onClick={() => act(() => backend.taskStop(task.name))}>Stoppen</ActionButton>
          : <ActionButton onClick={start}>Starten</ActionButton>}
      </Flex>
      {error && <p className="svc-error">{error}</p>}
      {run
        ? <ConsoleView name={`task:${task.name}`} />
        : <NoticeCard title="Noch nicht gelaufen" tone="neutral">Die Ausgabe erscheint hier live, sobald der Task gestartet ist.</NoticeCard>}
      <RecentRuns runs={history.filter((r) => r.name === task.name)} />
    </div>
  );
}

function RecentRuns({ runs }: { runs: TaskRun[] }) {
  if (runs.length === 0) return null;
  return (
    <details className="tk-history">
      <summary>Letzte Läufe ({runs.length})</summary>
      <table>
        <tbody>
          {runs.map((r) => {
            const b = describeState(r.state, r.exitCode, r.reason);
            return (
              <tr key={r.startedAt}>
                <td>{new Date(r.startedAt).toLocaleTimeString('de-DE')}</td>
                <td><StatusBadge tone={b.tone}>{b.label}</StatusBadge></td>
                <td>{formatDuration(r.durationMs)}</td>
                <td className="tk-history-args">{r.args.length > 0 ? `-- ${r.args.join(' ')}` : '—'}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </details>
  );
}
