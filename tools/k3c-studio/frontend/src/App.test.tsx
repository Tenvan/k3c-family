// @vitest-environment happy-dom
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { App } from './App';
import { LABELS, PAGES } from './Header';

vi.mock('./api', () => ({ backend: { mock: true, on: () => () => undefined, info: () => new Promise(() => undefined) } }));

// WZ63.3/AC-03: Studio-Rahmen; die sieben Seiten stehen als eine flache Reiterleiste in der Kopfzeile wie früher in der Workbench.
describe('Studio-Rahmen (WZ63.3)', () => {
  const header = (html: string) => html.slice(html.indexOf('<header'), html.indexOf('</header>'));
  /** Beschriftungen der gewählten Reiter in Reihenfolge. */
  const active = (html: string) => [...html.matchAll(/aria-selected="true".*?TabsTriggerInner">([^<]+)</g)].map((m) => m[1]);
  const label = (s: string) => `>${s.replace('&', '&amp;')}<`;

  it('Kopfzeile: Titel, sieben Reiter in alter Reihenfolge, Dark-Schalter, kein Bereichsumschalter, kein Ticket-Knopf', () => {
    localStorage.removeItem('k3c-studio.app.page');
    const head = header(renderToStaticMarkup(<App />));
    expect(head).toContain('K3C Studio');
    expect(head).toContain('Dark');
    expect(head).not.toMatch(/aria-label="Bereich"/);
    expect(head).not.toMatch(/<button[^>]*>Ticket<\/button>/);
    expect(head.match(/role="tab"/g)).toHaveLength(7);
    const positions = PAGES.map((p) => head.indexOf(label(LABELS[p])));
    expect(positions.every((i) => i >= 0)).toBe(true);
    expect(positions).toEqual([...positions].sort((a, b) => a - b));
    expect(active(head)).toEqual(['Review']); // Startseite
  });
});
