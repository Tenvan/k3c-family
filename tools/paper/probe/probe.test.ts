// AC-05 (PV1.3): resvg rendert die Fixture-SVGs zweimal byte-gleich und gleich den eingecheckten Golden-Hashes
// (unter Windows erzeugt). Läuft unverändert in der CI auf Linux; Ergebnis dort entscheidet die Golden-Regel.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { HOEHE, quellen, render, RENDER_DIR, sha256 } from './resvg-probe';

const hashes = JSON.parse(readFileSync(join(RENDER_DIR, 'hashes.json'), 'utf8')) as Record<string, string>;

describe('resvg-Probe', () => {
  it('kennt alle sechs Quell-SVGs', () => {
    expect(quellen().map((q) => `${q.asset}/${q.name}.png`).sort()).toEqual(Object.keys(hashes).sort());
    expect(quellen()).toHaveLength(6);
  });

  it.each(quellen())('$name: zwei Läufe byte-gleich, Hash wie hashes.json, PNG wie eingecheckt', (q) => {
    const a = render(q.datei), b = render(q.datei);
    expect(a.hoehe).toBe(HOEHE);
    expect(a.png.equals(b.png)).toBe(true);
    expect(sha256(a.png)).toBe(hashes[`${q.asset}/${q.name}.png`]);
    expect(sha256(readFileSync(join(RENDER_DIR, q.asset, `${q.name}.png`)))).toBe(sha256(a.png));
  });
});
