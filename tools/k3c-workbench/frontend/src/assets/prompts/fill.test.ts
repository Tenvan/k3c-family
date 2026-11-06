import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { AssetEintrag, Spec, Vorlage } from '../api';
import { fillPrompt, zustandVon } from './fill';

// WB1.3/AC-03: Platzhalter-Ersetzung laut docs/standards/paper-mcp.md › 3 (nur {asset} {rig} {teil} {pfad} {zustand}
// {basis} {eingabe}); Pflicht: {teil} bei ort teil, {eingabe} bei ort review.

const fixture = <T,>(name: string) =>
  JSON.parse(readFileSync(resolve(__dirname, '../../../../../paper/fixtures/ui', `${name}.json`), 'utf-8')) as T;
const VORLAGEN = fixture<Vorlage[]>('AssetsPrompts');
const WERTE = {
  asset: 'monarch', rig: 'figur', pfad: 'public/grafik/figuren/monarch.paper.json', zustand: 'gültig, Version 1',
  teil: 'kopf', basis: 'HEAD', eingabe: 'Krone etwas größer',
};
const v = (text: string, ort: Vorlage['ort'] = 'asset'): Vorlage => ({ id: 't', titel: 'T', ort, text });

describe('fillPrompt (WB1.3)', () => {
  it('ersetzt alle Platzhalter, auch mehrfach', () => {
    expect(fillPrompt(v('{asset} {rig} {pfad} {zustand} {asset}'), WERTE)).toBe(
      'monarch figur public/grafik/figuren/monarch.paper.json gültig, Version 1 monarch',
    );
    expect(fillPrompt(v('{teil}/{basis}/{eingabe}', 'review'), WERTE)).toBe('kopf/HEAD/Krone etwas größer');
  });

  it('füllt jede Vorlage der Fixture vollständig', () => {
    expect(VORLAGEN).toHaveLength(10);
    for (const vorlage of VORLAGEN) {
      const text = fillPrompt(vorlage, WERTE);
      expect(text, vorlage.id).not.toMatch(/\{[a-z]+\}/);
      expect(text, vorlage.id).toMatch(/^Rufe zuerst `assets_guide \{rig: figur\}` auf/);
    }
    expect(fillPrompt(VORLAGEN.find((x) => x.id === 'material-aendern')!, WERTE)).toContain('`monarch` (public/grafik/figuren/monarch.paper.json)');
  });

  it('lehnt fehlende Pflichtwerte ab statt halbe Prompts zu liefern', () => {
    expect(() => fillPrompt(v('{teil}', 'teil'), { ...WERTE, teil: '' })).toThrow('Teil wählen');
    expect(() => fillPrompt(v('x', 'teil'), { ...WERTE, teil: undefined })).toThrow('Teil wählen');
    expect(() => fillPrompt(v('{eingabe}', 'review'), { ...WERTE, eingabe: '  ' })).toThrow('Eingabe fehlt');
    expect(() => fillPrompt(v('{basis}'), { asset: 'monarch' })).toThrow('Wert für {basis} fehlt');
  });

  it('meldet unbekannte Platzhalter als Vorlagen-Fehler', () => {
    expect(() => fillPrompt(v('{xyz} {asset}'), WERTE)).toThrow('unbekannter Platzhalter {xyz}');
  });
});

describe('zustandVon (WB1.3)', () => {
  const spec = { id: 'monarch', rig: 'figur', version: 1, teile: [] } as Spec;
  const eintrag = (e: Partial<AssetEintrag>): AssetEintrag => ({ id: 'x', rig: 'figur', gruppe: 'g', pfad: 'p', gueltig: true, geaendert: false, ...e });
  it('nennt gültig mit Version oder die Regelverstöße', () => {
    expect(zustandVon(eintrag({}), spec)).toBe('gültig, Version 1');
    expect(zustandVon(eintrag({ gueltig: false, fehler: [{ feld: 'a', regel: 'r1', text: 't' }, { feld: 'b', regel: 'r2', text: 't' }] }), spec)).toBe(
      'ungültig: a · r1; b · r2',
    );
  });
});
