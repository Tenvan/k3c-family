// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { WorldPage } from './WorldPage';
import { forgetMode } from './worldTesting';

// B-690/AC-03 (B-665/AC-01) im DOM: Im Modus Stufe bearbeiten gibt es keine Steuerzeile über der Chunk-Vorschau; Hub-Stufe,
// Tageszeit, Zoom und „Springe zu“ stehen in einer Werkzeugleiste direkt über der Live-Vorschau. Die Bühne ist eine Attrappe.
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class { show() {} setPosition() {} setLook() {} settle() {} destroy() {} } }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

async function mount(): Promise<HTMLElement> {
  forgetMode();
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  return host;
}

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
});

describe('Werkzeugleiste über der Live-Vorschau (B-690/AC-03, B-665/AC-01)', () => {
  it('steht zwischen Chunk-Vorschau und Live-Vorschau und trägt Hub-Stufe, Tageszeit, Zoom und „Springe zu“', async () => {
    const h = await mount();
    const bars = h.querySelectorAll('[role="toolbar"]');
    expect(bars).toHaveLength(1);
    const bar = bars[0] as HTMLElement;
    expect(bar.getAttribute('aria-label')).toBe('Ansicht der Bühne');
    expect(bar.previousElementSibling!.matches('.wd-chunks')).toBe(true);
    expect(bar.nextElementSibling!.matches('.wd-live')).toBe(true);
    for (const label of ['Hub-Stufe', 'Tageszeit', 'Zoom']) expect(bar.querySelector(`[aria-label="${label}"]`), label).not.toBeNull();
    expect(bar.textContent).toContain('Springe zu');
  });

  it('über der Chunk-Vorschau steht keine Steuerzeile', async () => {
    const h = await mount();
    const main = h.querySelector('.wd-stage-main')!;
    expect(main.firstElementChild!.matches('.wd-chunks')).toBe(true);
    expect(h.querySelector('.wd-chunks')!.querySelector('[role="toolbar"], [aria-label="Hub-Stufe"], [aria-label="Tageszeit"]')).toBeNull();
  });
});
