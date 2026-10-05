/**
 * Aktionen-Overlay (S3.3, B-125): Aktion und Gerät → Taste, Text und Gewicht, ohne Phaser, damit es getestet werden kann.
 * Der Client rechnet nichts: Schlag, Skills, Lernen und Respec kommen aus `actions` des Servers; Bauen und Zahlen aus
 * derselben Bedingung wie das Preisschild (`siteView.ts`, unbezahlter Bauplatz bzw. Bogen-Regal in Reichweite).
 */
import { nameOf, t } from '../core/texts';
import { BUILDINGS } from '../model/data';
import type { Player, Site, World } from '../model/types';
import { slotBindings, type Device, type SlotAction } from '../input/slotBindings';
import { skillName } from './skillMenuLogic';
import { PRICE_TAG_RANGE } from './viewRules';

export type Hint =
  | { action: 'revive' }
  | { action: 'build'; name: string }
  | { action: 'pay'; name: string }
  | { action: 'attack' }
  | { action: 'skill'; slot: number; skill: string }
  | { action: 'learn' }
  | { action: 'respec' };

export interface HintView {
  /** Tastensymbol des Geräts (bis S6 Text, danach Glyphe) */
  key: string;
  /** ganze Zeile, z. B. „A halten: Mauer bauen“ */
  text: string;
  /** größer = wichtiger, steht zuerst und groß */
  weight: number;
}

const confirmKey = (device: Device): string => (device === 'pad' ? 'A' : device === 'touch' ? '🪙' : t('hint.space'));
const siteName = (kind: Site['kind']): string => nameOf('site', kind, BUILDINGS[kind].name);
const WEIGHT: Record<Hint['action'], number> = { revive: 5, build: 4, pay: 4, attack: 2, skill: 1, learn: 1, respec: 1 };

const slotKey = (device: Device, action: SlotAction): string => slotBindings(device).find((b) => b.action === action)?.label ?? '';

function keyOf(h: Hint, device: Device): string {
  switch (h.action) {
    case 'attack':
      return slotKey(device, 'attack');
    case 'skill':
      return slotKey(device, `skill${h.slot}` as SlotAction);
    case 'learn':
    case 'respec':
      return slotKey(device, 'skillMenu');
    default:
      return confirmKey(device);
  }
}

function whatOf(h: Hint): string {
  switch (h.action) {
    case 'build':
      return t('hint.build', { name: h.name });
    case 'pay':
      return t('hint.pay', { name: h.name });
    case 'skill':
      return skillName(h.skill);
    default:
      return t(`hint.${h.action}`);
  }
}

/** Aktion → Taste und Text für ein Gerät; Bestätigen-Aktionen (A, Leertaste, Münz-Taste) werden gehalten */
export function hintView(h: Hint, device: Device): HintView {
  const key = keyOf(h, device);
  const hold = h.action === 'revive' || h.action === 'build' || h.action === 'pay';
  return { key, text: t(hold ? 'hint.hold' : 'hint.press', { key, what: whatOf(h) }), weight: WEIGHT[h.action] };
}

/** Bauen bzw. Zahlen am nächsten Ziel in Reichweite des Preisschilds (gleiche Bedingung wie `siteView.ts`) */
function siteHint(world: World, p: Player): Hint | null {
  const rack = BUILDINGS.workshop.bowRack ?? 0;
  const near = world.sites.filter((s) => Math.abs(s.x - p.x) < PRICE_TAG_RANGE).sort((a, b) => Math.abs(a.x - p.x) - Math.abs(b.x - p.x));
  for (const s of near) {
    if (s.state === 'unpaid') return { action: 'build', name: siteName(s.kind) };
    if (s.state === 'built' && s.kind === 'workshop' && s.bows < rack) return { action: 'pay', name: t('site.bow') };
  }
  return null;
}

/** Gültige Aktionen eines Spielers, wichtigste zuerst; ein gefallener Monarch hat keine */
export function playerHints(world: World, p: Player, device: Device): HintView[] {
  if (p.respawnIn > 0) return [];
  const site = siteHint(world, p);
  const hints: Hint[] = site ? [site] : [];
  for (const a of p.actions) hints.push(a);
  return hints.map((h) => hintView(h, device)).sort((a, b) => b.weight - a.weight);
}
