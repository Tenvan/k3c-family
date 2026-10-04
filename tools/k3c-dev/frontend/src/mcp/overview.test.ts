import { describe, expect, it } from 'vitest';
import type { ToolStats } from '../api';
import { parseInline, parseMarkdown } from './markdown';
import { durationText, share, uptimeText, weightedAvg } from './overview';

const tool = (name: string, calls: number, avgMs = 0, errors = 0): ToolStats => ({
  name, description: `${name} tut etwas`, calls, errors, avgMs, lastCall: '',
});

describe('Markdown-Teilmenge', () => {
  it('Inline: code und fett', () => {
    expect(parseInline('nutze `check_run` statt **Shell**.')).toEqual([
      { kind: 'text', text: 'nutze ' },
      { kind: 'code', text: 'check_run' },
      { kind: 'text', text: ' statt ' },
      { kind: 'bold', text: 'Shell' },
      { kind: 'text', text: '.' },
    ]);
    expect(parseInline('<b>kein HTML</b>')).toEqual([{ kind: 'text', text: '<b>kein HTML</b>' }]);
  });

  it('Blöcke: Überschriften, Absatz, Liste mit Folgezeile, Codeblock', () => {
    const md = '# Titel\n\nZeile eins\nZeile zwei\n\n## Prüfen\n\n- `a` erster\n  weiter\n- zweiter\n\n```\nx < y\n```';
    expect(parseMarkdown(md)).toEqual([
      { kind: 'h', level: 1, inline: [{ kind: 'text', text: 'Titel' }] },
      { kind: 'p', inline: [{ kind: 'text', text: 'Zeile eins Zeile zwei' }] },
      { kind: 'h', level: 2, inline: [{ kind: 'text', text: 'Prüfen' }] },
      { kind: 'ul', items: [
        [{ kind: 'code', text: 'a' }, { kind: 'text', text: ' erster weiter' }],
        [{ kind: 'text', text: 'zweiter' }],
      ] },
      { kind: 'pre', text: 'x < y' },
    ]);
  });

  it('Absatz direkt nach einer Liste', () => {
    expect(parseMarkdown('- a\nText').map((b) => b.kind)).toEqual(['ul', 'p']);
  });
});

describe('Übersicht', () => {
  it('Anteil und gewichtetes Ø ohne Division durch null', () => {
    expect(share(1, 4)).toBe(25);
    expect(share(0, 0)).toBe(0);
    expect(weightedAvg([tool('a', 3, 10), tool('b', 1, 50)])).toBe(20);
    expect(weightedAvg([tool('a', 0, 0)])).toBe(null);
    expect(durationText(null)).toBe('–');
  });

  it('Laufzeit', () => {
    const now = Date.parse('2026-09-30T12:00:00Z');
    expect(uptimeText('2026-09-30T09:47:00Z', now)).toBe('2 h 13 min');
    expect(uptimeText('kaputt', now)).toBe('–');
  });
});
