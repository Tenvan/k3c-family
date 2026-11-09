import { describe, expect, it } from 'vitest';
import { BUDGET_SHARE, FrameStats, SMOOTH_FPS, defaultConfig, glErrorName, stageCount, stageLine, stageMiB, textureMiB, verdict, type Loss, type Stage } from './textureProbe';

const stage = (n: number, mib: number, fps = 60, error: string | null = null): Stage => ({ stage: n, mib, textures: n, fps, frameP95: 17, error });

describe('Textur-Grenze: Stufen', () => {
  it('Speicher je Textur: 1024² = 4 MiB, 2048² = 16 MiB, 4096² = 64 MiB', () => {
    expect(textureMiB(1024)).toBe(4);
    expect(textureMiB(2048)).toBe(16);
    expect(textureMiB(4096)).toBe(64);
  });

  it('Standard: etwa 64 MiB je Stufe, Stufen bis 1 GiB', () => {
    const cfg = defaultConfig(2048);
    expect(cfg.perStage * textureMiB(cfg.size)).toBe(64);
    expect(stageMiB(cfg, 3)).toBe(192);
    expect(stageCount(cfg)).toBe(16);
    expect(defaultConfig(4096).perStage).toBe(1);
    expect(defaultConfig(1024).perStage).toBe(16);
  });
});

describe('Frame-Statistik', () => {
  it('FPS aus den Frame-Zeiten, p95 aus den langsamsten 5 %', () => {
    const f = new FrameStats();
    for (let i = 0; i < 95; i++) f.add(10);
    for (let i = 0; i < 5; i++) f.add(50);
    expect(f.count).toBe(100);
    expect(f.fps()).toBe(Math.round((100 / (95 * 10 + 5 * 50)) * 10000) / 10);
    expect(f.p95()).toBe(10);
    f.add(80);
    expect(f.p95()).toBe(50);
  });

  it('leer und ungültige Zeiten stören nicht', () => {
    const f = new FrameStats();
    expect(f.fps()).toBe(0);
    expect(f.p95()).toBe(0);
    f.add(0);
    f.add(-3);
    expect(f.count).toBe(0);
    f.add(20);
    f.reset();
    expect(f.count).toBe(0);
  });
});

describe('Auswertung', () => {
  const stages = [stage(1, 64), stage(2, 128), stage(3, 192, SMOOTH_FPS - 5), stage(4, 256)];

  it('mit Verlust: sichere Grenze ist die letzte Stufe davor, Budget 70 % auf 10 MiB abgerundet', () => {
    const loss: Loss = { mib: 320, atMs: 9000, restoredAfterMs: 400 };
    const v = verdict(stages, loss);
    expect(v).toMatchObject({ kind: 'limit', safeMiB: 256, lossMiB: 320, budgetMiB: Math.floor((256 * BUDGET_SHARE) / 10) * 10 });
    expect(v.smoothMiB).toBe(256); // 192 MiB war zu langsam, 256 wieder flüssig
  });

  it('ohne Verlust und ohne Fehler: keine Grenze bis zur letzten Stufe', () => {
    expect(verdict(stages, null)).toMatchObject({ kind: 'none', safeMiB: 256, lossMiB: null });
  });

  it('ein Speicherfehler ohne Verlust zählt als Grenze: die Stufen ab dem Fehler sind nicht sicher', () => {
    const v = verdict([stage(1, 64), stage(2, 128), stage(3, 192, 60, 'OUT_OF_MEMORY')], null);
    expect(v).toMatchObject({ kind: 'error', safeMiB: 128, lossMiB: null });
  });

  it('Verlust schon in der ersten Stufe: sichere Grenze 0, Budget 0', () => {
    expect(verdict([], { mib: 64, atMs: 1, restoredAfterMs: null })).toMatchObject({ kind: 'limit', safeMiB: 0, budgetMiB: 0, smoothMiB: 0 });
  });
});

describe('Ausgabe', () => {
  it('Stufenzeile mit und ohne Fehler', () => {
    expect(stageLine(stage(3, 192, 58.3))).toBe('192 MiB · 3 · 58.3 FPS · p95 17 ms');
    expect(stageLine(stage(3, 192, 58.3, 'OUT_OF_MEMORY'))).toContain('· OUT_OF_MEMORY');
  });

  it('WebGL-Fehlercodes: 0 = kein Fehler, OUT_OF_MEMORY und Unbekanntes', () => {
    expect(glErrorName(0)).toBeNull();
    expect(glErrorName(0x0505)).toBe('OUT_OF_MEMORY');
    expect(glErrorName(0x9999)).toBe('0x9999');
  });
});
