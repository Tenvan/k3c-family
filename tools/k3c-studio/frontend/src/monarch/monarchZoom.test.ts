import { describe, expect, it } from 'vitest';
import { ZOOM_MAX, ZOOM_MIN, stepZoom, wheelZoom } from './monarchZoom';

// B-502: Zoom der Monarch-Ansicht
describe('Zoom der Monarch-Ansicht', () => {
  it('Schritte vergrößern und verkleinern in den Grenzen', () => {
    expect(stepZoom(1, 1)).toBe(1.25);
    expect(stepZoom(1, -1)).toBe(0.8);
    expect(stepZoom(ZOOM_MAX, 1)).toBe(ZOOM_MAX);
    expect(stepZoom(ZOOM_MIN, -1)).toBe(ZOOM_MIN);
  });

  it('nur Alt + Rad zoomt, nach oben vergrößert', () => {
    expect(wheelZoom(1, { altKey: true, deltaY: -100 })).toBe(1.25);
    expect(wheelZoom(1, { altKey: true, deltaY: 100 })).toBe(0.8);
    expect(wheelZoom(1, { altKey: false, deltaY: -100 })).toBeNull();
    expect(wheelZoom(1, { altKey: true, deltaY: 0 })).toBeNull();
    expect(wheelZoom(1, { altKey: true, deltaY: 0, deltaX: -100 })).toBe(1.25); // seitliches Rad mit Alt
  });
});
