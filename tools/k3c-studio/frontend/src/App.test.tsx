// @vitest-environment happy-dom
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { App } from './App';
import { AREA_LABELS, AREA_PAGES, AREAS, LABELS } from './Header';
import { savePref } from './lib/prefs';

vi.mock('./api', () => ({ backend: { mock: true, on: () => () => undefined, info: () => new Promise(() => undefined) } }));

// WZ63.3/AC-03: Studio-Rahmen mit drei Bereichen; die Seiten sind die der früheren Workbench-Bereiche.
describe('Studio-Rahmen (WZ63.3)', () => {
  const header = (html: string) => html.slice(html.indexOf('<header'), html.indexOf('</header>'));
  /** Beschriftung des gewählten Reiters. */
  const active = (head: string) => /aria-selected="true".*?TabsTriggerInner">([^<]+)</.exec(head)?.[1];
  const label = (s: string) => `>${s.replace('&', '&amp;')}<`;

  it('Kopfleiste: Titel, Bereiche Welten, Assets, Hub, kein Ticket-Knopf', () => {
    savePref('app.area', 'welten');
    const head = header(renderToStaticMarkup(<App />));
    expect(head).toContain('K3C Studio');
    expect(head).toMatch(/role="group" aria-label="Bereich"/);
    for (const a of AREAS) expect(head, a).toContain(label(AREA_LABELS[a]));
    expect(head).toMatch(/aria-pressed="true"[^>]*>Welten</);
    expect(head).not.toMatch(/<button[^>]*>Ticket<\/button>/);
  });

  it('jeder Bereich zeigt seine Reiter; Assets startet mit dem Review', () => {
    localStorage.removeItem('k3c-studio.app.assetsPage');
    for (const a of AREAS) {
      savePref('app.area', a);
      const head = header(renderToStaticMarkup(<App />));
      for (const p of AREA_PAGES[a]) expect(head, `${a}: ${p}`).toContain(label(LABELS[p]));
      expect(active(head), a).toBe({ welten: 'Welten-Editor', assets: 'Review', hub: 'Gebäude-Planer' }[a]);
    }
  });

  it('merkt die Seite der Assets', () => {
    savePref('app.area', 'assets');
    savePref('app.assetsPage', 'styleguide');
    expect(active(header(renderToStaticMarkup(<App />)))).toBe('Styleguide');
  });
});
