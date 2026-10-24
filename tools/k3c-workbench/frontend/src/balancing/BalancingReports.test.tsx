// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { pickValue } from '@webkit/testing/select';
import { backend } from '../api';
import { BalancingPage } from './BalancingPage';

// WZ28.13/AC-14 (B-612): Berichte bewerten, nach Rückfrage löschen, Vergleichsbasis mit Erklärung (Mock-Backend).

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
const wait = (ms: number) => act(async () => new Promise((r) => setTimeout(r, ms)));
async function flush() {
  for (let i = 0; i < 5; i++) await wait(0);
}

async function mount(): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><BalancingPage /></Theme>));
  await flush();
  return host;
}

const list = (host: HTMLElement) => host.querySelector('[aria-label="Berichte"]') as HTMLElement;
const row = (host: HTMLElement, name: string) => [...list(host).querySelectorAll<HTMLElement>('.bal-report-row')].find((r) => r.textContent?.includes(name))!;
const button = (el: ParentNode, text: string) => [...el.querySelectorAll<HTMLButtonElement>('button')].find((b) => b.textContent?.trim() === text)!;

async function click(el: HTMLElement) {
  await act(async () => el.click());
  await flush();
}

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

describe('Berichte verwalten', () => {
  it('bewertet einen Bericht und zeigt die Bewertung', async () => {
    const host = await mount();
    expect(row(host, 'balance-1.json').dataset.rating).toBe('referenz');
    await pickValue(host, 'Bewertung simtest-run-2', 'schlecht');
    await flush();
    expect(row(host, 'simtest-run-2').dataset.rating).toBe('schlecht');
    expect((await backend.balanceReports()).find((r) => r.run === 'simtest-run-2')?.rating).toBe('schlecht');
  });

  it('löscht erst nach Rückfrage', async () => {
    const host = await mount();
    const before = list(host).querySelectorAll('.bal-report-row').length;
    await click(host.querySelector<HTMLElement>('[aria-label="Löschen simtest-run-2"]')!);
    const dialog = document.querySelector('[role="alertdialog"]') as HTMLElement;
    expect(dialog.textContent).toContain('reports/simtest-run-2/balance-2.json');
    await click(button(dialog, 'Abbrechen'));
    expect(list(host).querySelectorAll('.bal-report-row')).toHaveLength(before);
    await click(host.querySelector<HTMLElement>('[aria-label="Löschen simtest-run-2"]')!);
    await click(button(document.querySelector('[role="alertdialog"]')!, 'Löschen'));
    expect(list(host).querySelectorAll('.bal-report-row')).toHaveLength(before - 1);
    expect(list(host).textContent).not.toContain('simtest-run-2');
  });

  it('erklärt die Vergleichsbasis', async () => {
    const host = await mount();
    await click(button(row(host, 'balance-1.json'), 'Vergleichsbasis'));
    expect(list(host).textContent).toContain('A = balance-1.json (Basis)');
    await click(button(row(host, 'tuning-1.json'), 'tuning-1.json')); // der Mock ist geteilt: simtest-run-2 ist schon gelöscht
    expect(list(host).textContent).toContain('B = tuning-1.json');
    await click(list(host).querySelector<HTMLElement>('[aria-label="Hilfe: Berichte"]')!);
    expect(document.querySelector('[role="dialog"]')!.textContent).toContain('Maßstab A');
    await act(async () => (document.querySelector('[role="dialog"]') as HTMLElement).dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true })));
    await click(button(list(host), 'Basis aufheben'));
    expect(list(host).textContent).not.toContain('A = ');
  });
});
