import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { App } from './App';

describe('Asset-Studio-Rahmen (B-459)', () => {
  it('zeigt Titel und die Navigation Grafik und Sound', () => {
    const html = renderToStaticMarkup(<App initial="grafik" />);
    expect(html).toContain('>Asset-Studio<');
    expect(html).toMatch(/role="tab"[^>]*aria-selected="true"[^>]*>.*?Grafik/);
    expect(html).toContain('Sound');
  });

  it('öffnet die gewählte Seite', () => {
    expect(renderToStaticMarkup(<App initial="sound" />)).toMatch(/aria-selected="true"[^>]*>.*?Sound/);
  });
});
