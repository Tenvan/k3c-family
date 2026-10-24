import { describe, expect, it } from 'vitest';
import { SFX_URLS } from './events';
import { SKILL_FALLBACK, skillSlot } from './skills';

const lines = { 'sfx:skill': 'a.ogg', 'sfx:skill:tank': 't.ogg', 'sfx:skill:healer': 'h.ogg' };

describe('Skill → Slot (SO8.2, B-576)', () => {
  it('aktiver Skill spielt den Slot seiner Linie, ohne Linien-Slot den gemeinsamen (AC-02)', () => {
    expect(skillSlot('taunt', lines)).toBe('sfx:skill:tank');
    expect(skillSlot('groupHeal', lines)).toBe('sfx:skill:healer');
    expect(skillSlot('fireball', lines)).toBe(SKILL_FALLBACK);
  });

  it('passiver Skill ohne eigenen Slot bleibt still, mit Slot spielt er ihn; der Skill-Slot hat Vorrang vor der Linie (AC-03)', () => {
    expect(skillSlot('regeneration', lines)).toBeNull();
    expect(skillSlot('regeneration', { ...lines, 'sfx:skill:regeneration': 'r.ogg' })).toBe('sfx:skill:regeneration');
    expect(skillSlot('taunt', { ...lines, 'sfx:skill:taunt': 'x.ogg' })).toBe('sfx:skill:taunt');
  });

  it('unbekannter Skill bleibt still', () => {
    expect(skillSlot('gibtEsNicht', lines)).toBeNull();
  });

  it('der Katalog hat einen Slot je Klasse (SO8.1)', () => {
    expect(['taunt', 'fireball', 'heal', 'backstab'].map((id) => skillSlot(id, SFX_URLS))).toEqual(['sfx:skill:tank', 'sfx:skill:mage', 'sfx:skill:healer', 'sfx:skill:thief']);
  });
});
