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
});
