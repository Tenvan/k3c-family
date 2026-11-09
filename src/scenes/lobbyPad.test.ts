import { describe, expect, it } from 'vitest';
import { heldPadKeys, type LobbyPad } from './lobbyPad';

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

  it('B und View bleiben ohne Wirkung; fehlende Pads stören nicht', () => {
    expect(heldPadKeys([pad([1, 8]), null, undefined]).size).toBe(0);
  });
});
