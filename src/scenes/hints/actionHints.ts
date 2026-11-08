/**
 * Aktionen-Overlay (S3.3, B-125): Aktion und Gerät → Taste, Text und Gewicht, ohne Phaser, damit es getestet werden kann.
 * Ohne Tastennamen (B-741). Der Client rechnet nichts: Schlag, Skills, Lernen, Respec, Ausbilden und Tauschen kommen aus `actions` des Servers; im
 * Bereich eines Preisschilds (`siteView.ts`, unbezahlter Bauplatz bzw. Bogen-Regal) zeigt das Overlay nur noch Ausbilden
 * und Tauschen, sonst keinen Hinweis (B-319, W11.4).
 */
import { clientLog } from '../../core/clientLog';
import { nameOf, t } from '../../core/texts';
import type { Player, PlayerAction, World } from '../../model/types';
import type { FontName } from '../render/fontRules';
import { offersPrice } from '../render/priceTagRules';
import { skillName } from '../hud/skillMenuLogic';
import { professionLabel } from '../hud/citizenLabels';
import { PRICE_TAG_RANGE } from '../render/viewRules';

export type Hint =
  | { action: 'revive' }
  | { action: 'build'; name: string }
  | { action: 'pay'; name: string }
  | { action: 'attack' }
  | { action: 'skill'; slot: number; skill: string }
  | { action: 'learn' }
  | { action: 'respec' }
  | { action: 'lair' }
  | { action: 'train'; profession: string }
  | { action: 'trade'; side: 'buy' | 'sell'; resource: string };

export interface HintView {
  /** ganze Zeile ohne Tastennamen, z. B. „Mauer bauen“ (B-741: die Belegung steht im Menü › Tastenbelegung) */
  text: string;
  /** größer = wichtiger, steht zuerst und groß */
  weight: number;
}

/** Ausbilden und Tauschen wiegen wie Bezahlen: Sie sagen, was die gehaltene Münz-Taste am Ort tut (W11.4) */
export const WEIGHT: Record<Hint['action'], number> = { revive: 5, build: 4, pay: 4, train: 4, trade: 4, attack: 2, skill: 1, learn: 1, respec: 1, lair: 1 };

/** Aktionen, die der Server aus dem Zahlziel am Ort ableitet (`sim.TrainAt`, `sim.TradeAt`): stehen auch am Preisschild */
const AT_PAY_TARGET = new Set<Hint['action']>(['train', 'trade']);

function whatOf(h: Hint): string {
  switch (h.action) {
    case 'build':
      return t('hint.build', { name: h.name });
    case 'pay':
      return t('hint.pay', { name: h.name });
    case 'skill':
      return skillName(h.skill);
    case 'train':
      return t('hint.train', { name: professionLabel(h.profession, false)?.text ?? h.profession });
    case 'trade':
      return t(h.side === 'sell' ? 'hint.sell' : 'hint.buy', { name: nameOf('res', h.resource, h.resource) });
    default:
      return t(`hint.${h.action}`);
  }
}

/** Aktion → Text ohne Taste und Gewicht (B-741) */
export function hintView(h: Hint): HintView {
  return { text: whatOf(h), weight: WEIGHT[h.action] };
}

/** Schrift des Hinweises: Nebeninfo (`docs/rules/bedienung.md` § 2, B-319) */
export const HINT_FONT: FontName = 'controlsHint';

/** Steht der Spieler im Bereich eines Preisschilds (gleiche Bedingung wie `siteView.ts`: unbezahlter Bauplatz bzw. Bogen-Regal)? */
function atPriceTag(world: World, p: Player): boolean {
  return world.sites.some((s) => Math.abs(s.x - p.x) < PRICE_TAG_RANGE && offersPrice(s));
}

/**
 * Ein Hinweis je Spieler (B-319, „ein Element je Weltposition“ je Bildschirmzelle): im Bereich eines Preisschilds nur
 * Ausbilden und Tauschen, sonst keiner, das Preisschild trägt die Aktion. Ausbilden und Tauschen nennt der Server nur,
 * wenn das Zahlziel am Ort ein Angebot bzw. der Händler ist; dann sagt der Hinweis, was das Bezahlen tut, auch wenn ein
 * Preisschild in Reichweite steht (W11.4). Die Server-Aktionen haben keinen eigenen Ort und stehen am Spieler
 * (Entfernung 0); bei Gleichstand gewinnt das höhere Gewicht. Ein gefallener Monarch hat keinen.
 */
export function playerHint(world: World, p: Player): HintView | null {
  if (p.respawnIn > 0) return null;
  const tag = atPriceTag(world, p);
  return p.actions.filter(known).filter((h) => !tag || AT_PAY_TARGET.has(h.action)).map((h) => hintView(h)).sort((a, b) => b.weight - a.weight)[0] ?? null;
}

const warned = new Set<string>();

/** Aktionen, die der Client (noch) nicht kennt, ignoriert er mit einer Warnung je Art (Protokoll › Aktionsliste, W11.3) */
function known(h: PlayerAction): h is PlayerAction & Hint {
  if (h.action in WEIGHT) return true;
  if (!warned.has(h.action)) {
    warned.add(h.action);
    clientLog('warn', `🤒 unbekannte Aktion ${h.action} ignoriert`);
  }
  return false;
}
