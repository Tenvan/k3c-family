import { describe, expect, it } from 'vitest';
import bossesJson from '@data/bosses.json';
import spritesJson from '@data/sprites.json';
import { bossBar, eventBanner, warnCircles } from '@game/scenes/kampfAnzeige';
import { BOSS_DATA, DISPLAY_GAP, MISSING_DISPLAYS, abilityIn, bossCards, bossWorld, displaysOf, phaseCount, phaseShare, phaseTexts } from './bossLogic';
import { FALLBACK_ID } from './enemyLogic';

// B-464 (WZ11.2): Bosse je Tiefe, Grafik wie im Spiel, Lücken der Anzeigen (Anbindung B-469).

const DATA = bossesJson as unknown as { bosses: { id: string }[]; endBosses: { id: string; phases: { ability?: unknown }[] }[] };
const SPRITES = spritesJson as unknown as { enemies: Record<string, unknown> };

describe('Boss-Gruppe je Tiefe (B-464/AC-07)', () => {
  it('6 Bosse: 5 Minibosse (Tiefe 0–4) und der Endboss, nach Tiefe geordnet', () => {
    expect(BOSS_DATA).toHaveLength(6);
    expect(BOSS_DATA).toHaveLength(DATA.bosses.length + DATA.endBosses.length);
    expect(BOSS_DATA.map((b) => b.id)).toEqual(['goblinLeader', 'trollKing', 'ratQueen', 'lavaGolem', 'shardTitan', 'crystalHeartWarden']);
    expect(BOSS_DATA.map((b) => b.depth)).toEqual([0, 1, 2, 3, 4, 4]);
    expect(bossCards().map((c) => c.id)).toEqual(BOSS_DATA.map((b) => b.id));
  });
});

describe('Boss-Grafik wie im Spiel (B-464/AC-08)', () => {
  it('Boss-IDs haben keinen Eintrag in sprites.json: Rückfall goblin und die Lücke „keine eigene Boss-Grafik“', () => {
    for (const card of bossCards()) {
      expect(SPRITES.enemies[card.id], card.id).toBeUndefined();
      expect(card.spec).toEqual(SPRITES.enemies[FALLBACK_ID]);
      expect(card.gaps.map((g) => g.reason)).toEqual(['Lücke: keine eigene Boss-Grafik']);
    }
  });
});

describe('Anzeigen je Boss (B-464/AC-12)', () => {
  it('Warnkreis nur bei Flächenschlag, Phasen-Text nur beim Endboss mit Phasen, Boss-Leiste und Event-Banner bei jedem Boss', () => {
    const by = Object.fromEntries(BOSS_DATA.map((b) => [b.id, displaysOf(b)]));
    expect(by.goblinLeader).toEqual(['leiste', 'banner']); // kein Warnkreis, keine Phasen
    expect(by.trollKing).toEqual(['warnkreis', 'leiste', 'banner']);
    expect(by.ratQueen).toEqual(['leiste', 'banner']);
    expect(by.lavaGolem).toEqual(['leiste', 'banner']);
    expect(by.shardTitan).toEqual(['leiste', 'banner']);
    expect(by.crystalHeartWarden).toEqual(['warnkreis', 'leiste', 'phasen', 'banner']);
  });

  it('nur das Event-Banner bleibt Lücke (6 Einträge, jeder mit dem Text „Anzeige im Spiel fehlt …“): das Spiel kennt keine Boss-Ereignisse', () => {
    expect(MISSING_DISPLAYS).toEqual(['banner']);
    const gaps = bossCards().flatMap((c) => c.boss!.displays);
    expect(gaps).toHaveLength(6);
    for (const g of gaps) {
      expect(g.reason).toContain(DISPLAY_GAP);
      expect(g.reason).toContain('Event-Banner');
    }
  });
});

describe('Beispielzustände für die Anzeigen des Spiels (B-469)', () => {
  const boss = (id: string) => BOSS_DATA.find((b) => b.id === id)!;

  it('Boss-Leiste: Name, HP-Anteil und Phasen-Text kommen aus dem Zeichencode des Spiels', () => {
    const bar = bossBar(bossWorld(boss('crystalHeartWarden'), { phase: 2, share: 0.55, x: 0 }))!;
    expect(bar).toMatchObject({ name: 'Kristallherz-Wächter', phase: 'Phase 2 von 3' });
    expect(bar.share).toBeCloseTo(0.55);
    expect(bossBar(bossWorld(boss('goblinLeader'), { share: 1, x: 0 }))).toMatchObject({ name: 'Goblin-Anführer', phase: '', share: 1 });
  });

  it('Warnkreis nur bei Flächenschlag, mit dem Radius der Daten in Pixeln und der Mitte des Bosses', () => {
    expect(warnCircles(bossWorld(boss('goblinLeader'), { share: 1, x: 4 }))).toEqual([]);
    const [c] = warnCircles(bossWorld(boss('trollKing'), { share: 1, x: 4 }));
    expect(c).toMatchObject({ x: 4 * 32, r: 3 * 32 });
    expect(c!.urgency).toBeCloseTo(0.5);
  });

  it('Endboss: Phase 1 ruft Spinnen (kein Warnkreis), Phase 2 und 3 Flächenschlag Radius 5; Phase 3 erbt ihn', () => {
    const b = boss('crystalHeartWarden');
    expect(phaseCount(b)).toBe(3);
    expect(abilityIn(b, 1)?.type).toBe('summon');
    expect(abilityIn(b, 2)).toMatchObject({ type: 'aoe', radius: 5 });
    expect(abilityIn(b, 3)).toEqual(abilityIn(b, 2));
    expect(warnCircles(bossWorld(b, { phase: 1, share: 1, x: 0 }))).toEqual([]);
    expect(warnCircles(bossWorld(b, { phase: 3, share: 0.2, x: 0 }))[0]!.r).toBe(5 * 32);
  });

  it('Beispiel-HP je Phase liegen knapp unter ihrer Schwelle; Minibosse haben keine Phasen', () => {
    const b = boss('crystalHeartWarden');
    expect([1, 2, 3].map((p) => phaseShare(b, p))).toEqual([1, 0.55, 0.2]);
    expect(phaseCount(boss('trollKing'))).toBe(0);
    expect(bossWorld(boss('trollKing'), { phase: 2, share: 1, x: 0 }).enemies![0]!.phase).toBeUndefined();
  });

  it('das Event-Banner des Spiels kennt keine Boss-Ereignisse: ohne Nacht-Event keine Zeile', () => {
    expect(eventBanner(bossWorld(boss('trollKing'), { share: 1, x: 0 }))).toBeNull();
  });
});

describe('Phasen (B-464)', () => {
  it('der Endboss hat drei Phasen; eine Phase ohne Fähigkeit nennt die der vorigen', () => {
    const phases = DATA.endBosses[0]!.phases;
    expect(phases).toHaveLength(3);
    expect(phases[2]!.ability).toBeUndefined();
    const texts = phaseTexts(phases as never);
    expect(texts[0]).toContain('ruft 2× crystalSpider');
    expect(texts[1]).toContain('Flächenschlag');
    expect(texts[2]).toContain('Fähigkeit der vorigen Phase');
    expect(texts[2]).toContain('Flächenschlag');
    expect(texts[2]).toContain('Tempo ×1.5');
    expect(phaseTexts(undefined)).toEqual([]);
  });
});
