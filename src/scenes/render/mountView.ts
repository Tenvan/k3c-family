import Phaser from 'phaser';
import { clientLog } from '../../core/clientLog';
import { MONARCH } from '../../model/data';
import type { Player } from '../../model/types';
import { mountGait, mountedAnim } from './mountGait';
import { PAPER_MOUNTED, animFrame, paperTextureKey } from './paperAssets';
import { type AnimName, PLAYER_SPRITES, face, makeSprite, paperManifest, playPaper } from './sprites';

/**
 * Monarch auf dem Reittier als Papierschnitt (US1, US2): Das Monarch-Asset zeigt Reiter und Reittier zusammen, das Pferd in
 * `schritt`, `trab` und `galopp`, andere Reittiere in `<gangart>-<reittier>` (`mountedAnim`, sonst Pferd). Container mit einem
 * Sprite; liest nur Snapshot-Felder (`index`, `vx`, `facing`) und das Reittier aus `data/monarch.json`.
 */

const REF_SPEED = MONARCH.base.speed * MONARCH.mount.speedFactor;
const warned = new Set<string>();

/** Spieler-Spec mit dem berittenen Asset (Variante des Spielers bleibt) */
const mountedSpec = (p: Player) => {
  const spec = PLAYER_SPRITES[p.index % PLAYER_SPRITES.length]!;
  return { ...spec, paper: { ...PAPER_MOUNTED, variante: spec.paper?.variante } };
};

/** Reiter samt Reittier `mountKey` (Standard aus `data/monarch.json`) oder `null` (Asset nicht geladen): dann zeichnet der
 *  Aufrufer die Figur zu Fuß. Hat das Monarch-Asset das Reittier nicht, reitet er das Pferd (eine Warnung je Schlüssel). */
export function createRider(scene: Phaser.Scene, p: Player, mountKey: string = MONARCH.mount.sprite): Phaser.GameObjects.Container | null {
  const spec = mountedSpec(p);
  const key = `${spec.paper.gruppe}/${spec.paper.id}`;
  if (!paperManifest(key)?.animationen.schritt || !scene.textures.exists(paperTextureKey(key, 'schritt'))) {
    if (!warned.has(key)) {
      warned.add(key);
      clientLog('warn', '🤒 Reittier-Grafik fehlt', { key });
    }
    return null;
  }
  const rider = makeSprite(scene, spec);
  // Oberkante beritten aus dem Gang-Rahmen (Pferd samt Reiter ist höher als die Figur zu Fuß, Review US1, W2)
  const m = paperManifest(key)!;
  const top = ((rider.getData('target') as number) * animFrame(m, 'schritt').rahmen[1]) / -m.rahmen[1];
  const view = scene.add.container(0, 0, [rider]).setData('top', top).setData('mount', mountKey);
  updateRider(view, p);
  return view;
}

/** Gangart nach Tempo; im Stand steht der erste Schritt-Frame. `anim` (Schlag usw.) hat beritten keine eigene Grafik. */
export function updateRider(view: Phaser.GameObjects.Container, p: Player, _anim: AnimName = 'idle', _restart = false): void {
  const rider = view.getAt(0) as Phaser.GameObjects.Sprite;
  const { gait, moving } = mountGait(p.vx, REF_SPEED);
  face(rider, p.facing);
  const anims = paperManifest(rider.getData('paper') as string)?.animationen ?? {};
  const mountKey = view.getData('mount') as string;
  const pick = mountedAnim(mountKey, gait, rider.getData('variante') as string | undefined, (a) => a in anims);
  if (pick.fallback && !warned.has(`mount:${mountKey}`)) {
    warned.add(`mount:${mountKey}`);
    clientLog('warn', '🤒 Reittier ohne Monarch-Animation, stattdessen Pferd', { key: mountKey });
  }
  playPaper(rider, pick.anim);
  if (moving && !rider.anims.isPlaying) rider.anims.resume();
  if (!moving && rider.anims.isPlaying) rider.anims.pause(rider.anims.currentAnim?.frames[0]);
}
