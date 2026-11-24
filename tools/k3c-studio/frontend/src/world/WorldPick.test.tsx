// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import type { PicksView } from '../api';
import { PickSelects } from './WorldPick';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
let root: Root | null = null;
afterEach(() => { act(() => root?.unmount()); root = null; document.body.innerHTML = ''; });

const picks: PicksView = {
  values: { weather: 'taifun', mount: '', hermit: 'altesAlt' },
  options: { weather: ['taifun', 'flut'], mount: ['camel'], hermit: ['swordsmith'] },
  error: '',
};

// Die drei Auswahlfelder je Art; „keins“ steht vorn, ein unbekannter Bestandswert ist markiert.
describe('Wetter, Reittier, Einsiedler je Welt (SI7.1)', () => {
  it('zeigt je Art ein Feld mit keins, den gültigen IDs und markiert einen unbekannten Bestandswert', () => {
    const host = document.createElement('div');
    document.body.appendChild(host);
    root = createRoot(host);
    act(() => root!.render(<Theme><PickSelects picks={picks} onChange={() => undefined} /></Theme>));
    const labels = [...host.querySelectorAll('[aria-label$="wählen"]')].map((el) => el.getAttribute('aria-label'));
    expect(labels).toEqual(['Wetter wählen', 'Reittier wählen', 'Einsiedler wählen']);
    expect(host.textContent).toContain('keins');
    expect(host.textContent).toContain('altesAlt (unbekannt)');
  });
});
