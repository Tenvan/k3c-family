// RD1.2: Zeitregel (Phase n / frames, Physik 1/120 s, 240 Schritte Einschwingen) und Render der Figur.
import { mkdtempSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { renderSpec } from '../render/ausgabe';
import { ladeStil } from '../render/stil';
import { EINSCHWINGEN, frameSchritte, simuliere } from './animation';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const monarch = JSON.parse(readFileSync(join(SPECS, 'monarch.paper.json'), 'utf8')) as Spec;
const stil = ladeStil();

describe('Animation der Figur', () => {
  it('Frame-Schritte: idle 8 Frames → Schritt 0, 30, 60 …; lauf → round(n · 9.375)', () => {
    expect(frameSchritte(0.5, 8)).toEqual([0, 30, 60, 90, 120, 150, 180, 210]);
    expect(frameSchritte(1.6, 8)).toEqual([0, 9, 19, 28, 38, 47, 56, 66]);
  });

  it('Einschwingen: 240 Schritte vor Frame 0, Pose läuft rückwärts periodisch weiter', () => {
    expect(EINSCHWINGEN).toBe(240);
    const a = simuliere(monarch, 'idle'), b = simuliere(monarch, 'idle');
    expect(a).toHaveLength(8);
    expect(a).toEqual(b);
    const u = a[0].umhang!;
    expect(u).toHaveLength(8);
    expect(u[7][0]).toBeLessThan(u[0][0]);
  });

  it('Umhang hängt am Anker der Figur (umhang.punkt durch die Matrix von rumpf, mit Versatz)', () => {
    const f = simuliere(monarch, 'lauf')[0];
    expect(f.umhang![0]).toEqual([-4 + f.versatz[0], -91 + f.versatz[1]]);
  });

  it('render monarch ohne reiten: idle.png, lauf.png, idle@spieler2.png, lauf@spieler2.png', () => {
    const dir = mkdtempSync(join(tmpdir(), 'paper-figur-'));
    const s = structuredClone(monarch);
    delete s.animationen!.reiten;
    writeFileSync(join(dir, 'monarch.paper.json'), JSON.stringify(s));
    renderSpec(join(dir, 'monarch.paper.json'), { stil });
    expect(readdirSync(join(dir, 'monarch')).sort()).toEqual(['idle.png', 'idle@spieler2.png', 'lauf.png', 'lauf@spieler2.png', 'manifest.json']);
    const m = JSON.parse(readFileSync(join(dir, 'monarch/manifest.json'), 'utf8'));
    expect([m.frameW, m.frameH]).toEqual([108, 192]);
    expect(m.animationen.idle.hashes).toHaveLength(8);
    expect(new Set(m.animationen.lauf.hashes).size).toBe(8);
    expect(m.animationen['idle@spieler2'].hashes[0]).not.toBe(m.animationen.idle.hashes[0]);
  });
});
