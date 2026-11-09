import { describe, expect, it } from 'vitest';
import { ATTEMPTS, attemptLine, failed, summarize, type AttemptResult } from './glProbe';

const good = (id: string): AttemptResult => ({ id, ok: true, error: null, version: 'WebGL 1.0', renderer: 'ANGLE', vendor: 'Google', maxTexture: 8192, textureUnits: 16, extensions: 30, timerQuery: false, ms: 5 });

describe('WebGL-Diagnose: Versuche', () => {
  it('beginnt mit den Einstellungen von Phaser, jeder Versuch hat einen eigenen Namen', () => {
    expect(ATTEMPTS[0]).toMatchObject({ id: 'webgl-phaser', kind: 'webgl', attrs: { antialias: true, powerPreference: 'high-performance', failIfMajorPerformanceCaveat: false } });
    expect(new Set(ATTEMPTS.map((a) => a.id)).size).toBe(ATTEMPTS.length);
  });

  it('deckt WebGL 2, WebGL 1 und den alten Namen ab', () => {
    expect(new Set(ATTEMPTS.map((a) => a.kind))).toEqual(new Set(['webgl', 'webgl2', 'experimental-webgl']));
  });
});

describe('WebGL-Diagnose: Auswertung', () => {
  it('nichts gelungen: kind none, Gründe ohne Wiederholung, ein fehlender Grund steht als null', () => {
    const s = summarize([failed('a', 'null', 1), failed('b', 'null', 1), failed('c', 'BLOCKLISTED: x', 1)]);
    expect(s).toEqual({ ok: 0, first: null, kind: 'none', reasons: ['null', 'BLOCKLISTED: x'] });
  });

  it('einige gelungen: der erste gelungene Versuch führt die Messung', () => {
    const s = summarize([failed('a', 'null', 1), good('b'), good('c')]);
    expect(s).toMatchObject({ ok: 2, first: 'b', kind: 'partial', reasons: ['null'] });
  });

  it('alle gelungen', () => {
    expect(summarize([good('a'), good('b')])).toMatchObject({ kind: 'all', first: 'a', reasons: [] });
  });

  it('keine Versuche: nichts gelungen', () => {
    expect(summarize([])).toMatchObject({ ok: 0, first: null, kind: 'none' });
  });
});

describe('WebGL-Diagnose: Ausgabe', () => {
  it('gelungen: Version, Renderer, maximale Texturgröße und der Hinweis auf die Timer-Erweiterung', () => {
    expect(attemptLine(good('webgl'))).toBe('✓ webgl: WebGL 1.0 · ANGLE · maxTex 8192');
    expect(attemptLine({ ...good('webgl'), timerQuery: true })).toContain('Timer-Query');
  });

  it('gescheitert: mit Grund, ohne Grund steht ein erklärender Satz', () => {
    expect(attemptLine(failed('webgl', 'BLOCKLISTED: GPU', 1))).toBe('✗ webgl: BLOCKLISTED: GPU');
    expect(attemptLine(failed('webgl', 'null', 1))).toContain('lieferte null');
  });
});
