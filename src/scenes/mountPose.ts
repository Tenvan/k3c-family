import type { MountData, SheetData, SpriteSpec } from './sprites';

/**
 * Reittier-Darstellung des Monarchen (B-173, S7.1): reine Funktion aus Snapshot-Werten, Frame und Daten.
 * Keine Phaser-Importe, kein Zustand, nichts wird verändert. Das Zeichnen übernimmt der Renderer (S7.2).
 */

/** Grenzen für `timeScale`, damit Lauf und Sprint sichtbar schneller, aber nie unlesbar laufen */
export const TIME_SCALE_MIN = 0.5;
export const TIME_SCALE_MAX = 2.5;
/** Darunter steht der Monarch (heutige Schwelle aus `updatePlayer`) */
export const MOVE_EPS = 0.05;

export interface MountContext {
  /** Sprite-Schlüssel aus `data/monarch.json › mount` */
  key: string | undefined;
  rider: SpriteSpec;
  /** Lauftempo ohne Sprint (`base.speed × mount.speedFactor`), Bezug für die Bildrate */
  refSpeed: number;
  mounts: Record<string, MountData>;
  sheets: Record<string, SheetData>;
  riderWaist: number;
}

export interface MountPose {
  key: string;
  sheet: string;
  scale: number;
  tint?: string;
  anim: 'idle' | 'run';
  timeScale: number;
  flip: boolean;
  saddleFrame: number;
  /** Oberkörper des Reiters, Pixel relativ zum Fußpunkt des Monarchen; `cropHeight` in Frame-Pixeln von oben */
  rider: { x: number; y: number; originX: number; originY: number; cropHeight: number; scale: number };
  /** Oberkante des Reiters (negativ) für Goldbeutel und HP-Balken */
  top: number;
}

/** Sattelpunkt: Frame außerhalb → `frame % Länge`; Animation fehlt → `idle[0]`; sonst `null`. */
function saddleOf(mount: MountData, anim: 'idle' | 'run', frame: number): { at: [number, number]; index: number } | null {
  const list = mount.saddle[anim];
  if (list?.length) {
    const index = ((Math.trunc(frame) % list.length) + list.length) % list.length;
    return { at: list[index]!, index };
  }
  const idle = mount.saddle.idle?.[0];
  return idle ? { at: idle, index: 0 } : null;
}

/** Wählt Reittier, Animation, Bildrate und Reiter-Position; `null` = Rückfall auf die bisherige Figur. */
export function mountPose(p: { vx: number; facing: 1 | -1 }, frame: number, ctx: MountContext): MountPose | null {
  const mount = ctx.key ? ctx.mounts[ctx.key] : undefined;
  const ms = mount && ctx.sheets[mount.sheet];
  const rs = ctx.sheets[ctx.rider.sheet];
  if (!ctx.key || !mount || !ms || !rs) return null;

  const speed = Math.abs(p.vx);
  const anim = speed > MOVE_EPS ? 'run' : 'idle';
  const timeScale = anim === 'idle' ? 1 : Math.min(TIME_SCALE_MAX, Math.max(TIME_SCALE_MIN, speed / ctx.refSpeed));
  const saddle = saddleOf(mount, anim, frame);
  if (!saddle) return null;

  const flip = p.facing < 0;
  const sx = (saddle.at[0] - ms.originX * ms.frameWidth) * mount.scale;
  const sy = (saddle.at[1] - ms.originY * ms.frameHeight) * mount.scale;
  // Hüfte des Reiters (Frame-Pixel von oben) sitzt auf dem Sattelpunkt, darunter wird abgeschnitten
  const waist = rs.originY * rs.frameHeight - rs.height * (1 - ctx.riderWaist);
  const scale = rs.scale * (ctx.rider.scale ?? 1);

  return {
    key: ctx.key,
    sheet: mount.sheet,
    scale: mount.scale,
    ...(mount.tint ? { tint: mount.tint } : {}),
    anim,
    timeScale,
    flip,
    saddleFrame: saddle.index,
    rider: {
      x: flip ? -sx : sx,
      y: sy,
      originX: flip ? 1 - rs.originX : rs.originX,
      originY: waist / rs.frameHeight,
      cropHeight: waist,
      scale,
    },
    top: sy - rs.height * ctx.riderWaist * scale,
  };
}
