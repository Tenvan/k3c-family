// AC-06 (PV1.4): data/asset-prompts.json gegen prompts.schema.json und die Regeln aus docs/standards/paper-mcp.md.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import Ajv2020Import from 'ajv/dist/2020.js';
import { describe, expect, it } from 'vitest';
import schema from './prompts.schema.json';

type Vorlage = { id: string; titel: string; ort: string; text: string };
const ROOT = join(import.meta.dirname, '../../..');
const vorlagen = JSON.parse(readFileSync(join(ROOT, 'data/asset-prompts.json'), 'utf8')) as Vorlage[];
const Ajv2020 = ((Ajv2020Import as unknown as { default?: typeof Ajv2020Import }).default ?? Ajv2020Import);

/** Erlaubte Platzhalter (paper-mcp.md › Prompt-Vorlagen). */
export const PLATZHALTER = ['asset', 'rig', 'teil', 'pfad', 'zustand', 'basis', 'eingabe', 'material', 'ton', 'slot', 'datei', 'pack', 'lizenz', 'treffer'];
export const platzhalter = (text: string) => [...new Set([...text.matchAll(/\{([a-z_]+)\}/g)].map((m) => m[1]))];

/** Orte der Asset-Arbeit mit festem Anfang und Ende; die übrigen (Styleguide, Sound, Quellen) sind kurze Prompts (B-639). */
const ASSET_ORTE = ['asset', 'teil', 'review'];
/** Pflicht-Platzhalter der kurzen Orte (B-727 bis B-729). */
const PFLICHT: Record<string, string[]> = { material: ['material'], ton: ['ton'], papier: [], sound: ['slot', 'datei'],
  datei: ['datei', 'pack', 'lizenz'], treffer: ['treffer', 'slot', 'datei', 'pack', 'lizenz'] };

describe('Prompt-Vorlagen', () => {
  it('sind gültig gegen prompts.schema.json', () => {
    const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
    expect(validate(vorlagen), JSON.stringify(validate.errors)).toBe(true);
  });

  it('IDs eindeutig, Pflicht-Vorlagen vorhanden', () => {
    const ids = vorlagen.map((v) => v.id);
    expect(new Set(ids).size).toBe(ids.length);
    expect(ids).toEqual(expect.arrayContaining(['material-aendern', 'proportion-anpassen', 'detail-ergaenzen', 'variante-anlegen',
      'animation-aendern', 'neues-wie-dieses', 'teil-material', 'teil-form', 'aenderung-zuruecknehmen', 'nachbessern']));
  });

  it.each(vorlagen.filter((v) => !ASSET_ORTE.includes(v.ort)))('$id: kurzer Prompt, eine Zeile, Pflicht-Platzhalter des Orts', (v) => {
    expect(platzhalter(v.text).filter((p) => !PLATZHALTER.includes(p))).toEqual([]);
    expect(v.text).not.toContain('\n');
    expect(v.text.length).toBeLessThanOrEqual(120);
    expect(platzhalter(v.text)).toEqual(expect.arrayContaining(PFLICHT[v.ort]));
  });

  it.each(vorlagen.filter((v) => ASSET_ORTE.includes(v.ort)))('$id: nur erlaubte Platzhalter, Anfang und Ende fest, Ort passt', (v) => {
    expect(platzhalter(v.text).filter((p) => !PLATZHALTER.includes(p))).toEqual([]);
    expect(v.text.startsWith('Rufe zuerst `assets_guide {rig: {rig}}` auf')).toBe(true);
    expect(v.text.endsWith('Prüfe das Ergebnis mit `assets_preview` und rufe zum Schluss `assets_review` auf.')).toBe(true);
    if (v.ort === 'teil') expect(platzhalter(v.text)).toContain('teil');
    if (v.ort === 'review') expect(platzhalter(v.text)).toEqual(expect.arrayContaining(['basis', 'eingabe']));
  });
});
