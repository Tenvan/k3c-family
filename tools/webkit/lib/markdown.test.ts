import { describe, expect, it } from 'vitest';
import { parseMarkdown } from './markdown';

describe('markdown', () => {
  it('liest Tabellen, Links als Text und Überschriften bis Ebene 4', () => {
    const b = parseMarkdown('#### Titel\n\n| A | B |\n|---|---|\n| [x](y.md) | `c` |\n| 1 | 2 |\nDanach');
    expect(b.map((x) => x.kind)).toEqual(['h', 'table', 'p']);
    const t = b[1];
    if (t.kind !== 'table') throw new Error('Tabelle erwartet');
    expect(t.head).toHaveLength(2);
    expect(t.rows).toHaveLength(2);
    expect(t.rows[0][0]).toEqual([{ kind: 'text', text: 'x' }]);
  });

  it('macht aus nummerierten Zeilen einen ol-Block mit allen Punkten (B-213/AC-01)', () => {
    const b = parseMarkdown('1. Status setzen\n2. Prüfen');
    expect(b).toEqual([{ kind: 'ol', items: [[{ kind: 'text', text: 'Status setzen' }], [{ kind: 'text', text: 'Prüfen' }]] }]);
  });

  it('ersetzt [ ]/[x] am Punktanfang durch ☐/☑ (B-213/AC-02)', () => {
    const b = parseMarkdown('- [x] AC-01: fertig\n- [ ] AC-02: offen\n1. [X] nummeriert');
    expect(b.map((x) => x.kind)).toEqual(['ul', 'ol']);
    const [ul, ol] = b;
    if (ul.kind !== 'ul' || ol.kind !== 'ol') throw new Error('Listen erwartet');
    expect(ul.items[0][0]).toEqual({ kind: 'text', text: '☑ AC-01: fertig' });
    expect(ul.items[1][0]).toEqual({ kind: 'text', text: '☐ AC-02: offen' });
    expect(ol.items[0][0]).toEqual({ kind: 'text', text: '☑ nummeriert' });
  });

  it('lässt [ ], [x] und Zahlen mit Punkt im Fließtext unverändert (B-213/AC-02)', () => {
    const b = parseMarkdown('Ein [ ] Kästchen und [x] hier\n2026. war ein Jahr');
    expect(b).toEqual([{ kind: 'p', inline: [{ kind: 'text', text: 'Ein [ ] Kästchen und [x] hier 2026. war ein Jahr' }] }]);
    expect(parseMarkdown('Im Jahr 2026. war viel los').map((x) => x.kind)).toEqual(['p']);
  });

  it('trennt eine --Liste und eine direkt folgende 1.-Liste in zwei Blöcke', () => {
    expect(parseMarkdown('- a\n- b\n1. c\n2. d\n- e').map((x) => x.kind)).toEqual(['ul', 'ol', 'ul']);
  });
});
