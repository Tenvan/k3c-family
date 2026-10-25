/**
 * Verdrahtung der Leistungsmessung mit der GameScene (PM1): Diagnose-Zeile immer (B-351), Performance-Modus mit Bericht
 * nur mit `?perf=1` (B-334). Kennt die Szene nur über `ProbeScene`, damit Tests ohne Phaser auskommen. Gemessen wird je
 * Gerät, ein und zwei Spieler (Split-Screen) gleich; Spieler und Kameras stehen im Bericht.
 */
import { clientLog } from '../core/clientLog';
import type { LatencyStats } from '../online/clientLatency';
import type { Status } from '../online/clientConnection';
import { collectPerf, startDiag, type PerfValues } from '../online/clientPerf';
import { FRAME_DROP_MS, PerfRecorder, STATE_GAP_MS, perfEnabled, sendPerfReport, type Drop, type PerfReport, type ReportMeta } from '../online/clientPerfReport';

/** Was die Messung von der GameScene liest (dieselben Quellen wie das Debug-Overlay) */
export interface ProbeScene {
  game: { loop: { actualFps: number; rawDelta: number } };
  events: { on(event: 'update', fn: () => void): unknown; off(event: 'update', fn: () => void): unknown; once(event: 'shutdown', fn: () => void): unknown };
  cameras: { cameras: unknown[] };
  delayMs: number;
  client: { status: Status; latency: LatencyStats | null; lastSnapshotAt: number | null; roomCode: string | null; deviceId: string; you: readonly unknown[] };
}

/** Browser-Anteile, in Tests ersetzbar */
export interface ProbeEnv {
  search: string;
  now(): number;
  hidden(): boolean;
  isoNow(): string;
  userAgent: string;
  viewport(): string;
  send(report: PerfReport): Promise<boolean>;
  onLeave(fn: () => void): () => void;
}

function browserEnv(): ProbeEnv {
  return {
    search: location.search,
    now: () => performance.now(),
    hidden: () => document.hidden,
    isoNow: () => new Date().toISOString(),
    userAgent: navigator.userAgent,
    viewport: () => `${window.innerWidth}x${window.innerHeight}`,
    send: (r) => sendPerfReport(r),
    onLeave: (fn) => {
      window.addEventListener('pagehide', fn);
      return () => window.removeEventListener('pagehide', fn);
    },
  };
}

/** Werte wie im Debug-Overlay; null ohne Raum oder im Hintergrund (Messung pausiert). */
function reader(scene: ProbeScene, env: ProbeEnv): () => PerfValues | null {
  return () =>
    scene.client.status === 'room' && !env.hidden()
      ? collectPerf({ fps: scene.game.loop.actualFps, latency: scene.client.latency, delayMs: scene.delayMs })
      : null;
}

/** Startet die Messung für einen Lauf der GameScene; sie endet mit `shutdown` der Szene. */
export function startPerfProbe(scene: ProbeScene, env: ProbeEnv = browserEnv()): void {
  const read = reader(scene, env);
  const stopDiag = startDiag(read);
  const finish = perfEnabled(env.search) ? startPerfMode(scene, env, read) : null;
  scene.events.once('shutdown', () => {
    stopDiag();
    finish?.('end');
  });
}

/** Performance-Modus: misst je Frame und je Zustand, sendet einmal am Ende oder beim Verlassen der Seite. */
function startPerfMode(scene: ProbeScene, env: ProbeEnv, read: () => PerfValues | null): (reason: ReportMeta['reason']) => void {
  const rec = new PerfRecorder(env.now());
  const startedAt = env.isoNow();
  let lastSnap: number | null = null;
  let done = false;
  const note = (d: Drop | null) => d && clientLog('info', `🐢 Einbruch (${d.kind}) ${d.ms} ms`, { ...d });
  const onUpdate = () => {
    const v = read();
    if (!v) {
      rec.pause(); // kein Raum oder Tab im Hintergrund: keine Schein-Einbrüche
      lastSnap = null;
      return;
    }
    const now = env.now();
    note(rec.frame(now, scene.game.loop.rawDelta, v));
    const at = scene.client.lastSnapshotAt;
    if (at === null || at === lastSnap) return;
    if (lastSnap !== null) note(rec.state(now, at - lastSnap, v));
    lastSnap = at;
  };
  const finish = (reason: ReportMeta['reason']) => {
    if (done) return;
    done = true;
    scene.events.off('update', onUpdate);
    stopLeave();
    const c = scene.client;
    const meta: ReportMeta = { reason, device: c.deviceId, room: c.roomCode, players: c.you.length, cameras: scene.cameras.cameras.length, userAgent: env.userAgent, viewport: env.viewport(), startedAt };
    const report = rec.report(meta, env.now());
    const ctx = { reason, drops: report.dropsTotal };
    void env.send(report).then((ok) => (ok ? clientLog('info', '📄 Leistungsbericht gesendet', ctx) : clientLog('warn', '❌ Leistungsbericht nicht gesendet', ctx)));
  };
  scene.events.on('update', onUpdate);
  const stopLeave = env.onLeave(() => finish('pagehide'));
  clientLog('info', '🚀 Performance-Modus aktiv', { frameDropMs: FRAME_DROP_MS, stateGapMs: STATE_GAP_MS });
  return finish;
}
