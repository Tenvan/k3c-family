import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { App, PAGES } from './App';

vi.mock('./api', () => ({ backend: { mock: true } }));

describe('Asset-Studio-Rahmen (B-459)', () => {
  it('zeigt Titel, die Navigation Grafik und Sound und den Mock-Hinweis', () => {
    const html = renderToStaticMarkup(<App initial="grafik" />);
    expect(html).toContain('>Asset-Studio<');
    expect(html).toMatch(/role="tab"[^>]*aria-selected="true"[^>]*>.*?Grafik/);
    expect(html).toContain('Sound');
    expect(html).toContain('Mock');
  });

  it('öffnet die gewählte Seite', () => {
    expect(renderToStaticMarkup(<App initial="sound" />)).toMatch(/aria-selected="true"[^>]*>.*?Sound/);
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
    expect(tabs).toEqual(['Grafik', 'Sound', 'Welten Editor', 'Gebäude-Planer', 'Monarch', 'Gegner', 'Quellen']);
  });

  it('bietet die Seite Gebäude-Planer an (B-462) und öffnet sie', () => {
    const html = renderToStaticMarkup(<App initial="planer" />);
    expect(html).toMatch(/aria-selected="true"[^>]*>.*?Gebäude-Planer/);
    expect(html).toContain('class="pl-page"');
  });

  it('bietet die Seite Quellen an (WZ30.4/AC-04) und öffnet sie', () => {
    const html = renderToStaticMarkup(<App initial="quellen" />);
    expect(html).toMatch(/aria-selected="true"[^>]*>.*?Quellen/);
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
