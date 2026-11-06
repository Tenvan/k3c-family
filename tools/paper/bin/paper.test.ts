// RD1.1: Render-CLI validate und render nach dem Vertrag (paper-spec.md › 8), in-process über run().
import { copyFileSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { run } from './paper';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const antwort = (args: string[]) => {
  const r = run(args);
  return { ...r, json: r.stdout ? JSON.parse(r.stdout) : undefined };
};
const ERWARTET: Record<string, string> = {
  'hex-farbe': 'hex-farbe', 'unbekanntes-feld': 'schema', 'fremdes-material': 'material-unbekannt',
  'pflicht-teil-fehlt': 'pflicht-teil-fehlt', proportion: 'proportion',
};

describe('paper-CLI', { timeout: 60_000 }, () => {
  it('validate monarch: Exit 0, {ok: true, fehler: []}', () => {
    expect(antwort(['validate', join(SPECS, 'monarch.paper.json')])).toMatchObject({ code: 0, stderr: '', json: { ok: true, fehler: [] } });
  });

  it.each(Object.entries(ERWARTET))('validate ungültige Fixture %s: Exit 1 mit genau der Regel %s', (datei, regel) => {
    const r = antwort(['validate', join(SPECS, `ungueltig/${datei}.paper.json`)]);
    expect(r.code).toBe(1);
    expect(r.json.ok).toBe(false);
    expect([...new Set(r.json.fehler.map((f: { regel: string }) => f.regel))]).toEqual([regel]);
    expect(Object.keys(r.json.fehler[0]).sort()).toEqual(['feld', 'regel', 'text']);
  });

  it('render mauer: Exit 0 mit manifeste und dateien', () => {
    const dir = mkdtempSync(join(tmpdir(), 'paper-cli-'));
    copyFileSync(join(SPECS, 'mauer.paper.json'), join(dir, 'mauer.paper.json'));
    const r = antwort(['render', join(dir, 'mauer.paper.json')]);
    expect(r.code).toBe(0);
    expect(r.json.ok).toBe(true);
    expect(r.json.manifeste).toHaveLength(1);
    expect(r.json.dateien).toHaveLength(2);
    expect(readdirSync(join(dir, 'mauer')).sort()).toEqual(['manifest.json', 'stand.png', 'stand.s2.png']);
  });

  it('unbekannter Befehl und fehlende Datei: Exit 2, nichts auf stdout', () => {
    expect(run(['zeichne'])).toMatchObject({ code: 2, stdout: '' });
    expect(run([])).toMatchObject({ code: 2, stdout: '' });
    expect(run(['validate', join(SPECS, 'gibts-nicht.paper.json')])).toMatchObject({ code: 2, stdout: '' });
    expect(run(['render', '--animation'])).toMatchObject({ code: 2, stdout: '' });
    expect(run(['validate']).stderr).not.toBe('');
  });
});

describe('validate prüft auch die Render-Regeln (Sprint-Review RD1.5)', { timeout: 60_000 }, () => {
  const mit = (id: string, aendern: (s: Record<string, any>) => void, extra: string[] = []) => {
    // Eigene Gruppen-Ebene: die Reittier-Suche schaut auch in Nachbar-Ordner (sonst fände sie andere Temp-Tests).
    const dir = join(mkdtempSync(join(tmpdir(), 'paper-validate-')), 'gruppe');
    mkdirSync(dir);
    const s = JSON.parse(readFileSync(join(SPECS, `${id}.paper.json`), 'utf8'));
    aendern(s);
    writeFileSync(join(dir, `${id}.paper.json`), JSON.stringify(s));
    for (const e of extra) copyFileSync(join(SPECS, `${e}.paper.json`), join(dir, `${e}.paper.json`));
    return antwort(['validate', join(dir, `${id}.paper.json`)]);
  };
  const regeln = (r: { json: { fehler: { regel: string }[] } }) => r.json.fehler.map((f) => f.regel);

  it('figur-Animation ohne Keyframes außer idle/lauf: Exit 1 animation-unbekannt', () => {
    const r = mit('monarch', (s) => { s.animationen.winken = { frames: 4 }; }, ['pferd']);
    expect(r.code).toBe(1);
    expect(regeln(r)).toEqual(['animation-unbekannt']);
  });

  it('Reittier fehlt: Exit 1 reittier-unbekannt', () => {
    const r = mit('monarch', () => {});
    expect(r.code).toBe(1);
    expect(regeln(r)).toEqual(['reittier-unbekannt']);
  });

  it('vierbeiner ohne gangart oder ohne bein: Exit 1', () => {
    expect(regeln(mit('pferd', (s) => { s.animationen.stand = { frames: 1 }; }))).toEqual(['animation-unbekannt']);
    expect(regeln(mit('pferd', (s) => { delete s.bein; }))).toEqual(['pflicht-teil-fehlt']);
  });

  it('gültige Specs bleiben gültig', () => {
    expect(mit('monarch', () => {}, ['pferd']).code).toBe(0);
    expect(mit('mauer', () => {}).code).toBe(0);
  });
});
