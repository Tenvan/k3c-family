// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { BalancingPage } from './BalancingPage';

// WZ28.5/AC-05: Die Seite startet mit dem Mock-Backend (Testlauf = Mock) einen Lauf, zeigt Fortschritt, öffnet den Bericht
// und vergleicht zwei Berichte.

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

const button = (host: HTMLElement, text: string) =>
  [...host.querySelectorAll('button')].find((b) => b.textContent?.trim() === text) as HTMLButtonElement;
const buttons = (host: HTMLElement, text: string) => [...host.querySelectorAll('button')].filter((b) => b.textContent?.trim() === text);

async function until(host: HTMLElement, ok: () => boolean, ms = 8000) {
  for (let t = 0; t < ms && !ok(); t += 100) await wait(100);
  expect(ok(), host.textContent ?? '').toBe(true);
}

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

describe('Balancing-Seite', () => {
  it('zeigt Konfiguration und Berichtsliste, validiert die Eingabe', async () => {
    const host = await mount();
    expect(host.querySelector('[aria-label="Konfiguration"]')).not.toBeNull();
    expect(host.querySelectorAll('[aria-label="Berichte"] .bal-report-row')).toHaveLength(3);
    const seeds = host.querySelector<HTMLInputElement>('input[aria-label="Seeds"]')!;
    const set = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!;
    await act(async () => {
      set.call(seeds, '0');
      seeds.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await act(async () => button(host, 'Lauf starten').click());
    expect(host.querySelector('[role="alert"]')?.textContent).toMatch(/Seeds/);
    expect(host.querySelector('[aria-label="Läufe"] .bal-run')).toBeNull();
  });

  it('startet einen Lauf, zeigt Fortschritt und Abbruch, danach den Bericht mit Diagrammen', async () => {
    const host = await mount();
    await act(async () => button(host, 'Lauf starten').click());
    await flush();
    const runs = host.querySelector('[aria-label="Läufe"]')!;
    expect(runs.querySelectorAll('.bal-run')).toHaveLength(1);
    await until(host, () => /läuft/.test(runs.textContent ?? '') && /\d\/6/.test(runs.textContent ?? ''));
    expect(button(host, 'Abbrechen')).toBeDefined();
    await until(host, () => runs.querySelector('.bal-run')?.getAttribute('data-state') === 'fertig', 12000);
    expect(host.querySelectorAll('[aria-label="Berichte"] .bal-report-row')).toHaveLength(4);
    await act(async () => button(host, 'Bericht öffnen').click());
    await flush();
    const report = host.querySelector('[aria-label="Bericht"]')!;
    expect(report.textContent).toContain('Überleben Nacht 3');
    expect(report.querySelectorAll('svg').length).toBeGreaterThanOrEqual(7);
    expect(button(host, 'Export Markdown')).toBeDefined();
    expect(button(host, 'Export JSON')).toBeDefined();
  }, 20000);

  it('bricht einen wartenden Lauf ab', async () => {
    const host = await mount();
    await act(async () => button(host, 'Lauf starten').click());
    await act(async () => button(host, 'Lauf starten').click());
    await flush();
    const stops = buttons(host, 'Abbrechen');
    expect(stops.length).toBeGreaterThanOrEqual(1);
    await act(async () => stops[stops.length - 1].click());
    await flush();
    const states = [...host.querySelectorAll('.bal-run')].map((r) => r.getAttribute('data-state'));
    expect(states).toContain('abgebrochen');
  });

  it('vergleicht zwei Berichte mit Differenzen', async () => {
    const host = await mount();
    const row = (name: string) => [...host.querySelectorAll('[aria-label="Berichte"] .bal-report-row')].find((r) => r.textContent?.includes(name))!;
    await act(async () => [...row('simtest-run-2').querySelectorAll('button')].find((b) => b.textContent === 'Vergleichsbasis')!.click());
    await act(async () => row('balance-1.json').querySelector('button')!.click());
    await flush();
    await until(host, () => host.querySelector('[aria-label="Vergleich"]') !== null || host.querySelector('[aria-label="Bericht"]') !== null);
    const cmp = host.querySelector('[aria-label="Vergleich"]');
    expect(cmp).not.toBeNull();
    expect(cmp!.textContent).toContain('gekippt');
    expect(cmp!.querySelectorAll('table tbody tr').length).toBeGreaterThan(3);
  });
});

describe('Balancing-Seite: Abstimmen', () => {
  it('zeigt ein Abstimm-Ergebnis mit Diagramm, Tabelle und kopierbarem Diff', async () => {
    const host = await mount();
    const row = [...host.querySelectorAll('[aria-label="Berichte"] .bal-report-row')].find((r) => r.textContent?.includes('tuning-1.json'))!;
    expect(row.textContent).toContain('Abstimmung');
    expect(row.textContent).not.toContain('Vergleichsbasis');
    await act(async () => row.querySelector('button')!.click());
    await flush();
    const view = host.querySelector('[aria-label="Abstimmung"]')!;
    expect(view).not.toBeNull();
    expect(view.textContent).toContain('Bester Punkt: buildings.json:wall.cost = 0.6');
    expect(view.querySelectorAll('svg').length).toBeGreaterThanOrEqual(1);
    expect(view.querySelectorAll('table tbody tr')).toHaveLength(5);
    expect(view.querySelector('pre')?.textContent).toContain('+  "cost": 0.6');
    const writes: string[] = [];
    Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: async (t: string) => void writes.push(t) } });
    await act(async () => button(host, 'Diff kopieren').click());
    await flush();
    expect(writes[0]).toContain('+++ b/data/buildings.json');
    expect(host.textContent).toContain('Diff kopiert.');
    Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: async () => Promise.reject(new Error('gesperrt')) } });
    await act(async () => button(host, 'Diff kopieren').click());
    await flush();
    expect(host.textContent).toContain('Kopieren fehlgeschlagen');
  });

  it('startet einen Abstimm-Lauf und prüft die Parameter', async () => {
    const host = await mount();
    const set = Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, 'value')!.set!;
    const select = host.querySelector<HTMLElement>('[aria-label="Abstimm-Modus"]')!;
    expect(select).not.toBeNull();
    expect(host.querySelector('textarea[aria-label="Parameter"]')).toBeNull();
    await act(async () => select.click());
    await flush();
    const opt = [...document.querySelectorAll<HTMLElement>('[role="option"]')].find((o) => /Sweep/.test(o.textContent ?? ''));
    await act(async () => opt?.click());
    await flush();
    const params = host.querySelector<HTMLTextAreaElement>('textarea[aria-label="Parameter"]')!;
    await act(async () => {
      set.call(params, 'kaputt');
      params.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await act(async () => button(host, 'Lauf starten').click());
    expect(host.querySelector('[role="alert"]')?.textContent).toMatch(/Parameter/);
    await act(async () => {
      set.call(params, 'buildings.json:wall.cost=1..5');
      params.dispatchEvent(new Event('input', { bubbles: true }));
    });
    const before = host.querySelectorAll('[aria-label="Läufe"] .bal-run').length;
    await act(async () => button(host, 'Lauf starten').click());
    await flush();
    expect(host.querySelectorAll('[aria-label="Läufe"] .bal-run')).toHaveLength(before + 1);
  });
});
