import { Theme } from '@radix-ui/themes';
import type { ReactElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { Splitter } from '@webkit/atoms';
import { clampSplit, splitShare } from '@webkit/lib/split';

vi.mock('../api', () => ({ backend: {} }));

const render = (el: ReactElement) => renderToStaticMarkup(<Theme>{el}</Theme>);

describe('Splitter der Planungsseite (B-400)', () => {
  it('AC-01: waagerechter Balken mit Anteil, Grenzen und Klasse', () => {
    const html = render(<Splitter horizontal className="pl-split" value={35} onChange={() => {}} onReset={() => {}} />);
    expect(html).toContain('role="separator"');
    expect(html).toContain('aria-orientation="horizontal"');
    expect(html).toContain('aria-valuenow="35"');
    expect(html).toContain('class="wk-split wk-split-h pl-split"');
  });

  it('AC-01: der Balken der Tasks-Seite bleibt senkrecht', () => {
    const html = render(<Splitter className="tk-split" value={40} onChange={() => {}} onReset={() => {}} />);
    expect(html).toContain('aria-orientation="vertical"');
    expect(html).toContain('class="wk-split wk-split-v tk-split"');
  });

  it('AC-06: senkrecht misst den Anteil von der linken Kante, waagerecht von unten', () => {
    const box = { left: 100, bottom: 500, width: 800, height: 400 };
    expect(splitShare(false, box, 300, 0)).toBe(25); // (300 − 100) / 800
    expect(splitShare(true, box, 0, 300)).toBe(50); // (500 − 300) / 400
  });

  it('AC-06: der Anteil wird auf 20–80 % geklemmt und gerundet', () => {
    expect([clampSplit(5), clampSplit(95), clampSplit(37.4), clampSplit(37.6)]).toEqual([20, 80, 37, 38]);
  });
});
