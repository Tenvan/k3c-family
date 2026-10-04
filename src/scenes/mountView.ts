import Phaser from 'phaser';
import { clientLog } from '../core/clientLog';
import { MONARCH } from '../model/data';
import type { Player } from '../model/types';
import { type MountContext, type MountPose, mountPose } from './mountPose';
import { MOUNTS, PLAYER_SPRITES, RIDER_WAIST, SHEETS, face, makeSprite, playAnim } from './sprites';

/**
 * Monarch auf dem Standard-Reittier (B-173, S7.2): Container aus Reittier- und Reiter-Sprite. Liest nur Snapshot-Felder
 * (`index`, `vx`, `facing`) und Daten, alle Entscheidungen trifft `mountPose`. Index 0 = Reittier, 1 = Reiter.
 */

const ctxFor = (p: Player): MountContext => ({
  key: MONARCH.mount.sprite,
  rider: PLAYER_SPRITES[p.index % PLAYER_SPRITES.length],
  refSpeed: MONARCH.base.speed * MONARCH.mount.speedFactor,
  mounts: MOUNTS,
  sheets: SHEETS,
  riderWaist: RIDER_WAIST,
});

const warned = new Set<string>();

/** Reiter samt Reittier oder `null` (unbekannter Schlüssel, Textur fehlt): dann zeichnet der Aufrufer die bisherige Figur. */
export function createRider(scene: Phaser.Scene, p: Player): Phaser.GameObjects.Container | null {
  const ctx = ctxFor(p);
  const pose = mountPose(p, 0, ctx);
  if (!pose || !scene.textures.exists(`${pose.sheet}-idle`)) {
    if (ctx.key && !warned.has(ctx.key)) {
      warned.add(ctx.key);
      clientLog('warn', '🐴 Reittier unbekannt', { key: ctx.key });
    }
    return null;
  }
  const mount = scene.add.sprite(0, 0, `${pose.sheet}-idle`, 0).setData('sheet', pose.sheet);
  const rider = makeSprite(scene, ctx.rider);
  const view = scene.add.container(0, 0, [mount, rider]).setData('ctx', ctx).setData('top', pose.top);
  updateRider(view, p);
  return view;
}

export function updateRider(view: Phaser.GameObjects.Container, p: Player): void {
  const ctx = view.getData('ctx') as MountContext;
  const mount = view.getAt(0) as Phaser.GameObjects.Sprite;
  const rider = view.getAt(1) as Phaser.GameObjects.Sprite;
  const first = mountPose(p, Number(mount.frame.name), ctx);
  if (!first) return;
  const key = `${first.sheet}-${first.anim}`;
  if (mount.anims.currentAnim?.key !== key) mount.play(key);
  mount.anims.timeScale = first.timeScale;
  // Sattelpunkt hängt am Frame, der nach dem Wechsel der Animation gilt
  const pose = mountPose(p, Number(mount.frame.name), ctx) ?? first;
  styleMount(mount, pose, p.facing);
  styleRider(rider, pose);
}

function styleMount(mount: Phaser.GameObjects.Sprite, pose: MountPose, facing: number): void {
  face(mount, facing);
  mount.setScale(pose.scale);
  if (pose.tint) mount.setTint(Phaser.Display.Color.HexStringToColor(pose.tint).color);
}

function styleRider(rider: Phaser.GameObjects.Sprite, pose: MountPose): void {
  const r = pose.rider;
  playAnim(rider, 'idle');
  rider.setFlipX(pose.flip).setOrigin(r.originX, r.originY).setScale(r.scale).setPosition(r.x, r.y);
  rider.setCrop(0, 0, rider.frame.width, r.cropHeight);
}
