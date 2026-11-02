import { describe, expect, it } from 'vitest';
import { UNIT_PX } from '@game/core/constants';
import { skillLook } from '@game/scenes/effects/skillLooks';
import { STATUS_KINDS, type StatusKind } from '@game/scenes/effects/statusLooks';
import { cycleTime, statusAt, type TimedEffect } from '../playback/timeline';
import { DEFAULT_CONTROL } from '../playback/figure';
import { enemyStatusEffects } from '../enemies/statusLogic';
import { enemyItems, cardOf } from '../enemies/enemyLogic';
import { SKILL_RING_PX, skillPlay } from './actions';
import { DEFAULT_MOUNT, monarchItems, moveOf } from './monarchLogic';
import { skillItems } from './skillItems';
import { CASTLE_OFFSET, TARGET_DISTANCE, skillStatus } from './skillTarget';

// B-676/AC-01, AC-02: Flächenzauber am Ziel-Gegner mit dem Radius des Skills, Status-Effekte des Spiels über ihre Dauer.

const goblin = { kind: 'goblin', distance: TARGET_DISTANCE };
const monarchs = monarchItems(DEFAULT_MOUNT, moveOf('stand'), 'idle');

describe('Flächenzauber am Ziel (B-676/AC-01)', () => {
  it.each([['fireball', 3], ['meteor', 15], ['iceWall', 3], ['lightningStorm', 5]])('%s mit Goblin in 8 Units: Ring am Goblin mit %i Units Radius', (id, radius) => {
    const play = skillPlay(id, goblin);
    const items = skillItems(monarchs, play, goblin);
    expect(items.map((i) => i.id)).toEqual(['monarch-0', 'ziel-goblin']);
    expect(items[1]!.x - items[0]!.x).toBe(TARGET_DISTANCE);
    expect(items[0]!.timed?.some((e) => e.kind === 'burst')).toBe(false);
    expect(items[1]!.timed?.[0]).toEqual({ kind: 'burst', look: { ...skillLook(id)!, radius: radius * UNIT_PX }, cycleMs: skillLook(id)!.durationMs + 700, y: SKILL_RING_PX });
  });

  it('Ziel außerhalb der Reichweite oder ohne Ziel: „Skill ohne Ziel“ am Monarchen, kein Effekt am Gegner', () => {
    const far = { kind: 'goblin', distance: 11 }; // fireball range 10
    const items = skillItems(monarchs, skillPlay('fireball', far), far);
    expect(items[0]!.effect?.kind).toBe('noTarget');
    expect(items[1]!.timed).toBeUndefined();
    expect(skillItems(monarchs, skillPlay('fireball', null), null).every((i) => i.effect?.kind === 'noTarget')).toBe(true);
  });

  it('Skill ohne Fläche bleibt am Monarchen (taunt), der Gegner zeigt den Spott', () => {
    const items = skillItems(monarchs, skillPlay('taunt', goblin), goblin);
    expect(items[0]!.timed?.[0]?.kind).toBe('burst');
    expect(items[1]!.timed).toEqual([expect.objectContaining({ kind: 'status', status: 'taunt', durationMs: 4000 })]);
  });
});

const statusOf = (effects: readonly TimedEffect[]): StatusKind[] => effects.flatMap((e) => (e.kind === 'status' ? [e.status] : []));

describe('Status-Effekte abspielbar (B-676/AC-02)', () => {
  it('jede Effektart aus B-675 kommt aus einem Skill, an Gegner bzw. Monarch', () => {
    const shown = new Set<StatusKind>();
    expect(statusOf(skillStatus('shieldBash', { kind: 'goblin', distance: 1 }).onTarget)).toEqual(['stun']);
    expect(statusOf(skillStatus('iceWall', goblin).onTarget)).toEqual(['slow']);
    expect(statusOf(skillStatus('lightningStorm', goblin).onTarget)).toEqual(['storm']);
    expect(statusOf(skillStatus('ironWall', null).onMonarch)).toEqual(['shield']);
    expect(statusOf(skillStatus('divineShield', null).onMonarch)).toEqual(['shield']);
    expect(statusOf(skillStatus('lastStand', null).onMonarch)).toEqual(['lastStand']);
    for (const id of ['armorAura', 'healingAura', 'blessings', 'guardian', 'regeneration', 'frostArmor']) expect(statusOf(skillStatus(id, null).onMonarch), id).toEqual(['aura']);
    for (const id of ['shieldBash', 'iceWall', 'lightningStorm', 'taunt', 'ironWall', 'lastStand', 'armorAura']) {
      const s = skillStatus(id, { kind: 'goblin', distance: 1 });
      statusOf([...s.onTarget, ...s.onMonarch]).forEach((k) => shown.add(k));
    }
    expect([...shown].sort()).toEqual([...STATUS_KINDS].sort());
  });

  it('Holy Ground sitzt als Ring an der Burg (Burgradius, nicht am Monarchen); Shield Bash ohne Ziel in 2 Units keine Betäubung', () => {
    const holy = skillStatus('holyGround', null);
    expect(statusOf(holy.onMonarch)).toEqual(['aura']);
    expect(holy.gaps).toEqual([]);
    const ring = statusAt(holy.onMonarch[0] as Extract<TimedEffect, { kind: 'status' }>, 0)[0]!;
    expect(ring).toMatchObject({ kind: 'aura', x: CASTLE_OFFSET * UNIT_PX, radius: 5 * UNIT_PX });
    const regen = statusAt(skillStatus('regeneration', null).onMonarch[0] as Extract<TimedEffect, { kind: 'status' }>, 0)[0]!;
    expect(regen.x).toBe(0);
    expect(skillStatus('shieldBash', goblin).onTarget).toEqual([]);
  });

  it('über die Dauer sichtbar, danach leer; Schleife und Einzelbild wirken (Zeichnung aus statusMarks des Spiels)', () => {
    const stun = skillStatus('shieldBash', { kind: 'goblin', distance: 1 }).onTarget[0] as Extract<TimedEffect, { kind: 'status' }>;
    expect(stun.durationMs).toBe(2000);
    expect(statusAt(stun, 1999).map((m) => m.kind)).toEqual(['stun']);
    expect(statusAt(stun, 2000)).toEqual([]);
    expect(cycleTime(stun, DEFAULT_CONTROL, stun.cycleMs + 10, 6)).toBe(10);
    expect(cycleTime(stun, { ...DEFAULT_CONTROL, frame: 0 }, 0, 4)).toBeLessThan(stun.durationMs);
    const storm = skillStatus('lightningStorm', goblin).onTarget[0] as Extract<TimedEffect, { kind: 'status' }>;
    expect(statusAt(storm, 0)[0]).toMatchObject({ kind: 'storm', radius: 5 * UNIT_PX });
  });

  it('Gegner-Seite, Modus „Status“: alle Gegner der Gruppe zeigen Betäubung, Verlangsamung, Spott und Sturm nacheinander', () => {
    const effects = enemyStatusEffects();
    expect(statusOf(effects)).toEqual(['stun', 'slow', 'taunt', 'storm']);
    const delays = effects.map((e) => (e.kind === 'status' ? e.delayMs : -1));
    expect(delays).toEqual([0, 2000, 7000, 11000]);
    const items = enemyItems([cardOf('goblin'), cardOf('wolf')], { anim: 'idle', status: effects });
    expect(items.every((i) => i.timed === effects)).toBe(true);
  });
});
