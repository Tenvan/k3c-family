import { describe, expect, it } from 'vitest';
import { chunkMarks, slotHints } from './chunkMarks';
import type { ModelChunk } from './levelView';

// B-485: Hinweise am Chunk der Chunk-Vorschau (reine Logik): Platz -> Markierung, dann Platz -> Abschnitt des Levels.
// wie im Spiel: Chunks 50 Units breit, zwei Hub-Abschnitte (5 und 6)
const chunks: ModelChunk[] = Array.from({ length: 12 }, (_, i) => ({ index: i, kind: i === 5 || i === 6 ? 'hub' : 'normal', from: i * 50, to: i * 50 + 50 }));
const level = { chunks };

describe('Hinweise je Platz', () => {
  it('ohne Planung keine Hinweise', () => {
    expect(slotHints([], { ok: true, slot: 0, error: '' })).toEqual([]);
  });

  it('jeder geplante Chunk wird „planned“ mit Name, Seed und festen Objekten im Tooltip', () => {
    const [a, b] = slotHints([{ slot: -1, kind: 'clearing' }, { slot: 2, kind: 'rocks', seed: 'x', objects: [{ kind: 'tree', at: 3 }] }], { ok: true, slot: 0, error: '' });
    expect(a).toEqual({ slot: -1, kind: 'planned', label: 'Geplant: Lichtung · ohne Chunk-Seed · ohne feste Objekte' });
    expect(b).toEqual({ slot: 2, kind: 'planned', label: 'Geplant: Felsen · Chunk-Seed „x“ · 1 festes Objekt' });
  });

  it('ein Prüffehler am Platz wird „error“ (statt „planned“), ein Fehler ohne Platz bleibt weg', () => {
    expect(slotHints([{ slot: 1, kind: 'volcano' }], { ok: false, slot: 1, error: 'unbekannt' })).toEqual([{ slot: 1, kind: 'error', label: 'Fehler: unbekannt' }]);
    expect(slotHints([{ slot: 1, kind: 'forest' }], { ok: false, slot: 0, error: 'Summe' })).toEqual([{ slot: 1, kind: 'planned', label: expect.stringContaining('Wald') }]);
  });
});

describe('Platz -> Abschnitt (rules/geplante-chunks.md § 1)', () => {
  it('+n ist der n-te Abschnitt rechts der Hub-Abschnitte, −n der n-te links davon', () => {
    const marks = chunkMarks(level, [{ slot: 1, kind: 'planned', label: 'a' }, { slot: -1, kind: 'error', label: 'b' }, { slot: 2, kind: 'planned', label: 'c' }]);
    expect([...marks.entries()]).toEqual([[7, { slot: 1, kind: 'planned', label: 'a' }], [4, { slot: -1, kind: 'error', label: 'b' }], [8, { slot: 2, kind: 'planned', label: 'c' }]]);
    // Anfang von +1 = Hub-Mitte (300) + 50, von −1 = Hub-Mitte − 50 − Breite: genau die Abschnittsanfänge
    expect(chunks[7].from).toBe(300 + 50);
    expect(chunks[4].from).toBe(300 - 50 - 50);
  });

  it('ohne Hub-Abschnitt keine Markierung', () => {
    expect(chunkMarks({ chunks: chunks.map((c) => ({ ...c, kind: 'normal' })) }, [{ slot: 1, kind: 'planned', label: '' }]).size).toBe(0);
  });

  it('Plätze außerhalb des Levels und Platz 0 bleiben ohne Abschnitt', () => {
    expect(chunkMarks(level, [{ slot: 30, kind: 'planned', label: '' }, { slot: 0, kind: 'error', label: '' }]).size).toBe(0);
  });
});
