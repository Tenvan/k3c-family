import { describe, expect, it } from 'vitest';
import buildingsJson from '../../data/buildings.json';
import hubJson from '../../data/hub.json';
import { HUB, LIGHT } from './data';

// B-209: Jede Platz-Art aus data/hub.json muss der Client kennen (SiteKind = Gebäude außer Burg).
const known = new Set(Object.keys(buildingsJson).filter((k) => k !== 'castle'));
const raw = hubJson as unknown as Record<'sites' | 'islandSites', Record<string, unknown>[]>;

describe('HUB passt zu data/hub.json', () => {
  for (const list of ['sites', 'islandSites'] as const) {
    it(`${list}: jede kind ist bekannt, offsetUnits und hubLevel sind Zahlen`, () => {
      for (const site of raw[list]) {
        expect(known.has(String(site.kind)), `unbekannte Platz-Art: ${String(site.kind)}`).toBe(true);
        expect(typeof site.offsetUnits).toBe('number');
        expect(typeof site.hubLevel).toBe('number');
      }
    });
  }

  it('Typ deckt sitePlacement ab', () => {
    expect(HUB.sitePlacement.zone1WallUnits).toBeGreaterThan(HUB.sitePlacement.zone1TowerUnits);
    expect(HUB.sitePlacement.zones).toBeGreaterThan(0);
  });
});

// W12.1 (B-472): Licht-Felder für Fackeln und Lagerfeuer stehen in data/light.json, Radius positiv, Stärke in (0, 1].
describe('LIGHT passt zu data/light.json', () => {
  it('Fackel und Lagerfeuer haben Radius und Stärke', () => {
    for (const kind of ['torch', 'campfire'] as const) {
      const { radius, strength } = LIGHT.sources[kind]!;
      expect(radius, kind).toBeGreaterThan(0);
      expect(strength, kind).toBeGreaterThan(0);
      expect(strength, kind).toBeLessThanOrEqual(1);
    }
  });

  it('Nacht-Untergrenze in (0, 1) und Lichtfarbe als #rrggbb (W13.1)', () => {
    expect(LIGHT.nightBrightness).toBeGreaterThan(0);
    expect(LIGHT.nightBrightness).toBeLessThan(1);
    expect(LIGHT.glowColor).toMatch(/^#[0-9a-f]{6}$/i);
  });
});
