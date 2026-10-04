import { describe, expect, it } from 'vitest';
import { MAX_DELAY_MS, MAX_EXTRAPOLATE_MS, Timeline } from './clientTimeline';
import type { WorldState } from './clientProtocol';

const TICK_MS = 1000 / 30;
const FRAME_MS = 1000 / 60;
/** Units je Tick des laufenden Monarchen */
const V = 0.1;

function state(x: number, other = 500): WorldState {
  return { players: [{ id: 1, x }, { id: 2, x: other }], troops: [], enemies: [], projectiles: [], events: [] } as unknown as WorldState;
}

/** Ankunftszeiten je Tick; TCP liefert in Reihenfolge, ein verspäteter Frame hält die folgenden auf. */
function inOrder(arrive: number[]): number[] {
  let latest = -Infinity;
  return arrive.map((t) => (latest = Math.max(latest, t)));
}

/** Spielt Frames zu den Ankunftszeiten ein und tastet mit 60 FPS ab; liefert die gezeichneten x von Spieler 1. */
function run(arrive: number[], untilMs: number, x = (tick: number) => tick * V): { xs: number[]; timeline: Timeline } {
  const timeline = new Timeline(TICK_MS);
  const xs: number[] = [];
  let next = 0;
  for (let now = 0; now <= untilMs; now += FRAME_MS) {
    while (next < arrive.length && arrive[next]! <= now) {
      timeline.push({ tick: next, receivedAt: arrive[next]!, state: state(x(next)) });
      next++;
    }
    const s = timeline.sample(now);
    if (s) xs.push(s.players[0]!.x);
  }
  return { xs, timeline };
}

function expectSmooth(xs: number[]): void {
  const limit = 1.5 * V * (FRAME_MS / TICK_MS);
  for (let i = 1; i < xs.length; i++) {
    const step = xs[i]! - xs[i - 1]!;
    expect(step, `Frame ${i}`).toBeGreaterThanOrEqual(0);
    expect(step, `Frame ${i}`).toBeLessThanOrEqual(limit + 1e-9);
  }
}

const ticks = (n: number) => Array.from({ length: n }, (_, i) => i);
/** Nach dem Einschwingen messen (die erste halbe Sekunde zählt nicht) */
const settled = (xs: number[]) => xs.slice(30);

describe('Timeline (B-277/AC-01): gleichmäßig trotz Schwankung', () => {
  it('gleichmäßige Frames: monoton, kein Sprung, Verzögerung an der Untergrenze', () => {
    const { xs, timeline } = run(ticks(120).map((t) => t * TICK_MS + 20), 3500);
    expectSmooth(settled(xs));
    expect(timeline.delayMs).toBeCloseTo(TICK_MS, 0);
  });

  it('gebündelte Frames (je zwei im selben Frame)', () => {
    const { xs } = run(ticks(120).map((t) => (t + (t % 2)) * TICK_MS + 20), 3500);
    expectSmooth(settled(xs));
  });

  it('verspätete Frames (jeder zehnte 60 ms zu spät) und eine Lücke von 80 ms', () => {
    const arrive = ticks(150).map((t) => t * TICK_MS + 20 + (t % 10 === 5 ? 60 : 0) + (t === 100 ? 80 : 0));
    const { xs, timeline } = run(inOrder(arrive), 4500);
    expectSmooth(settled(xs));
    expect(timeline.delayMs).toBeGreaterThan(TICK_MS);
    expect(timeline.delayMs).toBeLessThanOrEqual(MAX_DELAY_MS);
  });

  it('eine stehende Figur bleibt stehen', () => {
    const timeline = new Timeline(TICK_MS);
    timeline.push({ tick: 0, receivedAt: 0, state: state(0) });
    timeline.push({ tick: 1, receivedAt: TICK_MS, state: state(V) });
    expect(timeline.sample(TICK_MS)?.players[1]?.x).toBe(500);
  });

  it('ohne Frame kein Zustand', () => {
    expect(new Timeline(TICK_MS).sample(0)).toBeNull();
  });
});

describe('Timeline (B-277/AC-02): Extrapolation begrenzt, Teleport hart', () => {
  it('leerer Puffer: läuft höchstens MAX_EXTRAPOLATE_MS weiter und bleibt dann stehen', () => {
    const { xs } = run(ticks(30).map((t) => t * TICK_MS), 3000);
    const last = 29 * V;
    const end = xs.at(-1)!;
    expect(end).toBeGreaterThan(last); // über den letzten Zustand hinaus gelaufen
    expect(end).toBeLessThanOrEqual(last + V * (MAX_EXTRAPOLATE_MS / TICK_MS) + 1e-9);
    expect(xs.at(-20)).toBe(end); // steht
  });

  it('ein Sprung über TELEPORT_UNITS wird nicht überblendet', () => {
    const { xs } = run(ticks(60).map((t) => t * TICK_MS), 2000, (t) => (t < 30 ? 0 : 100));
    expect(xs.every((x) => x === 0 || x === 100)).toBe(true);
    expect(xs.at(-1)).toBe(100);
  });
});
