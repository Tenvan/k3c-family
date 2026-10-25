import { describe, expect, it, vi } from 'vitest';
import type { PerfValues } from './clientPerf';
import {
  FRAME_DROP_MS,
  HIST_MAX_MS,
  Histogram,
  MAX_DROPS,
  PerfRecorder,
  STATE_GAP_MS,
  detectDrop,
  perfEnabled,
  sendPerfReport,
  type ReportMeta,
} from './clientPerfReport';

const v: PerfValues = { fps: 60, latencyMs: 40, bufferMs: 100 };
const meta: ReportMeta = { reason: 'end', device: 'abcd', room: 'QWER', players: 2, cameras: 2, userAgent: 'test', viewport: '1920x1080', startedAt: '2026-10-25T10:00:00.000Z' };

describe('perfEnabled', () => {
  it('nur mit ?perf=1', () => {
    expect(perfEnabled('?perf=1')).toBe(true);
    expect(perfEnabled('?dev=1&perf=1&room=AB')).toBe(true);
    expect(perfEnabled('')).toBe(false);
    expect(perfEnabled('?perf=0')).toBe(false);
  });
});

describe('Histogram (Perzentile)', () => {
  it('p50, p95, p99 nach der Rang-Methode', () => {
    const h = new Histogram();
    for (let i = 1; i <= 100; i++) h.add(i);
    expect(h.quantiles()).toEqual({ count: 100, mean: 50.5, p50: 50, p95: 95, p99: 99, max: 100 });
  });

  it('ein Ausreißer zeigt sich in p99, nicht in p50', () => {
    const h = new Histogram();
    for (let i = 0; i < 99; i++) h.add(16.7);
    h.add(120);
    const q = h.quantiles()!;
    expect([q.p50, q.p95, q.p99, q.max]).toEqual([17, 17, 17, 120]);
    h.add(120);
    expect(h.quantiles()!.p99).toBe(120);
  });

  it('leer ist null, sehr lange Werte landen in der letzten Klasse', () => {
    const h = new Histogram();
    expect(h.quantiles()).toBeNull();
    h.add(99_999);
    expect(h.quantiles()).toMatchObject({ p50: HIST_MAX_MS, max: 99_999 });
  });
});

describe('detectDrop (AC-05)', () => {
  it('Frame über der Schwelle ist ein Einbruch mit Begleitwerten', () => {
    expect(detectDrop('frame', FRAME_DROP_MS, 1000, v)).toBeNull();
    expect(detectDrop('frame', 83.4, 1234.4, v)).toEqual({ atMs: 1234, kind: 'frame', ms: 83, fps: 60, latencyMs: 40, bufferMs: 100 });
  });

  it('Zustands-Lücke über der Schwelle', () => {
    expect(detectDrop('state', STATE_GAP_MS, 0, v)).toBeNull();
    expect(detectDrop('state', 400, 5000, v)).toMatchObject({ kind: 'state', ms: 400, atMs: 5000 });
  });
});

describe('PerfRecorder (AC-04, AC-05)', () => {
  it('Bericht enthält Frame-Zeit, Zustands-Abstand, Puffer, Latenz und Einbrüche', () => {
    const rec = new PerfRecorder(1000);
    let now = 1000;
    rec.frame(now, 999, v); // erster Frame misst die Pause mit und zählt nicht
    for (let i = 0; i < 100; i++) rec.frame((now += 16), 16, v);
    const drop = rec.frame((now += 90), 90, { fps: 45, latencyMs: 70, bufferMs: 120 });
    rec.state(now, 33, v);
    rec.state((now += 300), 300, v);
    const r = rec.report(meta, now);
    expect(drop).toMatchObject({ kind: 'frame', ms: 90, fps: 45, latencyMs: 70, bufferMs: 120 });
    expect(r).toMatchObject({ kind: 'perf', version: 1, ...meta, thresholds: { frameMs: FRAME_DROP_MS, stateGapMs: STATE_GAP_MS } });
    expect(r.frameMs).toMatchObject({ count: 101, p50: 16, p95: 16, p99: 16, max: 90 });
    expect(r.stateMs).toMatchObject({ count: 2, p50: 33, max: 300 });
    expect(r.bufferMs).toMatchObject({ count: 2, p50: 100 });
    expect(r.latencyMs).toMatchObject({ count: 2, p50: 40 });
    expect(r.dropsTotal).toBe(2);
    expect(r.drops.map((d) => d.kind)).toEqual(['frame', 'state']);
    expect(r.drops[1]).toEqual({ atMs: now - 1000, kind: 'state', ms: 300, ...v });
    expect(r.durationMs).toBe(now - 1000);
    expect(r.series.length).toBeGreaterThan(0);
  });

  it('nach einer Pause zählt der erste Frame nicht (kein Schein-Einbruch)', () => {
    const rec = new PerfRecorder(0);
    rec.frame(0, 16, v);
    rec.frame(16, 16, v);
    rec.pause();
    expect(rec.frame(5000, 4984, v)).toBeNull();
    expect(rec.report(meta, 5000).dropsTotal).toBe(0);
  });

  it('Einbrüche sind begrenzt, gezählt werden alle; ohne Latenz fehlt sie in der Verteilung', () => {
    const rec = new PerfRecorder(0);
    for (let i = 0; i <= MAX_DROPS + 5; i++) rec.state(i * 500, 500, { ...v, latencyMs: null });
    const r = rec.report(meta, 60_000);
    expect(r.drops).toHaveLength(MAX_DROPS);
    expect(r.dropsTotal).toBe(MAX_DROPS + 6);
    expect(r.latencyMs).toBeNull();
    expect(r.frameMs).toBeNull();
    expect(r.fps).toBeNull();
  });

  it('Bericht bleibt unter 64 KiB (keepalive)', () => {
    const rec = new PerfRecorder(0);
    for (let i = 0; i < 20_000; i++) rec.frame(i * 100, 100, v);
    expect(JSON.stringify(rec.report(meta, 2_000_000)).length).toBeLessThan(64 * 1024);
  });
});

describe('sendPerfReport', () => {
  const report = new PerfRecorder(0).report(meta, 0);

  it('POST /api/report mit JSON und keepalive', async () => {
    const fetchFn = vi.fn(async () => new Response('{"ok":true}', { status: 200 }));
    expect(await sendPerfReport(report, fetchFn)).toBe(true);
    const [url, init] = fetchFn.mock.calls[0] as unknown as [string, RequestInit];
    expect(url).toBe('/api/report');
    expect(init).toMatchObject({ method: 'POST', keepalive: true });
    expect(JSON.parse(init.body as string)).toMatchObject({ kind: 'perf', room: 'QWER' });
  });

  it('Server nicht erreichbar: false, kein Fehler', async () => {
    expect(await sendPerfReport(report, () => Promise.reject(new Error('offline')))).toBe(false);
    expect(await sendPerfReport(report, async () => new Response('', { status: 500 }))).toBe(false);
  });
});
