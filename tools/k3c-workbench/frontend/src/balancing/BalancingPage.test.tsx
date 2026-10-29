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

async function until(host: HTMLElement, ok: () => boolean, ms = 8000) {
  for (let t = 0; t < ms && !ok(); t += 100) await wait(100);
  expect(ok(), host.textContent ?? '').toBe(true);
}

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

const runs = (host: ParentNode) => [...host.querySelectorAll<HTMLElement>('.bal-item[data-id^="run:"]')];
const reports = (host: ParentNode) => [...host.querySelectorAll<HTMLElement>('.bal-item:not([data-id^="run:"])')];
const nameOf = (el: HTMLElement) => el.querySelector<HTMLElement>('.bal-item-name')!;

describe('Balancing-Seite', () => {
  it('zeigt Konfiguration und Liste, validiert die Eingabe', async () => {
    const host = await mount();
    expect(host.querySelector('[aria-label="Konfiguration"]')).not.toBeNull();
    expect(reports(host)).toHaveLength(5);
    const seeds = host.querySelector<HTMLInputElement>('input[aria-label="Seeds"]')!;
    const set = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!;
    await act(async () => {
      set.call(seeds, '0');
      seeds.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await act(async () => button(host, 'Lauf starten').click());
    expect(host.querySelector('[role="alert"]')?.textContent).toMatch(/Seeds/);
    expect(runs(host)).toHaveLength(0);
  });

  it('startet einen Lauf, zeigt Fortschritt und Abbruch, danach den Bericht mit Diagrammen', async () => {
    const host = await mount();
    await act(async () => button(host, 'Lauf starten').click());
    await flush();
    expect(runs(host)).toHaveLength(1);
    await act(async () => nameOf(runs(host)[0]).click());
    await until(host, () => /läuft/.test(runs(host)[0]?.textContent ?? '') && /\d\/6/.test(runs(host)[0]?.textContent ?? ''));
    expect(button(host, 'Abbrechen')).toBeDefined();
    await until(host, () => runs(host).length === 0, 12000);
    await flush();
    expect(reports(host)).toHaveLength(6);
    const report = host.querySelector<HTMLElement>('[aria-label="Bericht"]')!;
    expect(report).not.toBeNull();
    const tab = [...report.querySelectorAll<HTMLElement>('[role="tab"]')].find((t) => t.textContent?.startsWith('Diagramme'))!;
    await act(async () => tab.dispatchEvent(new MouseEvent('mousedown', { bubbles: true, button: 0 })));
    await flush();
    expect(report.querySelectorAll('svg').length).toBeGreaterThanOrEqual(4);
    await act(async () => button(report, 'Export ▾').click());
    await flush();
    expect(document.body.textContent).toContain('Markdown');
    expect(document.body.textContent).toContain('JSON');
  }, 20000);

  it('bricht einen wartenden Lauf ab', async () => {
    const host = await mount();
    await act(async () => button(host, 'Lauf starten').click());
    await act(async () => button(host, 'Lauf starten').click());
    await flush();
    const waiting = runs(host).find((r) => r.dataset.state === 'wartet')!;
    expect(waiting).toBeDefined();
    await act(async () => nameOf(waiting).click());
    await act(async () => button(host, 'Abbrechen').click());
    await flush();
    expect(runs(host).map((r) => r.dataset.state)).toContain('abgebrochen');
  });
});

describe('Balancing-Seite: Abstimmen', () => {
  it('zeigt ein Abstimm-Ergebnis mit Diagramm, Tabelle und kopierbarem Diff', async () => {
    const host = await mount();
    const row = reports(host).find((r) => r.textContent?.includes('tuning-1.json'))!;
    expect(row.textContent).toContain('Abstimmung');
    expect(row.textContent).not.toContain('Vergleichsbasis');
    await act(async () => nameOf(row).click());
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
    const before = runs(host).length;
    await act(async () => button(host, 'Lauf starten').click());
    await flush();
    expect(runs(host)).toHaveLength(before + 1);
  });
});
