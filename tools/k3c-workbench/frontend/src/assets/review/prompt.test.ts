import { describe, expect, it } from 'vitest';
import { fillPrompt, reviewWerte } from './prompt';
import { fixture } from './testing';
import type { Review, Vorlage } from './types';

// RV1.4 / AC-06: Prompt-Vorlagen am Review-Eintrag werden vollständig gefüllt.

const review = fixture<Review>('AssetsReview');
const vorlagen = fixture<Vorlage[]>('AssetsPrompts').filter((v) => v.ort === 'review');
const asset = (id: string) => review.assets.find((a) => a.id === id)!;
const PLATZHALTER = /\{(asset|rig|teil|pfad|zustand|basis|eingabe)\}/;

describe('fillPrompt am Review-Eintrag', () => {
  it('füllt alle Platzhalter', () => {
    expect(vorlagen.map((v) => v.id)).toEqual(['aenderung-zuruecknehmen', 'nachbessern']);
    for (const v of vorlagen) {
      const t = fillPrompt(v, reviewWerte(asset('monarch'), 'HEAD~1', 'etwas dunkler'));
      expect(t, v.id).not.toMatch(PLATZHALTER);
      for (const teil of ['monarch', '{rig: figur}', 'public/grafik/figuren/monarch.paper.json', 'HEAD~1', 'etwas dunkler']) expect(t, v.id).toContain(teil);
      if (v.id === 'nachbessern') expect(t).toContain('umhang.material: koenigsrot → purpur');
    }
  });

  it('Stiländerung im Zustand', () => {
    expect(reviewWerte(asset('mauer'), 'HEAD~1', 'x').zustand).toContain('Stil geändert');
  });

  it('ungültig im Zustand', () => {
    expect(reviewWerte(asset('banner'), 'HEAD~1', 'x').zustand).toContain('Material „scharlach“ fehlt in der Stil-Datei');
  });

  it('ohne Eingabe kein halber Prompt', () => {
    expect(() => fillPrompt(vorlagen[0]!, reviewWerte(asset('monarch'), 'HEAD~1', '  '))).toThrow('Eingabe fehlt');
  });
});
