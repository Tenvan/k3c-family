/**
 * Skill → Ton (SO8.2, B-576): reine Abbildung ohne Spielregel. Vorrang: Slot des Skills (`sfx:skill:<skillId>`), bei aktiven Skills
 * dann der Slot der Linie (`sfx:skill:<linie>`) und zuletzt der gemeinsame `sfx:skill`; ein passiver Skill ohne eigenen Slot bleibt still.
 */
import { MONARCH } from '../model/data';
import { SFX_URLS } from './events';

const SKILLS = MONARCH.skills as Record<string, { kind: string; line: string }>;

/** Rückfall für aktive Skills (Linie ohne Slot, Datei fehlt); derselbe Slot klingt beim Fund eines Skill-Punkts. */
export const SKILL_FALLBACK = 'sfx:skill';

/** Rein: Slot für das Wirken (aktiv) oder Lernen (passiv) eines Skills; `slots` = Effekt-Slots mit Datei, `null` = stumm. */
export function skillSlot(skillId: string, slots: Readonly<Record<string, string>> = SFX_URLS): string | null {
  const skill = SKILLS[skillId];
  const own = `sfx:skill:${skillId}`;
  if (slots[own]) return own;
  if (skill?.kind !== 'active') return null;
  const line = `sfx:skill:${skill.line}`;
  return slots[line] ? line : SKILL_FALLBACK;
}
