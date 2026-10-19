import monarchJson from '@data/monarch.json';
import spritesJson from '@data/sprites.json';
import { PLAYER_COLORS } from '@game/core/constants';
import type { AssetSlot } from '../api';
import { ANIMS, MOUNTS, SHEETS, animInfo, gap, type Anim, type FigureSpec, type Gap } from '../playback/figure';
import type { FigureItem } from '../playback/figureView';
import type { Action } from './actions';

// Reine Logik der Monarch-Seite (B-463, WZ10.2): Vorschau-Liste, Reittiere, Spielerfarben, Grafik-Auswahl. Werte stammen
// aus `data/monarch.json` und `data/sprites.json`; die Seite zeichnet nur (abspiel/).

const DATA = monarchJson as unknown as { base: { speed: number }; sprintMultiplier: number; mount: { sprite: string; speedFactor: number } };
const SPRITES = spritesJson as unknown as { players: FigureSpec[] };

/** Standard-Reittier des Spiels (`data/monarch.json` › `mount.sprite`) */
export const DEFAULT_MOUNT = DATA.mount.sprite;

/** Lauftempo des Monarchen auf dem Reittier ohne Sprint (Bezug für die Bildrate in `mountPose`) */
export const REF_SPEED = DATA.base.speed * DATA.mount.speedFactor;

/** Alle Reittiere aus `data/sprites.json` › `mounts` */
export const MOUNT_KEYS: readonly string[] = Object.keys(MOUNTS);

export type MoveId = 'stand' | 'walk' | 'sprint';

export interface Move {
  id: MoveId;
  label: string;
  /** Tempo in Units je Sekunde (0 = steht) */
  speed: number;
}

/** Bewegungen des Monarchen: Der Monarch ist immer beritten (Glossar › Monarch), deshalb gibt es kein Laufen und kein
 *  Sprinten zu Fuß und dafür auch keine Lücke. Laufen ist `base.speed`, Sprinten `base.speed × sprintMultiplier`. */
export const MOVES: readonly Move[] = [
  { id: 'stand', label: 'Stehen', speed: 0 },
  { id: 'walk', label: 'Laufen (beritten)', speed: REF_SPEED },
  { id: 'sprint', label: 'Sprinten (beritten)', speed: REF_SPEED * DATA.sprintMultiplier },
];

export const moveOf = (id: MoveId): Move => MOVES.find((m) => m.id === id) ?? MOVES[0]!;

export interface PlayerColor {
  index: number;
  /** Spielerfarbe: Sheet und tint aus `data/sprites.json` › `players` */
  spec: FigureSpec;
  /** Kennfarbe des Spielers (`PLAYER_COLORS`) als #rrggbb */
  badge: string;
}

/** Die vier Spielerfarben (Index 0–3) mit ihrer Kennfarbe */
export const PLAYERS: readonly PlayerColor[] = SPRITES.players.map((spec, index) => ({
  index, spec, badge: `#${PLAYER_COLORS[index % PLAYER_COLORS.length]!.toString(16).padStart(6, '0')}`,
}));

/** Animationen des Monarchen: die, die das Sheet des Spielers kennt (idle, run, attack) */
export const MONARCH_ANIMS: readonly Anim[] = ANIMS.filter((a) => PLAYERS.some((p) => animInfo(p.spec.sheet, a)));

/** Abstand der Monarchen nebeneinander in Units */
export const SPACING = 8;

export interface Choice {
  /** Ersatz-Sheet je Spieler-Index (Grafik-Auswahl `monarch:n`); fehlt ein Eintrag, gilt das Sheet aus `players` */
  riders?: readonly (string | undefined)[];
  /** Ersatz-Sheet des Reittiers (Grafik-Auswahl `mount:<key>`) */
  mountSheet?: string;
}

/** Vorschau-Figuren: die vier Spielerfarben nebeneinander im selben Takt, beritten auf dem Reittier `mountKey`. */
export function monarchItems(mountKey: string, move: Move, anim: Anim, choice: Choice = {}, action?: Action): FigureItem[] {
  return PLAYERS.map((p) => ({
    id: `monarch-${p.index}`,
    x: (p.index - (PLAYERS.length - 1) / 2) * SPACING,
    spec: { ...p.spec, sheet: choice.riders?.[p.index] ?? p.spec.sheet },
    anim: action?.anim ?? anim,
    mount: { key: mountKey, refSpeed: REF_SPEED, speed: action ? 0 : move.speed, ...(choice.mountSheet ? { sheet: choice.mountSheet } : {}) },
    ...(action?.effect ? { effect: action.effect } : {}),
    ...(action?.alpha !== undefined ? { alpha: action.alpha } : {}),
    ...(action?.fade ? { fade: action.fade } : {}),
    ...(action?.prop ? { prop: action.prop } : {}),
  }));
}

// ---- Grafik-Auswahl ------------------------------------------------------------------------------------------------

export interface SlotOption {
  /** Position in der Präferenz-Reihenfolge, 0 = Präferenz 1 (voreingestellt) */
  index: number;
  sheet: string;
  datei: string;
  /** Lücke, wenn das Sheet keinen Eintrag in `data/sprites.json` hat */
  gap: Gap | null;
}

/** Zuordnungen eines Slots als wählbare Vorschau; Slots ohne Sheet in der Zuordnung ergeben keine Option. */
export function slotOptions(slots: readonly AssetSlot[], id: string): SlotOption[] {
  const slot = slots.find((s) => s.id === id);
  return (slot?.wahl ?? []).flatMap((w, index) => (w.sheet ? [{
    index, sheet: w.sheet, datei: w.datei, gap: SHEETS[w.sheet] ? null : gap('sheet', `Lücke: Sheet ${w.sheet} fehlt in data/sprites.json`),
  }] : []));
}

/** Slot-ID des Monarchen für den Spieler-Index (`monarch:1` bis `monarch:4`) und des Reittiers (`mount:<key>`) */
export const monarchSlotId = (index: number): string => `monarch:${index + 1}`;
export const mountSlotId = (key: string): string => `mount:${key}`;
