// AC-06 (PV1.4): CLI-Attrappe, RPC-Beispielantworten, MCP-Beispiele und Review-Test-Repo passen zum Vertrag
// docs/standards/paper-mcp.md und docs/standards/paper-spec.md › CLI-Vertrag.
import { execFileSync } from 'node:child_process';
import { existsSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { run } from './cli-stub';
import { erzeugeReviewRepo } from './review-repo/erzeuge';

const FIX = import.meta.dirname;
const ROOT = join(FIX, '../../..');
const SPECS = join(FIX, 'specs');
const tmp = () => mkdtempSync(join(tmpdir(), 'paper-stub-'));
const antwort = (args: string[]) => {
  const r = run(args);
  return { ...r, json: r.stdout ? JSON.parse(r.stdout) : undefined };
};
const lies = (p: string) => JSON.parse(readFileSync(p, 'utf8'));

describe('CLI-Attrappe', () => {
  it('validate: monarch ok, ungültige Fixture Exit 1 mit Regel', () => {
    expect(antwort(['validate', join(SPECS, 'monarch.paper.json')])).toMatchObject({ code: 0, json: { ok: true, fehler: [] } });
    const r = antwort(['validate', join(SPECS, 'ungueltig/fremdes-material.paper.json')]);
    expect(r.code).toBe(1);
    expect(r.json).toMatchObject({ ok: false, fehler: [{ regel: 'material-unbekannt' }] });
  });

  it('neue gültige Spec: render und frame mit Platzhalter-PNG', () => {
    const dir = tmp(), spec = join(dir, 'kiste.paper.json');
    const kiste = { ...lies(join(SPECS, 'ungueltig/fremdes-material.paper.json')), id: 'kiste' };
    kiste.teile[0].lagen[0].material = 'holz';
    kiste.animationen = { stand: { frames: 1 } };
    writeFileSync(spec, JSON.stringify(kiste));
    expect(antwort(['render', spec])).toMatchObject({ code: 0, json: { ok: true, manifeste: [], dateien: ['tools/paper/fixtures/render/platzhalter.png'] } });
    const f = antwort(['frame', spec, '--animation', 'stand', '--frame', '0', '--out', join(dir, 'f.png')]);
    expect(f.json).toMatchObject({ ok: true, teile: { koerper: expect.any(Array) } });
    expect(existsSync(join(dir, 'f.png'))).toBe(true);
    expect(antwort(['frame', spec, '--animation', 'lauf', '--frame', '0', '--out', join(dir, 'g.png')]).code).toBe(1);
  });

  it('render eines bekannten Asset liefert Manifest und PNGs aus dem Manifest', () => {
    const r = antwort(['render', join(SPECS, 'mauer.paper.json')]);
    expect(r.json).toEqual({ ok: true, manifeste: ['tools/paper/fixtures/render/mauer/manifest.json'],
      dateien: ['tools/paper/fixtures/render/mauer/mauer-1.png', 'tools/paper/fixtures/render/mauer/mauer-2.png'] });
  });

  it('diff nennt Felder, Teile und Stil', () => {
    const dir = tmp(), alt = join(SPECS, 'monarch.paper.json'), neu = join(dir, 'monarch.paper.json');
    const m = lies(alt);
    m.umhang.material = 'purpur';
    m.teile.find((t: { id: string }) => t.id === 'kopf').lagen[0].material = 'gold';
    writeFileSync(neu, JSON.stringify(m));
    const stilNeu = join(dir, 'stil.json'), stil = lies(join(ROOT, 'public/grafik/stil.paper.json'));
    writeFileSync(stilNeu, JSON.stringify({ ...stil, licht: 'rechts-oben', papier: { ...stil.papier, falz_deckkraft: 0.2 } }));
    const r = antwort(['diff', alt, neu, '--stil-alt', join(ROOT, 'public/grafik/stil.paper.json'), '--stil-neu', stilNeu]);
    expect(r.json.felder).toEqual(expect.arrayContaining([{ pfad: 'umhang.material', alt: 'koenigsrot', neu: 'purpur' }]));
    expect(r.json.teile).toEqual(['kopf']);
    expect(r.json.stil).toBe(true);
  });

  it('pixeldiff wie AssetsPixelDiff; Aufruffehler Exit 2 ohne stdout', () => {
    const r = antwort(['pixeldiff', join(FIX, 'render/monarch/monarch-galopp-0.png'), join(FIX, 'render/monarch/monarch-galopp-0-purpur.png'), '--out', join(tmp(), 'o.png')]);
    const ui = lies(join(FIX, 'ui/AssetsPixelDiff.json'));
    expect([r.json.anteil, r.json.boxen]).toEqual([ui.anteil, ui.boxen]);
    expect(run(['frame', join(SPECS, 'monarch.paper.json')])).toMatchObject({ code: 2, stdout: '' });
    expect(run(['zeichne'])).toMatchObject({ code: 2, stdout: '' });
  });
});

describe('Beispielantworten', () => {
  const ui = readdirSync(join(FIX, 'ui')).filter((f) => f.endsWith('.json'));

  it('je RPC-Methode eine Datei; Review mit allen fünf Fällen', () => {
    expect(ui.map((f) => f.replace('.json', '')).sort()).toEqual(['AssetsFrame', 'AssetsGet', 'AssetsList', 'AssetsPixelDiff', 'AssetsPrompts', 'AssetsReview', 'AssetsStyle']);
    const review = lies(join(FIX, 'ui/AssetsReview.json'));
    expect(review.assets.map((a: { status: string; stil: boolean }) => (a.stil ? 'stil' : a.status)).sort())
      .toEqual(['geaendert', 'geloescht', 'neu', 'stil', 'ungueltig']);
    expect(lies(join(FIX, 'ui/AssetsFrame.json')).bild).toMatch(/^data:image\/png;base64,iVBOR/);
  });

  it('jede MCP-Beispielantwort verweist nur auf vorhandene Fixture-PNGs', () => {
    const texte = readdirSync(join(FIX, 'mcp')).map((f) => readFileSync(join(FIX, 'mcp', f), 'utf8')).join('\n');
    const pngs = [...texte.matchAll(/tools\/paper\/fixtures\/render\/\S+?\.png/g)].map((m) => m[0]);
    expect(pngs.length).toBeGreaterThan(5);
    expect(pngs.filter((p) => !existsSync(join(ROOT, p)))).toEqual([]);
  });

  it('Review-Test-Repo: Basis-Commit vorhanden, Arbeitsstand mit zwei Änderungen', () => {
    const dir = join(tmp(), 'repo');
    expect(erzeugeReviewRepo(dir).basis).toMatch(/^[0-9a-f]{7,}$/);
    expect(lies(join(FIX, 'ui/AssetsReview.json')).basisCommit).toMatch(/^[0-9a-f]{7,}$/);
    const status = execFileSync('git', ['status', '--short'], { cwd: dir, encoding: 'utf8' });
    expect(status.trimEnd().split('\n').sort()).toEqual([' M public/grafik/figuren/monarch.paper.json', ' M public/grafik/requisiten/banner.paper.json']);
  });
});
