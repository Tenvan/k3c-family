import { createRng } from '../../core/rng';
import type { BiomeConfig } from '../biome';
import { cycleAt, globalDayNight } from './cycle';
import { TROOPS } from './data';
import { spawnVagrant } from './units';
import { createWorld, type WorldOptions } from './world';
import type { SiteState, Stock, TroopKind, World } from './types';

/**
 * Spielstand laut GDD: Seed + Tiefe, Hub (Gebäude, Truppen, Vorrat), Gold pro Spieler-Platz, Fortschritt der Stufe.
 * Level-Layout, Gegner, Münzen am Boden und Geschosse werden nicht gespeichert (Layout kommt aus dem Seed).
 * Reine Daten ohne Phaser, damit Speichern → Laden im Test prüfbar ist.
 */

export const SAVE_VERSION = 1;

export interface SaveData {
  version: typeof SAVE_VERSION;
  /** ISO-Zeitpunkt, bei zwei Ständen (Server/localStorage) gewinnt der neuere */
  savedAt: string;
  seed: string;
  depth: number;
  /** Zeit im Tag/Nacht-Zyklus (Sekunden × cycleSpeed), damit ?fast=1 beim Laden keinen anderen Tag ergibt */
  cycleTime: number;
  wave: number;
  aggression: number | null;
  castleHp: number;
  stock: Stock;
  skillPoints: number;
  /** Gold pro Spieler-Platz (Index 0 = Spieler 1) */
  playerGold: number[];
  /** Gleiche Reihenfolge wie `world.sites` (kommt aus hub.json) */
  sites: SavedSite[];
  troops: SavedTroop[];
  /** Noch vorhandene Ressourcen per Position (abgebaute fehlen) */
  nodes: SavedNode[];
  /** Positionen der noch nicht eingesammelten Truhen/Skill-Punkte */
  pickups: number[];
}

export interface SavedSite {
  state: SiteState;
  paidGold: number;
  buildProgress: number;
  hp: number;
  bows: number;
  bowPaidGold: number;
}

export interface SavedTroop {
  kind: TroopKind;
  x: number;
  hp: number;
  anchorX: number;
  paidGold: number;
}

export interface SavedNode {
  x: number;
  paidGold: number;
  marked: boolean;
  progress: number;
}

export function toSave(w: World, savedAt = new Date().toISOString()): SaveData {
  const stock = { ...w.stock };
  // Unterwegs getragenes Material gleich dem Vorrat gutschreiben, Aufträge werden beim Laden neu vergeben.
  for (const t of w.troops) if (t.job?.type === 'carry') stock[t.job.resource] += t.job.amount;
  const slots = Math.max(w.players.length, w.savedGold.length);
  return {
    version: SAVE_VERSION,
    savedAt,
    seed: w.seed,
    depth: w.biome.depth,
    cycleTime: w.time * w.cycleSpeed,
    wave: w.wave,
    aggression: w.aggression,
    castleHp: w.castle.hp,
    stock,
    skillPoints: w.skillPoints,
    playerGold: Array.from({ length: slots }, (_, i) => w.players[i]?.gold ?? w.savedGold[i]),
    sites: w.sites.map((s) => ({
      state: s.state,
      paidGold: s.paidGold,
      buildProgress: s.buildProgress,
      hp: s.hp,
      bows: s.bows,
      bowPaidGold: s.bowPaidGold,
    })),
    troops: w.troops.map((t) => ({ kind: t.kind, x: t.x, hp: t.hp, anchorX: t.anchorX, paidGold: t.paidGold })),
    nodes: w.nodes.map((n) => ({ x: n.x, paidGold: n.paidGold, marked: n.marked, progress: n.progress })),
    pickups: w.pickups.map((p) => p.x),
  };
}

/** Baut die Welt aus dem Seed neu und legt den Spielstand darüber. `biome` muss zu `data.depth` passen. */
export function loadWorld(biome: BiomeConfig, data: SaveData, options: WorldOptions = {}): World {
  if (!isSaveData(data)) throw new Error('Ungültiger Spielstand');
  const w = createWorld(biome, data.seed, options);
  w.rng = createRng(`${biome.id}:${data.seed}:sim:${data.cycleTime}`);
  w.time = data.cycleTime / w.cycleSpeed;
  w.cycle = cycleAt(globalDayNight(), data.cycleTime);
  w.wave = data.wave;
  if (w.aggression !== null && data.aggression !== null) w.aggression = data.aggression;
  w.castle.hp = Math.min(w.castle.maxHp, data.castleHp);
  w.stock = { ...data.stock };
  w.skillPoints = data.skillPoints;
  w.savedGold = [...data.playerGold];

  w.sites.forEach((s, i) => {
    const saved = data.sites[i];
    if (saved) Object.assign(s, saved, { workerId: null });
  });

  w.troops = [];
  for (const t of data.troops) {
    const troop = spawnVagrant(w, t.anchorX, t.x);
    Object.assign(troop, { kind: t.kind, hp: t.hp, maxHp: TROOPS[t.kind].hp, paidGold: t.paidGold });
  }

  const nodesByX = new Map(data.nodes.map((n) => [n.x, n]));
  w.nodes = w.nodes.filter((n) => nodesByX.has(n.x));
  for (const n of w.nodes) Object.assign(n, nodesByX.get(n.x));

  const pickups = new Set(data.pickups);
  w.pickups = w.pickups.filter((p) => pickups.has(p.x));
  return w;
}

/** Grobe Prüfung, damit ein kaputter oder fremder Stand das Spiel nicht zerlegt. */
export function isSaveData(data: unknown): data is SaveData {
  const d = data as Partial<SaveData> | null;
  return (
    !!d &&
    typeof d === 'object' &&
    d.version === SAVE_VERSION &&
    typeof d.seed === 'string' &&
    typeof d.depth === 'number' &&
    typeof d.cycleTime === 'number' &&
    Array.isArray(d.playerGold) &&
    Array.isArray(d.sites) &&
    Array.isArray(d.troops) &&
    Array.isArray(d.nodes) &&
    Array.isArray(d.pickups) &&
    !!d.stock
  );
}
