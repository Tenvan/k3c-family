import { describe, expect, it } from 'vitest';
import { DUCK } from '@game/audio/ducking';
import type { AssetSlot } from '../api';
import { combatMix, distanceSteps, duckSummary, loudnessQueue, playable } from './probes';
import { eventRows, GLOBAL_SLOTS, HERKUNFT_LABEL, rowFilter, skillRows, skillSlot, soundRows, ROW_SCENES } from './soundRows';

const slot = (id: string, n = 1, bus = 'sfx'): AssetSlot => ({ id, art: 'sound', kategorie: 'sfx', beschreibung: id, bus, wahl: Array.from({ length: n }, (_, i) => ({ datei: `audio/${i}.ogg` })), status: n ? 'zugeordnet' : 'luecke' });

describe('Skill → Slot (vorläufig, ersetzt WZ29.2 durch SO8.2)', () => {
  it('Linien-Slot vor sfx:skill, sonst still', () => {
    const reg = [slot('sfx:skill'), slot('sfx:skill:healer')];
    expect(skillSlot('healer', reg)).toMatchObject({ herkunft: 'linie', slot: { id: 'sfx:skill:healer' } });
    expect(skillSlot('tank', reg)).toMatchObject({ herkunft: 'rueckfall', slot: { id: 'sfx:skill' } });
    expect(skillSlot('tank', [slot('sfx:skill:tank', 0)])).toEqual({ herkunft: 'still' }); // Slot ohne Zuordnung zählt nicht
    expect(skillSlot('mage', [])).toEqual({ herkunft: 'still' });
  });
  it('Zeilen: alle Skills nach Linie und Art (aktiv/passiv) mit Herkunft', () => {
    const rows = skillRows([slot('sfx:skill'), slot('sfx:skill:healer')]);
    const by = (id: string) => rows.find((r) => r.titel === id)!;
    expect(by('groupHeal')).toMatchObject({ gruppe: 'Heiler', linie: 'healer', art: 'aktiv', herkunft: 'linie' });
    expect(by('healingAura')).toMatchObject({ art: 'passiv', herkunft: 'linie' });
    expect(by('fireball')).toMatchObject({ linie: 'mage', herkunft: 'rueckfall' });
    expect(new Set(rows.map((r) => r.linie))).toEqual(new Set(['tank', 'mage', 'healer', 'thief']));
    expect(skillRows([]).every((r) => r.herkunft === 'still' && !r.choice)).toBe(true);
  });
});

describe('Ereignis-Szenen', () => {
  const reg = [slot('sfx:hit'), slot('sfx:kill', 0), slot('sfx:boss'), slot('sfx:warn'), slot('sfx:hub'), slot('sfx:ride'), slot('sfx:uiSelect')];
  it('Slots kommen aus dem Audio-Kern, Lücke und still sind unterscheidbar', () => {
    const g = eventRows('gegner', reg);
    expect(g.find((r) => r.titel === 'Treffer')).toMatchObject({ herkunft: 'slot', slot: { id: 'sfx:hit' } });
    expect(g.find((r) => r.titel === 'Tod')).toMatchObject({ herkunft: 'luecke', choice: undefined });
    expect(g.find((r) => r.titel === 'Pfeil')).toMatchObject({ herkunft: 'still' }); // sfx:arrow fehlt im Register
    expect(g.some((r) => r.gruppe === 'Boss' && r.titel.endsWith('Auftritt'))).toBe(true);
    expect(eventRows('bauen', reg).find((r) => r.titel === 'Hub-Ausbau')).toMatchObject({ herkunft: 'slot', global: true });
    expect(eventRows('reiten', reg)[0]!.slot?.id).toBe('sfx:ride');
    expect(eventRows('ui', reg).map((r) => r.herkunft)).toEqual(['slot', 'still']);
  });
  it('Warnungen sind global und senken die Musik', () => {
    const w = eventRows('warnungen', reg);
    expect(w.map((r) => r.titel)).toEqual(['Abenddämmerung', 'Welle', 'Burg gefallen', 'Boss erscheint']);
    expect(w.every((r) => r.global)).toBe(true);
    expect([...GLOBAL_SLOTS].sort()).toEqual(['sfx:boss', 'sfx:hub', 'sfx:warn']);
  });
  it('jede Szene liefert Zeilen und der Filter kennt Herkunft und Status', () => {
    for (const s of ROW_SCENES) expect(soundRows(s, reg).length).toBeGreaterThan(0);
    const rows = soundRows('gegner', reg);
    const f = rowFilter(rows, []);
    expect(f.dimensions.map((d) => d.key)).toContain('herkunft');
    const status = f.dimensions.find((d) => d.key === 'status')!;
    expect(rows.map((r) => status.values(r))).toContain('Lücke');
    expect(Object.keys(HERKUNFT_LABEL)).toHaveLength(5);
  });
});

describe('Proben', () => {
  const reg = [slot('sfx:hit'), slot('sfx:arrow'), slot('sfx:kill', 0), slot('music:tag', 1, 'music')];
  it('Mischung, Lautheit und Auswahl überspringen Lücken', () => {
    expect(combatMix(reg).map((p) => p.slot.id)).toEqual(['sfx:hit', 'sfx:arrow']);
    expect(loudnessQueue(reg, ['sfx:arrow', 'sfx:kill', 'sfx:hit']).map((p) => p.slot.id)).toEqual(['sfx:arrow', 'sfx:hit']);
    expect(playable(reg, 'music').map((p) => p.slot.id)).toEqual(['music:tag']);
    expect(playable(reg, 'sfx').map((p) => p.slot.id)).toEqual(['sfx:hit', 'sfx:arrow']);
  });
  it('Abstandsstufen nutzen die Formel des Spiels: nah 1, Rand 1, halbe Breite 0,25, eine Breite stumm', () => {
    expect(distanceSteps().map((s) => s.gain)).toEqual([1, 1, 0.25, 0]);
  });
  it('Ducking-Text nennt die Werte aus DUCK', () => {
    expect(duckSummary()).toContain(`${Math.round(DUCK.level * 100)} %`);
    expect(duckSummary()).toContain(`${DUCK.hold} s`);
  });
});
