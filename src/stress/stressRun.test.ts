import { describe, expect, it } from 'vitest';
import type { RenderInfo } from '../core/renderInfo';
import { OK_FPS, StressRun, WARMUP_MS, densityLimit, type RunMeta, type StageSummary } from './stressRun';

const info: RenderInfo = { kind: 'WebGL2', gpu: 'Mali', maxTexture: 4096, width: 1920, height: 1080, devicePixelRatio: 1, viewport: '1280×720', textures: 300, textureMiB: 410.04, request: 'auto' };
const meta: RunMeta = { runId: 'abc123-typisch-p1-s1-x', scenario: 'typisch', players: 1, seed: 1, fileHash: 'deadbeef', params: '?scenario=typisch', version: 'v1', userAgent: 'Edge', stages: 3, everyMs: 10_000 };

interface Line {
  level: string;
  msg: string;
  ctx?: Record<string, unknown>;
}

function setup() {
  const lines: Line[] = [];
  const run = new StressRun(meta, { log: (level, msg, ctx) => lines.push({ level, msg, ctx }), info: () => info, heapMiB: () => 61.234 });
  return { run, lines };
}

/** Stufe mit gleichmäßigen Frames von `ms` bis zum Stufenende; gibt die Zusammenfassung zurück */
function playStage(run: StressRun, stage: number, entities: number, ms: number) {
  run.startStage(stage, entities, 5);
  let done = false;
  for (let i = 0; i < 100_000 && !done; i++) done = run.frame(ms);
  return run.endStage([entities]);
}

describe('Stufen-Zeile (AC-14)', () => {
  it('genau eine Zeile je Stufe mit runId, FPS, p95, Texturspeicher, Figuren und Typen', () => {
    const { run, lines } = setup();
    playStage(run, 1, 26, 20);
    playStage(run, 2, 39, 20);
    const stageLines = lines.filter((l) => l.msg.startsWith('💓'));
    expect(stageLines).toHaveLength(2);
    expect(stageLines[0]!.msg).toBe('💓 Stufe 1/3: 26 Figuren, 5 Typen, FPS Ø 50 (min 50), p95 20 ms, 410 MiB [abc123-typisch-p1-s1-x]');
    expect(stageLines[0]!.ctx).toMatchObject({ runId: meta.runId, stage: 1, entities: 26, sheets: 5, fpsMean: 50, frameP95: 20, textureMiB: 410, textures: 300, heapMiB: 61.2, over50: 0, over33: 0 });
  });

  it('Start- und Ereigniszeile tragen die runId', () => {
    const { run, lines } = setup();
    run.begin();
    run.startStage(1, 10, 3);
    run.event('WebGL Context lost');
    expect(lines[0]!.msg).toContain('🚀 Stress typisch: 1 Spieler, Seed 1, 3 Stufen à 10 s');
    expect(lines[0]!.ctx).toMatchObject({ runId: meta.runId, fileHash: 'deadbeef', params: '?scenario=typisch', ua: 'Edge', version: 'v1' });
    expect(lines[1]!.msg).toBe(`🚦 WebGL Context lost (Stufe 1, 0 ms) [${meta.runId}]`);
    expect(lines[1]!.ctx).toMatchObject({ runId: meta.runId, stage: 1 });
  });

  it('langsame Stufe wird als Warnung geschrieben', () => {
    const { run, lines } = setup();
    playStage(run, 1, 26, 60);
    expect(lines[0]!.level).toBe('warn');
  });
});

describe('Aufwärmzeit und Perzentile (AC-15)', () => {
  it('die ersten 2 s zählen nicht in die Messung', () => {
    const { run } = setup();
    run.startStage(1, 10, 3);
    for (let i = 0; i < 5; i++) expect(run.frame(400)).toBe(false); // langsames Aufwärmen: 5 × 400 ms = 2000 ms
    for (let t = 0; t < 8000; t += 20) run.frame(20);
    const s = run.endStage([10]);
    expect(s.warmupFrames).toBe(5);
    expect(s.frames).toBe(400);
    expect(s.frameP95).toBe(20);
    expect(s.over50).toBe(0);
    expect(s.frameMax).toBe(20);
  });

  it('p95, über 50 ms, über 33 ms und der längste Frame werden gezählt', () => {
    const { run } = setup();
    run.startStage(1, 10, 3);
    for (let t = 0; t < WARMUP_MS; t += 100) run.frame(100);
    for (let i = 0; i < 95; i++) run.frame(16);
    for (let i = 0; i < 4; i++) run.frame(40);
    run.frame(120);
    const s = run.endStage([10]);
    expect(s.frames).toBe(100);
    expect(s.over33).toBe(5);
    expect(s.over50).toBe(1);
    expect(s.frameMax).toBe(120);
    expect(s.fpsMin).toBeCloseTo(8.3, 1);
    expect(s.frameP95).toBe(16);
  });

  it('eine Pause (großer Frame) schiebt die Stufe um höchstens 1 s', () => {
    const { run } = setup();
    run.startStage(1, 10, 3);
    run.frame(60_000);
    expect(run.stageClockMs).toBe(1000);
  });
});

describe('Endzeile und Dichte-Grenze (AC-15)', () => {
  const stage = (n: number, fps: number, entities: number): StageSummary => ({ stage: n, entities, sheets: 5, inView: [entities], durationMs: 8000, frames: 100, warmupFrames: 5, fpsMean: fps, fpsMin: fps, fpsP5: fps, frameMeanMs: 20, frameP50: 20, frameP95: 30, frameP99: 40, frameMax: 50, over50: 0, over33: 0, textures: 1, textureMiB: 1, heapMiB: null });

  it('die Grenze ist die letzte Stufe einer ununterbrochenen Reihe ab Stufe 1 mit Ø FPS ≥ 30', () => {
    expect(densityLimit([stage(1, 60, 26), stage(2, 45, 39), stage(3, 28, 58), stage(4, 55, 87)])).toEqual({ stage: 2, entities: 39 });
    expect(densityLimit([stage(1, OK_FPS, 26)])).toEqual({ stage: 1, entities: 26 });
  });

  it('ist schon Stufe 1 zu langsam, gibt es keine Grenze', () => {
    expect(densityLimit([stage(1, 12, 26)])).toBeNull();
    expect(densityLimit([])).toBeNull();
  });

  it('finish() schreibt eine Zeile mit runId, Grenze, Ende-Grund und Tabelle je Stufe', () => {
    const { run, lines } = setup();
    playStage(run, 1, 26, 16);
    playStage(run, 2, 39, 16);
    playStage(run, 3, 58, 60);
    const result = run.finish('done');
    const end = lines.filter((l) => l.msg.startsWith('✅'));
    expect(end).toHaveLength(1);
    expect(end[0]!.msg).toBe(`✅ Ergebnis typisch: Dichte-Grenze Stufe 2 (39 Figuren), Ende: Ziel erreicht [${meta.runId}]`);
    expect(end[0]!.ctx).toMatchObject({ runId: meta.runId, reason: 'done', limit: { stage: 2, entities: 39 } });
    expect((end[0]!.ctx!.stages as unknown[]).length).toBe(3);
    expect(result.densityLimit).toEqual({ stage: 2, entities: 39 });
  });

  it('ein zweites finish() schreibt keine zweite Zeile; Kontextverlust nennt den Grund', () => {
    const { run, lines } = setup();
    playStage(run, 1, 26, 16);
    run.finish('contextLost');
    run.finish('done');
    const end = lines.filter((l) => l.msg.startsWith('✅'));
    expect(end).toHaveLength(1);
    expect(end[0]!.msg).toContain('Kontextverlust');
    expect(end[0]!.level).toBe('error');
  });

  it('keine Grenze: Text nennt es', () => {
    const { run, lines } = setup();
    playStage(run, 1, 26, 100);
    run.finish('done');
    expect(lines.at(-1)!.msg).toContain('keine (Stufe 1 unter 30 FPS)');
  });
});
