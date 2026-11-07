// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { ListDetail } from './ListDetail';

// WB2/AC-02, AC-04: Liste und Detail haben je einen eigenen Scrollbereich in voller Höhe; der Kopf scrollt nicht mit.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

// Vitest ersetzt CSS-Importe durch einen leeren Text; theme.css kommt deshalb von der Platte (wie review/testing.tsx).
const theme = readFileSync(resolve(__dirname, '../theme.css'), 'utf8').replace(/\/\*[\s\S]*?\*\//g, '');
/** Rumpf der Regel mit genau diesem Selektor. */
const rule = (selector: string) => new RegExp(`(^|\\n)${selector.replace(/[.-]/g, '\\$&')}\\s*\\{([^}]*)\\}`).exec(theme)?.[2] ?? '';

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

function mount(node: React.ReactNode): HTMLElement {
  const host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  act(() => root!.render(<Theme>{node}</Theme>));
  return host;
}

describe('ListDetail', () => {
  it('Kopf, Liste und Detail in getrennten Bereichen', () => {
    const host = mount(<ListDetail header={<h1>Kopf</h1>} list={<ul><li>a</li></ul>} listWidth={300}>Detail</ListDetail>);
    const page = host.querySelector<HTMLElement>('.wk-listdetail')!;
    const list = page.querySelector<HTMLElement>('[data-scroll="list"]')!;
    const detail = page.querySelector<HTMLElement>('[data-scroll="detail"]')!;
    expect(list.textContent).toBe('a');
    expect(detail.textContent).toBe('Detail');
    expect(list.contains(detail) || detail.contains(list)).toBe(false);
    expect(page.querySelector('.wk-listdetail-head')!.textContent).toBe('Kopf');
    expect(list.contains(page.querySelector('h1'))).toBe(false);
    expect(page.style.getPropertyValue('--wk-list-w')).toBe('300px');
  });

  it('theme.css: volle Höhe, je eigener Scrollbereich', () => {
    expect(rule('.wk-listdetail')).toMatch(/height:\s*calc\(100vh/);
    expect(rule('.wk-listdetail-body')).toMatch(/min-height:\s*0/);
    for (const part of ['.wk-listdetail-list', '.wk-listdetail-detail']) {
      expect(rule(part), part).toMatch(/overflow-y:\s*auto/);
      expect(rule(part), part).toMatch(/min-height:\s*0/);
    }
  });
});
