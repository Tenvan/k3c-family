// B-740: Fortschrittszeilen auf stderr und angehängt in .work/grafik/gen.log (AC-02), Format von schritt (AC-03).
import { mkdtempSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { konsoleUndLog, schritt } from './fortschritt';

afterEach(() => vi.restoreAllMocks());

describe('fortschritt', () => {
  it('konsoleUndLog schreibt jede Zeile auf stderr und mit Zeitstempel ins Log, je Lauf angehängt (B-740/AC-02)', () => {
    const stderr: string[] = [];
    vi.spyOn(process.stderr, 'write').mockImplementation((z) => (stderr.push(String(z)), true));
    const log = join(mkdtempSync(join(tmpdir(), 'paper-log-')), 'neu', 'gen.log');
    konsoleUndLog(log)('🚀 gen:paper: 1 Gruppen, 1 Assets');
    konsoleUndLog(log)('✅ gen:paper fertig in 80 ms');
    expect(stderr).toEqual(['🚀 gen:paper: 1 Gruppen, 1 Assets\n', '✅ gen:paper fertig in 80 ms\n']);
    const zeilen = readFileSync(log, 'utf8').trimEnd().split('\n');
    expect(zeilen).toHaveLength(2);
    expect(zeilen[0]).toMatch(/^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d\.\d{3}Z 🚀 gen:paper: 1 Gruppen, 1 Assets$/);
    expect(zeilen[1]).toMatch(/Z ✅ gen:paper fertig in 80 ms$/);
  });

  it('schritt meldet ✅ mit Dauer, bei Fehler ❌ und wirft weiter (B-740/AC-03)', () => {
    const zeilen: string[] = [];
    expect(schritt((z) => zeilen.push(z), '📂', 'Specs spiegeln', () => 42)).toBe(42);
    expect(() => schritt((z) => zeilen.push(z), '📦', 'übertragen', () => { throw new Error('kaputt'); })).toThrow('kaputt');
    expect(zeilen[0]).toMatch(/^📂 Specs spiegeln ✅ \d+ ms$/);
    expect(zeilen[1]).toMatch(/^📦 übertragen ❌ \d+ ms$/);
  });
});
