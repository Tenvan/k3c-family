import { describe, expect, it } from 'vitest';
import { FONTS } from './render/fontRules';
import renderer from './render/worldRenderer.ts?raw';
import hud from './HudScene.ts?raw';

/** B-398: Das Gold steht nur im HUD (Spielerkarte), nicht als Text über dem Reiter in der Welt. */
describe('Gold nur im HUD (B-398)', () => {
  it('die Welt-Ansicht des Spielers erzeugt keinen Goldtext', () => {
    expect(renderer).not.toMatch(/\bpurse\b/);
    expect(renderer).not.toMatch(/\$\{p\.gold\}\s*Gold/);
    expect(Object.keys(FONTS)).not.toContain('purse');
  });

  it('die Spielerkarte nennt Gold und Maximum', () => {
    expect(hud).toMatch(/\$\{p\.gold\}\/\$\{max\}/);
  });
});
