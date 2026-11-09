import { describe, expect, it } from 'vitest';
import { freshPadButtons, heldPadKeys, type LobbyPad } from './lobbyPad';

const pad = (pressed: number[] = [], stick = { x: 0, y: 0 }): LobbyPad => ({ buttons: Array.from({ length: 17 }, (_, i) => ({ pressed: pressed.includes(i) })), leftStick: stick });

describe('Lobby-Tasten am Pad (B-767)', () => {
  it('X (2) und Y (3) lösen das Löschen aus', () => {
    expect([...heldPadKeys([pad([2])])]).toEqual(['x']);
    expect([...heldPadKeys([pad([3])])]).toEqual(['x']);
  });

  it('A bestätigt, D-Pad und Stick wählen', () => {
    expect([...heldPadKeys([pad([0])])]).toEqual(['a']);
    expect([...heldPadKeys([pad([12, 15])])].sort()).toEqual(['right', 'up']);
    expect([...heldPadKeys([pad([], { x: -0.8, y: 0.9 })])].sort()).toEqual(['down', 'left']);
  });

  it('Menu geht zurück; mit View zusammen (Home-Kombi) nicht', () => {
    expect([...heldPadKeys([pad([9])])]).toEqual(['back']);
    expect(heldPadKeys([pad([8, 9])]).size).toBe(0);
  });

  it('B geht zurück wie Menu', () => {
    expect([...heldPadKeys([pad([1])])]).toEqual(['back']);
  });

  it('View allein bleibt ohne Wirkung; fehlende Pads stören nicht', () => {
    expect(heldPadKeys([pad([8]), null, undefined]).size).toBe(0);
  });
});

describe('Neu gedrückte Tasten für das Log', () => {
  it('nennt nur neue Tasten als Pad:Taste und die Menge aller gedrückten', () => {
    const first = freshPadButtons([pad([2])], new Set(), 30);
    expect(first.fresh).toEqual(['0:2']);
    const again = freshPadButtons([pad([2, 3])], first.now, 30);
    expect(again.fresh).toEqual(['0:3']);
    expect([...again.now].sort()).toEqual(['0:2', '0:3']);
  });

  it('beachtet das Budget und fehlende Pads', () => {
    expect(freshPadButtons([pad([0, 1, 2])], new Set(), 2).fresh).toHaveLength(2);
    expect(freshPadButtons([pad([0])], new Set(), 0).fresh).toEqual([]);
    expect(freshPadButtons([null, pad([1])], new Set(), 5).fresh).toEqual(['1:1']);
  });
});

