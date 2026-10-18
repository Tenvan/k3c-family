import { describe, expect, it } from 'vitest';
import bossesJson from '@data/bosses.json';
import spritesJson from '@data/sprites.json';
import { BOSS_DATA, DISPLAY_GAP, bossCards, displaysOf, phaseTexts } from './bossLogic';
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

describe('Lücken der Boss-Anzeigen (B-464/AC-12)', () => {
  it('Warnkreis nur bei Flächenschlag, Phasen-Text nur beim Endboss mit Phasen, Boss-Leiste und Event-Banner bei jedem Boss', () => {
    const by = Object.fromEntries(BOSS_DATA.map((b) => [b.id, displaysOf(b)]));
    expect(by.goblinLeader).toEqual(['leiste', 'banner']); // kein Warnkreis, keine Phasen
    expect(by.trollKing).toEqual(['warnkreis', 'leiste', 'banner']);
    expect(by.ratQueen).toEqual(['leiste', 'banner']);
    expect(by.lavaGolem).toEqual(['leiste', 'banner']);
    expect(by.shardTitan).toEqual(['leiste', 'banner']);
    expect(by.crystalHeartWarden).toEqual(['warnkreis', 'leiste', 'phasen', 'banner']);
  });

  it('zusammen 15 Einträge, jeder mit dem Text „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“, kein Ersatz im Studio', () => {
    const gaps = bossCards().flatMap((c) => c.boss!.displays);
    expect(gaps).toHaveLength(15);
    for (const g of gaps) expect(g.reason).toContain(DISPLAY_GAP);
    expect(DISPLAY_GAP).toBe('Anzeige im Spiel fehlt (B-132, Anbindung B-469)');
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
