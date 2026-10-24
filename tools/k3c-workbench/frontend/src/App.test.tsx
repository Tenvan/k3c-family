// @vitest-environment happy-dom
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { App } from './App';
import { PAGES } from './Header';
import { savePref } from './lib/prefs';

vi.mock('./api', () => ({ backend: { mock: true, on: () => () => undefined, info: () => new Promise(() => undefined) } }));

describe('Workbench-Rahmen (WZ28.3/AC-03)', () => {
  it('zeigt den Ticket-Knopf in der Kopfzeile jeder Seite', () => {
    for (const page of PAGES) {
      savePref('app.page', page);
      const html = renderToStaticMarkup(<App />);
      const header = html.slice(html.indexOf('<header'), html.indexOf('</header>'));
      expect(header, page).toMatch(/<button[^>]*>Ticket<\/button>/);
    }
  });
});
