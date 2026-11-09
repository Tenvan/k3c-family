/**
 * Stresstest des echten Clients (B-766, RX1): Mit `?stress=<szenario>` (und `?dev=1`, Dev-Mode des Servers) setzt der Lauf in der
 * laufenden `GameScene` Stufe für Stufe mehr Gegner und Truppen über die Dev-Aktion `spawn` in die Stufe der Monarchen. Zeichnen,
 * Hintergründe, Vordergrund, Animation, Kamera und Netz sind die des Spiels; Messung und Log wie in `stress.html` (`StressRun`),
 * dazu Latenz, Puffer und die Zahl der Figuren im Zustand. Phaser-frei bis auf den Typ des Spiels; Uhr und Seite sind ersetzbar.
 */
import type Phaser from 'phaser';
import { clientLog } from '../core/clientLog';
import type { RenderInfo } from '../core/renderInfo';
import { formatBuild, CLIENT } from '../core/version';
import type { DevMessage } from '../online/clientProtocol';
import type { LatencyStats } from '../online/clientLatency';
import data from '../../data/stress.json';
import { KNOWN } from './known';
import { distinctTypes, figuresAt, hashText, stageCount, type Figure, type KnownTypes, type Scenario } from './stressPlan';
import { deviceTag, sendStressReport } from './stressReport';
import { StressRun, type RunEnv } from './stressRun';

/** Höchstzahl je Dev-Aktion (`sim.DevMaxSpawn`) */
export const SPAWN_CHUNK = 200;

/** Szenario-ID aus `?stress=`; nur mit `?dev=1` (die Aktion gibt es nur im Dev-Mode des Servers), sonst `null` */
export function realStressId(search: string): string | null {
  const p = new URLSearchParams(search);
  return p.get('dev') === '1' ? p.get('stress') || null : null;
}

type Counts = Map<string, number>;

function countsAt(s: Scenario, known: KnownTypes, stage: number, players: 1 | 2): Counts {
  const out: Counts = new Map();
  if (stage < 1) return out;
  for (const f of figuresAt(s, known, stage, players)) {
    const key = `${f.player}|${f.kind}|${f.type}`;
    out.set(key, (out.get(key) ?? 0) + 1);
  }
  return out;
}

/** Dev-Nachrichten, die von Stufe `stage − 1` auf `stage` fehlen; `slots[i]` ist der Slot des Spielers i. Höchstens `SPAWN_CHUNK` je Nachricht. */
export function spawnMessages(s: Scenario, known: KnownTypes, stage: number, slots: readonly number[]): DevMessage[] {
  const players = slots.length === 2 ? 2 : 1;
  const before = countsAt(s, known, stage - 1, players);
  const out: DevMessage[] = [];
  for (const [key, count] of countsAt(s, known, stage, players)) {
    const [player, kind, unit] = key.split('|') as [string, Figure['kind'], string];
    for (let left = count - (before.get(key) ?? 0); left > 0; left -= SPAWN_CHUNK) {
      out.push({ t: 'dev', action: 'spawn', slot: slots[Number(player)]!, kind, unit, amount: Math.min(left, SPAWN_CHUNK) });
    }
  }
  return out;
}

/** Was der Lauf von der `GameScene` liest */
export interface RealHost {
  game: Phaser.Game;
  events: { on(e: 'update', fn: () => void): unknown; off(e: 'update', fn: () => void): unknown; once(e: 'shutdown', fn: () => void): unknown };
  delayMs: number;
  world: { enemies: readonly unknown[]; troops: readonly unknown[] } | undefined;
  client: { status: string; latency: LatencyStats | null; you: readonly { slot: number }[]; sendDev(m: DevMessage): void };
}

/** Der Lauf: wartet auf den Raum, setzt je Stufe die fehlenden Figuren, misst und schreibt das Log */
export class RealStress {
  private run: StressRun | null = null;
  private stage = 0;
  private slots: number[] = [];
  private total = 0;
  private planned = 0;
  private done = false;
  private startedAt = '';
  private runId = '';

  constructor(
    private readonly host: RealHost,
    private readonly scenario: Scenario,
    private readonly id: string,
    private readonly search: string,
    private readonly info: () => RenderInfo,
    private readonly log: RunEnv['log'] = clientLog,
  ) {}

  /** Ein Frame der `GameScene` */
  update(): void {
    if (this.done) return;
    const h = this.host;
    if (!this.run) return this.tryStart();
    if (h.client.status !== 'room') return this.end('aborted'); // Raum verlassen oder Verbindung weg
    if (this.run.frame(h.game.loop.rawDelta)) this.nextStage();
  }

  /** Die Szene endet (Raum verlassen, Seite zu): offener Lauf bekommt seine Endzeile */
  abort(): void {
    if (this.run && !this.done) this.end('aborted');
  }

  private tryStart(): void {
    const h = this.host;
    if (h.client.status !== 'room' || !h.world || h.client.you.length === 0) return;
    const players = Math.min(2, h.client.you.length) as 1 | 2;
    this.slots = h.client.you.slice(0, players).map((s) => s.slot);
    const p = new URLSearchParams(this.search);
    const seedText = p.get('seed') ?? '1';
    this.startedAt = new Date().toISOString();
    this.runId = `${deviceTag()}-real-${this.id}-p${players}-d${p.get('depth') ?? '0'}-${Date.now().toString(36)}`;
    this.total = stageCount(this.scenario, p.get('ramp') !== '0');
    const env: RunEnv = {
      log: this.log,
      info: this.info,
      heapMiB: () => ((performance as unknown as { memory?: { usedJSHeapSize: number } }).memory?.usedJSHeapSize ?? NaN) / (1024 * 1024) || null,
    };
    this.run = new StressRun(
      { runId: this.runId, scenario: this.id, players, seed: seedText, fileHash: hashText(JSON.stringify(data)), params: this.search, version: formatBuild(CLIENT), userAgent: navigator.userAgent, stages: this.total, everyMs: this.scenario.ramp.everyMs },
      env,
    );
    this.run.begin();
    this.run.event('Echter Client', { mode: 'real', depth: Number(p.get('depth') ?? 0), room: seedText, slots: this.slots });
    this.enter(1);
  }

  private enter(stage: number): void {
    this.stage = stage;
    const figs = figuresAt(this.scenario, KNOWN, stage, this.slots.length === 2 ? 2 : 1);
    for (const m of spawnMessages(this.scenario, KNOWN, stage, this.slots)) this.host.client.sendDev(m);
    this.planned = figs.length;
    this.run!.startStage(stage, figs.length, distinctTypes(figs));
  }

  private nextStage(): void {
    const w = this.host.world;
    const real = (w?.enemies.length ?? 0) + (w?.troops.length ?? 0);
    if (this.planned > 0 && real === 0) this.run!.event('Keine Figuren im Zustand: spawn abgelehnt (Dev-Mode des Servers aus?)');
    this.run!.endStage([w ? w.enemies.length + w.troops.length : 0], {
      latencyMs: this.host.client.latency ? Math.round(this.host.client.latency.mean) : null,
      bufferMs: Math.round(this.host.delayMs),
      enemies: w?.enemies.length ?? 0,
      troops: w?.troops.length ?? 0,
    });
    if (this.stage < this.total) this.enter(this.stage + 1);
    else this.end('done');
  }

  private end(reason: 'done' | 'aborted'): void {
    if (this.done || !this.run) return;
    this.done = true;
    const result = this.run.finish(reason);
    sendStressReport({ runId: this.runId, scenario: this.id, players: this.slots.length, seed: new URLSearchParams(this.search).get('seed') ?? '1', startedAt: this.startedAt, mode: 'real' }, result, this.search);
  }
}
