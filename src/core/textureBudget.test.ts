import { describe, expect, it } from 'vitest';
import data from '../../data/render.json';
import { budgetState, budgetText, isXbox, textureBudgetMiB } from './textureBudget';

const XBOX_UA = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; Xbox; Xbox One) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0 Safari/537.36 Edge/150.0';
const PC_UA = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/150.0 Safari/537.36 Edg/150.0';

describe('Texturbudget (B-772/AC-01, AC-02)', () => {
  it('erkennt die Xbox am User-Agent, den PC nicht', () => {
    expect(isXbox(XBOX_UA)).toBe(true);
    expect(isXbox(PC_UA)).toBe(false);
    expect(isXbox('')).toBe(false);
  });

  it('das Budget kommt aus data/render.json: 130 MiB auf der Xbox, sonst keins', () => {
    expect(data.xbox.textureMiB).toBe(130);
    expect(textureBudgetMiB(XBOX_UA)).toBe(130);
    expect(textureBudgetMiB(PC_UA)).toBeNull();
    expect(textureBudgetMiB(XBOX_UA, { xbox: { textureMiB: 64, near: 0.5 } })).toBe(64);
  });

  it('bewertet ok, near und over an den Grenzen', () => {
    expect(budgetState(96, 130)).toBe('ok');
    expect(budgetState(110.4, 130)).toBe('ok'); // 85 % von 130 = 110,5 → knapp darunter ist noch ok
    expect(budgetState(111, 130)).toBe('near');
    expect(budgetState(129.9, 130)).toBe('near');
    expect(budgetState(130, 130)).toBe('over');
    expect(budgetState(410, 130)).toBe('over');
  });

  it('der Text nennt Verbrauch, Budget und Prozent', () => {
    expect(budgetText(96, 130)).toBe('Texturen 96 von 130 MiB (74 %)');
  });
});
