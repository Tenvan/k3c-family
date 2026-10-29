// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { pickValue, selectedValue } from '@webkit/testing/select';
import { backend } from '../api';
import { BalancingPage } from './BalancingPage';
import { EMPTY_FORM, formFromBaseline, specFromForm } from './spec';

// B-640/AC-04 (BS7.3): „Beispiele › Basis-Läufe“ listet gesicherte Läufe; „Übernehmen“ setzt Konfiguration und Baseline.

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
  // B-659: Das Formular steht im Detail und öffnet mit „Neuer Lauf“ in der Werkzeugleiste.
  await act(async () => [...host.querySelectorAll('button')].find((b) => b.textContent?.trim() === 'Neuer Lauf')!.click());
  await flush();
  return host;
}

const field = (host: ParentNode, label: string) => host.querySelector<HTMLInputElement | HTMLTextAreaElement>(`input[aria-label="${label}"], textarea[aria-label="${label}"]`)!;

async function click(el: HTMLElement) {
  await act(async () => el.click());
  await flush();
}

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

describe('Basis-Läufe im Ausgangspunkt', () => {
  it('„Basis-Lauf“ übernimmt Konfiguration und Baseline', async () => {
    const host = await mount();
    await click(host.querySelector<HTMLElement>('[aria-label="Ausgangspunkt"] [data-value="basis"]')!);
    await pickValue(host, 'Eintrag des Ausgangspunkts', 'M1');
    expect(selectedValue(host, 'Baseline')).toBe('M1');
    expect(host.querySelector('[aria-label="Seeds"] [data-value="1"]')!.getAttribute('data-state')).toBe('on');
    expect(field(host, 'Tage').value).toBe('40');
    expect(field(host, 'Label').value).toBe('Meilenstein 1');
  });

  it('der Lauf trägt die Baseline als Vergleich in die Spec', async () => {
    const [b] = await backend.balanceBaselines();
    const { spec, error } = specFromForm(formFromBaseline(b));
    expect(error).toBeUndefined();
    expect(spec).toMatchObject({ baseline: 'M1', bot: 'coop2', world: 'standard', days: 40, seeds: 1, players: 2 });
    expect(specFromForm(EMPTY_FORM).spec?.baseline).toBeUndefined();
  });

  it('eine Baseline ohne Spec nimmt Bot und Welt aus ihrer Konfiguration', () => {
    const form = formFromBaseline({ name: 'M0', saved: '2026-10-20T10:00:00Z', run: 'simtest-run-1', seeds: 3, seedMode: 'all', bots: ['saver'], worlds: ['standard'], pass: true, frozen: [] });
    expect(form).toMatchObject({ baseline: 'M0', bot: 'saver', world: 'standard' });
  });
});
