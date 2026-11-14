// B-856: Sammelaufruf `batch <schritte.json>` der Render-CLI, in-process über run() von paper.ts und der Attrappe.
import { mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { run } from '../bin/paper';
import { run as stub } from '../fixtures/cli-stub';
import type { Schritt } from './batch';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const GUELTIG = join(SPECS, 'mauer.paper.json'), UNGUELTIG = join(SPECS, 'ungueltig/hex-farbe.paper.json');

/** Schreibt die Schritte in eine Temp-Datei und ruft `batch` mit `lauf` auf. */
function sammel(schritte: unknown, lauf = run) {
  const datei = join(mkdtempSync(join(tmpdir(), 'paper-batch-')), 'schritte.json');
  writeFileSync(datei, JSON.stringify(schritte));
  const r = lauf(['batch', datei]);
  return { ...r, json: r.stdout ? JSON.parse(r.stdout) : undefined };
}

describe('paper batch (B-856)', { timeout: 60_000 }, () => {
  it.each([['paper.ts', run], ['Attrappe', stub]])('%s: Antwort je Schritt gleich dem Einzelaufruf', (_, lauf) => {
    const schritte: Schritt[] = [
      { args: ['validate', GUELTIG] },
      { args: ['validate', UNGUELTIG] },
      { args: ['validate', join(SPECS, 'gibts-nicht.paper.json')] },
    ];
    const r = sammel(schritte, lauf);
    expect(r.code).toBe(0);
    expect(r.json.ok).toBe(true);
    expect(r.json.schritte).toHaveLength(3);
    schritte.forEach((s, i) => {
      const einzeln = lauf(s.args);
      const erwartet = einzeln.code === 2 ? { code: 2, stderr: einzeln.stderr } : { code: einzeln.code, antwort: JSON.parse(einzeln.stdout) };
      expect(r.json.schritte[i]).toEqual(erwartet);
    });
  });

  it('stopp überspringt den Rest, braucht überspringt abhängige Schritte', () => {
    const r = sammel([
      { args: ['validate', UNGUELTIG] },
      { args: ['validate', GUELTIG], braucht: [0] },
      { args: ['validate', GUELTIG] },
      { args: ['validate', UNGUELTIG], stopp: true },
      { args: ['validate', GUELTIG] },
    ]);
    expect(r.json.schritte.map((s: { code?: number; uebersprungen?: boolean }) => s.uebersprungen ?? s.code)).toEqual([1, true, 0, 1, true]);
  });

  it('falsche Eingabe: Exit 2, nichts auf stdout', () => {
    expect(sammel({ args: ['validate'] })).toMatchObject({ code: 2, stdout: '' });
    expect(sammel([{ args: [] }])).toMatchObject({ code: 2, stdout: '' });
    expect(sammel([{ args: ['validate', GUELTIG], braucht: [0] }])).toMatchObject({ code: 2, stdout: '' });
    expect(sammel([{ args: ['batch', 'x.json'] }])).toMatchObject({ code: 2, stdout: '' });
    expect(run(['batch', join(SPECS, 'gibts-nicht.json')])).toMatchObject({ code: 2, stdout: '' });
  });
});
