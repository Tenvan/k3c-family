import { GROUND_Y, UNIT_PX } from '../../core/constants';
import { MONARCH } from '../../model/data';
import type { Player, World } from '../../model/types';
import { skillLook } from './skillLooks';

/**
 * Status-Effekte der Skills (B-675): Betäubung, Verlangsamung, Spott über dem Gegner, Sturm-Fläche am Boden, Schild-Blase
 * mit Balken und Last Stand am Monarchen, Aura-Ringe am Boden. Werte und Zeichnung an einer Stelle, damit das Studio sie
 * importiert (B-676). Nur Optik aus dem Zustand, keine Regel; Bewegung allein aus der Zeit, kein Zufall.
 */

export type StatusKind = 'stun' | 'slow' | 'taunt' | 'storm' | 'shield' | 'lastStand' | 'aura';

/** Alle Arten in Zeichen-Reihenfolge (Boden zuerst) */
export const STATUS_KINDS: readonly StatusKind[] = ['aura', 'storm', 'shield', 'lastStand', 'slow', 'stun', 'taunt'];

/** Farben (Palette DB32), Größen in Pixeln, Höhen über dem Boden, Takte in Millisekunden */
export const STATUS_LOOKS = {
  stun: { color: 0xfbf236, size: 7, count: 3, orbit: 16, y: 104, bossY: 190, spinMs: 900 },
  slow: { color: 0x5fcde4, size: 9, y: 104, bossY: 190 },
  taunt: { color: 0xdf7126, height: 18, y: 112, bossY: 200 },
  storm: { color: 0xfbf236, fill: 0x5b6ee1, fillAlpha: 0.2, bolts: 3, boltPx: 70, flashMs: 220 },
  shield: { color: 0x9badb7, radius: 46, y: 48, alpha: 0.28, barW: 44, barH: 5, barY: 104 },
  lastStand: { color: 0xac3232, radius: 40, y: 48, pulseMs: 600 },
  aura: { alpha: 0.14, line: 3, fallback: 0xffffff },
} as const;

/** Was gezeichnet wird: Ort in Pixeln der Stufen-Ebene (`y` = Mitte bzw. Boden), Radius in Pixeln, Anteil 0..1 (Schild-Balken). */
export interface StatusMark {
  kind: StatusKind;
  x: number;
  y: number;
  radius: number;
  color: number;
  share: number;
}

/** Ausschnitt des Zustands, den die Anzeige liest (Spiel: `World`, Studio: gestellte Figuren). */
export type StatusWorld = Partial<Pick<World, 'players' | 'enemies' | 'storms'>> & { castle?: { x: number } };

/** Größter Schild eines Skills (`effect.type` shield) als volle Balkenbreite */
export const SHIELD_MAX = Math.max(1, ...Object.values(MONARCH.skills as Record<string, { effect?: { type?: string; hp?: number } }>)
  .filter((s) => s.effect?.type === 'shield').map((s) => s.effect?.hp ?? 0));

const mark = (kind: StatusKind, x: number, y: number, radius: number, color: number, share = 1): StatusMark => ({ kind, x, y, radius, color, share });

/** Markierungen der Gegner: Betäubung, Verlangsamung und Spott über dem Kopf (nur lebende). */
function enemyMarks(world: StatusWorld, out: StatusMark[]): void {
  for (const e of world.enemies ?? []) {
    if (e.hp <= 0) continue;
    const x = e.x * UNIT_PX;
    const top = (look: { y: number; bossY: number }) => GROUND_Y - (e.boss ? look.bossY : look.y);
    if ((e.slowFor ?? 0) > 0) out.push(mark('slow', x - 18, top(STATUS_LOOKS.slow), STATUS_LOOKS.slow.size, STATUS_LOOKS.slow.color));
    if ((e.stun ?? 0) > 0) out.push(mark('stun', x, top(STATUS_LOOKS.stun), STATUS_LOOKS.stun.orbit, STATUS_LOOKS.stun.color));
    if ((e.tauntFor ?? 0) > 0) out.push(mark('taunt', x + 18, top(STATUS_LOOKS.taunt), STATUS_LOOKS.taunt.height, STATUS_LOOKS.taunt.color));
  }
}

/** Aura-Ringe eines Monarchen am Boden (Holy Ground an der Burg), mindestens eine Unit groß, in der Farbe des Skills. */
function auraMarks(p: Player, castle: StatusWorld['castle'], out: StatusMark[]): void {
  for (const a of p.auras ?? []) {
    const at = a.castle && castle ? castle.x : p.x;
    out.push(mark('aura', at * UNIT_PX, GROUND_Y, Math.max(a.radius * UNIT_PX, UNIT_PX), skillLook(a.skill)?.color ?? STATUS_LOOKS.aura.fallback));
  }
}

/** Markierungen der Monarchen: Auren am Boden, Schild-Blase mit Balken, Last Stand (nur lebende). */
function playerMarks(world: StatusWorld, out: StatusMark[]): void {
  const { shield, lastStand } = STATUS_LOOKS;
  for (const p of world.players ?? []) {
    if (p.respawnIn > 0) continue;
    auraMarks(p, world.castle, out);
    const x = p.x * UNIT_PX;
    const hp = p.shield ?? 0;
    if ((p.shieldFor ?? 0) > 0 && hp > 0) out.push(mark('shield', x, GROUND_Y - shield.y, shield.radius, shield.color, Math.min(1, hp / SHIELD_MAX)));
    if ((p.lastStandFor ?? 0) > 0) out.push(mark('lastStand', x, GROUND_Y - lastStand.y, lastStand.radius, lastStand.color));
  }
}

/** Alle Markierungen des Zustands in Zeichen-Reihenfolge (Boden zuerst); `out` wird geleert und wiederverwendet. */
export function statusMarks(world: StatusWorld, out: StatusMark[] = []): StatusMark[] {
  out.length = 0;
  playerMarks(world, out);
  for (const s of world.storms ?? []) if (s.left > 0) out.push(mark('storm', s.x * UNIT_PX, GROUND_Y, s.radius * UNIT_PX, STATUS_LOOKS.storm.color));
  enemyMarks(world, out);
  out.sort((a, b) => STATUS_KINDS.indexOf(a.kind) - STATUS_KINDS.indexOf(b.kind));
  return out;
}

/** Zeichenfläche, wie `Phaser.GameObjects.Graphics` sie bietet (das Studio und Tests geben eigene). */
export interface StatusPainter {
  fillStyle(color: number, alpha?: number): this;
  lineStyle(width: number, color: number, alpha?: number): this;
  fillCircle(x: number, y: number, r: number): this;
  strokeCircle(x: number, y: number, r: number): this;
  fillRect(x: number, y: number, w: number, h: number): this;
  fillTriangle(x0: number, y0: number, x1: number, y1: number, x2: number, y2: number): this;
  lineBetween(x0: number, y0: number, x1: number, y1: number): this;
}

/** Kleiner Stern (zwei Dreiecke) um (x, y) */
function star(g: StatusPainter, x: number, y: number, s: number): void {
  g.fillTriangle(x, y - s, x - s, y + s * 0.6, x + s, y + s * 0.6);
  g.fillTriangle(x, y + s, x - s, y - s * 0.6, x + s, y - s * 0.6);
}

/** Bodenfläche mit Blitzen; die Blitze springen je `flashMs` an andere Stellen (aus der Zeit, kein Zufall). */
function storm(g: StatusPainter, m: StatusMark, timeMs: number): void {
  const L = STATUS_LOOKS.storm;
  g.fillStyle(L.fill, L.fillAlpha).fillCircle(m.x, m.y, m.radius);
  g.lineStyle(3, m.color, 0.9);
  const step = Math.floor(timeMs / L.flashMs);
  for (let i = 0; i < L.bolts; i++) {
    const bx = m.x + (((step * 7 + i * 13) % 17) / 8 - 1) * m.radius * 0.8;
    const top = m.y - L.boltPx;
    g.lineBetween(bx, top, bx + 8, top + L.boltPx * 0.5);
    g.lineBetween(bx + 8, top + L.boltPx * 0.5, bx - 4, m.y);
  }
}

/** Eine Markierung zur Zeit `timeMs` zeichnen */
export function drawMark(g: StatusPainter, m: StatusMark, timeMs: number): void {
  switch (m.kind) {
    case 'aura':
      g.fillStyle(m.color, STATUS_LOOKS.aura.alpha).fillCircle(m.x, m.y, m.radius);
      g.lineStyle(STATUS_LOOKS.aura.line, m.color, 0.5).strokeCircle(m.x, m.y, m.radius);
      return;
    case 'storm': return storm(g, m, timeMs);
    case 'shield': {
      const L = STATUS_LOOKS.shield;
      g.fillStyle(m.color, L.alpha).fillCircle(m.x, m.y, m.radius);
      g.lineStyle(2, m.color, 0.8).strokeCircle(m.x, m.y, m.radius);
      g.fillStyle(0x000000, 0.6).fillRect(m.x - L.barW / 2, GROUND_Y - L.barY, L.barW, L.barH);
      g.fillStyle(m.color, 1).fillRect(m.x - L.barW / 2, GROUND_Y - L.barY, L.barW * m.share, L.barH);
      return;
    }
    case 'lastStand': {
      const pulse = 0.5 + 0.5 * Math.sin((timeMs / STATUS_LOOKS.lastStand.pulseMs) * Math.PI * 2);
      g.lineStyle(3, m.color, 0.4 + 0.5 * pulse).strokeCircle(m.x, m.y, m.radius);
      return;
    }
    case 'slow':
      g.lineStyle(3, m.color, 1);
      for (let i = 0; i < 3; i++) {
        const a = (i / 3) * Math.PI;
        g.lineBetween(m.x - Math.cos(a) * m.radius, m.y - Math.sin(a) * m.radius, m.x + Math.cos(a) * m.radius, m.y + Math.sin(a) * m.radius);
      }
      return;
    case 'stun': {
      const L = STATUS_LOOKS.stun;
      g.fillStyle(m.color, 1);
      for (let i = 0; i < L.count; i++) {
        const a = (timeMs / L.spinMs) * Math.PI * 2 + (i / L.count) * Math.PI * 2;
        star(g, m.x + Math.cos(a) * m.radius, m.y + Math.sin(a) * m.radius * 0.35, L.size);
      }
      return;
    }
    case 'taunt':
      g.fillStyle(m.color, 1).fillRect(m.x - 2.5, m.y - m.radius, 5, m.radius * 0.7);
      g.fillCircle(m.x, m.y + 3, 3);
      return;
    default:
  }
}

/** Alle Markierungen zur Zeit `timeMs` in `g` zeichnen (`g` vorher leeren). */
export function drawStatus(g: StatusPainter, marks: readonly StatusMark[], timeMs: number): void {
  for (const m of marks) drawMark(g, m, timeMs);
}
