// MG9.2 (B-717): Monarch reitet jedes Reittier der Gruppe, hat Reit-Idle und Zustände, vier Spielerfarben ohne Braun.
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { ROOT } from '../render/stil';

const GRAFIK = join(ROOT, 'public/grafik');
const lies = (p: string) => JSON.parse(readFileSync(join(GRAFIK, p), 'utf8')) as Spec;
const monarch = lies('figuren/monarch.paper.json');
const reittiere = readdirSync(join(GRAFIK, 'reittiere')).filter((f) => f.endsWith('.paper.json') && f !== 'index.paper.json').map((f) => lies(`reittiere/${f}`));
const anim = monarch.animationen!;

describe('Monarch (B-717)', () => {
  it('AC-01: reiten-idle auf dem Pferd mit der Gangart stand', () => {
    expect(anim['reiten-idle']).toMatchObject({ gangart: 'stand', reittier: 'pferd' });
  });

  it.each(reittiere.map((r) => [r.id, r]))('AC-01: %s hat jede erlaubte Gangart als <gangart>-<reittier> (Pferd ohne Zusatz)', (id, r) => {
    const erlaubt = (r as Spec).proportionen?.gangarten ?? ['schritt', 'trab', 'galopp'];
    for (const g of erlaubt) {
      const name = id === 'pferd' ? g : `${g}-${id}`;
      expect(anim[name], name).toMatchObject({ gangart: g, reittier: id });
    }
    if (!erlaubt.includes('galopp')) expect(anim[`galopp-${id}`]).toBeUndefined();
  });

  it.each(['give', 'hit', 'death', 'respawn'])('AC-02: Zustand %s hat Keyframes', (n) => {
    expect(anim[n].keyframes!.length).toBeGreaterThan(1);
  });

  it('AC-03: vier Spieler mit verschiedenen Rock- und Satteldeckenfarben, keine auf Braun', () => {
    const farben = ['spieler1', 'spieler2', 'spieler3', 'spieler4'].map((v) => monarch.varianten![v].tausch.koenigsblau);
    expect(new Set(farben).size).toBe(4);
    for (const f of farben) expect(['leder', 'holz', 'fell-fuchs', 'haar', 'maehne', 'kupfer']).not.toContain(f);
  });
});
