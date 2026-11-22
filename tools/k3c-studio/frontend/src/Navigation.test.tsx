// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { describe, expect, it } from 'vitest';
import { navigateTo } from '@webkit/lib/navigate';
import { useNavigation } from './Header';
import { savePref } from './lib/prefs';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

function Probe() {
  const nav = useNavigation();
  return <p data-nav>{`${nav.area}:${nav.page}`}</p>;
}

// WZ62.4: Eine Seite bittet per navigateTo um eine andere Seite; Bereich und Reiter wechseln, Fremdes wird ignoriert.
describe('Seitenwechsel auf Wunsch (B-1171)', () => {
  it('wechselt vom Welten-Editor in den Bereich Assets auf den Katalog, ignoriert unbekannte Ziele', async () => {
    savePref('app.area', 'welten');
    savePref('app.assetsPage', 'styleguide');
    const host = document.createElement('div');
    document.body.appendChild(host);
    const root = createRoot(host);
    act(() => root.render(<Probe />));
    expect(host.textContent).toBe('welten:welten');
    await act(async () => navigateTo('gibt-es-nicht'));
    expect(host.textContent).toBe('welten:welten');
    await act(async () => navigateTo('katalog'));
    expect(host.textContent).toBe('assets:katalog');
    await act(async () => navigateTo('planer'));
    expect(host.textContent).toBe('hub:planer');
    act(() => root.unmount());
  });
});
