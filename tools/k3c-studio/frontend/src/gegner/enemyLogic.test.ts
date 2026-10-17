import { describe, expect, it } from 'vitest';
import enemiesJson from '@data/enemies.json';
import spritesJson from '@data/sprites.json';
import { BIOMES } from '@game/model/biome';
import { EFFECT_CONFIG } from '@game/scenes/effects';
import { DEFAULT_MOUNT } from '../monarch/monarchLogic';
import { ENEMY_ACTIONS, ENEMY_ANIMS, ENEMY_IDS, FALLBACK_ID, SPACING, cardOf, enemyItems, groups } from './enemyLogic';
import { resolveAnim, SHEETS } from '../abspiel/figure';

// B-464 (WZ11.1): Gegnertypen nach Biom, Karten, Merkmale, Lücken, Animationen mit den Parametern des Spiels.

const ENEMIES = enemiesJson as unknown as Record<string, { traits?: string[] }>;
const SPRITES = spritesJson as unknown as { enemies: Record<string, { sheet: string; tint?: string; scale?: number; alpha?: number; y?: number }> };

describe('Gegnertypen (B-464/AC-01, AC-02)', () => {
  it('alle Gegnertypen aus data/enemies.json sind gelistet (heute 17)', () => {
    expect(ENEMY_IDS).toEqual(Object.keys(ENEMIES));
    expect(ENEMY_IDS).toHaveLength(17);
  });

  it('Gruppen je Biom nach Tiefe, Gegner des Portals vor denen der Nacht', () => {
    const g = groups();
    const biomes = g.filter((x) => x.depth !== null);
    expect(biomes.map((x) => x.depth)).toEqual([...BIOMES].map((b) => b.depth).sort((a, b) => a - b));
    expect(g.at(-1)!.id).toBe('none');
    const forest = g.find((x) => x.id === 'forest')!;
    expect(forest.ids).toEqual(['greed', 'goblin', 'goblinArcher', 'wolf']);
    for (const b of BIOMES) {
      const group = g.find((x) => x.id === b.id)!;
      expect(group.ids, b.id).toEqual([...b.enemies.portal, ...b.enemies.night].filter((id, i, all) => all.indexOf(id) === i));
    }
  });

  it('die Gruppe „ohne Biom“ trägt genau die Typen, die kein Biom nennt (alphaWolf), und kein Typ fehlt', () => {
    const g = groups();
    const named = new Set(g.filter((x) => x.depth !== null).flatMap((x) => x.ids));
    const none = g.find((x) => x.id === 'none')!;
    expect(none.ids).toContain('alphaWolf');
    expect(none.ids.every((id) => !named.has(id))).toBe(true);
    expect(new Set([...named, ...none.ids]).size).toBe(ENEMY_IDS.length);
  });
});

describe('Parameter des Spiels (B-464/AC-03)', () => {
  it('die Figuren tragen Sheet, tint, scale, alpha und y aus data/sprites.json', () => {
    const cards = ENEMY_IDS.map(cardOf);
    const items = enemyItems(cards, { anim: 'idle' });
    items.forEach((item, i) => expect(item.spec).toEqual(SPRITES.enemies[ENEMY_IDS[i]!]));
    expect(Object.values(SPRITES.enemies).some((s) => (s.y ?? 0) > 0)).toBe(true);
    expect(items.map((i) => i.x)).toEqual(ENEMY_IDS.map((_, i) => i * SPACING));
  });

  it('die Grafik-Auswahl ersetzt nur das Sheet einer Figur', () => {
    const items = enemyItems([cardOf('greed'), cardOf('wolf')], { anim: 'run', sheets: { wolf: 'goblin' } });
    expect(items[0]!.spec).toEqual(SPRITES.enemies.greed);
    expect(items[1]!.spec).toEqual({ ...SPRITES.enemies.wolf, sheet: 'goblin' });
    expect(items.every((i) => i.anim === 'run')).toBe(true);
  });
});

describe('Animationen und Lücken (B-464/AC-04, AC-05, AC-06)', () => {
  it('fehlende Animation: idle und Lücke (mineGhost hat kein run)', () => {
    const sheet = SPRITES.enemies.mineGhost!.sheet;
    expect(SHEETS[sheet]!.anims.run).toBeUndefined();
    expect(resolveAnim(sheet, 'run')).toEqual({ anim: 'idle', gap: { kind: 'anim', reason: 'Lücke: run fehlt' } });
    expect(ENEMY_ANIMS).toEqual(['idle', 'run', 'attack']);
  });

  it('Getroffen zeigt hit, Sterben kill aus EFFECT_CONFIG; eigene Animationen sind eine Lücke', () => {
    expect(ENEMY_ACTIONS.map((a) => a.effect.kind)).toEqual(['hit', 'kill']);
    for (const a of ENEMY_ACTIONS) {
      expect(Object.keys(EFFECT_CONFIG)).toContain(a.effect.kind);
      expect(a.gaps.length).toBeGreaterThan(0);
    }
    const items = enemyItems([cardOf('greed')], { anim: 'idle', action: ENEMY_ACTIONS[1]! });
    expect(items[0]!.effect).toEqual({ kind: 'kill' });
  });

  it('Merkmale stehen an der Karte; ohne Darstellung im Spiel mit Lücke, fliegende Gegner zeigen sich über die Höhe', () => {
    const greed = cardOf('greed');
    expect(greed.traits.map((t) => t.trait)).toEqual(ENEMIES.greed!.traits);
    expect(greed.traits.every((t) => t.gap?.kind === 'display')).toBe(true);
    const flyer = ENEMY_IDS.find((id) => ENEMIES[id]!.traits?.includes('flying') && (SPRITES.enemies[id]?.y ?? 0) > 0);
    expect(flyer).toBeDefined();
    expect(cardOf(flyer!).traits.find((t) => t.trait === 'flying')!.gap).toBeNull();
  });

  it('ein Gegner ohne Eintrag in den Daten oder in sprites.json ergibt eine Karte mit Lücke, keinen Absturz', () => {
    const card = cardOf('gibtsnicht');
    expect(card.def).toBeNull();
    expect(card.spec).toEqual(SPRITES.enemies[FALLBACK_ID]);
    expect(card.gaps.map((g) => g.reason)).toEqual(['Lücke: Gegner fehlt in data/enemies.json', 'Lücke: keine eigene Grafik']);
    expect(cardOf('greed').gaps).toEqual([]);
  });
});

describe('Monarch-Vergleich (B-464/AC-13)', () => {
  it('Monarch 0 beritten auf dem Standard-Reittier links neben den Gegnern, alle auf derselben Bodenlinie', () => {
    const items = enemyItems([cardOf('greed'), cardOf('wolf')], { anim: 'idle', withMonarch: true });
    expect(items).toHaveLength(3);
    expect(items[0]).toMatchObject({ id: 'monarch-vergleich', x: -SPACING, mount: { key: DEFAULT_MOUNT } });
    expect(items.slice(1).every((i) => i.mount === undefined)).toBe(true);
    expect(enemyItems([cardOf('greed')], { anim: 'idle' })).toHaveLength(1);
  });
});
