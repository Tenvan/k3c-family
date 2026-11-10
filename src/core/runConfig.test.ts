import { afterEach, describe, expect, it } from 'vitest';
import { patchRun, resetRun, runParam, runSearch, setRun } from './runConfig';

afterEach(() => resetRun());

describe('RunConfig (BD11.3)', () => {
  it('ohne URL und ohne Menü-Auswahl ist die Konfiguration leer', () => {
    expect(runSearch()).toBe('');
    expect(runParam('dev')).toBeNull();
  });

  it('setRun ersetzt alles durch genau diese Werte', () => {
    setRun({ dev: '1', autostart: '1', save: 'stress-wald' });
    expect(runSearch()).toBe('?dev=1&autostart=1&save=stress-wald');
    setRun({ dev: '1' });
    expect(runSearch()).toBe('?dev=1');
    expect(runParam('autostart')).toBeNull();
  });

  it('patchRun ändert einzelne Werte, null entfernt', () => {
    setRun({ dev: '1', seed: 'a' });
    patchRun({ seed: 'b', gfx: '0' });
    expect(runSearch()).toBe('?dev=1&seed=b&gfx=0');
    patchRun({ dev: null });
    expect(runSearch()).toBe('?seed=b&gfx=0');
  });

  it('setRun mit null-Einträgen lässt sie weg', () => {
    setRun({ dev: '1', room: null });
    expect(runSearch()).toBe('?dev=1');
  });

  it('Werte werden für die Abfrage kodiert und gelesen', () => {
    setRun({ seed: 'a b&c' });
    expect(runParam('seed')).toBe('a b&c');
    expect(runSearch()).toBe('?seed=a+b%26c');
  });

  it('resetRun geht zurück auf die URL der Seite (hier leer)', () => {
    setRun({ dev: '1' });
    resetRun();
    expect(runSearch()).toBe('');
  });
});
