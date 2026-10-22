import { PLAYER_COLORS } from '../../core/constants';
import { ECONOMY } from '../../model/data';
import type { GameEvent, ResourceNode } from '../../model/types';

/**
 * Wimpel über markierten Knoten in der Farbe des markierenden Spielers (GEB10, B-456). Der Snapshot kennt nur `marked`;
 * wer markiert hat, steht im Ereignis `coinGive` (`to: 'mark'`, Ort = Spieler). Reine Logik ohne Phaser und ohne Zufall.
 */

/** Wimpel ohne bekannten Spieler (Plantage, nach Neuladen, Beitritt): helles Grau, keine Spielerfarbe */
export const PENNANT_NEUTRAL = 0xdddddd;
/** Höchste Lage über dem Boden (px): Baumhöhe 11,25 Kacheln; Wurzeln (15 Kacheln) reichen sonst bis an den Bildrand */
export const PENNANT_MAX_HEIGHT = 360;
/** Zahl-Reichweite (`data/economy.json`) plus Rundung des Ereignis-Orts auf eine Nachkommastelle */
const MARK_RANGE = ECONOMY.payRangeUnits + 0.1;

/** Farbe je Spieler wie der Punkt unter dem Monarchen (`PLAYER_COLORS`); unbekannt → neutral */
export function pennantColor(player: number | undefined): number {
  return player === undefined || player < 0 ? PENNANT_NEUTRAL : PLAYER_COLORS[player % PLAYER_COLORS.length]!;
}

type MarkNode = Pick<ResourceNode, 'id' | 'x' | 'marked'> & { kind?: string };

/** Wie `gatherOf` im Server (`engine/sim/world_vein.go`): nur Sammel-Knoten und Adern lassen sich markieren; Busch, Plantage-Baum u. a. nicht */
const MARKABLE = new Set([...Object.keys(ECONOMY.gatherables), ...Object.keys(ECONOMY.veins)]);
const markable = (n: MarkNode): boolean => n.kind === undefined || MARKABLE.has(n.kind);

/** Knoten, den eine Münze am Ort `x` bezahlt: der nächste unmarkierte in Reichweite, sonst der nächste markierte; keiner → `null` */
export function paidNodeId(nodes: readonly MarkNode[], x: number): number | null {
  let best: MarkNode | null = null;
  for (const n of nodes) {
    if (Math.abs(n.x - x) > MARK_RANGE || !markable(n)) continue;
    const better = !best || (n.marked === best.marked ? Math.abs(n.x - x) < Math.abs(best.x - x) : !n.marked);
    if (better) best = n;
  }
  return best?.id ?? null;
}

/** Merkt je Knoten den Spieler der letzten Markier-Münze eines Frames (`owners`: Knoten-ID → Spieler-Index) */
export function noteMarkers(owners: Map<number, number>, nodes: readonly MarkNode[], events: readonly GameEvent[]): void {
  for (const e of events) {
    if (e.type !== 'coinGive' || e.to !== 'mark') continue;
    const id = paidNodeId(nodes, e.x);
    if (id !== null) owners.set(id, e.player);
  }
}
