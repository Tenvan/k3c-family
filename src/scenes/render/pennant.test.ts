import { describe, expect, it } from 'vitest';
import { PLAYER_COLORS } from '../../core/constants';
import type { GameEvent } from '../../model/types';
import { PENNANT_NEUTRAL, noteMarkers, markerOf, paidNodeId, pennantColor } from './pennant';

const node = (id: number, x: number, marked = false) => ({ id, x, marked });
const mark = (player: number, x: number): GameEvent => ({ type: 'coinGive', player, x, to: 'mark' });

describe('Wimpel-Farbe (GEB10, B-456/AC-01)', () => {
  it('Spielerfarbe aus PLAYER_COLORS, Spieler 1 und 2 verschieden', () => {
    expect(pennantColor(0)).toBe(PLAYER_COLORS[0]);
    expect(pennantColor(1)).toBe(PLAYER_COLORS[1]);
    expect(pennantColor(0)).not.toBe(pennantColor(1));
  });

  it('unbekannter Spieler neutral, keine Spielerfarbe', () => {
    expect(pennantColor(undefined)).toBe(PENNANT_NEUTRAL);
    expect(pennantColor(-1)).toBe(PENNANT_NEUTRAL);
    expect(PLAYER_COLORS).not.toContain(PENNANT_NEUTRAL);
  });

  it('mehr Spieler als Farben: reihum', () => {
    expect(pennantColor(PLAYER_COLORS.length)).toBe(PLAYER_COLORS[0]);
  });
});

describe('Wer markiert (GEB10)', () => {
  it('nächster unmarkierter Knoten in Reichweite, markierte nur als Rückfall', () => {
    const nodes = [node(1, 10, true), node(2, 11.5), node(3, 20)];
    expect(paidNodeId(nodes, 10.2)).toBe(2);
    expect(paidNodeId([node(1, 10, true)], 10.2)).toBe(1);
    expect(paidNodeId(nodes, 15)).toBeNull();
  });

  it('nicht markierbare Knoten (Busch, Plantage-Baum) bekommen keinen Wimpel, auch wenn sie näher stehen', () => {
    const nodes = [{ ...node(1, 10.4), kind: 'bush' }, { ...node(2, 11), kind: 'tree' }, { ...node(3, 12), kind: 'ironVein' }];
    expect(paidNodeId(nodes, 10.2)).toBe(2);
    expect(paidNodeId([nodes[0]!], 10.2)).toBeNull();
    expect(paidNodeId([nodes[2]!], 12)).toBe(3);
  });

  it('markedBy aus dem Snapshot gilt vor der Ereignis-Zuordnung (B-532)', () => {
    const owners = new Map([[1, 0]]);
    expect(markerOf({ id: 1, markedBy: 1 }, owners)).toBe(1);
    expect(markerOf({ id: 1 }, owners)).toBe(0);
    expect(markerOf({ id: 2 }, owners)).toBeUndefined();
  });

  it('nur Markier-Münzen zählen, die letzte gewinnt; zwei Spieler an zwei Knoten', () => {
    const owners = new Map<number, number>();
    const nodes = [node(1, 10), node(2, 30)];
    noteMarkers(owners, nodes, [mark(0, 10), { type: 'coinGive', player: 1, x: 10, to: 'site' }, mark(1, 30.4)]);
    expect([...owners]).toEqual([[1, 0], [2, 1]]);
    noteMarkers(owners, nodes, [mark(1, 9.6)]);
    expect(owners.get(1)).toBe(1);
  });
});
