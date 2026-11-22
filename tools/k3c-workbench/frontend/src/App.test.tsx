// @vitest-environment happy-dom
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { App } from './App';
import { LABELS, PAGES } from './Header';
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

describe('Kopfzeile in schmalen Fenstern (B-668/AC-02)', () => {
  it('Reiter liegen in einem eigenen Scroll-Container, die rechte Leiste bricht um', () => {
    savePref('app.page', 'dienste');
    const html = renderToStaticMarkup(<App />);
    expect(html.slice(html.indexOf('<header'), html.indexOf('</header>'))).toContain('wk-header-tabs');
    const css = readFileSync(resolve(__dirname, '../../../webkit/theme.css'), 'utf-8');
    expect(css).toMatch(/\.wk-header-tabs\s*\{[^}]*min-width:\s*0;[^}]*overflow-x:\s*auto/);
    expect(css).toMatch(/\.wk-header-right\s*\{[^}]*flex-wrap:\s*wrap/);
  });
});

describe('Seiten der Workbench (WB1, WZ63.3)', () => {
  const header = (html: string) => html.slice(html.indexOf('<header'), html.indexOf('</header>'));
  /** Beschriftung des gewählten Reiters. */
  const active = (head: string) => /aria-selected="true".*?TabsTriggerInner">([^<]+)</.exec(head)?.[1];

  it('alle Seiten samt Balancing in einer Reiterleiste, ohne Bereichsumschalter und Studio-Seiten', () => {
    for (const page of PAGES) {
      savePref('app.page', page);
      const head = header(renderToStaticMarkup(<App />));
      for (const p of PAGES) expect(head, `${page}: ${p}`).toContain(`>${LABELS[p].replace('&', '&amp;')}<`);
      expect(active(head)).toBe(LABELS[page].replace('&', '&amp;'));
      expect(head).not.toMatch(/aria-label="Bereich"/);
      for (const studio of ['Welten-Editor', 'Gebäude-Planer', 'Katalog', 'Styleguide']) expect(head).not.toContain(`>${studio}<`);
    }
  });
});
