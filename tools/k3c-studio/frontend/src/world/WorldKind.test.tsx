// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { KindBadge } from './WorldKind';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
let root: Root | null = null;
afterEach(() => { act(() => root?.unmount()); root = null; document.body.innerHTML = ''; });

describe('Art der Welt (B-1217)', () => {
  it('die Marke nennt Ableger oder Neu und fehlt ohne Art', () => {
    const host = document.createElement('div');
    document.body.appendChild(host);
    root = createRoot(host);
    act(() => root!.render(<Theme><KindBadge kind="spinoff" /><KindBadge kind="new" /><KindBadge /></Theme>));
    expect([...host.querySelectorAll('[data-kind]')].map((b) => [b.getAttribute('data-kind'), b.textContent])).toEqual([['spinoff', 'Ableger'], ['new', 'Neu']]);
  });
});
