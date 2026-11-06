// AC-03 (PV1.3): Hash-Regel – CRLF und Schlüsselreihenfolge ändern specHash nicht, Stil und Renderer ändern renderKey.
import { describe, expect, it } from 'vitest';
import { canonical, num, renderKey, RENDERER_VERSION, specHash } from './hash';

const spec = '{\n  "id": "x",\n  "teile": [{ "id": "a", "pivot": [0, -1.5] }],\n  "beschreibung": "zwei\\nZeilen"\n}\n';

describe('specHash', () => {
  it('gleich für LF und CRLF', () => {
    expect(specHash(spec.replace(/\n/g, '\r\n'))).toBe(specHash(spec));
  });

  it('gleich für umsortierte Schlüssel und ohne $schema', () => {
    const umsortiert = '{"beschreibung":"zwei\\nZeilen","teile":[{"pivot":[0,-1.5],"id":"a"}],"id":"x","$schema":"s.json"}';
    expect(specHash(umsortiert)).toBe(specHash(spec));
  });

  it('CRLF in Texten zählt als LF; andere Werte ändern den Hash', () => {
    expect(specHash(spec.replace('\\n', '\\r\\n'))).toBe(specHash(spec));
    expect(specHash(spec.replace('-1.5', '-1.6'))).not.toBe(specHash(spec));
    expect(specHash(spec)).toMatch(/^[0-9a-f]{64}$/);
  });

  it('kanonische Form ohne Leerraum, Reihenfolge von Listen bleibt', () => {
    expect(canonical({ b: [2, 1], a: { d: 1, c: 'x' } })).toBe('{"a":{"c":"x","d":1},"b":[2,1]}');
  });
});

describe('renderKey und Koordinaten', () => {
  it('hängt an Spec, Stil und Renderer-Version', () => {
    const k = renderKey('a', 'b');
    expect(k).toBe(renderKey('a', 'b', RENDERER_VERSION));
    expect(new Set([k, renderKey('a', 'c'), renderKey('c', 'b'), renderKey('a', 'b', 'paper-1')]).size).toBe(4);
  });

  it('num rundet auf zwei Nachkommastellen', () => {
    expect([num(1.005), num(-0.001), num(2.5), num(-12.3456), num(3)]).toEqual(['1', '0', '2.5', '-12.35', '3']);
  });
});
