import { describe, expect, it } from 'vitest';
import { FALLBACK_SPEED, PULL, Predictor, SNAP_UNITS } from './clientPredict';
import type { SlotSeat, WorldState } from './clientProtocol';

const TICK_MS = 1000 / 30;
const FRAME_MS = 1000 / 60;
/** Lokaler Monarch 0 (Slot 0), fremder Monarch 1 */
const YOU: SlotSeat[] = [{ slot: 0, monarch: 0, depth: 0 }];

function state(mine: number, other = 50): WorldState {
  return { players: [{ id: 10, index: 0, x: mine }, { id: 11, index: 1, x: other }] } as unknown as WorldState;
}
const right = [{ slot: 0, moveX: 1, sprint: false, pay: false }];
const idle = [{ slot: 0, moveX: 0, sprint: false, pay: false }];
const mineX = (s: WorldState) => s.players[0]!.x;

describe('Predictor (B-277/AC-03)', () => {
  it('bewegt den lokalen Monarchen im ersten Frame nach der Eingabe, den fremden nicht', () => {
    const p = new Predictor(TICK_MS);
    p.observe({ tick: 0, state: state(10) }, YOU);
    expect(mineX(p.draw(state(10), 0, idle, YOU))).toBe(10);
    const drawn = p.draw(state(10), FRAME_MS, right, YOU);
    expect(mineX(drawn)).toBeCloseTo(10 + FALLBACK_SPEED * (FRAME_MS / 1000) * (1 - PULL));
    expect(drawn.players[1]!.x).toBe(50);
  });

  it('ohne Eingabe baut sich der Abstand zum Server-Zustand ab (Server lehnt Bewegung ab)', () => {
    const p = new Predictor(TICK_MS);
    p.observe({ tick: 0, state: state(10) }, YOU);
    let now = 0;
    for (let i = 0; i < 10; i++) p.draw(state(10), (now += FRAME_MS), right, YOU); // Server bewegt sich nicht
    const ahead = mineX(p.draw(state(10), (now += FRAME_MS), idle, YOU)) - 10;
    expect(ahead).toBeGreaterThan(0);
    let x = 0;
    for (let i = 0; i < 30; i++) x = mineX(p.draw(state(10), (now += FRAME_MS), idle, YOU));
    expect(x - 10).toBeLessThan(ahead / 10);
  });

  it('großer Abstand (Teleport, Respawn) übernimmt sofort den Server-Wert', () => {
    const p = new Predictor(TICK_MS);
    p.observe({ tick: 0, state: state(10) }, YOU);
    p.draw(state(10), FRAME_MS, idle, YOU);
    p.observe({ tick: 1, state: state(10 + SNAP_UNITS + 1) }, YOU);
    expect(mineX(p.draw(state(10), 2 * FRAME_MS, idle, YOU))).toBe(10 + SNAP_UNITS + 1);
  });

  it('Geschwindigkeit aus der beobachteten Bewegung (Sprint), solange er läuft', () => {
    const p = new Predictor(TICK_MS);
    for (let t = 0; t < 30; t++) p.observe({ tick: t, state: state(10 + t * 0.4) }, YOU); // 12 Units/s
    for (let t = 30; t < 40; t++) p.observe({ tick: t, state: state(10 + 29 * 0.4) }, YOU); // steht: Tempo bleibt
    const x0 = mineX(p.draw(state(0), 0, right, YOU)); // startet am neuesten Server-x
    const x1 = mineX(p.draw(state(0), FRAME_MS, right, YOU));
    expect((x1 - x0) / (1 - PULL)).toBeCloseTo(12 * (FRAME_MS / 1000), 2);
  });

  it('ohne Zustand des Monarchen bleibt der gezeichnete Zustand unverändert', () => {
    const p = new Predictor(TICK_MS);
    const s = state(10);
    expect(p.draw(s, 0, right, YOU)).toBe(s);
  });
});
