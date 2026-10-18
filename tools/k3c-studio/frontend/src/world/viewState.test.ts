import { describe, expect, it } from 'vitest';
import { levelModel } from './levelView';
import { DEFAULT_SCALE, MAX_SCALE, MIN_SCALE, ZOOM_PERCENTS, clampLeft, clampScale, jumpPoints, leftFor, scaleOfPercent, zoomView } from './viewState';

const model = levelModel({
  seed: 's', biomeId: 'forest', widthUnits: 900, chunkWidthUnits: 50, hubCenterUnits: 450, warnings: [],
  chunks: [{ index: 0, kind: 'edge', startUnits: 0 }],
  entities: [{ kind: 'portal', x: 250 }, { kind: 'portal', x: 650 }, { kind: 'exit', x: 880 }, { kind: 'tree', x: 40 }],
} as never);

describe('Zoom (B-477/AC-03)', () => {
  it('der Maßstab bleibt in den Grenzen', () => {
    expect(clampScale(0.1)).toBe(MIN_SCALE);
    expect(clampScale(999)).toBe(MAX_SCALE);
    expect(clampScale(10)).toBe(10);
  });

  it('feste Stufen sind Prozent des Standard-Maßstabs', () => {
    expect(scaleOfPercent(100)).toBe(DEFAULT_SCALE);
    expect(scaleOfPercent(200)).toBe(DEFAULT_SCALE * 2);
    for (const p of ZOOM_PERCENTS) expect(scaleOfPercent(p)).toBeGreaterThanOrEqual(MIN_SCALE);
    expect(scaleOfPercent(100000)).toBe(MAX_SCALE);
  });

  it('die Mitte der Ansicht bleibt beim Zoomen an ihrer Stelle', () => {
    const view = { left: 100, scale: 10 };
    const next = zoomView(view, 20, 900, 1000); // 1000 px: Mitte bei 100 + 50 = 150 Units
    expect(next.scale).toBe(20);
    expect(next.left + 1000 / 20 / 2).toBeCloseTo(150);
  });

  it('beim Herauszoomen am Rand bleibt die Ansicht in der Welt', () => {
    const next = zoomView({ left: 0, scale: 20 }, 2, 900, 1000); // Ansicht 500 Units breit, Welt 900
    expect(next.left).toBe(0);
    expect(zoomView({ left: 400, scale: 20 }, 2, 900, 1000).left).toBe(175); // Mitte 425 bleibt: 425 - 250
  });

  it('der Zoom wird begrenzt, ohne Fehler', () => {
    expect(zoomView({ left: 0, scale: 12 }, 9999, 900, 1000).scale).toBe(MAX_SCALE);
    expect(zoomView({ left: 0, scale: 12 }, 0, 900, 1000).scale).toBe(MIN_SCALE);
  });
});

describe('Position (B-477/AC-01)', () => {
  it('clampLeft hält die Ansicht in der Welt; ist die Welt schmaler als die Ansicht, gilt 0', () => {
    expect(clampLeft(-5, 900, 100)).toBe(0);
    expect(clampLeft(5000, 900, 100)).toBe(800);
    expect(clampLeft(300, 900, 100)).toBe(300);
    expect(clampLeft(10, 50, 100)).toBe(0);
  });

  it('leftFor stellt x in die Mitte, am Rand begrenzt', () => {
    expect(leftFor(450, 900, 100)).toBe(400);
    expect(leftFor(0, 900, 100)).toBe(0);
    expect(leftFor(900, 900, 100)).toBe(800);
  });
});

describe('Sprungpunkte (B-477/AC-02)', () => {
  const byId = (gates: number[] = [321, 300]) => Object.fromEntries(jumpPoints(model, gates).map((p) => [p.id, p.x]));

  it('links, Mitte, rechts, Hub, Tor, Portale und Ausgang', () => {
    expect(byId()).toEqual({ links: 0, mitte: 450, rechts: 900, hub: 450, tor: 300, portal1: 250, portal2: 650, ausgang: 880 });
  });

  it('fehlende Punkte sind null (ausgegraut)', () => {
    const none = Object.fromEntries(jumpPoints(model, []).map((p) => [p.id, p.x]));
    expect(none.tor).toBeNull();
    const empty = Object.fromEntries(jumpPoints(null, []).map((p) => [p.id, p.x]));
    expect(Object.values(empty).every((x) => x === null)).toBe(true);
    const one = levelModel({ seed: 's', biomeId: 'forest', widthUnits: 100, chunkWidthUnits: 50, chunks: [], entities: [{ kind: 'portal', x: 10 }] } as never);
    const pts = Object.fromEntries(jumpPoints(one, []).map((p) => [p.id, p.x]));
    expect(pts.portal1).toBe(10);
    expect(pts.portal2).toBeNull();
    expect(pts.ausgang).toBeNull();
    expect(pts.hub).toBeNull();
  });
});
