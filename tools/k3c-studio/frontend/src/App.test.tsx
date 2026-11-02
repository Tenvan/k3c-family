import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { App, PAGES } from './App';

vi.mock('./api', () => ({ backend: { mock: true } }));

describe('Asset-Studio-Rahmen (B-459)', () => {
  it('zeigt Titel, die Seite Assets (WZ33) und den Mock-Hinweis', () => {
    const html = renderToStaticMarkup(<App initial="assets" />);
    expect(html).toContain('>Asset-Studio<');
    expect(html).toMatch(/role="tab"[^>]*aria-selected="true"[^>]*>.*?Assets/);
    expect(html).toContain('Mock');
  });

  it('kennt die alten Seiten Grafik, Sound und Quellen nicht mehr (WZ33/AC-01)', () => {
    expect(PAGES).toEqual(['assets', 'welt', 'planer', 'monarch', 'gegner']);
  });

  it('bietet die Seite Welten Editor an (B-460) und öffnet sie', () => {
    const html = renderToStaticMarkup(<App initial="welt" />);
    expect(html).toMatch(/aria-selected="true"[^>]*>.*?Welten Editor/);
    expect(html).toContain('data-maximized="true"');
  });

  it('bietet die Seite Monarch an (B-463) und öffnet sie', () => {
    const html = renderToStaticMarkup(<App initial="monarch" />);
    expect(html).toMatch(/aria-selected="true"[^>]*>.*?Monarch/);
    expect(html).toContain('Reittier');
    const tabs = [...html.matchAll(/role="tab"[^>]*>(?:<[^>]+>)*([^<]+)/g)].map((m) => m[1]);
    expect(tabs).toEqual(['Assets', 'Welten Editor', 'Gebäude-Planer', 'Monarch', 'Gegner']);
  });

  it('bietet die Seite Gebäude-Planer an (B-462) und öffnet sie', () => {
    const html = renderToStaticMarkup(<App initial="planer" />);
    expect(html).toMatch(/aria-selected="true"[^>]*>.*?Gebäude-Planer/);
    expect(html).toContain('class="pl-page"');
  });

  it('zeigt den Chat rechts auf jeder Seite mit Knopf zum Ein- und Ausblenden (WZ19.2/AC-03)', () => {
    for (const page of PAGES) {
      const html = renderToStaticMarkup(<App initial={page} />);
      expect(html, page).toContain('data-testid="chat-panel"');
      expect(html, page).toContain('data-testid="chat-toggle"');
    }
  });

  it('zeigt den Ticket-Knopf in der Kopfzeile jeder Seite (WZ28.3/AC-03)', () => {
    for (const page of PAGES) {
      const html = renderToStaticMarkup(<App initial={page} />);
      const header = html.slice(html.indexOf('<header'), html.indexOf('</header>'));
      expect(header, page).toMatch(/<button[^>]*>Ticket<\/button>/);
    }
  });
});
