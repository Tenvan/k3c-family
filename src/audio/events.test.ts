import { describe, expect, it } from 'vitest';
import { attenuation, cueFor, EDGE_GAIN, type Listener } from './events';

const p1: Listener = { center: 20, span: 20 }; // sieht 10..30
const p2: Listener = { center: 100, span: 20 }; // sieht 90..110

describe('attenuation (AC-06)', () => {
  it('Quelle im Bereich von Spieler 1: laut', () => expect(attenuation(25, [p1, p2], false)).toBe(1));
  it('Quelle im Bereich von Spieler 2: ebenfalls laut (jeder hört seinen Bereich)', () => expect(attenuation(95, [p1, p2], false)).toBe(1));
  it('außerhalb beider Bereiche: leiser, mit Abstand weiter fallend bis 0', () => {
    const near = attenuation(32, [p1, p2], false);
    expect(near).toBeLessThan(1);
    expect(near).toBeLessThanOrEqual(EDGE_GAIN);
    expect(attenuation(50, [p1, p2], false)).toBeLessThan(near);
    expect(attenuation(60, [p1, p2], false)).toBe(0);
  });
  it('Warnungen sind global laut', () => expect(attenuation(500, [p1, p2], true)).toBe(1));
  it('ohne Listener oder Ort: keine Dämpfung', () => {
    expect(attenuation(500, [], false)).toBe(1);
    expect(attenuation(undefined, [p1], false)).toBe(1);
  });
  it('1 Spieler: im sichtbaren Bereich keine Dämpfung', () => expect(attenuation(10, [p1], false)).toBe(1));
});

describe('cueFor', () => {
  it('built → Demo-Ton positionsgebunden, wave/castleFallen → global, sonst stumm', () => {
    expect(cueFor({ type: 'built', kind: 'wall' })).toEqual({ sprite: 'coin', global: false });
    expect(cueFor({ type: 'wave', wave: 1, count: 3 })?.global).toBe(true);
    expect(cueFor({ type: 'castleFallen' })?.global).toBe(true);
    expect(cueFor({ type: 'dusk' })).toBeNull();
  });
});
