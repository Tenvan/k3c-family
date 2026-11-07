// RD1.3: Reiten als Kombination aus Reittier, Umhang und Reiter (README › D-09, D-10).
import { copyFileSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { gen } from '../cli/gen';
import { renderSpec, rendereAsset } from '../render/ausgabe';
import { Abgelehnt } from '../render/fehler';
import { ladeStil } from '../render/stil';
import { lageSvg } from '../render/svg';
import { ausgaben } from '../render/stufen';
import { findeReittier, reitRahmen, reitSegmente, reitZustand, reittierHash } from './reiten';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const lies = (id: string) => JSON.parse(readFileSync(join(SPECS, `${id}.paper.json`), 'utf8')) as Spec;
const monarch = lies('monarch'), pferd = lies('pferd'), stil = ladeStil();
const kontext = { stil, specPfad: join(SPECS, 'monarch.paper.json') };
const reiten = ausgaben(monarch).find((a) => a.schluessel === 'reiten')!;
const tmp = () => mkdtempSync(join(tmpdir(), 'paper-reiten-'));

describe('Reiten', { timeout: 60_000 }, () => {
  it('Reittier-Suche: erst Ordner der Spec, dann ../*/<id>.paper.json; fehlt es → Exit 1, Regel reittier-unbekannt', () => {
    expect(findeReittier(join(SPECS, 'monarch.paper.json'), 'pferd')).toBe(join(SPECS, 'pferd.paper.json'));
    const wurzel = tmp();
    mkdirSync(join(wurzel, 'figuren'));
    mkdirSync(join(wurzel, 'reittiere'));
    copyFileSync(join(SPECS, 'pferd.paper.json'), join(wurzel, 'reittiere/pferd.paper.json'));
    expect(findeReittier(join(wurzel, 'figuren/monarch.paper.json'), 'pferd')).toBe(join(wurzel, 'reittiere/pferd.paper.json'));
    expect(() => findeReittier(join(wurzel, 'figuren/monarch.paper.json'), 'esel')).toThrow(Abgelehnt);
  });

  it('reittier ohne rig vierbeiner → Exit 1, Regel reittier-unbekannt', () => {
    const dir = tmp();
    const m = structuredClone(monarch);
    m.animationen!.reiten.reittier = 'mauer';
    writeFileSync(join(dir, 'monarch.paper.json'), JSON.stringify(m));
    copyFileSync(join(SPECS, 'mauer.paper.json'), join(dir, 'mauer.paper.json'));
    try {
      renderSpec(join(dir, 'monarch.paper.json'), { stil });
      expect.unreachable();
    } catch (e) {
      expect((e as Abgelehnt).fehler[0].regel).toBe('reittier-unbekannt');
    }
  });

  it('Reihenfolge: Teile des Reittiers bis vor hals, Umhang, Reiter, hals und Rest, Zügel', () => {
    const seg = reitSegmente(monarch, pferd, reitZustand(monarch, pferd, reiten, kontext)[0], stil);
    expect(seg.map((s) => s.name)).toEqual(['reittier-hinten', 'umhang', 'reiter', 'reittier-vorn', 'zuegel']);
    expect(seg[3].svg).toContain('cx="57.6" cy="-96.2"');
    expect(seg[0].svg).not.toContain('cx="57.6" cy="-96.2"');
    expect(seg[4].svg).toContain(`stroke="${stil.materialien.leder.schatten}" stroke-width="1.3"`);
  });

  it('oberschenkel-h samt Kindern ist beim Reiten ausgeblendet', () => {
    const seg = reitSegmente(monarch, pferd, reitZustand(monarch, pferd, reiten, kontext)[0], stil);
    // Die Lagen von -h und -v haben gleiche Formen, unterscheiden sich aber im Ton (fern dunkler).
    const lage = (id: string, i: number) => lageSvg(monarch.teile.find((t) => t.id === id)!.lagen[i], stil);
    expect(seg[2].svg).not.toContain(lage('oberschenkel-h', 0));
    expect(seg[2].svg).not.toContain(lage('unterschenkel-h', 1));
    expect(seg[2].svg).toContain(lage('oberschenkel-v', 0));
  });

  it('Reiter: versatz [−2, −54 + riderY], Drehung lean um (0, −50), Winkel aus gangPose', () => {
    const z = reitZustand(monarch, pferd, reiten, kontext)[0];
    const seg = reitSegmente(monarch, pferd, z, stil);
    expect(z.pose.reiter.versatz[0]).toBe(-2);
    expect(seg[2].svg.startsWith(`<g transform="translate(-2 `)).toBe(true);
    expect(seg[2].svg).toContain(' 0 -50)">');
  });

  it('Umhang beim Reiten: Boden −103, Wind aus der Gangart', () => {
    for (const z of reitZustand(monarch, pferd, reiten, kontext))
      for (const [, y] of z.umhang!.slice(1)) expect(y).toBeLessThanOrEqual(-103);
  });

  it('Reit-Rahmen: Hülle aus reittier.rahmen · massstab und reiter.rahmen + [−2, −54], Pixel je Einheit wie der Reiter', () => {
    expect(reitRahmen(monarch, pferd)).toEqual({ rahmen: [-89, -199, 190, 205], frameW: 242, frameH: 261, eigen: true });
    const dir = tmp();
    copyFileSync(join(SPECS, 'monarch.paper.json'), join(dir, 'monarch.paper.json'));
    copyFileSync(join(SPECS, 'pferd.paper.json'), join(dir, 'pferd.paper.json'));
    renderSpec(join(dir, 'monarch.paper.json'), { stil });
    const m = JSON.parse(readFileSync(join(dir, 'monarch/manifest.json'), 'utf8'));
    expect([m.frameW, m.frameH]).toEqual([108, 192]);
    expect(m.animationen.reiten).toMatchObject({ rahmen: [-89, -199, 190, 205], frameW: 242, frameH: 261 });
    expect(readdirSync(join(dir, 'monarch'))).toContain('reiten@spieler2.png');
  }, 60_000);

  it('Monarch mit schritt, trab, galopp als Reit-Animationen rendert ohne Fehler', () => {
    const dir = tmp(), m = structuredClone(monarch);
    delete m.animationen!.reiten;
    for (const g of ['schritt', 'trab', 'galopp'] as const) m.animationen![g] = { frames: 8, reittier: 'pferd', gangart: g };
    writeFileSync(join(dir, 'monarch.paper.json'), JSON.stringify(m));
    copyFileSync(join(SPECS, 'pferd.paper.json'), join(dir, 'pferd.paper.json'));
    renderSpec(join(dir, 'monarch.paper.json'), { stil });
    expect(readdirSync(join(dir, 'monarch')).filter((f) => !f.includes('@')).sort())
      .toEqual(['galopp.png', 'idle.png', 'lauf.png', 'manifest.json', 'schritt.png', 'trab.png']);
  }, 60_000);
});

describe('Reiten: Randfälle (Review RD1.3)', { timeout: 60_000 }, () => {
  it('Reittier-id mit Pfadanteilen: reittier-unbekannt', () => {
    expect(() => findeReittier(join(SPECS, 'monarch.paper.json'), '../specs/pferd')).toThrow(Abgelehnt);
  });
});

describe('renderKey mit Reittier (B-713)', { timeout: 120_000 }, () => {
  const wurzel = () => {
    const dir = tmp();
    mkdirSync(join(dir, 'figuren'));
    mkdirSync(join(dir, 'reittiere'));
    copyFileSync(join(SPECS, 'monarch.paper.json'), join(dir, 'figuren/monarch.paper.json'));
    copyFileSync(join(SPECS, 'pferd.paper.json'), join(dir, 'reittiere/pferd.paper.json'));
    return dir;
  };
  const key = (dir: string) => (rendereAsset(JSON.parse(readFileSync(join(dir, 'figuren/monarch.paper.json'), 'utf8')) as Spec,
    { stil, specPfad: join(dir, 'figuren/monarch.paper.json') }).manifest as { renderKey: string }).renderKey;

  it('reittierHash: nur mit Reit-Animation und Pfad, sortiert nach id', () => {
    expect(reittierHash(monarch, undefined)).toBeUndefined();
    expect(reittierHash(pferd, join(SPECS, 'pferd.paper.json'))).toBeUndefined();
    expect(reittierHash(monarch, join(SPECS, 'monarch.paper.json'))).toMatch(/^pferd=[0-9a-f]{64}$/);
  });

  it('Pferd-Spec geändert: renderKey des Monarchen ändert sich; Monarch ohne Reittier-Änderung bleibt gleich (AC-01)', () => {
    const dir = wurzel(), a = key(dir);
    expect(key(dir)).toBe(a);
    const p = JSON.parse(readFileSync(join(dir, 'reittiere/pferd.paper.json'), 'utf8')) as Spec;
    p.beschreibung += ' (geändert)';
    writeFileSync(join(dir, 'reittiere/pferd.paper.json'), JSON.stringify(p));
    expect(key(dir)).not.toBe(a);
  });

  it('gen rendert nach einer Pferd-Änderung auch figuren/monarch neu (AC-02)', () => {
    const dir = wurzel();
    copyFileSync(join(import.meta.dirname, '../../../public/grafik/stil.paper.json'), join(dir, 'stil.paper.json'));
    const manifest = () => (JSON.parse(readFileSync(join(dir, 'figuren/monarch/manifest.json'), 'utf8')) as { renderKey: string }).renderKey;
    gen(dir);
    const a = manifest();
    const p = JSON.parse(readFileSync(join(dir, 'reittiere/pferd.paper.json'), 'utf8')) as Spec;
    p.beschreibung += ' (geändert)';
    writeFileSync(join(dir, 'reittiere/pferd.paper.json'), JSON.stringify(p));
    gen(dir);
    expect(manifest()).not.toBe(a);
  });
});
