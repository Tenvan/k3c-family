import spritesJson from '@data/sprites.json';
import { UNIT_PX } from '@game/core/constants';
import { mountPose, type MountContext, type MountPose } from '@game/scenes/mountPose';
import type { MountData, SheetData, SpriteSpec } from '@game/scenes/sprites';

// Abspiel-Bausteine für Studio-Seiten (B-463, WZ10.1; WZ11 nutzt sie für die Gegner-Seite): reine Logik ohne Phaser und
// DOM. Alle Figuren-Eingaben sind Parameter (Sheet, Animation, tint, scale, alpha, y), es steckt keine Monarch-Annahme
// darin. Bildrate, Bildzahl, Ursprung und Maßstab stammen aus `data/sprites.json`, nie aus Zahlen im Studio-Code.

export type Anim = 'idle' | 'run' | 'attack';
export const ANIMS: readonly Anim[] = ['idle', 'run', 'attack'];

/** Was eine Figur zeigt: Sheet und die Parameter des Spiels (`SpriteSpec`) */
export type FigureSpec = SpriteSpec;

const DATA = spritesJson as unknown as {
  sheets: Record<string, SheetData>;
  mounts: Record<string, MountData>;
  riderWaist: number;
  players: SpriteSpec[];
  troops: Record<string, SpriteSpec>;
  enemies: Record<string, SpriteSpec>;
};

export const SHEETS = DATA.sheets;
export const MOUNTS = DATA.mounts;

// ---- Lücken --------------------------------------------------------------------------------------------------------

export type GapKind = 'anim' | 'sheet' | 'mount' | 'image' | 'effect' | 'skill' | 'display';

/** Etwas, das das Spiel nicht darstellt: die Seite zeigt den Ersatz des Spiels und die Marke „Lücke“ mit dem Grund. Sie
 *  baut keinen Ersatz; die Lücken sind die Grundlage für Grafik- und Client-Tickets. */
export interface Gap {
  kind: GapKind;
  reason: string;
}

export const gap = (kind: GapKind, reason: string): Gap => ({ kind, reason });

// ---- Animation auflösen -------------------------------------------------------------------------------------------

export interface AnimInfo {
  frames: number;
  fps: number;
}

/** Bildzahl und Bildrate der Animation aus `data/sprites.json` (null = das Sheet hat sie nicht) */
export function animInfo(sheet: string, anim: Anim): AnimInfo | null {
  return SHEETS[sheet]?.anims[anim] ?? null;
}

/** Wie `playAnim` im Spiel: fehlt die Animation, bleibt es bei `idle`; die Lücke nennt den Grund. */
export function resolveAnim(sheet: string, anim: Anim): { anim: Anim; gap: Gap | null } {
  if (!SHEETS[sheet]) return { anim: 'idle', gap: gap('sheet', `Lücke: Sheet ${sheet} fehlt in data/sprites.json`) };
  if (animInfo(sheet, anim)) return { anim, gap: null };
  return { anim: 'idle', gap: gap('anim', `Lücke: ${anim} fehlt`) };
}

// ---- Maßstab -------------------------------------------------------------------------------------------------------

/** Oberkante der Figur relativ zum Fußpunkt (negativ), rechnet wie `makeSprite`/`spriteTop` im Spiel. */
export function figureTop(spec: FigureSpec): number {
  const d = SHEETS[spec.sheet];
  if (!d) return 0;
  return -(spec.y ?? 0) - d.height * d.scale * (spec.scale ?? 1);
}

export const unitsToPx = (units: number): number => units * UNIT_PX;

/** Marken der Unit-Skala an der Bodenlinie: jede `step`-te Unit von `from` bis `to` mit Pixel-Lage */
export function unitTicks(from: number, to: number, step = 1): { unit: number; px: number }[] {
  const out: { unit: number; px: number }[] = [];
  for (let u = Math.ceil(from / step) * step; u <= to; u += step) out.push({ unit: u, px: unitsToPx(u) });
  return out;
}

// ---- Steuerung -----------------------------------------------------------------------------------------------------

/** Tempo-Stufen der Wiedergabe (Faktor auf die Bildrate des Spiels) */
export const TEMPO_STEPS = [0.5, 1, 2] as const;

/** Wiedergabe: Schleife an/aus, Tempo, Einzelbild (null = läuft) */
export interface Control {
  loop: boolean;
  tempo: number;
  frame: number | null;
}

export const DEFAULT_CONTROL: Control = { loop: true, tempo: 1, frame: null };

/** Tempo setzen; nur die Stufen aus TEMPO_STEPS gelten, anderes lässt die Steuerung unverändert. */
export function setTempo(c: Control, tempo: number): Control {
  return (TEMPO_STEPS as readonly number[]).includes(tempo) ? { ...c, tempo } : c;
}

export const toggleLoop = (c: Control): Control => ({ ...c, loop: !c.loop });

/** Einzelbild vor oder zurück (hält die Wiedergabe an, läuft über die Bildzahl hinaus im Kreis). `current` ist das
 *  Bild, bei dem die Wiedergabe steht. */
export function stepFrame(c: Control, frames: number, dir: 1 | -1, current = 0): Control {
  if (frames <= 0) return c;
  const from = c.frame ?? current;
  return { ...c, frame: (((from + dir) % frames) + frames) % frames };
}

export const resume = (c: Control): Control => ({ ...c, frame: null });

/** Bildrate der Wiedergabe: Bildrate aus den Daten mal Tempo */
export const effectiveFps = (info: AnimInfo, c: Control): number => info.fps * c.tempo;

// ---- Bilder laden --------------------------------------------------------------------------------------------------

/** Sheets, die der Figuren-Atlas enthält: die mit einer Rolle (Spieler, Truppen, Gegner), wie `USED` im Spiel. */
const IN_ATLAS = new Set([...DATA.players, ...Object.values(DATA.troops), ...Object.values(DATA.enemies)].map((s) => s.sheet));

export const inAtlas = (sheet: string): boolean => IN_ATLAS.has(sheet);

export interface SheetFile {
  key: string;
  anim: Anim;
  url: string;
  frameWidth: number;
  frameHeight: number;
}

/** Einzel-PNGs eines Sheets außerhalb des Atlas (Reittiere außer dem Standard, Slot-Alternativen) nach dem Muster von
 *  `preloadSprites`: `sprites/<sheet>/<anim>.png`, Textur-Schlüssel `<sheet>-<anim>`. */
export function sheetFiles(sheet: string): SheetFile[] {
  const d = SHEETS[sheet];
  if (!d) return [];
  return ANIMS.filter((a) => d.anims[a]).map((anim) => ({
    key: `${sheet}-${anim}`, anim, url: `sprites/${sheet}/${anim}.png`, frameWidth: d.frameWidth, frameHeight: d.frameHeight,
  }));
}

// ---- Reittier ------------------------------------------------------------------------------------------------------

/** Kontext von `mountPose` für einen Reiter auf dem Reittier `mountKey`; `refSpeed` ist das Lauftempo ohne Sprint. */
export function riderContext(mountKey: string, rider: FigureSpec, refSpeed: number): MountContext {
  return { key: mountKey, rider, refSpeed, mounts: MOUNTS, sheets: SHEETS, riderWaist: DATA.riderWaist };
}

/** Pose des Reiters wie im Spiel (`mountPose`) bei Tempo `speed` (0 = steht); null mit Lücke, wenn das Reittier fehlt. */
export function mountPoseOf(mountKey: string, rider: FigureSpec, refSpeed: number, speed: number, frame = 0): { pose: MountPose | null; gap: Gap | null } {
  const pose = mountPose({ vx: speed, facing: 1 }, frame, riderContext(mountKey, rider, refSpeed));
  if (pose) return { pose, gap: null };
  return { pose: null, gap: gap('mount', MOUNTS[mountKey] ? `Lücke: Reittier ${mountKey} ohne Sattelpunkt` : `Lücke: Reittier ${mountKey} unbekannt`) };
}
