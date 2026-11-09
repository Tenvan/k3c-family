/**
 * Kachelseiten im Stil der Landingpage (B-768, BD11, Entwurf A „Kachel-Hub“): das Pausenmenü im Spiel und die Startseite der Lobby
 * zeigen ihre Einträge als große, farbige Kacheln (Nachtblau, Gold). Reine Logik ohne Phaser: Lage, Farben, Texte, Auswahl in vier
 * Richtungen, Aktion je Kachel. `hubView.ts` zeichnet. Lesbar aus Sofa-Abstand: Beschriftung ≥ 52 px, Zeile darunter ≥ 32 px (PRODUCT.md).
 */
import { pickTarget, type Box } from '../core/padNav';
import { t } from '../core/texts';
import { stepChoice } from './optionsLogic';

/** Pause: resume, options, keys, removePlayer, leave. Start (Lobby): resume (Fortsetzen), new, join, options, pages. */
export type HubTileId = 'resume' | 'options' | 'keys' | 'removePlayer' | 'leave' | 'new' | 'join' | 'pages';
export type HubDir = 'up' | 'down' | 'left' | 'right';
/** Was ein bestätigter Eintrag auslöst */
export type HubAction = 'close' | 'settings' | 'keys' | 'removePlayer' | 'leave' | 'new' | 'join' | 'pages';

export interface HubTile<Id extends string = HubTileId> {
  id: Id;
  /** Lage in 1920 × 1080: links, oben, Breite, Höhe */
  x: number;
  y: number;
  w: number;
  h: number;
  /** Füllfarbe: Gold = Spielen, Grün = Neues Spiel, Blau = Beitreten und Spieler, Lila = Optionen, Türkis = Seiten und Tasten, Koralle = Verlassen */
  color: number;
  /** Schriftgröße der Beschriftung in px */
  labelPx: number;
}

/** Farben der Kacheln (Landingpage: Gold #ffd166, Nachtblau #0b1026, grüner A-Knopf); dunkle Schrift darauf, ausgegraut bleibt gedämpft */
export const HUB_COLORS = { gold: 0xffd166, green: 0x3fb950, blue: 0x85b7eb, purple: 0xafa9ec, teal: 0x5dcaa5, coral: 0xf09b7b, pink: 0xed93b1, off: 0x2b3558, night: 0x0b1026 } as const;
export const HUB_TEXT = '#0b1026';
export const HUB_TEXT_OFF = '#8a93b2';

const COLOR: Record<HubTileId, number> = {
  resume: HUB_COLORS.gold,
  new: HUB_COLORS.green,
  join: HUB_COLORS.blue,
  removePlayer: HUB_COLORS.blue,
  options: HUB_COLORS.purple,
  keys: HUB_COLORS.teal,
  pages: HUB_COLORS.teal,
  leave: HUB_COLORS.coral,
};

/** Titel und Hinweiszeile; oben bleiben 70 px für den Home-Button frei */
export const HUB_LAYOUT = { titleY: 110, titlePx: 64, worldY: 182, worldPx: 40, hintY: 1020, hintPx: 28, subPx: 32, resumePx: 84, pad: 30, gap: 20 } as const;

const LEFT = 120;
const TOP = 250;
const BIG_W = 700;
const GRID_H = 640;
const COL_W = 470; // 120 + 700 + 20 + 470 + 20 + 470 = 1800: rechts derselbe Rand wie links
const ROW_H = (GRID_H - HUB_LAYOUT.gap) / 2;
const RIGHT_X = LEFT + BIG_W + HUB_LAYOUT.gap;

/** Die erste Kachel liegt links groß, die vier weiteren rechts 2 × 2 (Reihenfolge: oben links, oben rechts, unten links, unten rechts) */
export function hubTiles(ids: readonly [HubTileId, HubTileId, HubTileId, HubTileId, HubTileId]): readonly HubTile[] {
  const small = (id: HubTileId, col: number, row: number): HubTile => ({
    id,
    x: RIGHT_X + col * (COL_W + HUB_LAYOUT.gap),
    y: TOP + row * (ROW_H + HUB_LAYOUT.gap),
    w: COL_W,
    h: ROW_H,
    color: COLOR[id],
    labelPx: 52,
  });
  return [
    { id: ids[0], x: LEFT, y: TOP, w: BIG_W, h: GRID_H, color: COLOR[ids[0]], labelPx: HUB_LAYOUT.resumePx },
    small(ids[1], 0, 0),
    small(ids[2], 1, 0),
    small(ids[3], 0, 1),
    small(ids[4], 1, 1),
  ];
}

/**
 * Gleichmäßiges Raster für Listen von Kacheln (Kategorien und Einträge der Seiten, BD11.4): bis 4 Einträge in 2 Spalten mit der
 * großen Schrift, bis 9 in 3 Spalten mit etwas kleinerer (44 px, immer noch aus Sofa-Abstand lesbar); mehr passt nicht auf eine Seite.
 */
export const GRID_MAX = 9;
export function gridTiles<Id extends string>(items: readonly { id: Id; color: number }[]): readonly HubTile<Id>[] {
  const cols = items.length <= 4 ? 2 : 3;
  const rows = Math.max(1, Math.ceil(items.length / cols));
  const w = (1680 - (cols - 1) * HUB_LAYOUT.gap) / cols;
  const h = (GRID_H - (rows - 1) * HUB_LAYOUT.gap) / rows;
  return items.map((item, i) => ({
    id: item.id,
    x: LEFT + (i % cols) * (w + HUB_LAYOUT.gap),
    y: TOP + Math.floor(i / cols) * (h + HUB_LAYOUT.gap),
    w,
    h,
    color: item.color,
    labelPx: items.length <= 4 ? 52 : 44,
  }));
}

/** Pausenmenü im Spiel */
export const PAUSE_TILES = hubTiles(['resume', 'options', 'keys', 'removePlayer', 'leave']);
/** Startseite der Lobby (Einstieg): Fortsetzen, Neues Spiel, Raum beitreten, Optionen, Seiten */
export const START_TILES = hubTiles(['resume', 'new', 'join', 'options', 'pages']);

const boxOf = (tile: HubTile<string>): Box => ({ left: tile.x, top: tile.y, right: tile.x + tile.w, bottom: tile.y + tile.h });

/**
 * Nächste Kachel in der Richtung; am Rand bleibt die Auswahl. Hoch/Runter springt nur zu Kacheln, die über oder unter der
 * gewählten liegen (gleiche Spalte), Links/Rechts nur zu Kacheln in derselben Zeile: von der großen Kachel geht „hoch“ nirgendwohin.
 */
export function moveHub(tiles: readonly HubTile<string>[], selected: number, dir: HubDir): number {
  const from = boxOf(tiles[selected]!);
  const vertical = dir === 'up' || dir === 'down';
  const overlap = (b: Box): boolean => (vertical ? b.left < from.right && b.right > from.left : b.top < from.bottom && b.bottom > from.top);
  const near = tiles.map((_, i) => i).filter((i) => i === selected || overlap(boxOf(tiles[i]!)));
  const pick = pickTarget(near.map((i) => boxOf(tiles[i]!)), near.indexOf(selected), dir);
  return pick < 0 ? selected : near[pick]!;
}

/** Richtung dieses Frames: Hoch/Runter (`dir` −1/1) vor Links/Rechts (`act`); `null` ohne Richtung */
export function hubDir(dir: number, act: string | null): HubDir | null {
  if (dir !== 0) return dir < 0 ? 'up' : 'down';
  return act === 'left' || act === 'right' ? act : null;
}

/** Kachel unter einem Punkt (Tippen oder Maus), −1 daneben */
export function hubTileAt(tiles: readonly HubTile<string>[], x: number, y: number): number {
  return tiles.findIndex((tile) => x >= tile.x && x <= tile.x + tile.w && y >= tile.y && y <= tile.y + tile.h);
}

/** Wählbar ist, was einen Zustand dazu hat: „Spieler abmelden“ braucht einen Slot des Öffners, „Fortsetzen“ der Startseite einen Spielstand-Namen */
export const hubEnabled = (id: HubTileId, targets: readonly number[]): boolean => id !== 'removePlayer' || targets.length > 0;

const ACTION: Record<HubTileId, HubAction> = {
  resume: 'close',
  options: 'settings',
  keys: 'keys',
  removePlayer: 'removePlayer',
  leave: 'leave',
  new: 'new',
  join: 'join',
  pages: 'pages',
};

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
    case 'new':
      return t('hub.new');
    case 'join':
      return t('hub.join');
    case 'pages':
      return t('hub.pages');
  }
}

/** Zeile unter der Beschriftung; bei „Spieler abmelden“ mit mehreren Zielen der gewählte Spieler, bei „Fortsetzen“ der Startseite der Spielstand */
export function hubSub(id: HubTileId, targets: readonly number[], choice: number, save?: string): string {
  switch (id) {
    case 'resume':
      return save === undefined ? t('hub.resumeSub') : t('hub.resumeSave', { save });
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
    case 'new':
      return t('hub.newSub');
    case 'join':
      return t('hub.joinSub');
    case 'pages':
      return t('hub.pagesSub');
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
