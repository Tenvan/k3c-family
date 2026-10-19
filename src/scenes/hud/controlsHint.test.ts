import { describe, expect, it } from 'vitest';
import { CONTROLS_HINT_MS, stepControlsHint, type ControlsTrack } from './controlsHint';

const run = (prev: ControlsTrack | null, now: number, seats = 1, device = 'keyboard', guide = true) =>
  stepControlsHint(prev, now, { seats, device, guide });

describe('stepControlsHint (B-399)', () => {
  it('steht die feste Zeit seit dem Beitritt und blendet danach aus', () => {
    let r = run(null, 1000);
    expect(r.visible).toBe(true);
    r = run(r.track, 1000 + CONTROLS_HINT_MS - 1);
    expect(r.visible).toBe(true);
    r = run(r.track, 1000 + CONTROLS_HINT_MS);
    expect(r.visible).toBe(false);
  });

  it('bleibt aus, solange nichts Neues passiert', () => {
    let r = run(null, 0);
    r = run(r.track, CONTROLS_HINT_MS + 5000);
    r = run(r.track, CONTROLS_HINT_MS + 60000);
    expect(r.visible).toBe(false);
  });

  it('ein neuer Beitritt zeigt ihn erneut für die feste Zeit', () => {
    let r = run(null, 0);
    r = run(r.track, CONTROLS_HINT_MS + 1000);
    r = run(r.track, CONTROLS_HINT_MS + 2000, 2);
    expect(r.visible).toBe(true);
    r = run(r.track, CONTROLS_HINT_MS + 2000 + CONTROLS_HINT_MS, 2);
    expect(r.visible).toBe(false);
  });

  it('ein Gerätewechsel startet die Zeit neu', () => {
    let r = run(null, 0, 1, 'keyboard');
    r = run(r.track, CONTROLS_HINT_MS + 1, 1, 'keyboard');
    expect(r.visible).toBe(false);
    r = run(r.track, CONTROLS_HINT_MS + 2, 1, 'pad');
    expect(r.visible).toBe(true);
  });

  it('mit zwei Spielern startet der ständige Gerätewechsel die Zeit nicht neu', () => {
    let r = run(null, 0, 2, 'keyboard');
    r = run(r.track, CONTROLS_HINT_MS + 1, 2, 'keyboard2');
    expect(r.visible).toBe(false);
    r = run(r.track, CONTROLS_HINT_MS + 2, 2, 'keyboard');
    expect(r.visible).toBe(false);
  });

  it('„Hinweise“ wieder einschalten zeigt ihn erneut', () => {
    let r = run(null, 0, 1, 'keyboard', false);
    r = run(r.track, CONTROLS_HINT_MS + 1, 1, 'keyboard', false);
    expect(r.visible).toBe(false);
    r = run(r.track, CONTROLS_HINT_MS + 2, 1, 'keyboard', true);
    expect(r.visible).toBe(true);
  });

  it('ein Abmelden (weniger Plätze) zeigt ihn nicht erneut', () => {
    let r = run(null, 0, 2);
    r = run(r.track, CONTROLS_HINT_MS + 1, 2);
    r = run(r.track, CONTROLS_HINT_MS + 2, 1);
    expect(r.visible).toBe(false);
  });
});
