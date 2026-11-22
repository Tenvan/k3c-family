import { describe, expect, it } from 'vitest';
import { shortSpan, statsLines } from './format';

describe('Auswertungs-Statistik', () => {
  it('kurze Dauer', () => {
    expect(shortSpan(42_000)).toBe('42 s');
    expect(shortSpan(185_000)).toBe('3 min 5 s');
  });

  it('Zeilen nennen Dauer je Minute Aufnahme und Faktor', () => {
    const l = statsLines({ totalMs: 66000, transcribeMs: 60000, framesMs: 6000, audioMs: 150000, segments: 35, words: 210, frames: 8, secPerMin: 26.4, factor: 0.44 });
    expect(l[0]).toBe('Auswertung dauerte 1 min 6 s für 2 min 30 s Aufnahme');
    expect(l[1]).toContain('26,4 s je Minute');
    expect(l[1]).toContain('0,4×');
    expect(l.join(' ')).toContain('35 Sätze, 210 Wörter');
  });

  it('ohne Aufnahmelänge keine Verhältniszeile', () => {
    expect(statsLines({ totalMs: 1000, transcribeMs: 1000, framesMs: 0, audioMs: 0, segments: 0, words: 0, frames: 0, secPerMin: 0, factor: 0 })).toHaveLength(3);
  });
});
