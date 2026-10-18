import { describe, expect, it } from 'vitest';
import monarchJson from '@data/monarch.json';
import workerSource from '@game/scenes/worldRenderer.ts?raw';
import { EFFECT_CONFIG } from '@game/scenes/effects';
import { ACTIONS, RESPAWN_ALPHA, actionOf, effectMs, skillRows } from './actions';
import { MOVES, monarchItems, moveOf } from './monarchLogic';

// B-463 (WZ10.3): Aktionen, Skills und Lücken der Monarch-Seite.

const MONARCH = monarchJson as unknown as { skills: Record<string, unknown>; presets: Record<string, { activeSkills: string[] }> };

describe('Schlagen (B-463/AC-05)', () => {
  it('spielt attack einmal beritten und den Effekt swing aus EFFECT_CONFIG', () => {
    const swing = actionOf('swing')!;
    expect(swing.anim).toBe('attack');
    expect(swing.effect).toEqual({ kind: 'swing' });
    expect(swing.gaps).toEqual([]);
    expect(effectMs('swing')).toBe(EFFECT_CONFIG.swing.durationMs);
    const items = monarchItems('horse', moveOf('sprint'), 'run', {}, swing);
    expect(items.every((i) => i.anim === 'attack' && i.effect?.kind === 'swing' && i.mount?.speed === 0)).toBe(true);
  });
});

describe('Münzen, Treffer, Tod und Respawn (B-463/AC-08, AC-09)', () => {
  it('Münze geben zeigt coinGive, aufheben zeigt coin mit der Münze des Spiels; der Wurf des Monarchen ist eine Lücke', () => {
    const give = actionOf('coinGive')!;
    const pickup = actionOf('coinPickup')!;
    expect([give.effect?.kind, pickup.effect?.kind]).toEqual(['coinGive', 'coin']);
    expect([give.prop, pickup.prop]).toEqual(['coin', 'coin']);
    expect(give.gaps.map((g) => g.reason).join()).toContain('Wurf');
    expect(pickup.gaps).toEqual([]);
  });

  it('Getroffen zeigt hit, Tod zeigt down mit der Deckkraft des Spiels, Respawn wechselt zurück auf volle Deckkraft', () => {
    expect(actionOf('hit')!.effect?.kind).toBe('hit');
    const down = actionOf('down')!;
    expect([down.effect?.kind, down.alpha]).toEqual(['down', RESPAWN_ALPHA]);
    expect(actionOf('respawn')!.fade).toEqual({ from: RESPAWN_ALPHA, to: 1 });
    for (const id of ['hit', 'down', 'respawn'] as const) expect(actionOf(id)!.gaps.length, id).toBeGreaterThan(0);
  });

  it('die Deckkraft steht noch so im Spiel (worldRenderer.ts setAlpha bei respawnIn > 0)', () => {
    expect(workerSource).toContain(`setAlpha(p.respawnIn > 0 ? ${RESPAWN_ALPHA} : 1)`);
  });

  it('Skill ohne Ziel zeigt noTarget mit dem Text des Spiels', () => {
    const fx = actionOf('noTarget')!.effect!;
    expect(fx.kind).toBe('noTarget');
    expect(fx.text).toBe('kein Ziel');
  });

  it('jeder Effekt der Aktionen ist ein Effekt aus EFFECT_CONFIG; Bewegung und Aktionen kennen kein Laufen zu Fuß', () => {
    for (const a of ACTIONS) if (a.effect) expect(Object.keys(EFFECT_CONFIG)).toContain(a.effect.kind);
    expect(JSON.stringify(MOVES) + JSON.stringify(ACTIONS)).not.toMatch(/zu Fuß/i);
  });
});

describe('Skills (B-463/AC-06, AC-07)', () => {
  it('alle Skills aus data/monarch.json mit Linie, Stufe und Art, jeder ohne Darstellung mit der Lücke', () => {
    const rows = skillRows();
    const fromData = Object.keys(MONARCH.skills);
    expect(fromData).toHaveLength(24);
    for (const id of fromData) {
      const row = rows.find((r) => r.id === id)!;
      expect(row, id).toBeDefined();
      expect(row.line, id).not.toBe('');
      expect(row.tier, id).toBeGreaterThan(0);
      expect(row.kind, id).not.toBe('');
      expect(row.gap.reason, id).toBe('Lücke: keine Darstellung im Spiel');
    }
  });

  it('Skills der Presets, die in skills fehlen (backstab, smokeBomb), stehen mit der Lücke „Skill fehlt in data/monarch.json“, kein Absturz', () => {
    const rows = skillRows();
    const missing = Object.values(MONARCH.presets).flatMap((p) => p.activeSkills).filter((id) => !(id in MONARCH.skills));
    expect(missing).toEqual(expect.arrayContaining(['backstab', 'smokeBomb']));
    for (const id of missing) {
      const row = rows.find((r) => r.id === id)!;
      expect(row.gap.reason, id).toBe('Lücke: Skill fehlt in data/monarch.json');
      expect(row.tier).toBeNull();
    }
    expect(rows).toHaveLength(Object.keys(MONARCH.skills).length + new Set(missing).size);
  });
});
