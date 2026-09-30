import { describe, expect, it } from 'vitest';
import { formatBytes, formatDuration, formatNumber, formatPercent, formatTime, formatUptime } from './format';

describe('format', () => {
  it('Zahlen und Prozent deutsch', () => {
    expect(formatNumber(1234.5, 1)).toBe('1.234,5');
    expect(formatNumber(7)).toBe('7');
    expect(formatPercent(3.14)).toBe('3,1 %');
    expect(formatPercent(0)).toBe('0,0 %');
  });

  it('Bytes wie formatBytes in Go', () => {
    expect(formatBytes(512)).toBe('512 B');
    expect(formatBytes(2048)).toBe('2,0 KB');
    expect(formatBytes(480 * 1024 * 1024)).toBe('480,0 MB');
    expect(formatBytes(1.5 * 1024 ** 3)).toBe('1,5 GB');
  });

  it('Laufzeit wie formatUptime in Go', () => {
    expect(formatUptime(30_000)).toBe('< 1 min');
    expect(formatUptime(42 * 60_000)).toBe('42 min');
    expect(formatUptime((2 * 60 + 13) * 60_000)).toBe('2 h 13 min');
    expect(formatUptime((3 * 24 * 60 + 4 * 60 + 5) * 60_000)).toBe('3 T 4 h');
  });

  it('Dauer wie formatMs in Go', () => {
    expect(formatDuration(468)).toBe('468 ms');
    expect(formatDuration(12_400)).toBe('12,4 s');
    expect(formatDuration(125_000)).toBe('2 min 5 s');
    expect(formatDuration(62 * 60_000)).toBe('1 h 2 min');
  });

  it('Uhrzeit zweistellig', () => {
    const t = new Date(2026, 8, 30, 8, 3, 15);
    expect(formatTime(t)).toBe('08:03');
    expect(formatTime(t, true)).toBe('08:03:15');
  });
});
