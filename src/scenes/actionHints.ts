/**
 * Aktionen-Overlay (S3.3, B-125): Aktion und Gerät → Taste, Text und Gewicht, ohne Phaser, damit es getestet werden kann.
 * Der Client rechnet nichts: Schlag, Skills, Lernen und Respec kommen aus `actions` des Servers; im Bereich eines
 * Preisschilds (`siteView.ts`, unbezahlter Bauplatz bzw. Bogen-Regal) zeigt das Overlay keinen Hinweis (B-319).
 */
import { t } from '../core/texts';
import { BUILDINGS } from '../model/data';
import type { Player, World } from '../model/types';
import type { Action, Device, SlotAction } from '../input/slotBindings';
import type { FontName } from './fontRules';
import { glyphOf, type Glyph } from './glyphs';
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
  /** Tastensymbol des Geräts als Text */
  key: string;
  /** Tastensymbol als Glyph (S6.2) */
  glyph: Glyph;
  /** Zeile vor und nach der Taste, damit das Overlay die Glyph dazwischen setzt */
  around: [string, string];
  /** ganze Zeile, z. B. „A halten: Mauer bauen“ */
  text: string;
  /** größer = wichtiger, steht zuerst und groß */
  weight: number;
}

const WEIGHT: Record<Hint['action'], number> = { revive: 5, build: 4, pay: 4, attack: 2, skill: 1, learn: 1, respec: 1 };

/** Taste eines Hinweises als Aktion der Belegung (`slotBindings.ts`) */
function actionOf(h: Hint): Action {
  switch (h.action) {
    case 'attack':
      return 'attack';
    case 'skill':
      return `skill${h.slot}` as SlotAction;
    case 'learn':
    case 'respec':
      return 'skillMenu';
    default:
      return 'confirm';
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
  const glyph = glyphOf(actionOf(h), device);
  const line = (key: string) => t(h.action === 'revive' || h.action === 'build' || h.action === 'pay' ? 'hint.hold' : 'hint.press', { key, what: whatOf(h) });
  const [before = '', after = ''] = line('\u0000').split('\u0000'); // Platzhalter ohne Leerzeichen, sonst zerfällt der Text
  return { key: glyph.label, glyph, around: [before, after], text: line(glyph.label), weight: WEIGHT[h.action] };
}

/** Schrift des Hinweises: Nebeninfo (`docs/rules/bedienung.md` § 2, B-319) */
export const HINT_FONT: FontName = 'controlsHint';

/** Steht der Spieler im Bereich eines Preisschilds (gleiche Bedingung wie `siteView.ts`: unbezahlter Bauplatz bzw. Bogen-Regal)? */
function atPriceTag(world: World, p: Player): boolean {
  const rack = BUILDINGS.workshop.bowRack ?? 0;
  return world.sites.some(
    (s) => Math.abs(s.x - p.x) < PRICE_TAG_RANGE && (s.state === 'unpaid' || (s.state === 'built' && s.kind === 'workshop' && s.bows < rack)),
  );
}

/**
 * Ein Hinweis je Spieler (B-319, „ein Element je Weltposition“ je Bildschirmzelle): im Bereich eines Preisschilds keiner,
 * das Preisschild trägt die Aktion. Die Server-Aktionen (`actions`) haben keinen eigenen Ort und stehen am Spieler
 * (Entfernung 0); bei Gleichstand gewinnt das höhere Gewicht. Ein gefallener Monarch hat keinen.
 */
export function playerHint(world: World, p: Player, device: Device): HintView | null {
  if (p.respawnIn > 0 || atPriceTag(world, p)) return null;
  return p.actions.map((h) => hintView(h, device)).sort((a, b) => b.weight - a.weight)[0] ?? null;
}
