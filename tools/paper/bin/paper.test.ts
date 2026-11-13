// RD1.1: Render-CLI validate und render nach dem Vertrag (paper-spec.md › 8), in-process über run().
import { copyFileSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { ROOT, STIL_PFAD } from '../render/stil';
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

  it('B-708/AC-01: Reittier fehlt → feld nennt die Animation (animationen.schritt.reittier)', () => {
    const r = mit('monarch', (s) => { s.animationen.schritt = { frames: 8, reittier: 'esel', gangart: 'schritt' }; }, ['pferd']);
    expect(r.code).toBe(1);
    expect(r.json.fehler[0]).toMatchObject({ feld: 'animationen.schritt.reittier', regel: 'reittier-unbekannt' });
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

describe('frame bei Reit-Animationen: Boxen des Reittiers (B-709/AC-01)', { timeout: 60_000 }, () => {
  it('paper frame monarch --animation galopp liefert Boxen reittier:<teil>, auch Beine und verdeckte Teile', () => {
    const out = join(mkdtempSync(join(tmpdir(), 'paper-reit-')), 'f.png');
    const r = antwort(['frame', join(ROOT, 'public/grafik/figuren/monarch.paper.json'), '--animation', 'galopp', '--frame', '2', '--out', out]);
    expect(r.code).toBe(0);
    const ids = Object.keys(r.json.teile), reittier = ids.filter((id) => id.startsWith('reittier:'));
    for (const t of ['rumpf-vorn', 'hals', 'oberbein-hf', 'unterbein-hf', 'huf-hf', 'oberbein-vn', 'unterbein-vn'])
      expect(reittier, t).toContain(`reittier:${t}`);
    // Reiter-Teile bleiben ohne Präfix und werden nicht von gleichnamigen Reittier-Teilen überschrieben
    expect(ids).toContain('rumpf');
    for (const id of reittier) {
      const [, , b, h] = r.json.teile[id];
      expect(b > 0 && h > 0, id).toBe(true);
    }
  });
});

describe('frame --ohne mit Teilen des Reittiers (B-845/AC-01)', { timeout: 60_000 }, () => {
  const monarch = join(ROOT, 'public/grafik/figuren/monarch.paper.json');
  const frame = (ohne: string, animation = 'galopp') => {
    const out = join(mkdtempSync(join(tmpdir(), 'paper-ohne-')), 'f.png');
    return { out, r: antwort(['frame', monarch, '--animation', animation, '--frame', '2', '--ohne', ohne, '--out', out]) };
  };

  it('--ohne reittier:hals bei galopp: Exit 0, Box fehlt, andere Boxen bleiben, Bild anders', () => {
    const vollPng = join(mkdtempSync(join(tmpdir(), 'paper-ohne-')), 'v.png');
    const voll = antwort(['frame', monarch, '--animation', 'galopp', '--frame', '2', '--out', vollPng]);
    const { out, r } = frame('reittier:hals');
    expect(r.code).toBe(0);
    const erwartet = Object.keys(voll.json.teile).filter((id) => id !== 'reittier:hals');
    expect(Object.keys(r.json.teile).sort()).toEqual(erwartet.sort());
    for (const id of erwartet) expect(r.json.teile[id], id).toEqual(voll.json.teile[id]);
    expect(readFileSync(out).equals(readFileSync(vollPng))).toBe(false);
  });

  it('--ohne reittier:oberbein-vn nimmt den Beinpfad: beide Bein-Boxen fehlen', () => {
    const { r } = frame('reittier:oberbein-vn,kopf');
    expect(r.code).toBe(0);
    for (const id of ['reittier:oberbein-vn', 'reittier:unterbein-vn', 'kopf']) expect(r.json.teile, id).not.toHaveProperty(id);
    expect(r.json.teile).toHaveProperty(['reittier:oberbein-vf']);
  });

  it('unbekanntes reittier:-Teil → Exit 1 teil-unbekannt mit Liste der Reittier-Teile', () => {
    const { r } = frame('reittier:fluegel');
    expect(r.code).toBe(1);
    expect(r.json.fehler[0]).toMatchObject({ regel: 'teil-unbekannt' });
    expect(r.json.fehler[0].text).toContain('reittier:fluegel');
    expect(r.json.fehler[0].text).toContain('reittier:hals');
  });

  it('reittier:-Teil bei einer Animation ohne Reittier → Exit 1 teil-unbekannt', () => {
    expect(frame('reittier:hals', 'idle').r).toMatchObject({ code: 1, json: { fehler: [{ regel: 'teil-unbekannt' }] } });
  });
});

describe('--stil <datei> für frame und render (B-705/AC-01)', { timeout: 60_000 }, () => {
  const mauer = () => {
    const dir = mkdtempSync(join(tmpdir(), 'paper-stil-'));
    copyFileSync(join(SPECS, 'mauer.paper.json'), join(dir, 'mauer.paper.json'));
    return dir;
  };
  /** Kopie des Standard-Stils, alle Materialien rot */
  const roterStil = (dir: string) => {
    const stil = JSON.parse(readFileSync(STIL_PFAD, 'utf8'));
    for (const m of Object.values(stil.materialien) as Record<string, string>[]) for (const t of Object.keys(m)) m[t] = '#cc2222';
    writeFileSync(join(dir, 'rot.json'), JSON.stringify(stil));
    return join(dir, 'rot.json');
  };
  const frame = (dir: string, out: string, extra: string[] = []) =>
    antwort(['frame', join(dir, 'mauer.paper.json'), '--animation', 'stand', '--frame', '0', '--out', join(dir, out), ...extra]);

  it('frame: ohne Option wie --stil public/grafik/stil.paper.json, mit eigenem Stil anderes Bild', () => {
    const dir = mauer();
    expect(frame(dir, 'a.png').code).toBe(0);
    expect(frame(dir, 'b.png', ['--stil', STIL_PFAD]).code).toBe(0);
    expect(frame(dir, 'c.png', ['--stil', roterStil(dir)]).code).toBe(0);
    const [a, b, c] = ['a', 'b', 'c'].map((n) => readFileSync(join(dir, `${n}.png`)));
    expect(a.equals(b)).toBe(true);
    expect(a.equals(c)).toBe(false);
  });

  it('render: --stil wirkt auf Bild und stilHash im Manifest', () => {
    const dir = mauer(), manifest = () => JSON.parse(readFileSync(join(dir, 'mauer/manifest.json'), 'utf8'));
    expect(antwort(['render', join(dir, 'mauer.paper.json')]).code).toBe(0);
    const standard = manifest().stilHash, png = readFileSync(join(dir, 'mauer/stand.png'));
    expect(antwort(['render', join(dir, 'mauer.paper.json'), '--stil', roterStil(dir)]).code).toBe(0);
    expect(manifest().stilHash).not.toBe(standard);
    expect(readFileSync(join(dir, 'mauer/stand.png')).equals(png)).toBe(false);
  });

  it('fehlende Datei → Exit 2 mit Pfad; ungültiger Stil → Exit 1 mit Dateinamen im Feld', () => {
    const dir = mauer(), fehlt = join(dir, 'gibts-nicht.json');
    const r = frame(dir, 'a.png', ['--stil', fehlt]);
    expect(r).toMatchObject({ code: 2, stdout: '' });
    expect(r.stderr).toContain(fehlt);
    writeFileSync(join(dir, 'kaputt.json'), JSON.stringify({ materialien: {} }));
    const k = antwort(['render', join(dir, 'mauer.paper.json'), '--stil', join(dir, 'kaputt.json')]);
    expect(k.code).toBe(1);
    expect(k.json.fehler[0].feld).toMatch(/^kaputt\.json:/);
  });
});
