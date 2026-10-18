import { describe, expect, it } from 'vitest';
import { levelModel } from './levelView';
import { DEFAULT_SCALE, MAX_SCALE, MIN_SCALE, ZOOM_PERCENTS, centerRange, clampCenter, paneUnits, clampLeft, clampScale, jumpPoints, panViews, paneView, scaleOfPercent, wheelAction, zoomPanes } from './viewState';

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

  it('der Zoom wird begrenzt, ohne Fehler', () => {
    expect(zoomPanes({ center: 450, chunk: 12, live: 12 }, ['chunk', 'live'], () => 9999, 900, 1000)).toMatchObject({ chunk: MAX_SCALE, live: MAX_SCALE });
    expect(zoomPanes({ center: 450, chunk: 12, live: 12 }, ['chunk'], () => 0, 900, 1000).chunk).toBe(MIN_SCALE);
  });
});

describe('Getrennter Zoom, gemeinsame Mitte (B-485/AC-05)', () => {
  const v = { center: 450, chunk: 10, live: 20 };
  const mid = (view: { left: number; scale: number }) => view.left + 1000 / view.scale / 2;

  it('beide Vorschauen zeigen dieselbe Weltposition in der Bildmitte, auch bei verschiedenem Zoom', () => {
    expect(paneView(v, 'chunk', 900, 1000)).toEqual({ left: 400, scale: 10 });
    expect(paneView(v, 'live', 900, 1000)).toEqual({ left: 425, scale: 20 });
    expect(mid(paneView(v, 'chunk', 900, 1000))).toBeCloseTo(mid(paneView(v, 'live', 900, 1000)));
  });

  it('zoomPanes ändert nur den Zoom der genannten Vorschau, die Mitte bleibt', () => {
    expect(zoomPanes(v, ['live'], (s) => s * 2, 900, 1000)).toEqual({ center: 450, chunk: 10, live: 40 });
    expect(zoomPanes(v, ['chunk', 'live'], () => 12, 900, 1000)).toEqual({ center: 450, chunk: 12, live: 12 });
  });

  it('Bildlauf verschiebt beide um dieselbe Welt-Strecke', () => {
    const next = panViews(v, 30, 900, 1000);
    expect(next.center).toBe(480);
    expect(paneView(next, 'chunk', 900, 1000).left - paneView(v, 'chunk', 900, 1000).left).toBe(30);
    expect(paneView(next, 'live', 900, 1000).left - paneView(v, 'live', 900, 1000).left).toBe(30);
  });

  it('die Mitte reicht so weit, dass die engste Ansicht an die Ränder kommt; die weitere bleibt am Rand stehen', () => {
    expect(centerRange({ chunk: 10, live: 20 }, 900, 1000)).toEqual([25, 875]); // halbe Ansicht der Live-Vorschau: 25 Units
    expect(clampCenter(0, { chunk: 10, live: 20 }, 900, 1000)).toBe(25);
    expect(clampCenter(5000, { chunk: 10, live: 20 }, 900, 1000)).toBe(875);
    expect(paneView({ center: 25, chunk: 10, live: 20 }, 'chunk', 900, 1000).left).toBe(0); // die weitere Ansicht hält am Rand
    expect(clampCenter(10, { chunk: 2, live: 2 }, 100, 1000)).toBe(50); // Welt schmaler als die Ansicht: Weltmitte
    expect(zoomPanes({ center: 50, chunk: 20, live: 20 }, ['chunk'], () => 10, 900, 1000).center).toBe(50);
    expect(zoomPanes({ center: 20, chunk: 20, live: 20 }, ['chunk', 'live'], () => 5, 900, 1000).center).toBe(100);
  });
});

describe('Jede Vorschau mit ihrer eigenen Breite (B-485/AC-05)', () => {
  const v = { center: 450, chunk: 10, live: 20 };
  const px = { chunk: 1000, live: 600 }; // Bühne schmaler als der Streifen

  it('Live-Bereich = Bildmitte ± (Breite der Bühne in px / Live-Zoom) / 2', () => {
    expect(paneUnits(v, 'live', px)).toBe(30);
    expect(paneView(v, 'live', 900, px)).toEqual({ left: 450 - 15, scale: 20 });
    expect(paneView(v, 'chunk', 900, px)).toEqual({ left: 450 - 50, scale: 10 });
  });

  it('die Bühne zentriert auf left + Breite der Bühne / Zoom / 2 (PreviewStage.applyView): das ist wieder die Mitte', () => {
    const live = paneView(v, 'live', 900, px);
    expect(live.left + px.live / live.scale / 2).toBe(v.center);
  });

  it('der Bereich der Mitte folgt der engsten Ansicht in Units', () => {
    expect(centerRange(v, 900, px)).toEqual([15, 885]);
    expect(centerRange({ chunk: 40, live: 10 }, 900, px)).toEqual([12.5, 887.5]);
  });
});

describe('Position (B-477/AC-01)', () => {
  it('clampLeft hält die Ansicht in der Welt; ist die Welt schmaler als die Ansicht, gilt 0', () => {
    expect(clampLeft(-5, 900, 100)).toBe(0);
    expect(clampLeft(5000, 900, 100)).toBe(800);
    expect(clampLeft(300, 900, 100)).toBe(300);
    expect(clampLeft(10, 50, 100)).toBe(0);
  });
});

describe('Mausrad (B-477/AC-01, AC-03)', () => {
  const wheel = (o: Partial<{ deltaX: number; deltaY: number; ctrlKey: boolean; shiftKey: boolean; altKey: boolean }>) => wheelAction({ deltaX: 0, deltaY: 0, ctrlKey: false, shiftKey: false, altKey: false, ...o });

  it('das gewöhnliche Rad bewirkt nichts (die Seite scrollt senkrecht)', () => {
    expect(wheel({ deltaY: 100 })).toBeNull();
  });

  it('Strg + Rad zoomt, Umschalt + Rad und seitliches Rad scrollen', () => {
    expect(wheel({ deltaY: -100, ctrlKey: true })).toEqual({ kind: 'zoom', factor: 1.25 });
    expect(wheel({ deltaY: 100, ctrlKey: true })).toEqual({ kind: 'zoom', factor: 0.8 });
    expect(wheel({ deltaY: 100, shiftKey: true })).toEqual({ kind: 'pan', px: 100 });
    expect(wheel({ deltaX: -60 })).toEqual({ kind: 'pan', px: -60 });
    expect(wheel({ ctrlKey: true })).toBeNull();
  });

  it('Alt + Rad zoomt nur die Vorschau unter dem Zeiger (B-485/AC-05)', () => {
    expect(wheel({ deltaY: -100, altKey: true })).toEqual({ kind: 'zoom', factor: 1.25, own: true });
    expect(wheel({ deltaY: 100, altKey: true })).toEqual({ kind: 'zoom', factor: 0.8, own: true });
    expect(wheel({ altKey: true })).toBeNull();
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
