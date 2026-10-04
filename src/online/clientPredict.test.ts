import { describe, expect, it } from 'vitest';
import { FALLBACK_SPEED, Predictor, SNAP_UNITS } from './clientPredict';
import type { SlotSeat, WorldState } from './clientProtocol';

const TICK_MS = 1000 / 30;
const FRAME_MS = 1000 / 60;
/** Lokaler Monarch 0 (Slot 0), fremder Monarch 1 */
const YOU: SlotSeat[] = [{ slot: 0, monarch: 0, depth: 0 }];

function state(mine: number, other = 50, extra: { respawnIn?: number; devPaused?: boolean } = {}): WorldState {
  return {
    players: [{ id: 10, index: 0, x: mine, respawnIn: extra.respawnIn ?? 0 }, { id: 11, index: 1, x: other, respawnIn: 0 }],
    devPaused: extra.devPaused,
  } as unknown as WorldState;
}
const frame = (tick: number, s: WorldState, receivedAt = tick * TICK_MS) => ({ tick, receivedAt, state: s });
const right = [{ slot: 0, moveX: 1, sprint: false, pay: false }];
const idle = [{ slot: 0, moveX: 0, sprint: false, pay: false }];
const mineX = (s: WorldState) => s.players[0]!.x;

describe('Predictor (B-277/AC-03)', () => {
  it('bewegt den lokalen Monarchen im ersten Frame nach der Eingabe, den fremden nicht', () => {
    const p = new Predictor(TICK_MS);
    p.observe(frame(0, state(10)), YOU);
    expect(mineX(p.draw(state(10), 0, idle, YOU))).toBe(10);
    const drawn = p.draw(state(10), FRAME_MS, right, YOU);
    expect(mineX(drawn)).toBeCloseTo(10 + FALLBACK_SPEED * (FRAME_MS / 1000));
    expect(drawn.players[1]!.x).toBe(50);
  });

  it('ohne Eingabe baut sich der Abstand zum Server-Zustand ab (Server lehnt Bewegung ab)', () => {
    const p = new Predictor(TICK_MS);
    p.observe(frame(0, state(10)), YOU);
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
    p.observe(frame(0, state(10)), YOU);
    p.draw(state(10), FRAME_MS, idle, YOU);
    p.observe(frame(1, state(10 + SNAP_UNITS + 1)), YOU);
    expect(mineX(p.draw(state(10), 2 * FRAME_MS, idle, YOU))).toBe(10 + SNAP_UNITS + 1);
  });

  it('Geschwindigkeit aus der beobachteten Bewegung (Sprint), solange er läuft', () => {
    const p = new Predictor(TICK_MS);
    for (let t = 0; t < 30; t++) p.observe(frame(t, state(10 + t * 0.4)), YOU); // 12 Units/s
    for (let t = 30; t < 40; t++) p.observe(frame(t, state(10 + 29 * 0.4)), YOU); // steht: Tempo bleibt
    const now = 39 * TICK_MS;
    const x0 = mineX(p.draw(state(0), now, right, YOU)); // startet am neuesten Server-x
    const x1 = mineX(p.draw(state(0), now + FRAME_MS, right, YOU));
    expect(x1 - x0).toBeCloseTo(12 * (FRAME_MS / 1000), 2);
  });

  it('ohne Zustand des Monarchen bleibt der gezeichnete Zustand unverändert', () => {
    const p = new Predictor(TICK_MS);
    const s = state(10);
    expect(p.draw(s, 0, right, YOU)).toBe(s);
  });

  it('tot (respawnIn > 0) oder Raum angehalten: keine Vorhersage-Bewegung', () => {
    for (const extra of [{ respawnIn: 2 }, { devPaused: true }]) {
      const p = new Predictor(TICK_MS);
      p.observe(frame(0, state(10, 50, extra)), YOU);
      p.draw(state(10), 0, right, YOU);
      expect(mineX(p.draw(state(10), FRAME_MS, right, YOU))).toBe(10);
    }
  });

  it('Zeitraffer: bei hoher Geschwindigkeit springt die Vorhersage nicht bei jedem Zustand (kein Sägezahn)', () => {
    const p = new Predictor(TICK_MS);
    const v = 30; // Units/s, z. B. Zeitraffer
    for (let t = 0; t < 30; t++) p.observe(frame(t, state(t * v * (TICK_MS / 1000))), YOU);
    let prev = mineX(p.draw(state(0), 29 * TICK_MS, right, YOU));
    for (let i = 1; i <= 60; i++) {
      const now = 29 * TICK_MS + i * FRAME_MS;
      if (i % 2 === 0) p.observe(frame(29 + i / 2, state(now * v * 0.001), now), YOU);
      const x = mineX(p.draw(state(0), now, right, YOU));
      expect(x).toBeGreaterThanOrEqual(prev); // läuft nur vorwärts
      prev = x;
    }
  });
});

/**
 * Simulierte Verbindung mit RTT `L`: Der Server bewegt den Monarchen mit der Eingabe von vor `L` ms, Zustände kommen
 * mit 30 Hz. Rechts halten 2 s, dann loslassen.
 */
function simulateRtt(latencyMs: number) {
  const p = new Predictor(TICK_MS);
  const holdUntil = 2000;
  const serverX = (now: number) => (FALLBACK_SPEED * Math.min(Math.max(0, now - latencyMs), holdUntil)) / 1000;
  const drawn: { now: number; x: number }[] = [];
  let tick = 0;
  for (let now = 0; now <= 3500; now += FRAME_MS) {
    while ((tick + 1) * TICK_MS <= now) {
      tick++;
      p.observe(frame(tick, state(serverX(tick * TICK_MS)), tick * TICK_MS), YOU);
    }
    if (tick === 0) p.observe(frame(0, state(0), 0), YOU);
    drawn.push({ now, x: mineX(p.draw(state(0), now, now < holdUntil ? right : idle, YOU, latencyMs)) });
  }
  return { drawn, final: serverX(holdUntil + latencyMs) };
}

describe('Predictor unter Latenz (Review N2.3)', () => {
  it('RTT 150 ms: gezeichnete Geschwindigkeit bei gehaltener Eingabe nahe der Laufgeschwindigkeit, auch beim Anlaufen (kein Gummiband)', () => {
    const { drawn } = simulateRtt(150);
    const at = (t: number) => drawn.find((d) => d.now >= t)!.x;
    for (const [from, to] of [[0, 300], [300, 600], [900, 1900]] as const) {
      const v = (at(to) - at(from)) / ((to - from) / 1000);
      expect(v).toBeGreaterThanOrEqual(0.8 * FALLBACK_SPEED);
      expect(v).toBeLessThanOrEqual(1.2 * FALLBACK_SPEED);
    }
  });

  it('RTT 150 ms: nach dem Loslassen weder Zurückgleiten noch Nachlaufen über die End-Position hinaus', () => {
    const { drawn, final } = simulateRtt(150);
    const after = drawn.filter((d) => d.now >= 2000);
    for (let i = 1; i < after.length; i++) expect(after[i]!.x).toBeGreaterThanOrEqual(after[i - 1]!.x - 1e-6);
    expect(Math.max(...after.map((d) => d.x))).toBeLessThanOrEqual(final + 0.1);
    expect(after.at(-1)!.x).toBeCloseTo(final, 2);
  });
});
