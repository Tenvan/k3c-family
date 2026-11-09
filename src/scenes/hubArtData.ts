/**
 * Daten der Illustrationen (B-768): welche Grafik des Spiels zu welcher Kachel gehört. Ohne Phaser, damit Tests sie prüfen können;
 * `hubArt.ts` stellt sie auf.
 */
import type { CategoryId } from './menuPages';
import type { HubTileId } from './pauseHub';

/** Was eine Illustration zeigt: Papierfigur des Spiels, Objekt (`objectSprite`-ID) oder Gebäude-Textur */
export type ArtPiece =
  | { kind: 'player'; slot: number }
  | { kind: 'troop'; troop: string }
  | { kind: 'object'; id: string }
  | { kind: 'texture'; key: string };

export interface TileArt {
  /** Teile von links nach rechts, mit Anteil der Kachelbreite für die Mitte (0–1) */
  pieces: readonly { piece: ArtPiece; at: number }[];
}

/** Welche Grafik zu welcher Kachel gehört: Fortsetzen = Monarch vor der Burg, Neues Spiel = Portal, Beitreten = Krieger und Bogenschütze … */
export const HUB_ART: Partial<Record<HubTileId | CategoryId, TileArt>> = {
  resume: { pieces: [{ piece: { kind: 'texture', key: 'grafik:hub-3' }, at: 0.66 }, { piece: { kind: 'player', slot: 0 }, at: 0.3 }] },
  new: { pieces: [{ piece: { kind: 'object', id: 'portal' }, at: 0.5 }] },
  join: { pieces: [{ piece: { kind: 'troop', troop: 'warrior' }, at: 0.36 }, { piece: { kind: 'troop', troop: 'archer' }, at: 0.66 }] },
  options: { pieces: [{ piece: { kind: 'object', id: 'pickup:chest' }, at: 0.5 }] },
  keys: { pieces: [{ piece: { kind: 'object', id: 'pickup:skillPoint' }, at: 0.5 }] },
  pages: { pieces: [{ piece: { kind: 'object', id: 'harbor' }, at: 0.5 }] },
  removePlayer: { pieces: [{ piece: { kind: 'troop', troop: 'peasant' }, at: 0.5 }] },
  leave: { pieces: [{ piece: { kind: 'object', id: 'exit' }, at: 0.5 }] },
  info: { pieces: [{ piece: { kind: 'object', id: 'pickup:skillPoint' }, at: 0.5 }] },
  dev: { pieces: [{ piece: { kind: 'object', id: 'pickup:chest' }, at: 0.5 }] },
  perf: { pieces: [{ piece: { kind: 'troop', troop: 'warrior' }, at: 0.5 }] },
};

/** Platz für das Leinen-Schild unter der Illustration, als Anteil der Kachelhöhe (große Kachel mehr Luft, kleine weniger) */
export const ART_BASE = { big: 0.3, small: 0.4 } as const;

