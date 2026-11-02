import { describe, expect, it, vi } from 'vitest';
import { GROUND_Y, UNIT_PX } from '../../core/constants';
import type { Enemy, Player } from '../../model/types';
import { SKILL_LOOKS } from './skillLooks';
import { drawStatus, SHIELD_MAX, STATUS_KINDS, STATUS_LOOKS, statusMarks, type StatusKind, type StatusPainter, type StatusWorld } from './statusLooks';

vi.mock('phaser', () => ({ default: {} }));
const { StatusView } = await import('./statusView');

const player = (over: Partial<Player> = {}): Player =>
  ({ id: 1, index: 0, x: 10, vx: 0, facing: 1, gold: 0, hp: 100, maxHp: 100, respawnIn: 0, payCooldown: 0, paying: false, payKey: null, payAmount: 0, points: 0, actions: [], ...over }) as Player;
const enemy = (over: Partial<Enemy> = {}): Enemy =>
  ({ id: 7, kind: 'goblin', x: 20, hp: 10, maxHp: 10, damage: 1, speed: 1, range: 1, traits: [], cooldown: 0, fleeing: false, carriedGold: 0, homeX: 0, ...over }) as Enemy;

/** Je Effektart ein Zustand, der genau sie zeigt */
const ONE: Record<StatusKind, StatusWorld> = {
  stun: { enemies: [enemy({ stun: 2 })] },
  slow: { enemies: [enemy({ slow: 0.5, slowFor: 5 })] },
  taunt: { enemies: [enemy({ tauntId: 1, tauntFor: 4 })] },
  storm: { storms: [{ x: 8, radius: 5, perSecond: 10, left: 5, owner: 1 }] },
  shield: { players: [player({ shield: 250, shieldFor: 10 })] },
  lastStand: { players: [player({ lastStandFor: 10 })] },
  aura: { players: [player({ auras: [{ skill: 'healingAura', radius: 8 }] })] },
};

/** Zeichenfläche, die Aufrufe zählt */
function painter(): StatusPainter & { calls: number } {
  const g = { calls: 0 } as StatusPainter & { calls: number };
  for (const fn of ['fillStyle', 'lineStyle', 'fillCircle', 'strokeCircle', 'fillRect', 'fillTriangle', 'lineBetween'] as const) {
    (g as unknown as Record<string, () => unknown>)[fn] = () => (g.calls++, g);
  }
  return g;
}

describe('Status-Effekte (B-675/AC-03)', () => {
  it.each(STATUS_KINDS)('%s wird gezeichnet und ohne Effekt wieder entfernt', (kind) => {
    const marks = statusMarks(ONE[kind]);
    expect(marks.map((m) => m.kind)).toEqual([kind]);
    const g = painter();
    drawStatus(g, marks, 500);
    expect(g.calls).toBeGreaterThan(0);
    expect(statusMarks({ players: [player()], enemies: [enemy()], storms: [] })).toEqual([]);
  });

  it('Sturm am Boden mit seinem Radius, Aura in der Farbe ihres Skills, Holy Ground an der Burg', () => {
    expect(statusMarks(ONE.storm)[0]).toMatchObject({ x: 8 * UNIT_PX, y: GROUND_Y, radius: 5 * UNIT_PX });
    expect(statusMarks(ONE.aura)[0]).toMatchObject({ x: 10 * UNIT_PX, radius: 8 * UNIT_PX, color: SKILL_LOOKS.healingAura!.color });
    const holy = statusMarks({ players: [player({ auras: [{ skill: 'holyGround', radius: 5, castle: true }] })], castle: { x: 40 } });
    expect(holy[0]).toMatchObject({ x: 40 * UNIT_PX, radius: 5 * UNIT_PX });
  });

  it('Schild-Balken folgt den Schild-HP, Symbole über dem Gegner, Boss höher', () => {
    expect(statusMarks(ONE.shield)[0]?.share).toBeCloseTo(250 / SHIELD_MAX);
    expect(statusMarks(ONE.stun)[0]).toMatchObject({ x: 20 * UNIT_PX, y: GROUND_Y - STATUS_LOOKS.stun.y });
    expect(statusMarks({ enemies: [enemy({ stun: 1, boss: true })] })[0]?.y).toBe(GROUND_Y - STATUS_LOOKS.stun.bossY);
  });

  it('tote Gegner und gefallene Monarchen zeigen nichts; Boden vor Symbolen', () => {
    expect(statusMarks({ enemies: [enemy({ hp: 0, stun: 2 })], players: [player({ respawnIn: 3, shield: 9, shieldFor: 9 })] })).toEqual([]);
    const all = statusMarks({ ...ONE.stun, ...ONE.aura, storms: ONE.storm.storms });
    expect(all.map((m) => m.kind)).toEqual(['aura', 'storm', 'stun']);
  });

  it('StatusView: Boden und Symbole in je eigener Fläche, jedes Bild geleert', () => {
    const made: (StatusPainter & { calls: number; cleared: number; depth: number })[] = [];
    const scene = { add: { graphics: () => { const g = Object.assign(painter(), { cleared: 0, depth: 0 }); Object.assign(g, { clear: () => (g.cleared++, g), setDepth: (d: number) => ((g.depth = d), g), destroy: () => undefined }); made.push(g); return g; } } };
    const view = new StatusView(scene as never, { add: () => undefined } as never);
    const [ground, over] = made;
    view.sync({ ...ONE.aura, ...ONE.stun }, 0);
    expect(ground!.calls).toBeGreaterThan(0);
    expect(over!.calls).toBeGreaterThan(0);
    expect(ground!.depth).toBeLessThan(over!.depth);
    const before = ground!.calls + over!.calls;
    view.sync({}, 16);
    expect(ground!.calls + over!.calls).toBe(before);
    expect(ground!.cleared).toBe(2);
  });
});
