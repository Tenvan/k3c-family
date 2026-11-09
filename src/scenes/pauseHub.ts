/**
 * Pausenmenü als Kachelseite (B-768, BD11.1, Entwurf A „Kachel-Hub“): die Einträge liegen als große, farbige Kacheln im Stil
 * der Landingpage (Nachtblau, Gold). Reine Logik ohne Phaser: Lage, Farben, Texte, Auswahl in vier Richtungen, Aktion je Kachel.
 * `OptionsScene` zeichnet nur. Lesbar aus Sofa-Abstand: Beschriftung ≥ 52 px, Zeile darunter ≥ 32 px (PRODUCT.md).
 */
import { pickTarget, type Box } from '../core/padNav';
import { t } from '../core/texts';
import { stepChoice } from './optionsLogic';

export type HubTileId = 'resume' | 'options' | 'keys' | 'removePlayer' | 'leave';
export type HubDir = 'up' | 'down' | 'left' | 'right';
/** Was ein bestätigter Eintrag auslöst */
export type HubAction = 'close' | 'settings' | 'keys' | 'removePlayer' | 'leave';

export interface HubTile {
  id: HubTileId;
  /** Lage in 1920 × 1080: links, oben, Breite, Höhe */
  x: number;
  y: number;
  w: number;
  h: number;
  /** Füllfarbe: Gold = Spielen, Lila = Optionen, Türkis = Seiten und Tasten, Blau = Spieler, Koralle = Verlassen */
  color: number;
  /** Schriftgröße der Beschriftung in px */
  labelPx: number;
}

/** Farben der Kacheln (Landingpage: Gold #ffd166, Nachtblau #0b1026); dunkle Schrift darauf, ausgegraut bleibt gedämpft */
export const HUB_COLORS = { gold: 0xffd166, purple: 0xafa9ec, teal: 0x5dcaa5, blue: 0x85b7eb, coral: 0xf09b7b, off: 0x2b3558, night: 0x0b1026 } as const;
export const HUB_TEXT = '#0b1026';
export const HUB_TEXT_OFF = '#8a93b2';

/** Titel und Hinweiszeile; oben bleiben 70 px für den Home-Button frei */
export const HUB_LAYOUT = { titleY: 110, titlePx: 64, worldY: 182, worldPx: 40, hintY: 1020, hintPx: 28, subPx: 32, resumePx: 84, pad: 30, gap: 20 } as const;

const LEFT = 120;
const TOP = 250;
const BIG_W = 700;
const GRID_H = 640;
const COL_W = 470; // 120 + 700 + 20 + 470 + 20 + 470 = 1800: rechts derselbe Rand wie links
const ROW_H = (GRID_H - HUB_LAYOUT.gap) / 2;
const RIGHT_X = LEFT + BIG_W + HUB_LAYOUT.gap;

/** Fortsetzen links groß, rechts 2 × 2 (Reihenfolge = Auswahl-Startindex 0 ist Fortsetzen) */
export const HUB_TILES: readonly HubTile[] = [
  { id: 'resume', x: LEFT, y: TOP, w: BIG_W, h: GRID_H, color: HUB_COLORS.gold, labelPx: HUB_LAYOUT.resumePx },
  { id: 'options', x: RIGHT_X, y: TOP, w: COL_W, h: ROW_H, color: HUB_COLORS.purple, labelPx: 52 },
  { id: 'keys', x: RIGHT_X + COL_W + HUB_LAYOUT.gap, y: TOP, w: COL_W, h: ROW_H, color: HUB_COLORS.teal, labelPx: 52 },
  { id: 'removePlayer', x: RIGHT_X, y: TOP + ROW_H + HUB_LAYOUT.gap, w: COL_W, h: ROW_H, color: HUB_COLORS.blue, labelPx: 52 },
  { id: 'leave', x: RIGHT_X + COL_W + HUB_LAYOUT.gap, y: TOP + ROW_H + HUB_LAYOUT.gap, w: COL_W, h: ROW_H, color: HUB_COLORS.coral, labelPx: 52 },
];

const boxOf = (tile: HubTile): Box => ({ left: tile.x, top: tile.y, right: tile.x + tile.w, bottom: tile.y + tile.h });

/**
 * Nächste Kachel in der Richtung; am Rand bleibt die Auswahl. Hoch/Runter springt nur zu Kacheln, die über oder unter der
 * gewählten liegen (gleiche Spalte), Links/Rechts nur zu Kacheln in derselben Zeile: von der großen Kachel geht „hoch“ nirgendwohin.
 */
export function moveHub(selected: number, dir: HubDir): number {
  const from = boxOf(HUB_TILES[selected]!);
  const vertical = dir === 'up' || dir === 'down';
  const overlap = (b: Box): boolean => (vertical ? b.left < from.right && b.right > from.left : b.top < from.bottom && b.bottom > from.top);
  const near = HUB_TILES.map((_, i) => i).filter((i) => i === selected || overlap(boxOf(HUB_TILES[i]!)));
  const pick = pickTarget(near.map((i) => boxOf(HUB_TILES[i]!)), near.indexOf(selected), dir);
  return pick < 0 ? selected : near[pick]!;
}

/** Richtung dieses Frames: Hoch/Runter (`dir` −1/1) vor Links/Rechts (`act`); `null` ohne Richtung */
export function hubDir(dir: number, act: string | null): HubDir | null {
  if (dir !== 0) return dir < 0 ? 'up' : 'down';
  return act === 'left' || act === 'right' ? act : null;
}

/** Kachel unter einem Punkt (Tippen oder Maus), −1 daneben */
export function hubTileAt(x: number, y: number): number {
  return HUB_TILES.findIndex((tile) => x >= tile.x && x <= tile.x + tile.w && y >= tile.y && y <= tile.y + tile.h);
}

/** „Spieler abmelden“ ist nur mit einem Slot des Öffners wählbar, alle anderen Kacheln immer */
export const hubEnabled = (id: HubTileId, targets: readonly number[]): boolean => id !== 'removePlayer' || targets.length > 0;

const ACTION: Record<HubTileId, HubAction> = { resume: 'close', options: 'settings', keys: 'keys', removePlayer: 'removePlayer', leave: 'leave' };

/** Aktion einer bestätigten Kachel; `null` bei einer ausgegrauten */
export const hubAction = (id: HubTileId, targets: readonly number[]): HubAction | null => (hubEnabled(id, targets) ? ACTION[id] : null);

/** Beschriftung der Kachel */
export function hubLabel(id: HubTileId): string {
  switch (id) {
    case 'resume':
      return t('hub.resume');
    case 'options':
      return t('opt.title');
    case 'keys':
      return t('opt.keys');
    case 'removePlayer':
      return t('opt.removePlayer');
    case 'leave':
      return t('opt.leave');
  }
}

/** Zeile unter der Beschriftung; bei „Spieler abmelden“ mit mehreren Zielen der gewählte Spieler */
export function hubSub(id: HubTileId, targets: readonly number[], choice: number): string {
  switch (id) {
    case 'resume':
      return t('hub.resumeSub');
    case 'options':
      return t('hub.optionsSub');
    case 'keys':
      return t('hub.keysSub');
    case 'removePlayer': {
      const slot = targets[choice];
      return targets.length > 1 && slot !== undefined ? t('hub.removeSubOf', { n: slot + 1 }) : t('hub.removeSub');
    }
    case 'leave':
      return t('hub.leaveSub');
  }
}

/**
 * Links/Rechts auf „Spieler abmelden“ mit mehreren Zielen (zwei Tastatur-Spieler) wählt das Ziel, solange es sich ändern lässt;
 * sonst bewegt es die Auswahl wie überall. Ergibt das neue Ziel oder `null` (dann zählt die Richtung als Bewegung).
 */
export function hubChoice(id: HubTileId, dir: HubDir, choice: number, count: number): number | null {
  if (id !== 'removePlayer' || count < 2 || (dir !== 'left' && dir !== 'right')) return null;
  const next = stepChoice(choice, dir, count);
  return next === choice ? null : next;
}
