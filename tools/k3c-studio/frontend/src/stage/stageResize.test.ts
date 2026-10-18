import { describe, expect, it } from 'vitest';
import { refreshToParent, settleToParent } from './stageResize';

// Fehler aus dem Live-Test (WZ17.5): Ändern sich Zeilen über der Live-Vorschau, ändert sich die Höhe der Bühne ohne
// Fenster-Resize. Phasers `refresh()` rechnet mit der zuletzt gemessenen Elterngröße (`parentSize`), die nur der eigene
// Takt alle 500 ms neu liest; ohne erneutes Messen blieb das Canvas eine Änderung lang (oder ohne Takt dauerhaft) zu groß.
describe('Bühne folgt dem Element', () => {
  it('misst die Elterngröße neu, bevor neu skaliert wird', () => {
    const calls: string[] = [];
    let parent = 615;
    const scale = {
      parentSize: { height: 615 },
      getParentBounds() { calls.push('bounds'); const changed = this.parentSize.height !== parent; this.parentSize.height = parent; return changed; },
      refresh() { calls.push(`refresh ${this.parentSize.height}`); },
    };
    parent = 594;
    refreshToParent(scale);
    expect(calls).toEqual(['bounds', 'refresh 594']);
  });

  it('ohne Größenänderung trotzdem neu skalieren (Lage kann sich geändert haben)', () => {
    const calls: string[] = [];
    refreshToParent({ getParentBounds: () => { calls.push('bounds'); return false; }, refresh: () => { calls.push('refresh'); } });
    expect(calls).toEqual(['bounds', 'refresh']);
  });

  it('nach dem Layout: neu skalieren nur, wenn sich Größe oder Lage geändert hat (keine Arbeit bei jedem Rendern)', () => {
    let changed = true;
    let refreshed = 0;
    const scale = { getParentBounds: () => changed, refresh: () => { refreshed++; } };
    settleToParent(scale);
    changed = false;
    settleToParent(scale);
    expect(refreshed).toBe(1);
  });
});
