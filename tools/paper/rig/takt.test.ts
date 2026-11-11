import { describe, expect, it } from 'vitest';
import type { Animation, Spec } from '../schema/types';
import { ausgabeFps } from './takt';

const spec = (rig: string) => ({ rig }) as unknown as Spec;
const anim = (a: Partial<Animation>) => ({ frames: 8, ...a }) as Animation;

describe('ausgabeFps', () => {
  it('Gangarten der Vierbeiner: Frequenz × Frames', () => {
    const pferd = spec('vierbeiner');
    expect(ausgabeFps(pferd, 'schritt', anim({ gangart: 'schritt' }), 8)).toBe(7.2);
    expect(ausgabeFps(pferd, 'trab', anim({ gangart: 'trab' }), 8)).toBe(12);
    expect(ausgabeFps(pferd, 'galopp', anim({ gangart: 'galopp' }), 8)).toBe(16.8);
    expect(ausgabeFps(pferd, 'vorn-trab', anim({ gangart: 'trab' }), 8)).toBe(12);
  });
  it('Reiter folgt der Gangart seines Reittiers', () => {
    expect(ausgabeFps(spec('figur'), 'galopp-einhorn', anim({ gangart: 'galopp' }), 8)).toBe(16.8);
    expect(ausgabeFps(spec('figur'), 'reiten-idle', anim({ gangart: 'stand' }), 8)).toBe(4);
  });
  it('Posen der Figur: lauf 1,6 Hz, idle 0,5 Hz', () => {
    expect(ausgabeFps(spec('figur'), 'lauf', anim({}), 8)).toBe(12.8);
    expect(ausgabeFps(spec('figur'), 'idle', anim({}), 8)).toBe(4);
  });
  it('fps der Spec gilt vor der Frequenz', () => {
    expect(ausgabeFps(spec('vierbeiner'), 'lauf', anim({ gangart: 'trab', fps: 14 }), 8)).toBe(14);
  });
  it('Keyframe-Animationen und Einzelbilder bleiben bei 8', () => {
    expect(ausgabeFps(spec('figur'), 'death', anim({}), 8)).toBe(8);
    expect(ausgabeFps(spec('gebaeude'), 'idle', anim({}), 8)).toBe(8);
    expect(ausgabeFps(spec('vierbeiner'), 'stand', anim({ gangart: 'stand', frames: 1 }), 1)).toBe(8);
  });
});
