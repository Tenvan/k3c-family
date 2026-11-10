import { describe, expect, it } from 'vitest';
import { PadNav, REPEAT_EVERY_MS, REPEAT_FIRST_MS, heldActions, pickTarget, type Box, type PadSnapshot } from './padNav';

const pad = (pressed: number[] = [], axes: number[] = [0, 0, 0, 0]): PadSnapshot => ({ buttons: Array.from({ length: 17 }, (_, i) => ({ pressed: pressed.includes(i) })), axes });

describe('gehaltene Tasten', () => {
  it('A, D-Pad und linker Stick', () => {
    expect([...heldActions(pad([0]))]).toEqual(['accept']);
    expect([...heldActions(pad([12]))]).toEqual(['up']);
    expect([...heldActions(pad([13, 15]))].sort()).toEqual(['down', 'right']);
    expect([...heldActions(pad([], [-0.9, 0.7, 0, 0]))].sort()).toEqual(['down', 'left']);
  });

  it('B geht zurück; X, Y, View und Menu lösen nichts aus', () => {
    expect([...heldActions(pad([1]))]).toEqual(['back']);
    expect(heldActions(pad([2, 3, 8, 9])).size).toBe(0);
  });

  it('kleiner Stickausschlag zählt nicht', () => {
    expect(heldActions(pad([], [0.3, -0.3, 0, 0])).size).toBe(0);
  });

  it('ein Pad ohne Tasten und Achsen stört nicht', () => {
    expect(heldActions({ buttons: [], axes: [] }).size).toBe(0);
  });
});

describe('Wiederholung', () => {
  it('Richtung: sofort, nach der ersten Frist, dann im Takt; Loslassen setzt zurück', () => {
    const nav = new PadNav();
    const down = [pad([13])];
    expect(nav.update(down, 0)).toEqual(['down']);
    expect(nav.update(down, REPEAT_FIRST_MS - 1)).toEqual([]);
    expect(nav.update(down, REPEAT_FIRST_MS)).toEqual(['down']);
    expect(nav.update(down, REPEAT_FIRST_MS + REPEAT_EVERY_MS - 1)).toEqual([]);
    expect(nav.update(down, REPEAT_FIRST_MS + REPEAT_EVERY_MS)).toEqual(['down']);
    expect(nav.update([pad()], 2000)).toEqual([]);
    expect(nav.update(down, 2010)).toEqual(['down']);
  });

  it('B wiederholt wie A nicht beim Halten', () => {
    const nav = new PadNav();
    expect(nav.update([pad([1])], 0)).toEqual(['back']);
    expect(nav.update([pad([1])], 5000)).toEqual([]);
  });

  it('A wiederholt nicht beim Halten', () => {
    const nav = new PadNav();
    expect(nav.update([pad([0])], 0)).toEqual(['accept']);
    expect(nav.update([pad([0])], 5000)).toEqual([]);
  });

  it('mehrere Pads: die Tasten aller zählen, je Aktion einmal', () => {
    const nav = new PadNav();
    expect(nav.update([pad([13]), pad([13]), null, undefined], 0)).toEqual(['down']);
  });
});

const box = (x: number, y: number, w = 100, h = 40): Box => ({ left: x, top: y, right: x + w, bottom: y + h });

describe('Zielwahl', () => {
  // Raster: zwei Spalten, drei Zeilen (Kacheln)
  const grid = [box(0, 0), box(200, 0), box(0, 100), box(200, 100), box(0, 200), box(200, 200)];

  it('nächstes Element in der Richtung', () => {
    expect(pickTarget(grid, 0, 'right')).toBe(1);
    expect(pickTarget(grid, 0, 'down')).toBe(2);
    expect(pickTarget(grid, 3, 'up')).toBe(1);
    expect(pickTarget(grid, 3, 'left')).toBe(2);
    expect(pickTarget(grid, 2, 'down')).toBe(4);
  });

  it('am Rand gibt es kein Ziel', () => {
    expect(pickTarget(grid, 0, 'up')).toBe(-1);
    expect(pickTarget(grid, 0, 'left')).toBe(-1);
    expect(pickTarget(grid, 5, 'down')).toBe(-1);
    expect(pickTarget(grid, 5, 'right')).toBe(-1);
  });

  it('versetzte Zeilen: das Element in der Spalte gewinnt gegen ein weit seitliches', () => {
    const list = [box(0, 0), box(0, 60), box(400, 60)];
    expect(pickTarget(list, 0, 'down')).toBe(1);
  });

  it('ohne Fokus: das oberste, linkeste Element; ohne Elemente -1', () => {
    expect(pickTarget(grid, -1, 'down')).toBe(0);
    expect(pickTarget([box(50, 100), box(0, 100), box(300, 20)], -1, 'right')).toBe(2);
    expect(pickTarget([], 0, 'up')).toBe(-1);
  });
});
