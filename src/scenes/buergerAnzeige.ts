import { UNIT_PX } from '../core/constants';
import { t, type TextKey } from '../core/texts';
import { BUILDINGS } from '../model/data';
import type { Site, Troop, World } from '../model/types';
import { resName } from './wirtschaftAnzeige';

/**
 * Bürger als Text (W6.1, B-126): Truppen-Limit, Berufe, Händler und Heilplatz aus den Server-Feldern (docs/protocol.md ›
 * Wirtschaft). Reine Funktionen ohne Phaser; der Client rechnet kein Limit nach, er zeigt `fighters` und `troopLimit`.
 * Fehlt ein Feld (älterer Server), kommt `null` bzw. []. `compact`: kurze Form für 3 bis 4 Spieler.
 */

/** Bekannte Berufe in fester Reihenfolge (`troops[].profession`). */
const PROFESSIONS = ['miner', 'builder', 'craftsman'] as const;
type Profession = (typeof PROFESSIONS)[number];

/** „Kämpfer/Limit“; Limit erreicht, sobald die Zahl das Limit erreicht. Fehlt eins der Felder → null. */
export function limitText(world: Pick<World, 'fighters' | 'troopLimit'>, compact: boolean): string | null {
  const { fighters: n, troopLimit: max } = world;
  if (n === undefined || max === undefined) return null;
  const full = n >= max;
  const key: TextKey = compact ? (full ? 'limit.fullShort' : 'limit.short') : full ? 'limit.full' : 'limit.text';
  return t(key, { n, max });
}

/** Beruf-ID für Text und Symbol: bekannter Beruf, sonst `unknown`; kein Beruf → null. */
function professionId(profession: string | undefined): Profession | 'unknown' | null {
  if (!profession) return null;
  return (PROFESSIONS as readonly string[]).includes(profession) ? (profession as Profession) : 'unknown';
}

const professionName = (id: Profession | 'unknown', compact: boolean): string => t(`prof.${id}${compact ? 'Short' : ''}` as TextKey);

/** Name und Symbolname eines Berufs; unbekannter Beruf → neutraler Text, kein Beruf → null. */
export function professionLabel(profession: string | undefined, compact: boolean): { icon: string; text: string } | null {
  const id = professionId(profession);
  return id ? { icon: `prof-${id}`, text: professionName(id, compact) } : null;
}

/** Berufe der Stufe gezählt, bekannte in fester Reihenfolge, unbekannte zusammen als neutraler Eintrag am Ende. */
export function professionSummary(troops: Pick<Troop, 'profession'>[] | undefined, compact: boolean): string[] {
  const counts = new Map<Profession | 'unknown', number>();
  for (const tr of troops ?? []) {
    const id = professionId(tr.profession);
    if (id) counts.set(id, (counts.get(id) ?? 0) + 1);
  }
  return [...PROFESSIONS, 'unknown' as const].filter((id) => counts.has(id)).map((id) => t('prof.count', { name: professionName(id, compact), n: counts.get(id)! }));
}

/** Händler mit Angebot (sein Material) und Abreisetag; laufender Kauf in der langen Form. Kein Händler → null. */
export function merchantText(world: Pick<World, 'merchant'>, compact: boolean): string | null {
  const m = world.merchant;
  if (!m) return null;
  const res = resName(m.resource);
  if (compact) return t('merchant.short', { res });
  return m.buyPaid ? t('merchant.paid', { res, day: m.leaves, paid: m.buyPaid }) : t('merchant.text', { res, day: m.leaves });
}

/** Heilradius in Pixeln je gebautem Heilplatz (Mitte `x` in Pixeln), Radius aus `data/buildings.json`. */
export function healerRanges(world: { sites?: Pick<Site, 'kind' | 'x' | 'state'>[] }): { x: number; radiusPx: number }[] {
  const radius = (BUILDINGS.healer as { heal?: { radiusUnits?: number } } | undefined)?.heal?.radiusUnits;
  if (!radius) return [];
  return (world.sites ?? []).filter((s) => s.kind === 'healer' && s.state === 'built').map((s) => ({ x: s.x * UNIT_PX, radiusPx: radius * UNIT_PX }));
}
