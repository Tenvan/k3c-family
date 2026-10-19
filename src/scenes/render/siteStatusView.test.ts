import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { setLanguage } from '../../core/texts';
import type { Site } from '../../model/types';
import { siteStatus, statusKey, statusLine } from './siteStatusView';

beforeEach(() => setLanguage('de'));
afterEach(() => setLanguage('de'));

const site = (over: Partial<Site>): Pick<Site, 'kind' | 'state' | 'upgrade' | 'level' | 'buildProgress'> => ({ kind: 'storage', state: 'unpaid', buildProgress: 0, ...over });

describe('Wartegrund am Bauplatz (AC-02, B-117)', () => {
  it('Bauer fehlt, Gefahr, Material; bezahlt oder gebaut ohne Grund bleibt leer', () => {
    expect(siteStatus(site({ state: 'waitingWorker' }), false)?.reason).toBe('worker');
    expect(siteStatus(site({ state: 'waitingWorker' }), true)?.reason).toBe('danger');
    expect(siteStatus(site({ state: 'waitingMaterial' }), false)?.reason).toBe('material');
    expect(siteStatus(site({ state: 'unpaid' }), false)).toBeNull();
    expect(siteStatus(site({ state: 'built' }), false)).toBeNull();
  });

  it('laufender Bau ohne Bauer zeigt keinen Text, Gefahr schon', () => {
    expect(siteStatus(site({ state: 'waitingWorker', buildProgress: 0.4 }), false)).toBeNull();
    expect(siteStatus(site({ state: 'waitingWorker', buildProgress: 0.4 }), true)?.reason).toBe('danger');
  });

  it('Ausbau eines gebauten Platzes zeigt seinen Grund', () => {
    expect(siteStatus(site({ state: 'built', upgrade: 'waitingWorker' }), false)?.reason).toBe('worker');
  });

  it('Schlüssel ändert sich mit dem Grund; Zeile nennt den Namen', () => {
    const a = siteStatus(site({ state: 'waitingWorker' }), false);
    const b = siteStatus(site({ state: 'waitingWorker' }), true);
    expect(statusKey(a)).not.toBe(statusKey(b));
    expect(statusKey(null)).toBe('');
    expect(statusLine('Lager', a!)).toBe('Lager: wartet auf Bauer');
  });
});
