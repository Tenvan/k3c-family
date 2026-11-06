// Erzeugt die RPC-Beispielantworten fixtures/ui/*.json (Vertrag docs/standards/paper-mcp.md › Oberfläche) aus den
// Fixtures und der CLI-Attrappe; Zustand wie im Review-Test-Repo (review-repo/erzeuge.ts, Basis HEAD~1).
// Aufruf: bun tools/paper/fixtures/beispiele.ts
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { Resvg } from '@resvg/resvg-js';
import { run } from './cli-stub';
import { pixeldiff } from './cli-pixel';
import { erzeugeReviewRepo } from './review-repo/erzeuge';

const FIX = import.meta.dirname;
const ROOT = join(FIX, '../../..');
const png = (p: string) => readFileSync(join(FIX, 'render', p));
const dataUrl = (b: Buffer) => `data:image/png;base64,${b.toString('base64')}`;
const json = (p: string) => JSON.parse(readFileSync(p, 'utf8'));
const schreibe = (name: string, wert: unknown) => writeFileSync(join(FIX, 'ui', `${name}.json`), JSON.stringify(wert, null, 2) + '\n');

/** Mauer nach einer Stiländerung (Ton `stein` heller): Quell-SVG mit getauschten Farben, gleich gerendert wie die Probe. */
function mauerNachStil(): Buffer {
  const svg = readFileSync(join(FIX, 'svg/mauer-1.svg'), 'utf8').replace(/\r\n/g, '\n')
    .replaceAll('#9a9386', '#a8a294').replaceAll('#6b6559', '#77716a');
  return Buffer.from(new Resvg(svg, { fitTo: { mode: 'height', value: 192 }, font: { loadSystemFonts: false } }).render().asPng());
}

export const REVIEW = {
  basis: 'HEAD~1',
  basisCommit: '', // HEAD~1 im Review-Test-Repo, beim Erzeugen gesetzt
  basisStand: 'aktuell',
  assets: [
    { id: 'monarch', rig: 'figur', pfad: 'public/grafik/figuren/monarch.paper.json', status: 'geaendert',
      felder: [{ pfad: 'umhang.material', alt: 'koenigsrot', neu: 'purpur' }], teile: [], stil: false, pixelAnteil: 0 },
    { id: 'pferd', rig: 'vierbeiner', pfad: 'public/grafik/reittiere/pferd.paper.json', status: 'neu', felder: [], teile: [], stil: false, pixelAnteil: 1 },
    { id: 'fass', rig: 'requisite', pfad: 'public/grafik/requisiten/fass.paper.json', status: 'geloescht', felder: [], teile: [], stil: false, pixelAnteil: 1 },
    { id: 'banner', rig: 'requisite', pfad: 'public/grafik/requisiten/banner.paper.json', status: 'ungueltig',
      fehler: [{ feld: 'teile[0].lagen[0].material', regel: 'material-unbekannt', text: 'Material „scharlach“ fehlt in der Stil-Datei' }],
      felder: [{ pfad: 'teile[0].lagen[0].material', alt: 'koenigsrot', neu: 'scharlach' }], teile: ['tuch'], stil: false, pixelAnteil: 0 },
    { id: 'mauer', rig: 'gebaeude', pfad: 'public/grafik/gebaeude/mauer.paper.json', status: 'geaendert', felder: [], teile: [], stil: true, pixelAnteil: 0 },
  ],
};

if (import.meta.main) {
  const tmp = mkdtempSync(join(tmpdir(), 'paper-'));
  REVIEW.basisCommit = erzeugeReviewRepo(join(tmp, 'repo')).basis;
  const umhang = pixeldiff(png('monarch/monarch-galopp-0.png'), png('monarch/monarch-galopp-0-purpur.png'));
  const stil = mauerNachStil();
  writeFileSync(join(FIX, 'render/mauer/mauer-1-stil.png'), stil);
  writeFileSync(join(FIX, 'render/monarch/pixeldiff-umhang.png'), umhang.overlay);
  REVIEW.assets[0].pixelAnteil = umhang.anteil;
  REVIEW.assets[4].pixelAnteil = pixeldiff(png('mauer/mauer-1.png'), stil).anteil;
  const frame = JSON.parse(run(['frame', join(FIX, 'specs/monarch.paper.json'), '--animation', 'reiten', '--frame', '0', '--out', join(tmp, 'f.png')]).stdout);
  schreibe('AssetsList', [
    { id: 'monarch', rig: 'figur', gruppe: 'figuren', pfad: 'public/grafik/figuren/monarch.paper.json', gueltig: true, geaendert: true },
    { id: 'pferd', rig: 'vierbeiner', gruppe: 'reittiere', pfad: 'public/grafik/reittiere/pferd.paper.json', gueltig: true, geaendert: true },
    { id: 'mauer', rig: 'gebaeude', gruppe: 'gebaeude', pfad: 'public/grafik/gebaeude/mauer.paper.json', gueltig: true, geaendert: true },
    { id: 'banner', rig: 'requisite', gruppe: 'requisiten', pfad: 'public/grafik/requisiten/banner.paper.json', gueltig: false, geaendert: true,
      fehler: REVIEW.assets[3].fehler },
  ]);
  const { $schema: _s, ...monarch } = json(join(FIX, 'specs/monarch.paper.json'));
  schreibe('AssetsGet', monarch);
  const { $schema: _t, ...stilDatei } = json(join(ROOT, 'public/grafik/stil.paper.json'));
  schreibe('AssetsStyle', stilDatei);
  schreibe('AssetsPrompts', json(join(ROOT, 'data/asset-prompts.json')));
  schreibe('AssetsReview', REVIEW);
  schreibe('AssetsFrame', { bild: dataUrl(png('monarch/monarch-galopp-0-purpur.png')), teile: frame.teile });
  schreibe('AssetsPixelDiff', { anteil: umhang.anteil, boxen: umhang.boxen, overlay: dataUrl(umhang.overlay) });
}
