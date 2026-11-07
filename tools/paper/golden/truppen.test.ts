// MG9.3 (B-718, B-717/AC-02): Truppen sind lesbar (Krieger-Angriff, Reste aus MG3) und haben die Zustände give, hit, death, respawn.
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { ROOT } from '../render/stil';

const DIR = join(ROOT, 'public/grafik/truppen');
const IDS = readdirSync(DIR).filter((f) => f.endsWith('.paper.json') && f !== 'index.paper.json').map((f) => f.replace('.paper.json', ''));
const spec = (id: string) => JSON.parse(readFileSync(join(DIR, `${id}.paper.json`), 'utf8')) as Spec;
const lagen = (id: string, teil: string) => spec(id).teile.find((t) => t.id === teil)!.lagen as { form?: string; material?: string }[];

describe('Truppen (MG9.3)', () => {
  it.each(IDS)('%s hat give, hit, death, respawn mit Keyframes', (id) => {
    for (const n of ['give', 'hit', 'death', 'respawn']) expect(spec(id).animationen![n].keyframes!.length, `${id}/${n}`).toBeGreaterThan(1);
  });

  it.each(['krieger', 'elite-krieger'])('%s: Angriff hält den Schwertarm vor den Körper, nie über den Kopf (oberarm-v ≥ −120)', (id) => {
    for (const kf of spec(id).animationen!.angriff.keyframes!) expect(kf.winkel['oberarm-v'], `${id} t=${kf.t}`).toBeGreaterThanOrEqual(-120);
  });

  it.each(['krieger', 'elite-krieger'])('%s: Nasal beginnt am Helmrand über den Augen (D-MG3-08)', (id) => {
    const formen = lagen(id, 'helm').map((l) => l.form);
    expect(formen).toContain('M5 -118H8.5V-99H5Z');
    expect(formen).not.toContain('M5 -106H8.5V-96H5Z');
  });

  it.each(['krieger', 'elite-krieger'])('%s: Schild ist im Stand sichtbar, Waffe schwingt im Lauf gedämpft (D-MG3-08)', (id) => {
    const a = spec(id).animationen!;
    expect(a.idle.keyframes![0].winkel['oberarm-h']).toBeLessThan(0);
    const schwung = a.lauf.keyframes!.map((k) => k.winkel['oberarm-v']);
    expect(Math.max(...schwung) - Math.min(...schwung)).toBeLessThanOrEqual(15);
  });

  it('Landstreicher: Stabknauf liegt unter dem Kinn (D-MG3-08)', () => {
    const ys = lagen('landstreicher', 'stock').flatMap((l) => (l.form ?? '').match(/-\d+\.?\d*/g)!.filter((_, i) => i % 2 === 1).map(Number));
    expect(Math.min(...ys)).toBeGreaterThan(-85);
  });

  it('Bauer: Strohhut nicht in Kronengold (D-MG3-08)', () => {
    expect(lagen('bauer', 'hut').map((l) => l.material)).not.toContain('gold');
  });
});
