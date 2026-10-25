import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { clientLog } from '../core/clientLog';
import { DIAG_EVERY_MS } from '../online/clientPerf';
import type { PerfReport } from '../online/clientPerfReport';
import { startPerfProbe, type ProbeEnv, type ProbeScene } from './perfProbe';

vi.mock('../core/clientLog', () => ({ clientLog: vi.fn() }));
const log = vi.mocked(clientLog);

/** Szene ohne Phaser: `update` und `shutdown` löst der Test aus */
function fakeScene() {
  const handlers = { update: new Set<() => void>(), shutdown: new Set<() => void>() };
  const scene: ProbeScene = {
    game: { loop: { actualFps: 60, rawDelta: 16 } },
    events: {
      on: (_e, fn) => handlers.update.add(fn),
      off: (_e, fn) => handlers.update.delete(fn),
      once: (_e, fn) => handlers.shutdown.add(fn),
    },
    cameras: { cameras: [{}, {}] },
    delayMs: 100,
    client: { status: 'room', latency: { mean: 40, p95: 60 }, lastSnapshotAt: null, roomCode: 'QWER', deviceId: 'abcd', you: [{}, {}] },
  };
  const update = () => handlers.update.forEach((fn) => fn());
  const shutdown = () => handlers.shutdown.forEach((fn) => fn());
  return { scene, handlers, update, shutdown };
}

function fakeEnv(search: string) {
  const sent: PerfReport[] = [];
  const leave = new Set<() => void>();
  let now = 0;
  const env: ProbeEnv = {
    search,
    now: () => now,
    hidden: () => false,
    isoNow: () => '2026-10-25T10:00:00.000Z',
    userAgent: 'test',
    viewport: () => '1920x1080',
    send: vi.fn(async (r: PerfReport) => {
      sent.push(r);
      return true;
    }),
    onLeave: (fn) => {
      leave.add(fn);
      return () => leave.delete(fn);
    },
  };
  return { env, sent, leave, tick: (ms: number) => (now += ms) };
}

describe('startPerfProbe', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    log.mockClear();
  });
  afterEach(() => vi.useRealTimers());

  it('AC-03: ohne ?perf=1 wird nichts gemessen oder gesendet', () => {
    const { scene, handlers, shutdown } = fakeScene();
    const { env, leave } = fakeEnv('?room=QWER');
    startPerfProbe(scene, env);
    expect(handlers.update.size).toBe(0);
    expect(leave.size).toBe(0);
    shutdown();
    expect(env.send).not.toHaveBeenCalled();
  });

  it('AC-01: Diagnose-Zeile im Takt, auch ohne Performance-Modus; endet mit der Szene', () => {
    const { scene, shutdown } = fakeScene();
    const { env } = fakeEnv('');
    startPerfProbe(scene, env);
    vi.advanceTimersByTime(DIAG_EVERY_MS);
    expect(log).toHaveBeenCalledWith('info', expect.stringMatching(/^💓 /), { fps: 60, latencyMs: 40, bufferMs: 100 });
    shutdown();
    vi.advanceTimersByTime(DIAG_EVERY_MS * 2);
    expect(log).toHaveBeenCalledTimes(1);
  });

  it('AC-04/AC-05: mit ?perf=1 ein Bericht am Ende, mit Einbruch und Begleitwerten', async () => {
    const { scene, handlers, update, shutdown } = fakeScene();
    const { env, sent, tick } = fakeEnv('?perf=1');
    startPerfProbe(scene, env);
    for (let i = 0; i < 10; i++) {
      tick(16);
      scene.client.lastSnapshotAt = i * 33;
      update();
    }
    scene.game.loop.rawDelta = 120;
    scene.game.loop.actualFps = 40;
    tick(120);
    update();
    shutdown();
    shutdown(); // zweites Ende sendet nicht nochmal
    await vi.runAllTimersAsync();
    expect(handlers.update.size).toBe(0);
    expect(sent).toHaveLength(1);
    const r = sent[0]!;
    expect(r).toMatchObject({ kind: 'perf', reason: 'end', room: 'QWER', players: 2, cameras: 2, dropsTotal: 1 });
    expect(r.frameMs).toMatchObject({ count: 10, p50: 16, max: 120 });
    expect(r.stateMs).toMatchObject({ count: 9, p50: 33 });
    expect(r.drops[0]).toEqual({ atMs: 280, kind: 'frame', ms: 120, fps: 40, latencyMs: 40, bufferMs: 100 });
    expect(log).toHaveBeenCalledWith('info', '📄 Leistungsbericht gesendet', expect.anything());
    expect(log).toHaveBeenCalledWith('info', expect.stringMatching(/^🐢 /), expect.anything());
  });

  it('Verlassen der Seite sendet den Bericht einmal', async () => {
    const { scene, shutdown } = fakeScene();
    const { env, sent, leave } = fakeEnv('?perf=1');
    startPerfProbe(scene, env);
    leave.forEach((fn) => fn());
    shutdown();
    await vi.runAllTimersAsync();
    expect(sent.map((r) => r.reason)).toEqual(['pagehide']);
    expect(leave.size).toBe(0);
  });

  it('außerhalb des Raums pausiert die Messung', async () => {
    const { scene, update, shutdown } = fakeScene();
    const { env, sent, tick } = fakeEnv('?perf=1');
    startPerfProbe(scene, env);
    scene.client.status = 'reconnecting';
    scene.game.loop.rawDelta = 3000;
    tick(3000);
    update();
    shutdown();
    await vi.runAllTimersAsync();
    expect(sent[0]).toMatchObject({ dropsTotal: 0, frameMs: null });
  });
});
