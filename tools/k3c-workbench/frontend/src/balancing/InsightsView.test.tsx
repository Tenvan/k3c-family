// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { exampleSummary } from '../api/mockBalance';
import { ReportView } from './ReportView';

// B-632/AC-05: Der Bericht zeigt Kurzauswertung, Ampeln, Hinweise und „Nächster Lauf“ und öffnet den HTML-Bericht (Mock-Backend).

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
const flush = async () => {
  for (let i = 0; i < 5; i++) await act(async () => new Promise((r) => setTimeout(r, 0)));
};
const PATH = 'reports/simtest-run-2/balance-2.json';

async function mount(): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><ReportView name={PATH} summary={exampleSummary(0.15, true)} /></Theme>));
  await flush();
  return host;
}

const button = (el: ParentNode, text: string) => [...el.querySelectorAll<HTMLButtonElement>('button')].find((b) => b.textContent?.trim() === text);

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

describe('Auswertung im Bericht', () => {
  it('zeigt Kurzauswertung, Ampeln, Hinweise und nächsten Lauf', async () => {
    const host = await mount();
    const panel = host.querySelector('[aria-label="Auswertung"]') as HTMLElement;
    expect(panel).toBeTruthy();
    expect(panel.querySelector('.bal-insight-summary')?.textContent).toContain('1 von 3 Zielen verletzt');
    expect(panel.querySelector('[aria-label="Ampeln"]')?.textContent).toContain('1 verletzt');
    expect(panel.querySelector('.bal-insight-hint[data-light="red"]')?.textContent).toContain('buildings.json:wall.levels.0.hp');
    expect(panel.textContent).toContain('Nächster Lauf');
    expect(button(panel, 'In Claude öffnen')).toBeTruthy();
    expect(button(panel, 'Kopieren')).toBeTruthy();
  });

  it('öffnet den HTML-Bericht', async () => {
    const host = await mount();
    await act(async () => button(host, 'HTML-Bericht öffnen')!.click());
    await flush();
    expect(host.textContent).toContain('Geöffnet: reports/simtest-run-2/balance-2.html');
  });
});
