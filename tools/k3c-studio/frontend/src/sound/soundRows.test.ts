import { describe, expect, it } from 'vitest';
import assets from '@data/assets.json';
import { DUCK } from '@game/audio/ducking';
import type { AssetSlot } from '../api';
import { combatMix, distanceSteps, duckSummary, loudnessQueue, playable } from './probes';
import { eventRows, GLOBAL_SLOTS, HERKUNFT_LABEL, rowFilter, skillRows, skillTone, soundRows, ROW_SCENES } from './soundRows';

const slot = (id: string, n = 1, bus = 'sfx'): AssetSlot => ({ id, art: 'sound', kategorie: 'sfx', beschreibung: id, bus, wahl: Array.from({ length: n }, (_, i) => ({ datei: `audio/${i}.ogg` })), status: n ? 'zugeordnet' : 'luecke' });

describe('Skill → Slot mit der Abbildung des Spiels (SO8.2, WZ29.2)', () => {
  it('Skill-Slot vor Linien-Slot vor sfx:skill; passiv ohne eigenen Slot still', () => {
    const reg = [slot('sfx:skill'), slot('sfx:skill:healer'), slot('sfx:skill:regeneration')];
    expect(skillTone('groupHeal', reg)).toMatchObject({ herkunft: 'linie', slot: { id: 'sfx:skill:healer' } });
    expect(skillTone('taunt', reg)).toMatchObject({ herkunft: 'rueckfall', slot: { id: 'sfx:skill' } });
    expect(skillTone('regeneration', reg)).toMatchObject({ herkunft: 'skill', slot: { id: 'sfx:skill:regeneration' } });
    expect(skillTone('healingAura', reg)).toEqual({ herkunft: 'still' });
    expect(skillTone('taunt', [slot('sfx:skill:tank', 0)])).toEqual({ herkunft: 'still' }); // Slot ohne Zuordnung zählt nicht, Rückfall fehlt
    expect(skillTone('fireball', [])).toEqual({ herkunft: 'still' });
  });
  it('Zeilen: alle Skills nach Linie und Art (aktiv/passiv) mit Herkunft; jeder Skill hat Ton oder ist still', () => {
    const rows = skillRows([slot('sfx:skill'), slot('sfx:skill:healer')]);
    const by = (id: string) => rows.find((r) => r.titel === id)!;
    expect(by('groupHeal')).toMatchObject({ gruppe: 'Heiler', linie: 'healer', art: 'aktiv', herkunft: 'linie' });
    expect(by('healingAura')).toMatchObject({ art: 'passiv', herkunft: 'still' });
    expect(by('fireball')).toMatchObject({ linie: 'mage', herkunft: 'rueckfall' });
    expect(new Set(rows.map((r) => r.linie))).toEqual(new Set(['tank', 'mage', 'healer', 'thief']));
    expect(rows.every((r) => (r.herkunft === 'still') === !r.choice)).toBe(true);
    expect(skillRows([]).every((r) => r.herkunft === 'still' && !r.choice)).toBe(true);
  });
  it('echter Katalog (data/assets.json): jeder aktive Skill hat den Ton seiner Klasse, passive sind still (SO8.1)', () => {
    const rows = skillRows(assets.slots as unknown as AssetSlot[]);
    for (const r of rows) expect(r.herkunft, r.titel).toBe(r.art === 'aktiv' ? 'linie' : 'still');
    expect(rows.find((r) => r.titel === 'backstab')?.slot?.id).toBe('sfx:skill:thief');
  });
  it('Filter Linie/Klasse zeigt den Namen der Klasse', () => {
    const rows = skillRows([slot('sfx:skill')]);
    const linie = rowFilter(rows, []).dimensions.find((d) => d.key === 'linie')!;
    expect(linie.label).toBe('Linie/Klasse');
    expect(linie.values(rows.find((r) => r.titel === 'groupHeal')!)).toBe('Heiler');
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
    expect(Object.keys(HERKUNFT_LABEL)).toHaveLength(6);
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
