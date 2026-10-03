import { describe, expect, it } from 'vitest';
import type { ServiceStatus } from '../api';
import { applyStatus, badgeFor, buttonsFor, levelShares, metricsOf, orderLine } from './tables';

const svc = (over: Partial<ServiceStatus> = {}): ServiceStatus => ({
  name: 'Vite', description: '', port: 5173, health: 'http://127.0.0.1:5173/', log: '', state: 'gestoppt', pid: 0, startedAt: '',
  restarts: 0, lastError: '', cpu: 0, memory: 0, seq: 1, ...over,
});

describe('Dienste-Tabellen', () => {
  it('Zustand → Badge, unbekannt neutral', () => {
    expect(badgeFor('läuft')).toEqual({ label: 'Läuft', tone: 'ok' });
    expect(badgeFor('startet').tone).toBe('info');
    expect(badgeFor('übernommen').tone).toBe('warn');
    expect(badgeFor('stoppt').tone).toBe('neutral');
    expect(badgeFor('gestoppt').tone).toBe('neutral');
    expect(badgeFor('fehlgeschlagen')).toEqual({ label: 'Fehlgeschlagen', tone: 'error' });
    expect(badgeFor('pausiert')).toEqual({ label: 'pausiert', tone: 'neutral' });
  });

  it('Zustand → Knöpfe (B-068 › Beispiele)', () => {
    expect(buttonsFor('läuft')).toEqual(['stop', 'restart']);
    expect(buttonsFor('gestoppt')).toEqual(['start']);
    expect(buttonsFor('fehlgeschlagen')).toEqual(['start']);
    expect(buttonsFor('startet')).toEqual(['stop']);
    expect(buttonsFor('übernommen')).toEqual(['stop']);
    expect(buttonsFor('stoppt')).toEqual([]);
    expect(buttonsFor('pausiert')).toEqual([]);
  });

  it('Ereignisse: neuere übernehmen, veraltete und unbekannte verwerfen', () => {
    const list = [svc({ seq: 5 }), svc({ name: 'Spielserver', port: 8080, seq: 2 })];
    expect(applyStatus(list, svc({ seq: 4, state: 'läuft' }))).toBe(list);
    expect(applyStatus(list, svc({ seq: 5, state: 'läuft' }))).toBe(list);
    expect(applyStatus(list, svc({ name: 'Weg', seq: 9 }))).toBe(list);
    const next = applyStatus(list, svc({ seq: 6, state: 'läuft' }));
    expect(next[0].state).toBe('läuft');
    expect(next[1]).toBe(list[1]);
  });

  it('Stopp-Reihenfolge rückwärts', () => {
    expect(orderLine(['Vite', 'Spielserver'])).toBe(
      '„Alle starten“ startet parallel · „Alle stoppen“ rückwärts: Spielserver → Vite',
    );
  });

  it('Level-Anteile: WARN und ERROR immer', () => {
    const shares = levelShares({ counts: { INFO: 3, WARN: 1 }, total: 4, budgetHit: false });
    expect(shares).toEqual([
      { level: 'INFO', count: 3, percent: 75 },
      { level: 'WARN', count: 1, percent: 25 },
      { level: 'ERROR', count: 0, percent: 0 },
    ]);
    expect(levelShares({ counts: {}, total: 0, budgetHit: false }).map((s) => s.percent)).toEqual([0, 0]);
  });

  it('Metriken deutsch, `–` solange nichts läuft', () => {
    expect(metricsOf(svc(), 0)).toEqual({ pid: '–', cpu: '–', memory: '–', uptime: '–' });
    const now = Date.parse('2026-09-30T12:00:00Z');
    const running = svc({ state: 'läuft', pid: 41232, cpu: 3.14, memory: 480 * 1024 * 1024,
      startedAt: new Date(now - (2 * 60 + 13) * 60_000).toISOString() });
    expect(metricsOf(running, now)).toEqual({ pid: '41232', cpu: '3,1 %', memory: '480,0 MB', uptime: '2 h 13 min' });
    expect(metricsOf(svc({ state: 'läuft', pid: 7, startedAt: '0001-01-01T00:00:00Z' }), now).uptime).toBe('–');
  });
});
