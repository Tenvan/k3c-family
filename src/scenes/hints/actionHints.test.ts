import { beforeEach, describe, expect, it } from 'vitest';
import { nameOf, setLanguage } from '../../core/texts';
import type { Player, PlayerAction, Site, World } from '../../model/types';
import type { Device } from '../../input/slotBindings';
import { GAME_WIDTH } from '../../core/constants';
import { HINT_FONT, WEIGHT, hintView, playerHint, type Hint } from './actionHints';
import { FONTS, minFontPx } from '../render/fontRules';

const player = (over: Partial<Player> = {}): Player =>
  ({ id: 0, index: 0, x: 10, vx: 0, facing: 1, gold: 0, hp: 100, maxHp: 100, respawnIn: 0, payCooldown: 0, paying: false, payKey: null, payAmount: 0, points: 0, actions: [], ...over }) as Player;
const site = (over: Partial<Site>): Site => ({ id: 1, kind: 'wall', x: 12, state: 'unpaid', paidGold: 0, buildProgress: 0, hp: 1, maxHp: 1, workerId: null, bows: 0, bowPaidGold: 0, ...over });
const world = (sites: Site[]): World => ({ sites }) as unknown as World;

const ALL: Hint[] = [
  { action: 'revive' },
  { action: 'build', name: 'Mauer' },
  { action: 'pay', name: 'Bogen' },
  { action: 'attack' },
  { action: 'skill', slot: 1, skill: 'taunt' },
  { action: 'learn' },
  { action: 'respec' },
];

beforeEach(() => setLanguage('de'));

describe('hintView: Aktion → Taste und Text je Gerät (B-125/AC-01)', () => {
  it.each<[Device, string[]]>([
    ['pad', ['A', 'A', 'A', 'X', 'LB', 'D-Pad ↓', 'D-Pad ↓']],
    ['keyboard', ['Leertaste', 'Leertaste', 'Leertaste', 'E', 'Q', 'K', 'K']],
    ['touch', ['🪙', '🪙', '🪙', '⚔', '1', '★', '★']],
  ])('%s', (device, keys) => {
    const views = ALL.map((h) => hintView(h, device));
    expect(views.map((v) => v.key)).toEqual(keys);
    expect(views.every((v) => v.text.includes(v.key) && v.text.length > v.key.length + 2)).toBe(true);
  });

  it('Texte wie im Regelwerk', () => {
    expect(hintView({ action: 'build', name: 'Mauer' }, 'pad').text).toBe('A halten: Mauer bauen');
    expect(hintView({ action: 'revive' }, 'pad').text).toBe('A halten: Wiederbeleben');
    expect(hintView({ action: 'attack' }, 'pad').text).toBe('X: Schlag');
    expect(hintView({ action: 'skill', slot: 2, skill: 'shieldBash' }, 'keyboard').text).toBe('R: Shield Bash');
  });

  it('Bau des Endbosses: Zeile ohne Taste (B-563/AC-01)', () => {
    const v = hintView({ action: 'lair' }, 'pad');
    expect(v.glyph.key).toBeNull();
    expect(v.text).toBe('Bau des Endbosses: Ankunft löst ihn aus');
    expect(playerHint(world([]), player({ actions: [{ action: 'lair' }] }), 'pad')?.text).toBe(v.text);
    // Kampf geht vor: im Bau-Radius bleibt „Schlag“ die Hauptzeile
    expect(playerHint(world([]), player({ actions: [{ action: 'attack' }, { action: 'lair' }] }), 'pad')?.text).not.toBe(v.text);
  });

  it('Text um die Glyph bleibt erhalten (S6.2)', () => {
    expect(hintView({ action: 'build', name: 'Mauer' }, 'pad').around).toEqual(['', ' halten: Mauer bauen']);
    expect(hintView({ action: 'attack' }, 'pad').around).toEqual(['', ': Schlag']);
  });

  it('Wichtigkeit: Wiederbeleben vor Bauen/Zahlen vor Schlag vor Skills', () => {
    const w = (h: Hint) => hintView(h, 'pad').weight;
    expect(w(ALL[0]!)).toBeGreaterThan(w(ALL[1]!));
    expect(w(ALL[1]!)).toBe(w(ALL[2]!));
    expect(w(ALL[2]!)).toBeGreaterThan(w(ALL[3]!));
    expect(w(ALL[3]!)).toBeGreaterThan(w(ALL[4]!));
  });
});

describe('Ausbilden und Tauschen (W11.4, B-410)', () => {
  const train = (profession: string): Hint => ({ action: 'train', profession });
  const trade = (side: 'buy' | 'sell', resource: string): Hint => ({ action: 'trade', side, resource });

  it('deutsche Texte: Taste halten, Beruf bzw. Material aus den Textdateien', () => {
    expect(hintView(train('miner'), 'pad').text).toBe('A halten: Bergmann ausbilden');
    expect(hintView(train('craftsman'), 'touch').text).toBe('🪙 halten: Handwerker ausbilden');
    expect(hintView(trade('buy', 'stone'), 'pad').text).toBe('A halten: Stein kaufen');
    expect(hintView(trade('sell', 'stone'), 'pad').text).toBe('A halten: Stein verkaufen');
  });

  it('englische Texte', () => {
    setLanguage('en');
    expect(hintView(train('miner'), 'pad').text).toBe('Hold A: train Miner');
    expect(hintView(trade('buy', 'copper'), 'pad').text).toBe(`Hold A: buy ${nameOf('res', 'copper', 'copper')}`);
    expect(hintView(trade('sell', 'copper'), 'pad').text).toMatch(/^Hold A: sell \S/);
  });

  it('unbekanntes Material oder unbekannter Beruf: Name aus dem Protokoll, kein Absturz', () => {
    expect(hintView(trade('buy', 'mithril'), 'pad').text).toBe('A halten: mithril kaufen');
    expect(hintView(train('wizard'), 'pad').text).toMatch(/^A halten: .+ ausbilden$/);
  });

  it('Gewicht wie Bezahlen: über Schlag und Skills, unter Wiederbeleben', () => {
    expect(WEIGHT.train).toBe(WEIGHT.pay);
    expect(WEIGHT.trade).toBe(WEIGHT.pay);
    expect(WEIGHT.trade).toBeGreaterThan(WEIGHT.attack);
    expect(WEIGHT.revive).toBeGreaterThan(WEIGHT.train);
    expect(hintView(trade('buy', 'stone'), 'pad').weight).toBe(WEIGHT.trade);
  });
});

describe('playerHint: ein Hinweis je Spieler (B-319)', () => {
  const skill = { action: 'skill', slot: 1, skill: 'taunt' } as const;

  it('höchstens ein Hinweis; die Server-Aktionen stehen am Spieler, bei Gleichstand gewinnt das Gewicht (AC-01)', () => {
    expect(playerHint(world([]), player({ actions: [skill, { action: 'attack' }] }), 'pad')?.text).toBe('X: Schlag');
    expect(playerHint(world([]), player({ actions: [{ action: 'learn' }] }), 'keyboard')?.text).toBe('K: Skill lernen');
    expect(playerHint(world([]), player(), 'pad')).toBeNull();
  });

  it('ignoriert Aktionen, die der Client nicht kennt (W11.3)', () => {
    const fremd = { action: 'grave' } as unknown as PlayerAction;
    expect(playerHint(world([]), player({ actions: [fremd, { action: 'attack' }] }), 'pad')?.text).toBe('X: Schlag');
    expect(playerHint(world([]), player({ actions: [fremd] }), 'pad')).toBeNull();
  });

  it('Ausbilden und Tauschen stehen auch am Preisschild: Sie sagen, was das Bezahlen tut (W11.4)', () => {
    const train = { action: 'train', profession: 'builder' } as const;
    expect(playerHint(world([site({})]), player({ actions: [{ action: 'attack' }, train] }), 'pad')?.text).toBe('A halten: Baumeister ausbilden');
    const sell = { action: 'trade', side: 'sell', resource: 'iron' } as const;
    expect(playerHint(world([]), player({ actions: [{ action: 'attack' }, sell] }), 'keyboard')?.text).toBe('Leertaste halten: Eisen verkaufen');
  });

  it('im Bereich eines Preisschilds kein Hinweis, das Preisschild trägt die Aktion (AC-02)', () => {
    expect(playerHint(world([site({})]), player({ actions: [{ action: 'attack' }] }), 'pad')).toBeNull();
    const shop = site({ kind: 'workshop', state: 'built', bows: 0 });
    expect(playerHint(world([shop]), player({ actions: [{ action: 'attack' }] }), 'pad')).toBeNull();
    expect(playerHint(world([site({ x: 40 })]), player({ actions: [{ action: 'attack' }] }), 'pad')?.text).toBe('X: Schlag');
    const full = site({ kind: 'workshop', state: 'built', bows: 99 });
    expect(playerHint(world([full]), player({ actions: [{ action: 'attack' }] }), 'pad')?.text).toBe('X: Schlag');
  });

  it('gefallener Monarch hat keinen Hinweis', () => {
    expect(playerHint(world([]), player({ respawnIn: 3, actions: [{ action: 'attack' }] }), 'pad')).toBeNull();
  });

  it('Schrift ist Nebeninfo nach bedienung.md § 2: 24 px bei 1–2 Spielern, mindestens 20 px im Viertel (AC-03)', () => {
    expect(FONTS[HINT_FONT].art).toBe('side');
    expect(FONTS[HINT_FONT].px).toBe(minFontPx(2, { w: GAME_WIDTH, kind: 'player' }, 'side'));
    expect(FONTS[HINT_FONT].px).toBeGreaterThanOrEqual(minFontPx(4, { w: GAME_WIDTH / 2, kind: 'player' }, 'side')!);
  });
});
