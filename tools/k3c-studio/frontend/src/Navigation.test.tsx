// @vitest-environment happy-dom
import { act, type ReactElement } from 'react';
import { createRoot } from 'react-dom/client';
import { describe, expect, it } from 'vitest';
import { navigateTo } from '@webkit/lib/navigate';
import { AppFrame } from '@webkit/templates';
import { Tabs } from '@webkit/organisms';
import { LABELS, PAGES, useNavigation } from './Header';
import { savePref } from './lib/prefs';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

function Probe() {
  const nav = useNavigation();
  return <p data-nav>{nav.page}</p>;
}

/** Rahmen wie App.tsx, aber mit Platzhaltern statt der Seiten: die Reiter der Kopfzeile schalten den Inhalt. */
function Frame() {
  const nav = useNavigation();
  return (
    <AppFrame title="K3C Studio" appearance="dark" tabs={nav.tabs}>
      {PAGES.map((p) => (
        <Tabs.Content key={p} value={p}>
          <p data-page>{p}</p>
        </Tabs.Content>
      ))}
    </AppFrame>
  );
}

function render(el: ReactElement) {
  const host = document.createElement('div');
  document.body.appendChild(host);
  const root = createRoot(host);
  act(() => root.render(el));
  return { host, root };
}

describe('Reiter der Kopfzeile', () => {
  it('sieben Reiter in der alten Reihenfolge, jede Seite erreichbar und gemerkt', () => {
    localStorage.clear();
    const { host, root } = render(<Frame />);
    expect(PAGES.map((p) => LABELS[p])).toEqual(['Styleguide', 'Katalog', 'Review', 'Sound & Quellen', 'Welten-Editor', 'Gebäude-Planer', 'Balancing']);
    const header = host.ownerDocument.querySelector('header')!;
    const tabs = [...header.querySelectorAll('[role="tab"]')];
    expect(tabs.map((t) => t.firstElementChild?.textContent ?? t.textContent)).toEqual(PAGES.map((p) => LABELS[p]));
    // Startseite ist Review
    expect(host.ownerDocument.querySelector('main [data-page]')?.textContent).toBe('review');
    for (const [i, p] of PAGES.entries()) {
      // Radix wählt einen Reiter bei mousedown (nicht bei click)
      act(() => tabs[i]!.dispatchEvent(new MouseEvent('mousedown', { bubbles: true, button: 0 })));
      expect(tabs[i]!.getAttribute('aria-selected'), p).toBe('true');
      expect(host.ownerDocument.querySelector('main [data-page]')?.textContent, p).toBe(p);
      expect(localStorage.getItem('k3c-studio.app.page'), p).toContain(p);
    }
    act(() => root.unmount());
  });
});

// WZ62.4: Eine Seite bittet per navigateTo um eine andere Seite; Fremdes wird ignoriert, alte Bereichsnamen sind Aliase.
describe('Seitenwechsel auf Wunsch (B-1171)', () => {
  it('wechselt zu Katalog, Planer, Balancing, Aliase hub und assets, ignoriert unbekannte Ziele', async () => {
    savePref('app.page', 'welten');
    const { host, root } = render(<Probe />);
    expect(host.textContent).toBe('welten');
    await act(async () => navigateTo('gibt-es-nicht'));
    expect(host.textContent).toBe('welten');
    for (const [ziel, erwartet] of [['katalog', 'katalog'], ['planer', 'planer'], ['balancing', 'balancing'], ['hub', 'planer'], ['assets', 'review'], ['sound', 'sound']]) {
      await act(async () => navigateTo(ziel!));
      expect(host.textContent, ziel).toBe(erwartet);
    }
    act(() => root.unmount());
  });
});
