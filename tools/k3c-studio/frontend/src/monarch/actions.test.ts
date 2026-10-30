import { describe, expect, it } from 'vitest';
import monarchJson from '@data/monarch.json';
import workerSource from '@game/scenes/render/worldRenderer.ts?raw';
import animSource from '@game/scenes/render/monarchAnim.ts?raw';
import { EFFECT_CONFIG } from '@game/scenes/effects/effects';
import { skillLook } from '@game/scenes/effects/skillLooks';
import castSource from '@game/scenes/effects/skillCasts.ts?raw';
import gameSceneSource from '@game/scenes/GameScene.ts?raw';
import { LINE_COLORS } from '@game/scenes/effects/effects';
import { ACTIONS, CAST_GAP, RESPAWN_ALPHA, SKILL_RING_PX, actionOf, effectMs, skillPlay, skillRows } from './actions';
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

  it('die Deckkraft steht noch so im Spiel (monarchAnim.ts DEAD_ALPHA bei respawnIn > 0, worldRenderer.ts setAlpha(pick.alpha))', () => {
    expect(animSource).toContain(`DEAD_ALPHA = ${RESPAWN_ALPHA}`);
    expect(animSource).toContain('alpha: DEAD_ALPHA');
    expect(workerSource).toContain('setAlpha(pick.alpha)');
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
  it('alle Skills aus data/monarch.json mit Linie, Stufe, Art und dem Effekt des Spiels, keine Lücke (B-562/AC-01)', () => {
    const rows = skillRows();
    const fromData = Object.keys(MONARCH.skills);
    expect(fromData).toHaveLength(26);
    for (const id of fromData) {
      const row = rows.find((r) => r.id === id)!;
      expect(row, id).toBeDefined();
      expect(row.line, id).not.toBe('');
      expect(row.tier, id).toBeGreaterThan(0);
      expect(row.kind, id).not.toBe('');
      expect(row.gap, id).toBeNull();
      expect(row.look, id).toEqual(skillLook(id));
    }
  });

  it('kein Preset nennt einen fehlenden Skill (B-504), die Liste hat genau die Skills aus data/monarch.json', () => {
    const missing = Object.values(MONARCH.presets).flatMap((p) => p.activeSkills).filter((id) => !(id in MONARCH.skills));
    expect(missing).toEqual([]);
    const rows = skillRows();
    expect(rows).toHaveLength(Object.keys(MONARCH.skills).length);
    expect(rows.some((r) => r.gap?.reason === 'Lücke: Skill fehlt in data/monarch.json')).toBe(false);
  });
});

describe('Skill abspielen (B-671/AC-06)', () => {
  it('jeder der 26 Skills spielt genau den Effekt aus skillLook (eigener Look oder Ring der Linie) oder zeigt seine Lücke', () => {
    const ids = Object.keys(MONARCH.skills);
    expect(ids).toHaveLength(26);
    for (const id of ids) {
      const play = skillPlay(id);
      const look = skillLook(id);
      if (!look) {
        expect(play.timed, id).toEqual([]);
        expect(play.gaps[0]!.reason, id).toContain('keine Darstellung');
        continue;
      }
      expect(play.look, id).toBe(look);
      expect(play.timed, id).toEqual([{ kind: 'burst', look, cycleMs: look.durationMs + 700, y: SKILL_RING_PX }]);
    }
  });

  it('ein Skill ohne eigenes Aussehen spielt den Ring in der Farbe seiner Linie', () => {
    const fallback = Object.keys(MONARCH.skills).find((id) => !skillPlay(id).own);
    if (fallback) expect(Object.values(LINE_COLORS)).toContain(skillPlay(fallback).look!.color);
    expect(skillPlay('gibtsnicht')).toMatchObject({ look: null, timed: [] });
    expect(skillPlay('gibtsnicht').gaps[0]!.reason).toContain('fehlt in data/monarch.json');
  });

  it('Ring in der Höhe des Spiels; das Spiel spielt beim Wirken keine Animation des Monarchen (Lücke)', () => {
    expect(castSource).toContain(`const RING_PX = ${SKILL_RING_PX};`);
    expect(skillPlay('fireball').gaps).toEqual([CAST_GAP]);
    const playSkill = gameSceneSource.slice(gameSceneSource.indexOf('private playSkill('), gameSceneSource.indexOf('private feedback('));
    expect(playSkill).not.toMatch(/attack|playAnim|swing/);
  });
});
