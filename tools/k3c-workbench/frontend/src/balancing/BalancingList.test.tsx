// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { pickValue } from '@webkit/testing/select';
import { backend, type BalanceRun } from '../api';
import { BalancingPage } from './BalancingPage';

// B-660/AC-01 bis AC-04 (WZ32.3): eine Liste für Läufe und Berichte mit Suche, Chips und Mehrfachauswahl, Detail mit
// fünf Reitern, Vergleich von drei Berichten (Mock-Backend; der Mock ist zwischen den Tests geteilt, das Löschen kommt zuletzt).

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

function unmount() {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
}

beforeEach(() => localStorage.clear());
afterEach(() => {
  unmount();
  localStorage.clear();
  vi.restoreAllMocks();
});

const items = (host: ParentNode) => [...host.querySelectorAll<HTMLElement>('.bal-item')];
const item = (host: ParentNode, text: string) => items(host).find((r) => r.textContent?.includes(text))!;
const button = (el: ParentNode, text: string) => [...el.querySelectorAll<HTMLButtonElement>('button')].find((b) => b.textContent?.trim() === text)!;
const startsWith = (el: ParentNode, text: string) => [...el.querySelectorAll<HTMLButtonElement>('button')].find((b) => b.textContent?.trim().startsWith(text))!;
const box = (host: ParentNode, label: string) => host.querySelector<HTMLElement>(`button[role="checkbox"][aria-label="Auswählen: ${label}"]`)!;
const byLabel = (host: ParentNode, label: string) => host.querySelector<HTMLElement>(`[aria-label="${label}"]`);

async function click(el: HTMLElement) {
  await act(async () => el.click());
  await flush();
}

/** Radix Tabs schalten bei mousedown um. */
async function openTab(host: ParentNode, name: string) {
  const tab = [...host.querySelectorAll<HTMLElement>('[role="tab"]')].find((t) => t.textContent?.startsWith(name))!;
  await act(async () => tab.dispatchEvent(new MouseEvent('mousedown', { bubbles: true, button: 0 })));
  await flush();
}

async function until(ok: () => boolean, ms = 8000) {
  for (let t = 0; t < ms && !ok(); t += 100) await wait(100);
  expect(ok()).toBe(true);
}

describe('Liste (AC-01)', () => {
  it('zeigt Zeilen mit Name, Meta, Ziele und nur Checkbox und Name als Bedienelemente', async () => {
    const host = await mount();
    const row = item(host, 'Insel 1 solo');
    expect(row.textContent).toContain('wirtschaft · 1 Sp. · Welt standard · 40 T. · 8 Seeds');
    expect(row.textContent).toContain('2/3 Ziele');
    expect(row.textContent).toContain('+1 Var.');
    expect(row.textContent).toContain('Fail');
    expect(row.querySelectorAll('button')).toHaveLength(2);
    expect(item(host, 'balance-1.json').dataset.rating).toBe('referenz');
    expect(item(host, 'Meilenstein 1').textContent).toContain('Baseline M1');
    expect(item(host, 'tuning-1.json').textContent).toContain('Abstimmung');
    expect([...host.querySelectorAll('.bal-group')].map((g) => g.getAttribute('aria-label'))).toEqual(['Heute', 'Gestern', 'Älter']);
  });

  it('ein laufender Lauf ist ein Eintrag und wird nach dem Ende an derselben Stelle zum Bericht', async () => {
    const host = await mount();
    const before = items(host).length;
    await click(button(host, 'Neuer Lauf'));
    await click(button(byLabel(host, 'Lauf-Editor')!, 'Lauf starten'));
    await until(() => items(host).some((r) => r.dataset.id?.startsWith('run:')));
    expect(items(host)).toHaveLength(before + 1);
    const run = items(host).find((r) => r.dataset.id?.startsWith('run:'))!;
    expect(run.closest('.bal-group')!.getAttribute('aria-label')).toBe('Läuft');
    await click(button(run, run.querySelector('.bal-item-name')!.textContent!));
    expect(byLabel(host, 'Lauf')).not.toBeNull();
    await until(() => !items(host).some((r) => r.dataset.id?.startsWith('run:')), 12000);
    await flush();
    expect(items(host)).toHaveLength(before + 1);
    expect(items(host).filter((r) => r.dataset.on === 'true')).toHaveLength(1);
    expect(byLabel(host, 'Bericht')).not.toBeNull();
    expect(byLabel(host, 'Lauf')).toBeNull();
  }, 20000);
});

describe('Suche und Chips (AC-02)', () => {
  it('filtert nach Suche und Chips; Chips bleiben gemerkt, die Suche nicht', async () => {
    const host = await mount();
    const search = host.querySelector<HTMLInputElement>('input[aria-label="Suchen"]')!;
    const set = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!;
    await act(async () => {
      set.call(search, 'startgold');
      search.dispatchEvent(new Event('input', { bubbles: true }));
    });
    expect(items(host).map((r) => r.querySelector('.bal-item-name')!.textContent)).toEqual(['Startgold 20']);
    await act(async () => {
      set.call(search, '');
      search.dispatchEvent(new Event('input', { bubbles: true }));
    });
    const all = items(host).length;
    await click(startsWith(byLabel(host, 'Status')!, 'Fail'));
    expect(items(host).length).toBeLessThan(all);
    expect(items(host).every((r) => r.dataset.state === 'Fail')).toBe(true);
    await act(async () => {
      set.call(search, 'xyz');
      search.dispatchEvent(new Event('input', { bubbles: true }));
    });
    unmount();
    const again = await mount();
    expect(startsWith(byLabel(again, 'Status')!, 'Fail').getAttribute('aria-pressed')).toBe('true');
    expect(again.querySelector<HTMLInputElement>('input[aria-label="Suchen"]')!.value).toBe('');
  });
});

describe('Mehrfachauswahl und Vergleich (AC-03)', () => {
  const three = ['balance-1.json', 'Insel 1 solo · ohne Hilfen', 'Startgold 20'];

  it('Vergleichen ab zwei Berichten zeigt drei Spalten mit Abstand zu A', async () => {
    const host = await mount();
    await click(box(host, three[0]));
    expect(button(byLabel(host, 'Auswahl')!, 'Vergleichen').disabled).toBe(true);
    await click(box(host, three[1]));
    await click(box(host, three[2]));
    expect(byLabel(host, 'Auswahl')!.textContent).toContain('3 gewählt');
    await click(button(byLabel(host, 'Auswahl')!, 'Vergleichen'));
    const cmp = byLabel(host, 'Vergleich')!;
    expect(cmp.querySelectorAll('.bal-col')).toHaveLength(3);
    await until(() => cmp.querySelectorAll('table tbody tr').length > 0);
    expect([...cmp.querySelectorAll('th')].map((t) => t.textContent)).toEqual(['Wert', 'Korridor', 'A', 'B', 'C']);
    expect(cmp.querySelectorAll('.bal-d').length).toBeGreaterThan(0);
    expect(cmp.textContent).toContain('kippt');
    await click(startsWith(cmp, 'Gekippt'));
    expect(cmp.querySelectorAll('table tbody tr')).toHaveLength(1);
    await click(button(cmp.querySelector('[data-column="B"]')!, 'Als A setzen'));
    expect(cmp.querySelector('[data-column="A"]')!.textContent).toContain('Insel 1 solo');
  });

  it('bewertet alle gewählten Berichte', async () => {
    const host = await mount();
    await click(box(host, three[0]));
    await click(box(host, three[2]));
    await pickValue(host, 'Auswahl bewerten', 'schlecht');
    await flush();
    expect(item(host, 'Startgold 20').dataset.rating).toBe('schlecht');
    expect(item(host, 'balance-1.json').dataset.rating).toBe('schlecht');
  });
});

describe('Detail (AC-04)', () => {
  const RUN: BalanceRun = { id: 'run-99', label: 'x', where: '', state: 'wartet', phase: 'wartet', done: 0, total: 6, started: new Date().toISOString(), verdict: '', report: '', lines: [] };

  it('zeigt fünf Reiter und startet den Auftrag des Berichts erneut', async () => {
    const start = vi.spyOn(backend, 'balanceStart').mockResolvedValue(RUN);
    const host = await mount();
    await click(item(host, 'Insel 1 solo').querySelector('.bal-item-name') as HTMLElement);
    const report = byLabel(host, 'Bericht')!;
    const tabs = [...report.querySelectorAll('[role="tab"]')].map((t) => t.textContent ?? '');
    expect(['Überblick', 'Ziele', 'Diagramme', 'Hinweise', 'Konfiguration'].map((n, i) => tabs[i]?.startsWith(n))).toEqual([true, true, true, true, true]);
    expect(tabs).toHaveLength(5);
    await until(() => report.textContent!.includes('Kurzfazit'));
    await click(button(report, 'Erneut starten'));
    expect(start).toHaveBeenCalledOnce();
    expect(start.mock.calls[0][0]).toMatchObject({ bot: 'wirtschaft', world: 'standard', days: 40, vary: ['bot.json:a=1'] });
    expect(items(host)[0].dataset.id).toBe('run:run-99');
    await openTab(report, 'Ziele');
    expect(report.querySelectorAll('table tbody tr').length).toBe(3);
    await openTab(report, 'Konfiguration');
    expect(byLabel(report, 'Auftrag')!.textContent).toContain('wirtschaft');
  });

  it('„Als Variante übernehmen“ öffnet das Formular mit der Zeile', async () => {
    const host = await mount();
    await click(item(host, 'Insel 1 solo').querySelector('.bal-item-name') as HTMLElement);
    await until(() => byLabel(host, 'Bericht')!.textContent!.includes('Kurzfazit'));
    await openTab(host, 'Hinweise');
    await click(button(host, 'Als Variante übernehmen'));
    await until(() => byLabel(host, 'Lauf-Editor') !== null);
    const rows = [...host.querySelectorAll('table[aria-label="Varianten"] tbody tr')];
    expect(rows).toHaveLength(2);
    expect(rows[0].textContent).toContain('bot.json');
    expect(rows[1].textContent).toContain('wall.levels.0.hp');
    expect(rows[1].querySelector('input')!.value).toBe('360');
  });

  it('der Reiter bleibt gemerkt und „Erneut starten“ ist ohne Auftrag aus', async () => {
    const host = await mount();
    await click(item(host, 'balance-1.json').querySelector('.bal-item-name') as HTMLElement);
    expect(button(byLabel(host, 'Bericht')!, 'Erneut starten').disabled).toBe(true);
    await openTab(host, 'Diagramme');
    unmount();
    const again = await mount();
    await click(item(again, 'balance-1.json').querySelector('.bal-item-name') as HTMLElement);
    expect(again.querySelector('[role="tab"][data-state="active"]')!.textContent).toContain('Diagramme');
  });
});

// Zuletzt: das Löschen entfernt Berichte aus dem geteilten Mock.
describe('Löschen (AC-03)', () => {
  const three = ['balance-1.json', 'Insel 1 solo · ohne Hilfen', 'Startgold 20'];
  it('Löschen fragt einmal und ruft removeMany mit allen drei', async () => {
    const spy = vi.spyOn(backend, 'balanceReportsDelete');
    const host = await mount();
    for (const name of three) await click(box(host, name));
    const before = items(host).length;
    await click(button(byLabel(host, 'Auswahl')!, 'Löschen'));
    expect(document.querySelectorAll('[role="alertdialog"]')).toHaveLength(1);
    await click(button(document.querySelector('[role="alertdialog"]')!, 'Löschen'));
    expect(spy).toHaveBeenCalledTimes(1);
    expect(spy.mock.calls[0][0]).toHaveLength(3);
    expect(items(host)).toHaveLength(before - 3);
    expect(byLabel(host, 'Auswahl')).toBeNull();
  });
});
