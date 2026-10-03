import { describe, expect, it } from 'vitest';
import type { TaskNamespace, TaskRun } from '../api';
import { countTasks, describeRun, filterNamespaces, splitArgs, upsertRun } from './tasks';

const t = (name: string, desc = '') => ({ name, namespace: '', leaf: '', desc, summary: '', aliases: [], file: '', line: 0 });
const tree: TaskNamespace[] = [
  { name: 'Workspace', tasks: [t('check', 'alles prüfen'), t('test')] },
  { name: 'dev', tasks: [t('dev:frontend', 'Frontend bauen')] },
];
const run = (name: string, state: TaskRun['state'], exitCode = 0): TaskRun => ({
  name, state, pid: 1, args: [], startedAt: '', endedAt: '', exitCode, durationMs: 0, reason: '',
});

describe('tasks', () => {
  it('filtert nach Name oder Beschreibung, leere Namensräume fallen weg', () => {
    expect(countTasks(filterNamespaces(tree, ' PRÜFEN '))).toBe(1);
    expect(filterNamespaces(tree, 'frontend').map((n) => n.name)).toEqual(['dev']);
    expect(filterNamespaces(tree, '')).toBe(tree);
  });

  it('trennt Argumente mit Anführungszeichen', () => {
    expect(splitArgs('-run "Foo Bar" --x')).toEqual(['-run', 'Foo Bar', '--x']);
    expect(splitArgs('')).toEqual([]);
  });

  it('beschreibt Läufe', () => {
    expect(describeRun(undefined).label).toBe('bereit');
    expect(describeRun(run('x', 'failed', 2)).label).toBe('fehlgeschlagen (Exit 2)');
  });

  it('ersetzt Läufe nach Name', () => {
    const list = upsertRun(upsertRun([], run('b', 'running')), run('a', 'running'));
    expect(upsertRun(list, run('b', 'succeeded')).map((r) => `${r.name}:${r.state}`)).toEqual(['a:running', 'b:succeeded']);
  });
});
