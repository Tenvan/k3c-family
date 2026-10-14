import { describe, expect, it } from 'vitest';
import { GOLD_FLASH_MS, stepGoldFlash } from './goldFlash';

describe('stepGoldFlash (B-398)', () => {
  it('der erste Wert blinkt nicht (nur Anzeige, keine Änderung)', () => {
    expect(stepGoldFlash(undefined, 10, 0).flash).toBe(false);
  });

  it('eine Änderung hebt die Zahl für die feste Zeit hervor', () => {
    let r = stepGoldFlash(undefined, 10, 0);
    r = stepGoldFlash(r.state, 7, 1000);
    expect(r.flash).toBe(true);
    r = stepGoldFlash(r.state, 7, 1000 + GOLD_FLASH_MS - 1);
    expect(r.flash).toBe(true);
    r = stepGoldFlash(r.state, 7, 1000 + GOLD_FLASH_MS);
    expect(r.flash).toBe(false);
  });

  it('eine neue Änderung startet die Zeit neu', () => {
    let r = stepGoldFlash(undefined, 10, 0);
    r = stepGoldFlash(r.state, 7, 1000);
    r = stepGoldFlash(r.state, 4, 1000 + GOLD_FLASH_MS - 100);
    r = stepGoldFlash(r.state, 4, 1000 + GOLD_FLASH_MS + 100);
    expect(r.flash).toBe(true);
  });
});
