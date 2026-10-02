import type { Castle, Enemy, LevelEntity, Player, Site } from '../model/types';
import { GAME_WIDTH } from '../core/constants';
import type { Cell } from './layout';

/**
 * Radar (B-090): eine schmale Leiste über das ganze Level mit Burg, Portalen, Ausgang, Treppen, Monarchen und Gegnern
 * sowie dem Kamera-Ausschnitt des Feldes. Reine Berechnung aus dem Weltzustand, gezeichnet wird in `radarView.ts`.
 */

export type RadarKind = 'castle' | 'portal' | 'exit' | 'stairs' | 'player' | 'enemy';

export interface RadarMarker {
  kind: RadarKind;
  /** Lage entlang der Levelbreite, 0..1 */
  pos: number;
  /** Nur `player`: Monarch-Index (Farbe), ob der Marker zum eigenen Feld gehört, ob der Monarch am Boden liegt */
  index?: number;
  own?: boolean;
  down?: boolean;
}

export interface RadarModel {
  markers: RadarMarker[];
  /** Sichtbarer Ausschnitt des Feldes, 0..1 */
  view: { from: number; to: number };
}

/** Was das Radar vom Weltzustand liest (eine ganze `World` passt hinein). */
export interface RadarWorld {
  widthUnits: number;
  castle: Pick<Castle, 'x'>;
  portals: readonly number[];
  sites: readonly Pick<Site, 'kind' | 'x'>[];
  players: readonly Pick<Player, 'index' | 'x' | 'respawnIn'>[];
  enemies: readonly Pick<Enemy, 'x'>[];
  level: { entities: readonly Pick<LevelEntity, 'kind' | 'x'>[] };
}

/** Sichtbarer Welt-Ausschnitt eines Feldes in Units. */
export interface RadarView {
  fromUnits: number;
  spanUnits: number;
}

const unit = (value: number, width: number): number => (width > 0 ? Math.min(1, Math.max(0, value / width)) : 0);

/**
 * Marker eines Feldes. `self` ist der Monarch, dessen Feld es ist (`null` beim Feld des Mitspielers);
 * gezeichnet wird in der Reihenfolge der Liste, Monarchen zuletzt (oben).
 */
export function radarMarkers(world: RadarWorld, view: RadarView, self: number | null): RadarModel {
  const at = (x: number) => unit(x, world.widthUnits);
  const markers: RadarMarker[] = [{ kind: 'castle', pos: at(world.castle.x) }];
  for (const x of world.portals) markers.push({ kind: 'portal', pos: at(x) });
  for (const e of world.level.entities) if (e.kind === 'exit') markers.push({ kind: 'exit', pos: at(e.x) });
  for (const s of world.sites) if (s.kind === 'stairsUp' || s.kind === 'stairsDown') markers.push({ kind: 'stairs', pos: at(s.x) });
  for (const e of world.enemies) markers.push({ kind: 'enemy', pos: at(e.x) });
  for (const p of world.players) markers.push({ kind: 'player', pos: at(p.x), index: p.index, own: p.index === self, down: p.respawnIn > 0 });
  return { markers, view: { from: at(view.fromUnits), to: at(view.fromUnits + view.spanUnits) } };
}

export interface RadarRect {
  x: number;
  y: number;
  w: number;
  h: number;
}

export const RADAR_HEIGHT = 20;
export const RADAR_MAX_WIDTH = 640;
/** Abstand der Oberkante der Leiste von der Unterkante des Feldes: über den Hinweisen am Bildschirmrand. */
export const RADAR_BOTTOM_OFFSET = 68;
/** Oben bleiben ca. 70 px für den Home-Button der Shell frei. */
export const TOP_STRIP = 70;

/** Abstand zum Außenrand, wenn die Leiste in einem schmalen Feld nach außen rückt. */
const EDGE_MARGIN = 24;

/**
 * Fläche der Leiste im Feld, unten. Im Vollbild-Streifen mittig und halb so breit wie das Feld (höchstens
 * `RADAR_MAX_WIDTH`). In schmalen Feldern (3 und 4 Spieler) sitzt der gemeinsame HUD-Block mittig am Kreuzpunkt:
 * dort ist die Leiste 40 % so breit und rückt an den äußeren Bildschirmrand.
 */
export function radarRect(cell: Pick<Cell, 'x' | 'y' | 'w' | 'h'>): RadarRect {
  const y = cell.y + cell.h - RADAR_BOTTOM_OFFSET;
  if (cell.w >= GAME_WIDTH) {
    const w = Math.min(cell.w / 2, RADAR_MAX_WIDTH);
    return { x: cell.x + (cell.w - w) / 2, y, w, h: RADAR_HEIGHT };
  }
  const w = Math.min(cell.w * 0.4, RADAR_MAX_WIDTH);
  const left = cell.x < GAME_WIDTH / 2;
  return { x: left ? cell.x + EDGE_MARGIN : cell.x + cell.w - EDGE_MARGIN - w, y, w, h: RADAR_HEIGHT };
}
