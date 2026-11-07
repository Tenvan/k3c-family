// RD2.4 (B-715): Proportionen je Tier (Fuß, Gangarten) und je Figur (Faktoren auf die Posen).
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { checkSpec } from '../schema/check';
import { Abgelehnt } from '../render/fehler';
import { ladeStil } from '../render/stil';
import { ausgaben } from '../render/stufen';
import { bildVon } from '../render/bild';
import './register';
import { figurPose } from './figur';
import { GANGARTEN, gangPose } from './gangarten';
import { beinLaengen, FUESSE } from './proportionen';
import { pruefeRenderRegeln } from './pruefen';
import { vierbeinerBild } from './vierbeinerBild';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const lies = (id: string) => JSON.parse(readFileSync(join(SPECS, `${id}.paper.json`), 'utf8')) as Spec;
const pferd = lies('pferd'), monarch = lies('monarch'), stil = ladeStil();
const mit = (s: Spec, id: string, p: Spec['proportionen']): Spec => ({ ...structuredClone(s), id, proportionen: p });
const regeln = (s: Spec) => checkSpec(s, stil).map((f) => f.regel);
const gangRegeln = (s: Spec) => pruefeRenderRegeln(s, join(SPECS, `${s.id}.paper.json`), stil).map((f) => f.regel);
const ohneGalopp = (s: Spec): Spec => {
  const c = structuredClone(s);
  delete c.animationen!.galopp;
  return c;
};

describe('Spec-Vertrag (AC-01)', () => {
  it.each(['huf', 'pfote', 'elefantenfuss'] as const)('fuss %s ist gültig; Bein reicht immer bis zum Boden (Hüfthöhe 56)', (fuss) => {
    const s = mit(pferd, 'pferd', { fuss });
    expect(regeln(s)).toEqual([]);
    const l = beinLaengen(s);
    expect(l.ober + l.unter).toBe(56);
    expect(FUESSE[fuss].spitze).toBeGreaterThan(0);
  });

  it('Beinlängen folgen den Pivots: Hüfte bei −70, Knie bei −40 → 30 und 40', () => {
    const s = structuredClone(pferd);
    s.teile.find((t) => t.id === 'oberbein-hf')!.pivot = [-26, -70];
    s.teile.find((t) => t.id === 'unterbein-hf')!.pivot = [-26, -40];
    expect(beinLaengen(s)).toEqual({ ober: 30, unter: 40 });
  });

  it('ohne proportionen: 25 und 31 wie in RD1 (Huf), auch ohne Beinteile', () => {
    expect(beinLaengen(pferd)).toEqual({ ober: 25, unter: 31 });
    expect(beinLaengen({ ...pferd, teile: [] })).toEqual({ ober: 25, unter: 31 });
  });

  it('unbekannte Fußform und unbekannte Felder werden vom Schema abgelehnt', () => {
    expect(regeln(mit(pferd, 'pferd', { fuss: 'tatze' as never })).length).toBeGreaterThan(0);
    expect(regeln(mit(pferd, 'pferd', { hirn: 1 } as never)).length).toBeGreaterThan(0);
    expect(regeln(mit(monarch, 'monarch', { arme: 0 })).length).toBeGreaterThan(0);
  });

  it('Felder am falschen Rig: proportionen-rig', () => {
    expect(regeln(mit(pferd, 'pferd', { arme: 0.5 }))).toEqual(['proportionen-rig']);
    expect(regeln(mit(monarch, 'monarch', { fuss: 'pfote' }))).toEqual(['proportionen-rig']);
    expect(regeln(mit(monarch, 'monarch', { arme: 0.5, rumpf: 2, beine: 1, kopf: 1 }))).toEqual([]);
  });

  it('Fußform ändert den Beinpfad (Pfote und Elefantenfuß ≠ Huf)', () => {
    const k = { stil, specPfad: join(SPECS, 'pferd.paper.json') }, a = ausgaben(pferd).find((x) => x.schluessel === 'schritt')!;
    const svg = (s: Spec) => vierbeinerBild(s, a, k).frames[0].svg;
    expect(svg(mit(pferd, 'pferd', { fuss: 'huf' }))).toBe(svg(pferd));
    expect(svg(mit(pferd, 'pferd', { fuss: 'pfote' }))).not.toBe(svg(pferd));
    expect(svg(mit(pferd, 'pferd', { fuss: 'elefantenfuss' }))).not.toBe(svg(mit(pferd, 'pferd', { fuss: 'pfote' })));
  });

  it('Pose des Pferds mit fuss huf ist die Pose ohne proportionen (Defaults = RD1)', () => {
    for (const g of Object.keys(GANGARTEN) as (keyof typeof GANGARTEN)[]) expect(gangPose(mit(pferd, 'pferd', { fuss: 'huf' }), g, 0.3)).toEqual(gangPose(pferd, g, 0.3));
  });
});

describe('Gangarten je Tier (AC-02)', () => {
  it('Elefant ohne galopp: gültig, wenn proportionen.gangarten galopp auslässt', () => {
    const elefant = mit(ohneGalopp(pferd), 'pferd', { fuss: 'elefantenfuss', gangarten: ['schritt', 'trab'] });
    expect(regeln(elefant)).toEqual([]);
    expect(gangRegeln(elefant)).toEqual([]);
  });

  it('Pferd ohne galopp: ungültig (gangart-fehlt)', () => {
    expect(gangRegeln(ohneGalopp(pferd))).toEqual(['gangart-fehlt']);
    expect(() => vierbeinerBild(ohneGalopp(pferd), ausgaben(ohneGalopp(pferd))[0], { stil })).toThrow(Abgelehnt);
  });

  it('Animation mit einer Gangart, die das Tier nicht hat: gangart-nicht-erlaubt', () => {
    expect(gangRegeln(mit(pferd, 'pferd', { gangarten: ['schritt', 'trab'] }))).toEqual(['gangart-nicht-erlaubt']);
  });
});

describe('Figur-Proportionen (AC-03)', () => {
  it('Faktoren wirken auf die Pose-Winkel und den Versatz; ohne proportionen bleibt alles wie in RD1', () => {
    const lang = mit(monarch, 'monarch', { arme: 0.5, beine: 2, kopf: 0.5, rumpf: 0.5 });
    const a = figurPose(monarch, 'lauf', 0.2), b = figurPose(lang, 'lauf', 0.2);
    expect(b.winkel['oberarm-v']).toBeCloseTo(a.winkel['oberarm-v'] * 0.5, 9);
    expect(b.winkel['unterarm-h']).toBeCloseTo(a.winkel['unterarm-h'] * 0.5, 9);
    expect(b.winkel['oberschenkel-v']).toBeCloseTo(a.winkel['oberschenkel-v'] * 2, 9);
    expect(b.winkel.kopf).toBeCloseTo(a.winkel.kopf * 0.5, 9);
    expect(b.versatz[1]).toBeCloseTo(a.versatz[1] * 0.5, 9);
    expect(figurPose(mit(monarch, 'monarch', { arme: 1 }), 'lauf', 0.2)).toEqual(a);
  });

  it('Fixture langarm (lange Arme, kurzer Rumpf) rendert alle Frames ohne Gelenkfehler: endliche Zahlen, Teile im Rahmen', () => {
    const langarm = lies('langarm'), pfad = join(SPECS, 'langarm.paper.json');
    expect(regeln(langarm)).toEqual([]);
    for (const a of ausgaben(langarm)) {
      const bild = bildVon(langarm, a, { stil, specPfad: pfad });
      expect(bild.frames.length).toBeGreaterThan(0);
      for (const f of bild.frames) {
        expect(f.svg).not.toMatch(/NaN|Infinity|undefined/);
        for (const g of f.geometrie) expect(g.m.every(Number.isFinite), `${a.schluessel}/${g.teil.id}`).toBe(true);
      }
    }
  });
});
