import { describe, expect, it } from 'vitest';
import type { Site } from '../model/types';
import { gateStage } from './gateRules';

type S = Pick<Site, 'kind' | 'x' | 'state' | 'level'>;
const wall = (x: number, level?: number, state: Site['state'] = 'built'): S => ({ kind: 'wall', x, state, level });
const gate = (x: number): S => ({ kind: 'gate', x, state: 'built' });

describe('Tor in der Stufe seiner Mauerlinie (GEB4 AC-05)', () => {
  it('links: Stufe der nächsten Mauer weiter innen', () => {
    // Beispiel der Spec: linke Linie 2 hat eine Mauer der Stufe 4 → ihr Tor zeigt Stufe 4
    const sites = [wall(456, 2), wall(436, 4), gate(432), gate(452)];
    expect(gateStage(gate(432), sites, 500)).toBe(4);
    expect(gateStage(gate(452), sites, 500)).toBe(2);
  });

  it('rechts ebenso, Mauern der anderen Seite zählen nicht', () => {
    const sites = [wall(544, 3), wall(564, 5), wall(456, 1)];
    expect(gateStage(gate(548), sites, 500)).toBe(3);
    expect(gateStage(gate(568), sites, 500)).toBe(5);
  });

  it('Mauer der Linie nicht gebaut → Stufe 1, auch wenn eine weiter innen gebaut ist', () => {
    const sites = [wall(544, 3), wall(564, 5, 'waitingWorker')];
    expect(gateStage(gate(568), sites, 500)).toBe(1);
  });

  it('level fehlt → Stufe 1; keine Mauer innen → Stufe 1', () => {
    expect(gateStage(gate(548), [wall(544)], 500)).toBe(1);
    expect(gateStage(gate(548), [wall(560, 4)], 500)).toBe(1);
  });
});
