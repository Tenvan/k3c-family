import { describe, expect, it } from 'vitest';
import assets from '../../data/assets.json';
import type { GameEvent } from '../model/types';
import { attenuation, cueFor, EDGE_GAIN, eventX, hubRaised, NIGHT_GAIN, nightGain, RIDE_GAIN, RIDE_MIN_SPEED, rideLoops, SFX_URLS, sfxUrls, type Listener, type SfxSlot } from './events';

const p1: Listener = { center: 20, span: 20 }; // sieht 10..30
const p2: Listener = { center: 100, span: 20 }; // sieht 90..110

describe('attenuation (AC-06)', () => {
  it('Quelle im Bereich von Spieler 1: laut', () => expect(attenuation(25, [p1, p2], false)).toBe(1));
  it('Quelle im Bereich von Spieler 2: ebenfalls laut (jeder hört seinen Bereich)', () => expect(attenuation(95, [p1, p2], false)).toBe(1));
  it('außerhalb beider Bereiche: leiser, mit Abstand weiter fallend bis 0', () => {
    const near = attenuation(32, [p1, p2], false);
    expect(near).toBeLessThan(1);
    expect(near).toBeLessThanOrEqual(EDGE_GAIN);
    expect(attenuation(50, [p1, p2], false)).toBeLessThan(near);
    expect(attenuation(60, [p1, p2], false)).toBe(0);
  });
  it('Warnungen sind global laut', () => expect(attenuation(500, [p1, p2], true)).toBe(1));
  it('ohne Listener oder Ort: keine Dämpfung', () => {
    expect(attenuation(500, [], false)).toBe(1);
    expect(attenuation(undefined, [p1], false)).toBe(1);
  });
  it('1 Spieler: im sichtbaren Bereich keine Dämpfung', () => expect(attenuation(10, [p1], false)).toBe(1));
});

/** Ein Beispiel je Ereignis-Typ, den der Katalog `docs/assets/sounds.md` einem Slot zuordnet */
const EVENTS: Record<string, GameEvent> = {
  'sfx:coin': { type: 'coinPickup', player: 0, x: 3 },
  'sfx:hit': { type: 'hit', x: 3, target: 'enemy', id: 1, damage: 1 },
  'sfx:kill': { type: 'kill', kind: 'goblin', x: 3, gold: 1 },
  'sfx:arrow': { type: 'arrow', from: 1, to: 2, x: 3, team: 'player' },
  'sfx:strike': { type: 'strike', from: 1, x: 3 },
  'sfx:build': { type: 'built', kind: 'wall' },
  'sfx:death': { type: 'playerDown', player: 0 },
  'sfx:revive': { type: 'revive', player: 0, x: 3 },
  'sfx:skill': { type: 'skillPoint', player: 0, total: 1 },
  'sfx:portal': { type: 'arrived', depth: 1, name: 'Höhle' },
  'sfx:warn': { type: 'dusk' },
  'sfx:coinGive': { type: 'coinGive', player: 0, x: 1, to: 'site' },
  'sfx:buildProgress': { type: 'buildProgress', site: 1, kind: 'wall', x: 1, percent: 25 },
  'sfx:revived': { type: 'revived', player: 0, x: 1 },
  'sfx:boss': { type: 'bossSpawned', boss: 'dragon', x: 4 },
};

describe('cueFor (B-167/AC-03, AC-04)', () => {
  it.each(Object.entries(EVENTS))('%s: coinPickup, strike, kill, built, dusk und die übrigen Ereignisse des Katalogs haben je einen Slot', (slot, e) => {
    expect(cueFor(e)?.slot).toBe(slot);
  });
  it('Warnungen (dusk, wave, castleFallen) sind global, alles andere positionsgebunden', () => {
    expect(cueFor({ type: 'dusk' })?.global).toBe(true);
    expect(cueFor({ type: 'wave', wave: 1, count: 3 })?.global).toBe(true);
    expect(cueFor({ type: 'castleFallen' })?.global).toBe(true);
    expect(cueFor(EVENTS['sfx:coin']!)?.global).toBe(false);
  });
  it('Lücken des Katalogs und unbekannte Typen sind stumm (kein Fehler)', () => {
    expect(cueFor({ type: 'night', day: 2 })).toBeNull();
  });
  it('Boss-Auftritt ist global (SO6.4)', () => expect(cueFor(EVENTS['sfx:boss']!)?.global).toBe(true));
  it('jeder Slot des Mappings hat eine Datei im Register (SO6.4)', () => {
    for (const slot of Object.keys(EVENTS)) expect(SFX_URLS[slot], slot).toBeDefined();
    expect(SFX_URLS['sfx:hub']).toBeDefined();
  });
  it('hubRaised: nur ein Anstieg zählt, ohne Vorwert kein Ausbau', () => {
    expect(hubRaised(1, 2)).toBe(true);
    expect(hubRaised(2, 2)).toBe(false);
    expect(hubRaised(3, 2)).toBe(false);
    expect(hubRaised(undefined, 2)).toBe(false);
    expect(hubRaised(2, undefined)).toBe(false);
  });
  it('cueFor liest nur: dasselbe Ereignis bleibt unverändert (reine Funktion)', () => {
    const e: GameEvent = { type: 'hit', x: 3, target: 'enemy', id: 1, damage: 4 };
    const copy = structuredClone(e);
    cueFor(e);
    expect(e).toEqual(copy);
  });
  it('jeder Effekt-Slot des Registers mit Datei, der ein Ereignis hat, ist über cueFor erreichbar; sfx:hub kommt aus dem Snapshot (onHubLevel), sfx:ride aus den Spielern (rideLoops, B-574), sfx:uiSelect und sfx:uiConfirm aus den Menü-Szenen (B-575), sfx:skill:* aus SkillCasts (skillSlot, SO8.2)', () => {
    const slots = (assets.slots as SfxSlot[])
      .filter((s) => s.art === 'sound' && s.bus === 'sfx' && s.wahl.length > 0 && !s.id.startsWith('sfx:skill:'))
      .map((s) => s.id);
    const reachable = new Set(Object.values(EVENTS).map((e) => cueFor(e)?.slot));
    expect(slots.filter((id) => !reachable.has(id))).toEqual(['sfx:hub', 'sfx:ride', 'sfx:uiSelect', 'sfx:uiConfirm']);
  });
});

describe('Slot → Datei (sfxUrls)', () => {
  it('erste Präferenz ohne Atlas-Sprite, nur Effekt-Slots mit Datei', () => {
    const slots: SfxSlot[] = [
      { id: 'sfx:a', art: 'sound', bus: 'sfx', wahl: [{ datei: 'audio/atlas.ogg', sprite: 'coin' }, { datei: 'audio/a.ogg' }] },
      { id: 'sfx:b', art: 'sound', bus: 'sfx', wahl: [] },
      { id: 'music:c', art: 'sound', bus: 'music', wahl: [{ datei: 'audio/c.ogg' }] },
      { id: 'x', art: 'grafik', wahl: [{ datei: 'grafik/x.png' }] },
    ];
    expect(sfxUrls(slots)).toEqual({ 'sfx:a': 'audio/a.ogg' });
  });
  it('das Register liefert für jeden Slot des Mappings eine Datei', () => {
    for (const e of Object.values(EVENTS)) expect(SFX_URLS[cueFor(e)!.slot], cueFor(e)!.slot).toMatch(/^audio\/.+\.(ogg|mp3|wav)$/);
  });
});

describe('Nacht leise (B-167/AC-06) und Ort', () => {
  it('nachts leiser als am Tag, sonst unverändert', () => {
    expect(nightGain('night')).toBe(NIGHT_GAIN);
    expect(NIGHT_GAIN).toBeLessThan(1);
    expect(nightGain('day')).toBe(1);
    expect(nightGain('dusk')).toBe(1);
    expect(nightGain(undefined)).toBe(1);
  });
  it('eventX nimmt x des Ereignisses, sonst den Ersatz, sonst nichts', () => {
    expect(eventX({ type: 'hit', x: 7, target: 'enemy', id: 1, damage: 1 }, 99)).toBe(7);
    expect(eventX({ type: 'built', kind: 'wall' }, 99)).toBe(99);
    expect(eventX({ type: 'dusk' })).toBeUndefined();
  });
});

describe('rideLoops (B-574)', () => {
  const pl = (index: number, x: number, vx: number, respawnIn = 0) => ({ index, x, vx, respawnIn });
  it('nur lebende, bewegte Spieler reiten; Schlüssel = Spieler-Index; zwei Reiter = zwei Schleifen', () => {
    const r = rideLoops([pl(0, 20, 5), pl(1, 22, -5), pl(2, 21, 0), pl(3, 21, RIDE_MIN_SPEED / 2), pl(4, 21, 5, 3)], [p1], 'day');
    expect(r.map((l) => l.key)).toEqual([0, 1]);
    expect(r[0]!.volume).toBe(RIDE_GAIN);
  });
  it('Absteigen (Stillstand) beendet die Schleife: leere Liste', () => expect(rideLoops([pl(0, 20, 0)], [p1])).toEqual([]));
  it('Nacht leiser, weit außerhalb der Kamera stumm', () => {
    expect(rideLoops([pl(0, 20, 5)], [p1], 'night')[0]!.volume).toBeCloseTo(RIDE_GAIN * NIGHT_GAIN);
    expect(rideLoops([pl(0, 500, 5)], [p1])).toEqual([]);
  });
  it('der Slot sfx:ride hat eine Datei im Register', () => expect(SFX_URLS['sfx:ride']).toMatch(/horse-gallop/));
});
