import { describe, expect, it } from 'vitest';
import { CULL_MARGIN, cullMask } from './cameraCull';

// PF1.2 (B-194): Container außerhalb des Ausschnitts einer Kamera werden in ihr nicht gezeichnet, je Kamera getrennt.
describe('cullMask', () => {
  const views = [
    { id: 1, left: 1000, right: 3000 },
    { id: 2, left: 8000, right: 10000 },
  ];

  it('im Ausschnitt oder im Rand: in dieser Kamera sichtbar', () => {
    expect(cullMask(2000, views) & 1).toBe(0);
    expect(cullMask(3000 + CULL_MARGIN, views) & 1).toBe(0);
    expect(cullMask(1000 - CULL_MARGIN, views) & 1).toBe(0);
  });

  it('je Kamera getrennt: in der einen sichtbar, in der anderen ausgeblendet', () => {
    expect(cullMask(2000, views)).toBe(2);
    expect(cullMask(9000, views)).toBe(1);
  });

  it('außerhalb beider Ausschnitte: in keiner Kamera gezeichnet', () => {
    expect(cullMask(5500, views)).toBe(3);
  });

  it('ohne Kamera: nichts ausgeblendet', () => {
    expect(cullMask(5500, [])).toBe(0);
  });
});
