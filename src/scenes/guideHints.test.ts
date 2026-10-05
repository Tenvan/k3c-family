import { describe, expect, it } from 'vitest';
import type { Coin, CycleInfo, Site, Troop } from '../model/types';
import { nextGuide, stepGuide, type GuideHint, type GuideWorld } from './guideHints';

const coin = (id: number, x: number) => ({ id, x }) as Coin;
const site = (id: number, x: number, state: Site['state'] = 'unpaid') => ({ id, x, state }) as Site;
const troop = (id: number, x: number, kind: Troop['kind'] = 'vagrant') => ({ id, x, kind }) as Troop;
const cycle = (phase: CycleInfo['phase']) => ({ phase }) as CycleInfo;

function world(over: Partial<GuideWorld> = {}): GuideWorld {
  return { coins: [], sites: [], troops: [], cycle: cycle('day'), ...over };
}

const at = (x: number) => ({ x, respawnIn: 0 });
const none = new Set<string>();

describe('nextGuide', () => {
  it('Reihenfolge: Münze vor Bauplatz vor Nacht vor Bauer', () => {
    const w = world({ coins: [coin(1, 12)], sites: [site(2, 8)], troops: [troop(3, 11)], cycle: cycle('dusk') });
    expect(nextGuide(w, at(10), none)).toEqual({ id: 'coin', x: 12, target: 1 });
    expect(nextGuide(w, at(10), new Set(['coin']))).toEqual({ id: 'pay', x: 8, target: 2 });
    expect(nextGuide(w, at(10), new Set(['coin', 'pay']))).toEqual({ id: 'dusk', x: 10, target: null });
    expect(nextGuide(w, at(10), new Set(['coin', 'pay', 'dusk']))).toEqual({ id: 'recruit', x: 11, target: 3 });
    expect(nextGuide(w, at(10), new Set(['coin', 'pay', 'dusk', 'recruit']))).toBeNull();
  });

  it('nur Objekte in Reichweite des Preisschilds, das nächste zuerst; gebaute Plätze und Bauern nicht', () => {
    expect(nextGuide(world({ coins: [coin(1, 40)] }), at(10), none)).toBeNull();
    expect(nextGuide(world({ coins: [coin(1, 14), coin(2, 9)] }), at(10), none)?.target).toBe(2);
    expect(nextGuide(world({ sites: [site(1, 10, 'built')], troops: [troop(2, 10, 'peasant')] }), at(10), none)).toBeNull();
  });

  it('Nacht naht nur in der Dämmerung laut Snapshot', () => {
    expect(nextGuide(world({ cycle: cycle('day') }), at(5), none)).toBeNull();
    expect(nextGuide(world({ cycle: cycle('night') }), at(5), none)).toBeNull();
    expect(nextGuide(world({ cycle: cycle('dusk') }), at(5), none)?.id).toBe('dusk');
  });

  it('ein gefallener Monarch bekommt keinen Hinweis', () => {
    expect(nextGuide(world({ coins: [coin(1, 10)] }), { x: 10, respawnIn: 3 }, none)).toBeNull();
  });
});

describe('stepGuide', () => {
  const run = (prev: GuideHint | null, w: GuideWorld, seen: Set<string>) => stepGuide(prev, w, at(10), seen, (id) => seen.add(id));

  it('nach der Handlung weg und als gesehen gemerkt; gesehener erscheint nicht erneut', () => {
    const seen = new Set<string>();
    const shown = run(null, world({ coins: [coin(1, 11)] }), seen);
    expect(shown?.id).toBe('coin');
    expect(run(shown, world(), seen)).toBeNull(); // aufgehoben
    expect(seen.has('coin')).toBe(true);
    expect(run(null, world({ coins: [coin(5, 10)] }), seen)).toBeNull(); // neue Münze: kein zweites Mal
  });

  it('Bauplatz bezahlt, Nacht da, Landstreicher ist Bauer → erledigt', () => {
    const seen = new Set<string>();
    const pay = run(null, world({ sites: [site(2, 9)] }), seen);
    expect(run(pay, world({ sites: [site(2, 9, 'waitingWorker')] }), seen)).toBeNull();
    const dusk = run(null, world({ cycle: cycle('dusk') }), seen);
    expect(run(dusk, world({ cycle: cycle('night') }), seen)).toBeNull();
    const hire = run(null, world({ troops: [troop(3, 12)] }), seen);
    expect(run(hire, world({ troops: [troop(3, 12, 'peasant')] }), seen)).toBeNull();
    expect([...seen]).toEqual(['pay', 'dusk', 'recruit']);
  });

  it('ignorierter Hinweis bleibt stehen', () => {
    const seen = new Set<string>();
    const w = world({ sites: [site(2, 9)] });
    const shown = run(null, w, seen);
    expect(run(shown, w, seen)).toEqual(shown);
    expect(seen.size).toBe(0);
  });

  it('2 Spieler: je Zelle ein eigener Hinweis, die Merkung gilt fürs Gerät', () => {
    const seen = new Set<string>();
    const w = world({ coins: [coin(1, 5)], sites: [site(2, 50)] });
    const p1 = stepGuide(null, w, at(4), seen, (id) => seen.add(id));
    const p2 = stepGuide(null, w, at(51), seen, (id) => seen.add(id));
    expect([p1?.id, p2?.id]).toEqual(['coin', 'pay']);
    const after = world({ coins: [], sites: [site(2, 50)] });
    expect(stepGuide(p1, after, at(4), seen, (id) => seen.add(id))).toBeNull();
    expect(stepGuide(p2, after, at(51), seen, (id) => seen.add(id))).toEqual(p2);
  });
});
