import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { App } from './App';

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

  it('bietet die Seite Welt-Betrachter an (B-460) und öffnet sie', () => {
    const html = renderToStaticMarkup(<App initial="welt" />);
    expect(html).toMatch(/aria-selected="true"[^>]*>.*?Welt-Betrachter/);
    expect(html).toContain('data-maximized="true"');
  });

  it('bietet die Seite Monarch an (B-463) und öffnet sie', () => {
    const html = renderToStaticMarkup(<App initial="monarch" />);
    expect(html).toMatch(/aria-selected="true"[^>]*>.*?Monarch/);
    expect(html).toContain('Reittier');
    const tabs = [...html.matchAll(/role="tab"[^>]*>(?:<[^>]+>)*([^<]+)/g)].map((m) => m[1]);
    expect(tabs).toEqual(['Grafik', 'Sound', 'Welt-Betrachter', 'Gebäude-Planer', 'Monarch', 'Gegner']);
  });

  it('bietet die Seite Gebäude-Planer an (B-462) und öffnet sie', () => {
    const html = renderToStaticMarkup(<App initial="planer" />);
    expect(html).toMatch(/aria-selected="true"[^>]*>.*?Gebäude-Planer/);
    expect(html).toContain('class="pl-page"');
  });
});
