import { describe, expect, it } from 'vitest';
import type { ConsoleLine, Source } from '../api';
import { color256, parseAnsi, stripAnsi } from './ansi';
import { dotTone, markOf, mergeLines, pickSource, upsertSource, visibleLines } from './lines';

const E = '\u001b[';
const line = (seq: number, text = `Zeile ${seq}`): ConsoleLine => ({ source: 'Vite', stream: 'stdout', text, seq });
const seqs = (l: ConsoleLine[]) => l.map((x) => x.seq);

describe('ANSI', () => {
  it('16 Standardfarben als Klassen, Reset', () => {
    expect(parseAnsi(`${E}31mrot${E}0m normal ${E}1;92mhell${E}m`)).toEqual([
      { text: 'rot', className: 'ansi-fg-1' },
      { text: ' normal ', className: '' },
      { text: 'hell', className: 'ansi-fg-10 ansi-bold' },
    ]);
    expect(parseAnsi(`${E}42m${E}30m ok ${E}39;49mx`)).toEqual([
      { text: ' ok ', className: 'ansi-fg-0 ansi-bg-2' },
      { text: 'x', className: '' },
    ]);
  });

  it('256 Farben und RGB als Stil', () => {
    expect(parseAnsi(`${E}38;5;208mo`)).toEqual([{ text: 'o', className: '', style: { color: 'rgb(255, 135, 0)' } }]);
    expect(parseAnsi(`${E}38;5;9mr`)).toEqual([{ text: 'r', className: 'ansi-fg-9' }]);
    expect(parseAnsi(`${E}48;2;255;107;107;1mb`)).toEqual([
      { text: 'b', className: 'ansi-bold', style: { backgroundColor: 'rgb(255, 107, 107)' } },
    ]);
    expect(color256(232)).toBe('rgb(8, 8, 8)');
    expect(color256(999)).toBe('rgb(238, 238, 238)');
  });

  it('andere Sequenzen und Steuerzeichen fallen weg', () => {
    expect(parseAnsi(`${E}2K${E}1Gfertig\u0007`)).toEqual([{ text: 'fertig', className: '' }]);
    expect(stripAnsi(`${E}33m[vite] WARN${E}0m x`)).toBe('[vite] WARN x');
    expect(parseAnsi('')).toEqual([]);
  });
});

describe('Konsole', () => {
  it('hängt neue Zeilen an und erkennt Lücken', () => {
    const a = mergeLines([line(1), line(2)], [line(3)]);
    expect(seqs(a.lines)).toEqual([1, 2, 3]);
    expect(a.gap).toBe(false);
    expect(mergeLines([line(1)], [line(4)]).gap).toBe(true);
    expect(mergeLines([], [line(7)]).gap).toBe(false);
  });

  it('sortiert Zeilen aus der Ladezeit über ihre Nummer ein, ohne Doppelte', () => {
    const loaded = [line(3), line(4), line(5)];
    const duringLoad = [line(5), line(2), line(6)];
    expect(seqs(mergeLines(loaded, duringLoad).lines)).toEqual([2, 3, 4, 5, 6]);
  });

  it('höchstens 2000 Zeilen, die neuesten bleiben', () => {
    const many = Array.from({ length: 2000 }, (_, i) => line(i + 1));
    const merged = mergeLines(many, [line(2001), line(2002)]).lines;
    expect(merged.length).toBe(2000);
    expect(merged[0].seq).toBe(3);
    expect(merged[1999].seq).toBe(2002);
  });

  it('Leeren blendet bis zur Nummer aus', () => {
    expect(seqs(visibleLines([line(1), line(2), line(3)], 2))).toEqual([3]);
    expect(seqs(visibleLines([line(1)], 0))).toEqual([1]);
  });

  it('Randmarke für WARN und ERROR, auch mit Farben', () => {
    expect(markOf('time=1 level=ERROR msg=x')).toBe('error');
    expect(markOf(`${E}33m[vite] WARN${E}0m Sourcemap`)).toBe('warn');
    expect(markOf('WARNING: veraltet')).toBe('warn');
    expect(markOf('ERRORS=0 und WARNUNG')).toBe(null);
  });
});

describe('Quellen', () => {
  const src = (name: string, kind: string, state: string): Source => ({ name, kind, state, detail: '' });

  it('Punkt-Töne je Art, unbekannt neutral', () => {
    expect(dotTone(src('Vite', 'service', 'läuft'))).toBe('ok');
    expect(dotTone(src('Vite', 'service', 'übernommen'))).toBe('warn');
    expect(dotTone(src('check:npm:test', 'run', 'running'))).toBe('info');
    expect(dotTone(src('check:npm:test', 'run', 'ok'))).toBe('ok');
    expect(dotTone(src('check:npm:test', 'run', 'failed'))).toBe('error');
    expect(dotTone(src('check:npm:test', 'run', 'timeout'))).toBe('error');
    expect(dotTone(src('k3c-dev', 'log', 'entries'))).toBe('ok');
    expect(dotTone(src('server', 'log', 'empty'))).toBe('neutral');
    expect(dotTone(src('x', 'run', 'rätselhaft'))).toBe('neutral');
    expect(dotTone(src('x', 'neu', 'ok'))).toBe('neutral');
  });

  it('gemerkte Auswahl, verschwundene fällt auf die erste zurück', () => {
    const list = [src('Vite', 'service', 'läuft'), src('k3c-dev', 'log', 'entries')];
    expect(pickSource(list, 'k3c-dev')).toBe('k3c-dev');
    expect(pickSource(list, 'check:weg')).toBe('Vite');
    expect(pickSource([], 'Vite')).toBe('');
  });

  it('ein neuer Lauf erscheint, ein bekannter wird ersetzt', () => {
    const list = [src('Vite', 'service', 'läuft')];
    const run = src('check:npm:test', 'run', 'running');
    const added = upsertSource(list, run);
    expect(added.map((s) => s.name)).toEqual(['Vite', 'check:npm:test']);
    expect(upsertSource(added, { ...run, state: 'ok' })[1].state).toBe('ok');
  });
});
