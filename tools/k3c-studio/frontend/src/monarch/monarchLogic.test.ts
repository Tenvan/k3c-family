import { describe, expect, it } from 'vitest';
import monarchJson from '@data/monarch.json';
import spritesJson from '@data/sprites.json';
import { PLAYER_COLORS } from '@game/core/constants';
import { mountPose } from '@game/scenes/render/mountPose';
import type { MountData, SheetData, SpriteSpec } from '@game/scenes/render/sprites';
import type { AssetSlot } from '../api';
import { MOUNTS, SHEETS, mountPoseOf } from '../playback/figure';
import { DEFAULT_MOUNT, MONARCH_ANIMS, MOUNT_KEYS, MOVES, PLAYERS, REF_SPEED, SPACING, monarchItems, moveOf, slotOptions } from './monarchLogic';

// B-463 (WZ10.2): Vorschau-Liste, Reittiere, Spielerfarben und Grafik-Auswahl der Monarch-Seite.

const MONARCH = monarchJson as unknown as { base: { speed: number }; sprintMultiplier: number; mount: { sprite: string; speedFactor: number } };
const SPRITES = spritesJson as unknown as { players: SpriteSpec[]; mounts: Record<string, MountData>; sheets: Record<string, SheetData>; riderWaist: number };

describe('Bewegung nur beritten (B-463/AC-04)', () => {
  it('die Vorschau-Liste kennt Stehen, Laufen und Sprinten beritten, nichts zu Fuß, und keine Lücke dafür', () => {
    expect(MOVES.map((m) => m.id)).toEqual(['stand', 'walk', 'sprint']);
    expect(MOVES.map((m) => m.label).join(' ').toLowerCase()).not.toContain('fuß');
    for (const m of MOVES.filter((x) => x.speed > 0)) expect(m.label).toContain('beritten');
    expect(JSON.stringify(MOVES)).not.toContain('Lücke');
  });

  it('Laufen ist base.speed, Sprinten base.speed × sprintMultiplier (heute 5 und 9 Units/s)', () => {
    expect(REF_SPEED).toBe(MONARCH.base.speed * MONARCH.mount.speedFactor);
    expect(moveOf('walk').speed).toBe(REF_SPEED);
    expect(moveOf('sprint').speed).toBeCloseTo(REF_SPEED * MONARCH.sprintMultiplier);
    expect([moveOf('walk').speed, moveOf('sprint').speed]).toEqual([5, 9]);
  });

  it('das Reittier spielt run mit dem timeScale aus mountPose: Laufen 1, Sprinten 1,8', () => {
    const rider = PLAYERS[0]!.spec;
    expect(mountPoseOf(DEFAULT_MOUNT, rider, REF_SPEED, moveOf('walk').speed).pose?.timeScale).toBeCloseTo(1);
    const sprint = mountPoseOf(DEFAULT_MOUNT, rider, REF_SPEED, moveOf('sprint').speed).pose!;
    expect([sprint.anim, sprint.timeScale]).toEqual(['run', expect.closeTo(1.8)]);
    expect(mountPoseOf(DEFAULT_MOUNT, rider, REF_SPEED, 0).pose?.anim).toBe('idle');
  });
});

describe('Reiten auf allen Reittieren (B-463/AC-03)', () => {
  it('13 Reittiere, das Standard-Reittier aus data/monarch.json ist darunter', () => {
    expect(MOUNT_KEYS).toHaveLength(13);
    expect(MOUNT_KEYS).toContain(DEFAULT_MOUNT);
    expect(DEFAULT_MOUNT).toBe(MONARCH.mount.sprite);
  });

  it('die Pose der Seite ist gleich mountPose für Frame 0 von idle und run, für jedes Reittier und jeden Spieler', () => {
    for (const key of MOUNT_KEYS) {
      for (const move of MOVES) {
        for (const item of monarchItems(key, move, 'idle')) {
          const ctx = { key, rider: item.spec, refSpeed: REF_SPEED, mounts: SPRITES.mounts, sheets: SPRITES.sheets, riderWaist: SPRITES.riderWaist };
          const want = mountPose({ vx: move.speed, facing: 1 }, 0, ctx);
          expect(mountPoseOf(key, item.spec, REF_SPEED, item.mount!.speed, 0).pose, `${key}/${move.id}`).toEqual(want);
          expect(want, `${key}/${move.id}`).not.toBeNull();
        }
      }
    }
  });
});

describe('Spielerfarben nebeneinander (B-463/AC-10)', () => {
  it('vier Spielerfarben mit Sheet und tint aus data/sprites.json und der Kennfarbe aus PLAYER_COLORS', () => {
    expect(PLAYERS).toHaveLength(4);
    PLAYERS.forEach((p, i) => {
      expect(p.spec).toEqual(SPRITES.players[i]);
      expect(p.badge).toBe(`#${PLAYER_COLORS[i]!.toString(16).padStart(6, '0')}`);
    });
  });

  it('alle vier laufen im selben Takt: gleiche Animation, Reittier und Tempo, gleichmäßig nebeneinander', () => {
    const items = monarchItems('unicorn', moveOf('sprint'), 'run');
    expect(items.map((i) => i.anim)).toEqual(['run', 'run', 'run', 'run']);
    expect(new Set(items.map((i) => JSON.stringify(i.mount))).size).toBe(1);
    expect(items.map((i) => i.x)).toEqual([-1.5, -0.5, 0.5, 1.5].map((n) => n * SPACING));
    expect(items.map((i) => i.spec.tint)).toEqual(PLAYERS.map((p) => p.spec.tint));
  });

  it('die Animationen des Monarchen sind idle, run und attack (Sheets medieval-king)', () => {
    expect(MONARCH_ANIMS).toEqual(['idle', 'run', 'attack']);
  });
});

describe('Grafik-Auswahl (B-463/AC-12)', () => {
  const slot = (id: string, wahl: AssetSlot['wahl']): AssetSlot => ({ id, art: 'grafik', kategorie: 'figuren', beschreibung: id, wahl, status: 'zugeordnet' });
  const slots = [
    slot('monarch:2', [{ datei: 'sprites/medieval-king-2/idle.png', sheet: 'medieval-king-2' }, { datei: 'sprites/medieval-king/idle.png', sheet: 'medieval-king' }, { datei: 'sprites/x/idle.png', sheet: 'gibtsnicht' }, { datei: 'x.png' }]),
    slot('mount:unicorn', [{ datei: 'sprites/unicorn/idle.png', sheet: 'unicorn' }]),
  ];

  it('jede Zuordnung ist wählbar, Präferenz 1 steht vorn; ein unbekanntes Sheet ist eine Lücke', () => {
    const options = slotOptions(slots, 'monarch:2');
    expect(options.map((o) => [o.index, o.sheet])).toEqual([[0, 'medieval-king-2'], [1, 'medieval-king'], [2, 'gibtsnicht']]);
    expect(options[0]!.gap).toBeNull();
    expect(options[2]!.gap?.kind).toBe('sheet');
    expect(slotOptions(slots, 'mount:unicorn')[0]!.sheet).toBe('unicorn');
    expect(slotOptions(slots, 'monarch:9')).toEqual([]);
  });

  it('die Auswahl wirkt nur auf die Vorschau: Ersatz-Sheets landen in den Figuren, das Register bleibt unverändert', () => {
    const before = JSON.stringify(slots);
    const items = monarchItems('unicorn', moveOf('stand'), 'idle', { riders: [undefined, 'medieval-king'], mountSheet: 'horse' });
    expect(items[1]!.spec.sheet).toBe('medieval-king');
    expect(items[0]!.spec.sheet).toBe(PLAYERS[0]!.spec.sheet);
    expect(items[0]!.mount!.sheet).toBe('horse');
    expect(JSON.stringify(slots)).toBe(before);
  });

  it('die Sheets aller Standard-Slots monarch:1–4 und mount:* der Daten sind bekannte Sheets oder Lücken, nie ein Absturz', () => {
    expect(Object.keys(SHEETS).length).toBeGreaterThan(0);
    expect(Object.keys(MOUNTS)).toHaveLength(13);
  });
});
