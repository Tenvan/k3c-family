// @vitest-environment happy-dom
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { App } from './App';
import { ASSET_LABELS, ASSET_PAGES, LABELS, PAGES } from './Header';
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

describe('Bereiche Development und Assets (WB1/AC-01)', () => {
  const header = (html: string) => html.slice(html.indexOf('<header'), html.indexOf('</header>'));
  /** Beschriftung des gewählten Reiters. */
  const active = (head: string) => /aria-selected="true".*?TabsTriggerInner">([^<]+)</.exec(head)?.[1];

  it('Kopfleiste schaltet zwischen Development und Assets, kein Studio-Knopf', () => {
    savePref('app.area', 'development');
    savePref('app.page', 'dienste');
    const head = header(renderToStaticMarkup(<App />));
    expect(head).toMatch(/role="group" aria-label="Bereich"/);
    expect(head).toMatch(/aria-pressed="true"[^>]*>Development</);
    expect(head).toMatch(/aria-pressed="false"[^>]*>Assets</);
    expect(head).not.toContain('Asset-Studio');
  });

  it('unter Development sind alle bisherigen Seiten erreichbar', () => {
    savePref('app.area', 'development');
    for (const page of PAGES) {
      savePref('app.page', page);
      const head = header(renderToStaticMarkup(<App />));
      for (const p of PAGES) expect(head, `${page}: ${p}`).toContain(`>${LABELS[p].replace('&', '&amp;')}<`);
      expect(active(head)).toBe(LABELS[page].replace('&', '&amp;'));
    }
  });

  it('Assets startet mit dem Review und zeigt Katalog, Styleguide, Sound, Quellen', () => {
    savePref('app.area', 'assets');
    localStorage.removeItem('k3c-workbench.app.assetsPage');
    const html = renderToStaticMarkup(<App />);
    const head = header(html);
    for (const p of ASSET_PAGES) expect(head, p).toContain(`>${ASSET_LABELS[p]}<`);
    expect(active(head)).toBe('Review');
    expect(head).not.toContain(`>${LABELS.dienste}<`);
    expect(html).toContain('Asset-Review');
  });

  it('merkt die Seite je Bereich', () => {
    savePref('app.area', 'assets');
    savePref('app.assetsPage', 'styleguide');
    savePref('app.page', 'git');
    expect(active(header(renderToStaticMarkup(<App />)))).toBe('Styleguide');
    savePref('app.area', 'development');
    expect(active(header(renderToStaticMarkup(<App />)))).toBe('Git');
  });
});
