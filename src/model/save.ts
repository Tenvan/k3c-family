import type { SiteState, Stock, TroopKind } from './types';

/** Spielstand-Format des Clients (aus types.ts ausgelagert, B-521). */

export const SAVE_VERSION = 1;

export interface HubSave {
  depth: number;
  castleHp: number;
  stock: Stock;
  wave: number;
  aggression: number | null;
  sites: { kind: string; x: number; state: SiteState; paidGold: number; buildProgress: number; hp: number; bows: number; bowPaidGold: number }[];
  /** Nur Bauern und Bogenschützen. Landstreicher kommen von allein aus den Camps. */
  troops: { kind: TroopKind; x: number; anchorX: number }[];
  /** Schlüssel `kind@x` der Bäume/Felsen, die schon abgebaut sind bzw. markiert */
  nodesGone: string[];
  nodesMarked: string[];
  pickupsTaken: string[];
}

export interface SaveGame {
  version: number;
  /** Gleiche id = gleiches Spiel. Der Server legt beim Überschreiben eines anderen Spiels eine Sicherung an. */
  campaignId: string;
  savedAt: string;
  seed: string;
  depth: number;
  unlockedDepth: number;
  time: number;
  skillPoints: number;
  players: { gold: number }[];
  hubs: HubSave[];
}

/** Prüft grob, ob ein JSON ein Spielstand ist, den wir laden können. */
export function isSaveGame(data: unknown): data is SaveGame {
  const s = data as SaveGame;
  return (
    !!s &&
    s.version === SAVE_VERSION &&
    typeof s.campaignId === 'string' &&
    typeof s.seed === 'string' &&
    typeof s.depth === 'number' &&
    typeof s.time === 'number' &&
    Array.isArray(s.hubs) &&
    Array.isArray(s.players)
  );
}
