// @vitest-environment happy-dom
import { act, type ReactElement } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { BarChart } from './BarChart';
import { CorridorBar } from './CorridorBar';
import { LineChart } from './LineChart';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

function render(el: ReactElement): HTMLElement {
  act(() => root?.unmount());
  const host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  act(() => root!.render(el));
  return host;
}

describe('LineChart', () => {
  it('zeichnet Serien mit role=img und Band', () => {
    const h = render(<LineChart xLabel="Nacht" yLabel="Überleben" band={{ lower: 0.8 }}
      series={[{ name: 'A', points: [{ x: 1, y: 0.9 }, { x: 2, y: 0.7 }] }, { name: 'B', points: [{ x: 1, y: 1 }, { x: 2, y: 0.95 }] }]} />);
    const svg = h.querySelector('svg')!;
    expect(svg.getAttribute('role')).toBe('img');
    expect(svg.getAttribute('aria-label')).toContain('Überleben');
    expect(h.querySelectorAll('path[data-series]')).toHaveLength(2);
    expect(h.querySelector('[data-testid=band]')).not.toBeNull();
  });
  it('leere Serien rendern ohne Fehler', () => {
    const h = render(<LineChart xLabel="x" yLabel="y" series={[]} />);
    expect(h.querySelector('svg')!.getAttribute('aria-label')).toContain('keine Daten');
    expect(h.querySelector('path')).toBeNull();
  });
  it('nur obere Grenze', () => {
    const h = render(<LineChart xLabel="x" yLabel="y" band={{ upper: 5 }} series={[{ name: 'A', points: [{ x: 0, y: 1 }, { x: 1, y: 3 }] }]} />);
    expect(h.querySelectorAll('[data-testid=band] line')).toHaveLength(1);
  });
});

describe('BarChart', () => {
  it('Balken mit Ton und Einheit', () => {
    const h = render(<BarChart unit="Gold" bars={[{ label: 'N1', value: 3 }, { label: 'N2', value: -2, tone: 'fail' }, { label: 'N3', value: 1, tone: 'warn' }]} band={{ lower: 0 }} />);
    expect(h.querySelectorAll('rect[data-tone]')).toHaveLength(3);
    expect(h.querySelector('rect[data-tone=fail]')).not.toBeNull();
    expect(h.querySelector('svg')!.getAttribute('aria-label')).toContain('Gold');
  });
  it('leer', () => {
    const h = render(<BarChart bars={[]} />);
    expect(h.querySelectorAll('rect[data-tone]')).toHaveLength(0);
  });
});

describe('CorridorBar', () => {
  const pass = (h: HTMLElement) => h.querySelector('svg')!.getAttribute('data-pass');
  it('Pass im Korridor, Fail außerhalb', () => {
    expect(pass(render(<CorridorBar value={5} lower={3} upper={8} />))).toBe('true');
    expect(pass(render(<CorridorBar value={9} lower={3} upper={8} />))).toBe('false');
  });
  it('nur obere bzw. untere Grenze', () => {
    expect(pass(render(<CorridorBar value={2} upper={3} unit="s" />))).toBe('true');
    const h = render(<CorridorBar value={2} lower={3} />);
    expect(pass(h)).toBe('false');
    expect(h.querySelector('svg')!.getAttribute('aria-label')).toContain('ab 3');
  });
});
