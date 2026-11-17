import { describe, expect, it } from 'vitest';
import type { TaskNamespace, TaskRun } from '../api';
import {
  countTasks, describeRun, favoriteTasks, filterNamespaces, pushHistory, splitArgs, upsertRun,
} from './tasks';

const info = (name: string) => t(name);

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

describe('Favoriten und letzte Läufe (Workbench-Spec § 4)', () => {
  const ns = [{ name: 'Workspace', tasks: [info('check'), info('dev')] }, { name: 'golden', tasks: [info('golden:update')] }];

  it('favoriteTasks folgt der Reihenfolge des Baums', () => {
    expect(favoriteTasks(ns, ['golden:update', 'check']).map((t) => t.name)).toEqual(['check', 'golden:update']);
  });

  it('pushHistory merkt nur beendete Läufe, neueste zuerst, ohne Doppel', () => {
    const r = (state: TaskRun['state'], startedAt: string): TaskRun =>
      ({ name: 'check', state, pid: 1, args: [], startedAt, endedAt: '', exitCode: 0, durationMs: 1, reason: '' });
    let h = pushHistory([], r('running', 'a'));
    h = pushHistory(h, r('succeeded', 'a'));
    h = pushHistory(h, r('failed', 'b'));
    h = pushHistory(h, r('failed', 'b'));
    expect(h.map((x) => `${x.startedAt}:${x.state}`)).toEqual(['b:failed', 'a:succeeded']);
  });
});
