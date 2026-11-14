// MG11.4 (B-723): Bäume als PNG in Spielhöhe mit mehreren Formen (AC-01), Münze dreht sich (AC-03).
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { specsIn } from '../render/ausgabe';
import { ROOT } from '../render/stil';
import type { Spec } from '../schema/types';

const lies = (p: string) => JSON.parse(readFileSync(p, 'utf8')) as Spec;
const objekte = specsIn(join(ROOT, 'public/grafik/objekte')).map(lies);

describe('Bäume (B-723/AC-01)', () => {
  it.each(['birke', 'tanne', 'pappel'])('%s: mindestens zwei Formen, PNG 340–380 px hoch', (art) => {
    const formen = objekte.filter((s) => s.id === art || new RegExp(`^${art}-\\d+$`).test(s.id));
    expect(formen.length).toBeGreaterThanOrEqual(2);
    for (const s of formen) {
      expect(s.ausgabe.hoehe_px, s.id).toBeGreaterThanOrEqual(340);
      expect(s.ausgabe.hoehe_px, s.id).toBeLessThanOrEqual(380);
    }
  });
});

describe('Münze (B-723/AC-03)', () => {
  it('idle dreht in mindestens 6 Frames, je Frame genau eine Phase sichtbar', () => {
    const m = lies(join(ROOT, 'public/grafik/icons/muenze.paper.json'));
    const idle = m.animationen!.idle;
    expect(idle.frames).toBeGreaterThanOrEqual(6);
    const kf = idle.keyframes!;
    expect(kf.length).toBe(idle.frames);
    const phasen = m.teile.map((t) => t.id);
    kf.forEach((k, i) => {
      expect(k.t).toBeCloseTo(i / idle.frames);
      // jeder Keyframe nennt alle Phasen, sonst erbt ein Teil den Zustand des Vorgängers (pose.ts)
      expect(Object.keys(k.sichtbar ?? {}).sort(), `t ${k.t}`).toEqual([...phasen].sort());
      expect(Object.values(k.sichtbar ?? {}).filter(Boolean).length, `t ${k.t}`).toBe(1);
    });
  });
});
