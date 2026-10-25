import type Phaser from 'phaser';
import type { Site } from '../../model/types';
import { siteWait, type WaitInfo, type WaitReason } from '../hud/economyLabels';

/**
 * Wartegrund am Bauplatz (W6.2, B-117): Symbol (Platzhalter bis GR3) und Text. Der Grund kommt vom Server
 * (`wirtschaftAnzeige.siteWait`); hier wird nur gezeichnet. Fehlt das Feld, bleibt die Anzeige leer.
 */

const COLOR: Record<WaitReason, { symbol: number; text: string }> = {
  worker: { symbol: 0x4cc9f0, text: '#ffffff' },
  material: { symbol: 0xff8fa3, text: '#ff8fa3' },
  danger: { symbol: 0xe63946, text: '#ff6b6b' },
};

/** Wartegrund eines Bauplatzes; läuft der Bau schon (Fortschritt > 0) und fehlt nur der Bauer, bleibt der Platz leer. `depth`: Biom-Tiefe (Treppen-Kosten). */
export function siteStatus(s: Pick<Site, 'kind' | 'state' | 'upgrade' | 'level' | 'buildProgress'>, danger: boolean | undefined, depth?: number): WaitInfo | null {
  const info = siteWait(s, danger, false, depth);
  if (info?.reason === 'worker' && s.state !== 'built' && s.buildProgress > 0) return null;
  return info;
}

/** Eintrag für den `key` des Neuzeichnens: ändert sich der Wartegrund, wird neu gezeichnet. */
export const statusKey = (info: WaitInfo | null): string => (info ? `${info.reason}:${info.text}` : '');

/** Zeile am Platz: Name und Wartegrund */
export const statusLine = (name: string, info: WaitInfo): string => `${name}: ${info.text}`;

/** Symbol (Platzhalter-Form je Grund) und Text; `y` ist die Unterkante des Textes. */
export function drawSiteStatus(g: Phaser.GameObjects.Graphics, label: Phaser.GameObjects.Text, info: WaitInfo, name: string, y: number): void {
  const c = COLOR[info.reason];
  label.setPosition(0, y).setText(statusLine(name, info)).setColor(c.text);
  const sy = y - label.height - 18;
  g.fillStyle(c.symbol).lineStyle(3, 0x000000);
  if (info.reason === 'worker') g.fillCircle(0, sy, 12).strokeCircle(0, sy, 12);
  else if (info.reason === 'material') g.fillRect(-12, sy - 12, 24, 24).strokeRect(-12, sy - 12, 24, 24);
  else g.fillTriangle(0, sy - 14, -14, sy + 12, 14, sy + 12).strokeTriangle(0, sy - 14, -14, sy + 12, 14, sy + 12);
}
