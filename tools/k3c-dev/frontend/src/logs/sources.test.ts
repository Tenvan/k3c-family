import { describe as group, expect, it } from 'vitest';
import type { ServiceStatus, Source } from '../api';
import { describe, logFor, otherGroups, tagsOf } from './sources';

const svc = (name: string, log = ''): ServiceStatus => ({
  name, log, description: '', port: 1, health: '', state: 'läuft', pid: 0, startedAt: '', restarts: 0, lastError: '',
  changeFile: '', changeAt: '', cpu: 0, memory: 0, seq: 1,
});
const src = (name: string, kind: string): Source => ({ name, kind, state: '', detail: '' });

group('Quellen ordnen', () => {
  const services = [svc('Vite'), svc('Spielserver', 'k3c-server')];
  const sources = [src('Vite', 'service'), src('Spielserver', 'service'), src('vite', 'log'), src('spielserver', 'log'),
    src('k3c-server', 'log'), src('k3c-dev', 'log'), src('check:task:test', 'run'), src('x', 'console')];

  it('hängt Logs an ihren Dienst: strukturiertes Log vor mitgeschriebener Ausgabe', () => {
    expect(logFor(sources[0], services, sources)).toBe('vite');
    expect(logFor(sources[1], services, sources)).toBe('k3c-server');
    expect(logFor(src('Vite', 'service'), services, [])).toBe('');
    expect(logFor(src('k3c-dev', 'log'), services, sources)).toBe('k3c-dev');
    expect(logFor(src('check:task:test', 'run'), services, sources)).toBe('');
  });

  it('listet nur Quellen ohne Dienst, gruppiert', () => {
    const groups = otherGroups(sources, services);
    expect(groups.map((g) => [g.title, g.items.map((x) => x.name)])).toEqual([
      ['Log-Dateien', ['k3c-dev']], ['Läufe (MCP check)', ['check:task:test']], ['Weitere Konsolen', ['x']],
    ]);
    expect(otherGroups(sources.slice(0, 5), services)).toEqual([]);
  });

  it('erklärt jede Quelle in einer Zeile', () => {
    expect(describe(src('k3c-dev', 'log'))).toContain('k3c-dev selbst');
    expect(describe(src('neu', 'log'))).toBe('JSON-Log logs/neu.jsonl');
    expect(describe(src('check:task:test', 'run'))).toContain('task:test');
  });

  it('ordnet Quellen Spiel/Tool und Backend/Client zu', () => {
    expect(tagsOf(src('k3c-client', 'log'))).toEqual(['Spiel', 'Client']);
    expect(tagsOf(src('k3c-dev', 'log'))).toEqual(['Tool']);
    expect(tagsOf(src('check:task:test', 'run'))).toEqual(['Tool']);
    expect(tagsOf(src('neu', 'log'))).toEqual([]);
    expect(tagsOf(src('x', 'console'))).toEqual([]);
  });
});
