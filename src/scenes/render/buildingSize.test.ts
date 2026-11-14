import { describe, expect, it } from 'vitest';
import { buildingFit, buildingSource } from './buildingSize';
import { BUILDING_TEXTURES } from './buildingSprites';
import { crispScale, fitTarget, validTarget } from './targetFit';

// B-685: Zielmaß je Gebäude und Stufe, die Grafik wird eingepasst.
describe('Zielmaß einpassen (B-685)', () => {
  it('AC-01: Quellen mit 32 und 128 px belegen dasselbe Zielmaß, die 128er feiner', () => {
    const ziel = { w: 8, h: 9 };
    const grob = fitTarget({ w: 256, h: 288 }, ziel);
    const fein = fitTarget({ w: 1024, h: 1152 }, ziel);
    expect([grob.w, grob.h]).toEqual([256, 288]);
    expect([fein.w, fein.h]).toEqual([256, 288]);
    expect(grob).toMatchObject({ scale: 1, smooth: false });
    expect(fein).toMatchObject({ scale: 0.25, smooth: true });
    // Beispiel der Spec: 300×200 px Zielfläche, 32- und 128-px-Quelle gleichen Seitenverhältnisses
    const t = { w: 300 / 32, h: 200 / 32 };
    expect(fitTarget({ w: 96, h: 64 }, t)).toMatchObject({ w: 300, h: 200 });
    expect(fitTarget({ w: 384, h: 256 }, t)).toMatchObject({ w: 300, h: 200 });
  });

  it('einpassen ohne Verzerrung: anderes Seitenverhältnis füllt eine Seite, die andere bleibt kleiner', () => {
    const fit = fitTarget({ w: 64, h: 64 }, { w: 4, h: 2 });
    expect(fit).toMatchObject({ scale: 1, w: 64, h: 64 });
    expect(fitTarget({ w: 32, h: 16 }, { w: 2, h: 2 })).toMatchObject({ scale: 2, w: 64, h: 32, smooth: false });
  });

  it('Zielmaß 0, negativ oder fehlend: heutiger Maßstab', () => {
    for (const t of [null, undefined, { w: 0, h: 3 }, { w: -1, h: 2 }, { w: 'x', h: 2 }, { w: Number.NaN, h: 1 }]) {
      expect(validTarget(t)).toBeNull();
      expect(fitTarget({ w: 16, h: 16 }, t, 2)).toMatchObject({ scale: 2, w: 32, h: 32 });
    }
  });

  it('scharf nur bei ganzzahliger Vergrößerung', () => {
    expect([1, 2, 3].map(crispScale)).toEqual([true, true, true]);
    expect([0.5, 1.5, 0.25].map(crispScale)).toEqual([false, false, false]);
  });
});

describe('Gebäude-Darstellung (CU1.2)', () => {
  it('jeder Schlüssel hat eine Datei; Papier: Maßstab 1, weich', () => {
    for (const key of Object.keys(BUILDING_TEXTURES)) {
      expect(buildingSource(key)?.file, key).toBe(BUILDING_TEXTURES[key]);
      expect(buildingFit(key, { w: 96, h: 160 }), key).toMatchObject({ scale: 1, w: 96, h: 160, smooth: true });
    }
    expect(buildingSource('grafik:gibtsnicht')).toBeNull();
  });
});
