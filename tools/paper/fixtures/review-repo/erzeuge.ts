// Erzeugt ein Test-Git-Repo mit allen Review-Fällen für RV1 (docs/standards/paper-mcp.md › Review-Test-Repo).
// Commit 1 (Basis): Stil-Datei, monarch, mauer, fass, banner. Commit 2: pferd neu, fass gelöscht, Stil `stein` heller.
// Arbeitsstand (nicht committet): Umhang des Monarchen purpur, banner mit fremdem Material (ungültig).
// Basis HEAD → geaendert + ungueltig; Basis HEAD~1 → alle fünf Fälle (neu, geaendert, geloescht, ungueltig, stil).
// Aufruf: bun tools/paper/fixtures/review-repo/erzeuge.ts <leerer Zielordner>; in Tests: erzeugeReviewRepo(ziel).
import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const FIX = join(import.meta.dirname, '..');
const ROOT = join(FIX, '../../..');
const ZEIT = '2026-11-06T12:00:00+01:00';
const ENV = { ...process.env, GIT_AUTHOR_NAME: 'Fixture', GIT_AUTHOR_EMAIL: 'fixture@k3c.local', GIT_AUTHOR_DATE: ZEIT,
  GIT_COMMITTER_NAME: 'Fixture', GIT_COMMITTER_EMAIL: 'fixture@k3c.local', GIT_COMMITTER_DATE: ZEIT };

const lies = (p: string) => JSON.parse(readFileSync(p, 'utf8'));
const text = (wert: unknown) => JSON.stringify(wert, null, 2) + '\n';

const requisite = (id: string, beschreibung: string, teil: string, material: string, form: string) => ({
  id, rig: 'requisite', version: 1, beschreibung,
  teile: [{ id: teil, pivot: [0, 0], lagen: [{ art: 'flaeche', form, material }, { art: 'flaeche', form, material, ton: 'schatten', auflage: true }] }],
  ausgabe: { hoehe_px: 64, rahmen: [-14, -34, 28, 36] },
});

/** Spec ohne `$schema` (im Test-Repo gibt es das Schema nicht). */
const spec = (name: string) => {
  const { $schema: _s, ...rest } = lies(join(FIX, 'specs', `${name}.paper.json`));
  return rest;
};

export function erzeugeReviewRepo(ziel: string): { head: string; basis: string } {
  const git = (...args: string[]) => execFileSync('git', ['-c', 'core.autocrlf=false', ...args], { cwd: ziel, env: ENV, encoding: 'utf8' }).trim();
  const schreibe = (rel: string, wert: unknown) => {
    mkdirSync(dirname(join(ziel, rel)), { recursive: true });
    writeFileSync(join(ziel, rel), text(wert));
  };
  mkdirSync(ziel, { recursive: true });
  git('init', '-q', '-b', 'develop');
  const stil = lies(join(ROOT, 'public/grafik/stil.paper.json'));
  schreibe('public/grafik/stil.paper.json', stil);
  schreibe('public/grafik/figuren/monarch.paper.json', spec('monarch'));
  schreibe('public/grafik/gebaeude/mauer.paper.json', spec('mauer'));
  schreibe('public/grafik/requisiten/fass.paper.json', requisite('fass', 'Holzfass', 'korpus', 'holz', 'M-10 0V-28H10V0Z'));
  schreibe('public/grafik/requisiten/banner.paper.json', requisite('banner', 'Rotes Banner', 'tuch', 'koenigsrot', 'M-8 -32H8V-4L0 0L-8 -4Z'));
  git('add', '-A');
  git('commit', '-q', '-m', 'Basis: monarch, mauer, fass, banner');
  const basis = git('rev-parse', '--short', 'HEAD');
  schreibe('public/grafik/reittiere/pferd.paper.json', spec('pferd'));
  rmSync(join(ziel, 'public/grafik/requisiten/fass.paper.json'));
  stil.materialien.stein = { ...stil.materialien.stein, basis: '#a8a294', schatten: '#77716a' };
  schreibe('public/grafik/stil.paper.json', stil);
  git('add', '-A');
  git('commit', '-q', '-m', 'pferd neu, fass gelöscht, Stil stein heller');
  const monarch = spec('monarch');
  monarch.umhang.material = 'purpur';
  schreibe('public/grafik/figuren/monarch.paper.json', monarch);
  const banner = requisite('banner', 'Rotes Banner', 'tuch', 'koenigsrot', 'M-8 -32H8V-4L0 0L-8 -4Z');
  banner.teile[0].lagen[0].material = 'scharlach';
  schreibe('public/grafik/requisiten/banner.paper.json', banner);
  return { head: git('rev-parse', '--short', 'HEAD'), basis };
}

if (import.meta.main) {
  const ziel = process.argv[2];
  if (!ziel) {
    process.stderr.write('Aufruf: bun tools/paper/fixtures/review-repo/erzeuge.ts <leerer Zielordner>\n');
    process.exit(2);
  }
  process.stdout.write(JSON.stringify({ ok: true, ziel, ...erzeugeReviewRepo(ziel) }) + '\n');
}
