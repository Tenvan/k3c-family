// RD1.4 (AC-05): Ein Vertragstest für Attrappe (PV1.4) und echten Renderer – CLI-Vertrag docs/standards/paper-spec.md › 8.
import { copyFileSync, existsSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { run as stubRun } from '../fixtures/cli-stub';
import { run } from '../bin/paper';

const FIX = join(import.meta.dirname, '../fixtures');
const SPECS = join(FIX, 'specs');
const STIL = join(import.meta.dirname, '../../../public/grafik/stil.paper.json');
const lies = (p: string) => JSON.parse(readFileSync(p, 'utf8'));
type Run = (args: string[]) => { code: number; stdout: string; stderr: string };
const pfad = expect.stringMatching(/^[^\\]+$/);

/** Temp-Ordner mit Kopien der Fixture-Specs (render schreibt neben die Spec). */
function ordner(...ids: string[]) {
  const dir = mkdtempSync(join(tmpdir(), 'paper-vertrag-'));
  for (const id of ids) copyFileSync(join(SPECS, `${id}.paper.json`), join(dir, `${id}.paper.json`));
  return dir;
}

describe.each([['Attrappe', stubRun as Run], ['Renderer', run as Run]])('CLI-Vertrag: %s', { timeout: 60_000 }, (_name, cli) => {
  const json = (args: string[]) => {
    const r = cli(args);
    return { ...r, json: r.stdout ? JSON.parse(r.stdout) : undefined };
  };

  it('validate: ok-Form und Exit 1 mit fehler[{feld, regel, text}]', () => {
    expect(json(['validate', join(SPECS, 'mauer.paper.json')])).toMatchObject({ code: 0, json: { ok: true, fehler: [] } });
    const r = json(['validate', join(SPECS, 'ungueltig/fremdes-material.paper.json')]);
    expect(r.code).toBe(1);
    expect(r.json.ok).toBe(false);
    expect(r.json.fehler[0]).toEqual({ feld: expect.any(String), regel: 'material-unbekannt', text: expect.any(String) });
  });

  it('render: {ok, manifeste: string[], dateien: string[]}, Pfade mit /', () => {
    const r = json(['render', join(ordner('mauer'), 'mauer.paper.json')]);
    expect(r.code).toBe(0);
    expect(r.json).toEqual({ ok: true, manifeste: [pfad], dateien: [pfad, pfad] });
    expect(r.json.manifeste[0]).toMatch(/mauer\/manifest\.json$/);
  }, 30_000);

  it('frame: {ok, datei, teile: {<teilId>: [x, y, b, h]}} mit ganzzahligen Pixeln; unbekannte Animation Exit 1 animation-unbekannt', () => {
    const dir = ordner('mauer'), out = join(dir, 'f.png');
    const r = json(['frame', join(dir, 'mauer.paper.json'), '--animation', 'stand', '--frame', '0', '--out', out]);
    expect(r.code).toBe(0);
    expect(r.json).toEqual({ ok: true, datei: pfad, teile: { korpus: [expect.any(Number), expect.any(Number), expect.any(Number), expect.any(Number)] } });
    expect(r.json.teile.korpus.every(Number.isInteger)).toBe(true);
    expect(existsSync(out)).toBe(true);
    const f = json(['frame', join(dir, 'mauer.paper.json'), '--animation', 'lauf', '--frame', '0', '--out', out]);
    expect(f).toMatchObject({ code: 1, json: { ok: false, fehler: [{ regel: 'animation-unbekannt' }] } });
  }, 30_000);

  it('diff: felder[{pfad, alt, neu}], teile über id, stil true bei anderem stilHash', () => {
    const dir = ordner(), alt = join(SPECS, 'monarch.paper.json'), neu = join(dir, 'monarch.paper.json');
    const m = lies(alt);
    m.umhang.material = 'purpur';
    m.teile.find((t: { id: string }) => t.id === 'kopf').lagen[0].material = 'gold';
    writeFileSync(neu, JSON.stringify(m));
    const stilNeu = join(dir, 'stil.json'), stil = lies(STIL);
    writeFileSync(stilNeu, JSON.stringify({ ...stil, papier: { ...stil.papier, falz_deckkraft: 0.2 } }));
    const r = json(['diff', alt, neu, '--stil-alt', STIL, '--stil-neu', stilNeu]);
    expect(r.code).toBe(0);
    expect(r.json.felder).toEqual(expect.arrayContaining([{ pfad: 'umhang.material', alt: 'koenigsrot', neu: 'purpur' }]));
    expect(r.json.teile).toEqual(['kopf']);
    expect(r.json.stil).toBe(true);
    expect(json(['diff', alt, alt]).json).toEqual({ ok: true, felder: [], teile: [], stil: false });
  });

  it('pixeldiff: anteil und boxen wie fixtures/ui/AssetsPixelDiff.json, Overlay geschrieben', () => {
    const out = join(ordner(), 'o.png');
    const r = json(['pixeldiff', join(FIX, 'render/monarch/monarch-galopp-0.png'), join(FIX, 'render/monarch/monarch-galopp-0-purpur.png'), '--out', out]);
    const ui = lies(join(FIX, 'ui/AssetsPixelDiff.json'));
    expect(r.code).toBe(0);
    expect([r.json.anteil, r.json.boxen]).toEqual([ui.anteil, ui.boxen]);
    expect(r.json.datei).toEqual(pfad);
    expect(existsSync(out)).toBe(true);
  });

  it('Aufruffehler: Exit 2, stdout leer', () => {
    expect(cli(['frame', join(SPECS, 'mauer.paper.json')])).toMatchObject({ code: 2, stdout: '' });
    expect(cli(['pixeldiff', join(SPECS, 'mauer.paper.json')])).toMatchObject({ code: 2, stdout: '' });
    expect(cli(['zeichne'])).toMatchObject({ code: 2, stdout: '' });
    expect(cli(['validate', join(SPECS, 'gibts-nicht.paper.json')])).toMatchObject({ code: 2, stdout: '' });
  });
});
