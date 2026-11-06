// Golden der Fixture-Specs (tools/paper/fixtures/specs/, nur lesen): Render im Speicher, Eintrag im Format von golden.json.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import type { Spec } from '../schema/types';
import { type Asset, rendereAsset, sha256 } from '../render/ausgabe';
import { ladeStil } from '../render/stil';

export const SPECS = join(import.meta.dirname, '../fixtures/specs');
export const GOLDEN_DATEI = join(import.meta.dirname, 'fixtures.golden.json');

/** Fixtures mit Golden; wächst mit den Rigs (RD1.2 monarch, RD1.3 pferd). */
export const FIXTURES = ['mauer', 'monarch'];
/** Animationen, die das Golden noch auslässt (Reiten folgt in RD1.3). */
const OHNE: Record<string, string[]> = { monarch: ['reiten'] };

export function rendereFixture(id: string): Asset {
  const pfad = join(SPECS, `${id}.paper.json`);
  const spec = JSON.parse(readFileSync(pfad, 'utf8')) as Spec;
  for (const a of OHNE[id] ?? []) delete spec.animationen?.[a];
  return rendereAsset(spec, { stil: ladeStil(), specPfad: pfad });
}

export type GoldenEintrag = { renderKey: string; dateien: Record<string, string> };

export const goldenEintrag = (a: Asset): GoldenEintrag =>
  ({ renderKey: a.manifest.renderKey as string, dateien: Object.fromEntries(a.pngs.map(([d, png]) => [d, sha256(png)])) });
