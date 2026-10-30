import { describe, expect, it } from 'vitest';
import bossesJson from '@data/bosses.json';
import effectsSource from '@game/scenes/effects/effects.ts?raw';
import kampfSource from '@game/scenes/kampfView.ts?raw';
import { DEFAULT_CONTROL } from '../playback/figure';
import { PROJECTILE, cycleTime, warnIn } from '../playback/timeline';
import { HIT_Y, SUMMON_STEP, abilityEntries, abilityPlay, attackEffects } from './abilityLogic';
import type { Ability, BossData } from './bossLogic';
import { cardOf } from './enemyLogic';

// B-672/AC-06 bis AC-08: Angriffe und Boss-Fähigkeiten aus den Daten, wie das Spiel sie zeigt, oder die Lücke.

const DATA = bossesJson as unknown as { bosses: BossData[]; endBosses: BossData[] };
const ALL = [...DATA.bosses, ...DATA.endBosses];
/** Fähigkeiten, deren Darstellung das Spiel hat */
const DRAWN = new Set(['aoe', 'summon', 'shardThrow']);

describe('Boss-Fähigkeiten (AC-06)', () => {
  it('jede Fähigkeit aus data/bosses.json lässt sich abspielen oder zeigt ihre Lücke', () => {
    const abilities = ALL.flatMap((b) => [b.ability, ...(b.phases ?? []).map((p) => p.ability)]).filter((a): a is Ability => Boolean(a));
    expect(abilities.length).toBeGreaterThanOrEqual(7);
    for (const a of abilities) {
      const play = abilityPlay(a, { x: 0, range: 8 });
      const shows = play.timed.length + play.extra.length > 0;
      expect(shows || play.gaps.length > 0, a.type).toBe(true);
      if (!DRAWN.has(a.type)) expect(play.gaps[0]!.reason, a.type).toContain(a.type);
      if (play.timed.length > 0) expect(play.timed.every((t) => t.cycleMs === (a.intervalSeconds ?? 1) * 1000), a.type).toBe(true);
    }
  });

  it('aoe: Warnkreis mit dem Radius der Daten in Units, ausgelöst im Intervall', () => {
    const troll = ALL.find((b) => b.id === 'trollKing')!;
    expect(abilityPlay(troll.ability!, { x: 7, range: troll.range }).timed).toEqual([{ kind: 'warn', radius: troll.ability!.radius, cycleMs: troll.ability!.intervalSeconds! * 1000 }]);
    expect(kampfSource).toMatch(/class WarnView/); // der Warnkreis des Spiels zeichnet
  });

  it('summon: die beschworenen Gegner mit ihrer Grafik neben dem Boss; ohne Sheet Rahmen und Lücke', () => {
    const play = abilityPlay({ type: 'summon', enemy: 'goblin', count: 2, intervalSeconds: 10 }, { x: 14, range: 1 });
    expect(play.extra.map((f) => f.x)).toEqual([14 + SUMMON_STEP, 14 - SUMMON_STEP]);
    expect(play.extra.every((f) => f.spec.sheet === 'goblin')).toBe(true);
    const unknown = abilityPlay({ type: 'summon', enemy: 'gibtsnicht', count: 1, intervalSeconds: 10 }, { x: 0, range: 1 });
    expect(unknown.extra).toEqual([]);
    expect(unknown.timed).toEqual([{ kind: 'frame', at: SUMMON_STEP, cycleMs: 10000 }]);
    expect(unknown.gaps[0]!.reason).toContain('gibtsnicht');
  });

  it('shardThrow wirft das Geschoss des Spiels bis zur Reichweite; flameTrail und Unbekanntes sind Lücken', () => {
    const shard = abilityPlay({ type: 'shardThrow', radius: 2, intervalSeconds: 2 }, { x: 0, range: 8 }, -1);
    expect(shard.timed[0]).toMatchObject({ kind: 'projectile', range: 8, dir: -1, speed: PROJECTILE.speed });
    expect(shard.anim).toBe('attack');
    expect(abilityPlay({ type: 'flameTrail', intervalSeconds: 1 }, { x: 0, range: 1 }).gaps[0]!.reason).toContain('flameTrail');
    expect(abilityPlay({ type: 'quake' }, { x: 0, range: 1 }).gaps[0]!.reason).toBe('Lücke: Fähigkeit quake ohne Darstellung');
  });

  it('Phasen des Endbosses: jede Phase mit geltender Fähigkeit hat einen Eintrag, Phase 3 erbt die der Phase 2', () => {
    const warden = abilityEntries().filter((e) => e.bossId === 'crystalHeartWarden');
    expect(warden.map((e) => [e.phase, e.ability.type, e.inherited])).toEqual([[1, 'summon', false], [2, 'aoe', false], [3, 'aoe', true]]);
    expect(abilityEntries([{ ...ALL[0]!, ability: undefined, phases: undefined } as BossData])).toEqual([]);
  });
});

describe('Angriff der Gegner (AC-07)', () => {
  it('Fernkämpfer: Geschoss bis zur Reichweite, Treffer beim Einschlag; Nahkämpfer: nur der Treffer in Reichweite', () => {
    const archer = cardOf('goblinArcher');
    const [shot, hit] = attackEffects(archer.def, archer.spec.sheet, 1);
    expect(shot).toMatchObject({ kind: 'projectile', range: 10, speed: PROJECTILE.speed });
    expect(hit).toMatchObject({ kind: 'burst', at: 10, delayMs: 500, y: HIT_Y });
    const goblin = cardOf('goblin');
    expect(attackEffects(goblin.def, goblin.spec.sheet, -1).map((t) => t.kind)).toEqual(['burst']);
    expect(attackEffects(null, 'goblin', 1)).toEqual([]);
    expect(effectsSource).toContain(`const BODY_PX = ${HIT_Y};`);
  });
});

describe('Wiedergabe wirkt auf Fähigkeiten (AC-08)', () => {
  it('Intervall in Schleife, Einzelbild steht im Zyklus, ohne Schleife löst es einmal aus', () => {
    const warn = abilityPlay({ type: 'aoe', radius: 3, intervalSeconds: 4 }, { x: 0, range: 1 }).timed[0]!;
    expect(warnIn(warn.cycleMs, cycleTime(warn, DEFAULT_CONTROL, 5000, 1))).toBe(3);
    expect(cycleTime(warn, { ...DEFAULT_CONTROL, frame: 0 }, 99999, 4)).toBe(500);
    expect(cycleTime(warn, { ...DEFAULT_CONTROL, loop: false }, 5000, 1)).toBe(4000);
  });
});
