// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { pickValue } from '@webkit/testing/select';
import { backend, type BalanceBaseline, type BalanceDataFileValue, type BalanceSpec } from '../../api';
import { EXAMPLES, formFromExample } from '../help';
import { EMPTY_FORM, formFromSpec, specFromForm, type Preset } from '../spec';
import { RunEditor, type RunEditorProps } from './RunEditor';

// B-661/AC-01 bis AC-04 (WZ32.4): Ausgangspunkte, Varianten-Tabelle, Abstimmung, Text ↔ Tabelle im Lauf-Editor.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const DATA: BalanceDataFileValue[] = [
  { file: 'economy.json', path: 'purse.startGold', value: 14 },
  { file: 'economy.json', path: 'gatherables.tree.markCost', value: 1 },
  { file: 'buildings.json', path: 'wall.cost.gold', value: 2 },
  { file: 'buildings.json', path: 'tower.cost.gold', value: 4 },
];
const BASELINE = { name: 'M1', saved: '2026-10-27', run: 'run-1', seeds: 1, seedMode: 'liste', bots: ['wirtschaft'], worlds: ['standard'], pass: true, frozen: [], spec: { seeds: 1, days: 40, world: 'standard', players: 1, label: 'Meilenstein 1' } } as BalanceBaseline;
const REPORT: BalanceSpec = { seeds: 8, world: 'standard', helps: 'voll', days: 40, players: 1, bot: 'wirtschaft', label: 'Insel 1 solo' };

let root: Root | null = null;
const wait = (ms: number) => act(async () => new Promise((r) => setTimeout(r, ms)));
const flush = async () => { for (let i = 0; i < 5; i++) await wait(0); };

async function mount(extra: Partial<RunEditorProps> = {}) {
  const props: RunEditorProps = {
    initial: formFromSpec(REPORT), origin: { kind: 'bericht', label: 'Insel 1 solo' }, baselines: [BASELINE], runs: [], presets: [],
    onPresets: () => {}, onStart: async () => {}, onClose: () => {}, ...extra,
  };
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><RunEditor {...props} /></Theme>));
  await flush();
  return host;
}

const q = <T extends HTMLElement>(host: ParentNode, sel: string) => host.querySelector<T>(sel)!;
const label = (host: ParentNode, name: string) => q<HTMLInputElement>(host, `[aria-label="${name}"]`);
const item = (host: ParentNode, group: string, value: string) => q(host, `[aria-label="${group}"] [data-value="${value}"]`);
const button = (host: ParentNode, text: string) => [...host.querySelectorAll('button')].find((b) => b.textContent?.trim() === text)!;
const click = async (el: HTMLElement) => { await act(async () => el.click()); await flush(); };
const isOn = (el: HTMLElement) => el.getAttribute('data-state') === 'on';

async function type(el: HTMLInputElement | HTMLTextAreaElement, value: string) {
  const proto = el instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype;
  await act(async () => {
    Object.getOwnPropertyDescriptor(proto, 'value')!.set!.call(el, value);
    el.dispatchEvent(new Event('input', { bubbles: true }));
  });
  await flush();
}

const startButton = (host: ParentNode) => [...host.querySelectorAll('footer button')].find((b) => (b.textContent ?? '').endsWith('starten'))! as HTMLButtonElement;
const addValue = (host: ParentNode, path: string, kind: 'feste Variante' | 'Bereich') => click(label(host, `${path} als ${kind}`));

beforeEach(() => { vi.spyOn(backend, 'balanceDataValuesAll').mockResolvedValue(DATA); });
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
  vi.restoreAllMocks();
});

describe('Ausgangspunkt (AC-01)', () => {
  it('Bericht füllt den Editor wie formFromSpec', async () => {
    const host = await mount();
    expect(isOn(item(host, 'Welt', 'standard'))).toBe(true);
    expect(isOn(item(host, 'Hilfen', 'voll'))).toBe(true);
    expect(isOn(item(host, 'Seeds', '8'))).toBe(true);
    expect(label(host, 'Tage').value).toBe('40');
    expect(label(host, 'Label').value).toBe('Insel 1 solo');
    expect(host.textContent).toContain('Hilfen sagt der Lauf nichts');
  });

  it('Beispiel, Basis-Lauf und Voreinstellung übernehmen alle Felder', async () => {
    const old = { seeds: '5', seedList: '', depth: '1', players: '2', bot: '', grade: 'hard', days: '', vary: 'economy.json:purse.startGold=9', label: 'alt', tune: { mode: '', params: '', steps: '', budget: '' } };
    const presets = [{ name: 'Alt', form: old }] as unknown as Preset[];
    const host = await mount({ presets });
    await click(item(host, 'Ausgangspunkt', 'beispiel'));
    await pickValue(host, 'Eintrag des Ausgangspunkts', 'Höhle, hart');
    expect(isOn(item(host, 'Grad', 'hard'))).toBe(true);
    expect(label(host, 'Label').value).toBe('Höhle hart');
    await click(item(host, 'Ausgangspunkt', 'basis'));
    await pickValue(host, 'Eintrag des Ausgangspunkts', 'M1');
    expect(label(host, 'Label').value).toBe('Meilenstein 1');
    expect(label(host, 'Tage').value).toBe('40');
    await click(item(host, 'Ausgangspunkt', 'voreinstellung'));
    await pickValue(host, 'Eintrag des Ausgangspunkts', 'Alt');
    expect(isOn(item(host, 'Grad', 'hard'))).toBe(true);
    expect(host.textContent).toContain('purse.startGold');
    expect(label(host, 'Neuer Wert für purse.startGold').value).toBe('9');
    await click(item(host, 'Ausgangspunkt', 'leer'));
    expect(isOn(item(host, 'Welt', ''))).toBe(true);
    expect(label(host, 'Tage').value).toBe('');
  });
});

describe('Szenario', () => {
  it('Hilfen gibt es nur mit Welt, die Tiefe nur ohne; der Wechsel leert das Verborgene', async () => {
    const host = await mount();
    expect(host.querySelector('[aria-label="Tiefe"]')).toBeNull();
    await click(item(host, 'Welt', ''));
    expect(host.querySelector('[aria-label="Hilfen"]')).toBeNull();
    expect(label(host, 'Tiefe')).not.toBeNull();
    await click(item(host, 'Welt', 'standard'));
    expect(isOn(item(host, 'Hilfen', ''))).toBe(true);
  });

  it('Spieler mit −/+, Bot je Spieler braucht zwei Spieler', async () => {
    const host = await mount();
    expect(label(host, 'Spieler').textContent).toBe('1');
    await click(label(host, 'Ein Spieler mehr'));
    await click(item(host, 'Bot-Zuordnung', 'je'));
    expect(label(host, 'Bot Spieler 2')).not.toBeNull();
    await click(label(host, 'Ein Spieler weniger'));
    expect(host.querySelector('[aria-label="Bot Spieler 2"]')).toBeNull();
  });

  it('Seeds: Liste schließt die Zahl aus', async () => {
    const host = await mount();
    await click(item(host, 'Seeds', 'liste'));
    await type(label(host, 'Seed-Liste'), 'k3c-dev,7');
    expect(isOn(item(host, 'Seeds', 'liste'))).toBe(true);
    expect(startButton(host).disabled).toBe(false);
    await click(item(host, 'Seeds', '20'));
    expect(host.querySelector('[aria-label="Seed-Liste"]')).toBeNull();
  });
});

describe('Varianten und Starten (AC-02 bis AC-04)', () => {
  it('Wert aus dem Daten-Browser, falscher Pfad sperrt Starten, Vorschlag übernehmen', async () => {
    const onStart = vi.fn(async () => {});
    const host = await mount({ onStart });
    await addValue(host, 'wall.cost.gold', 'feste Variante');
    await type(label(host, 'Neuer Wert für wall.cost.gold'), '3');
    expect(host.querySelector('.bal-ed-dev')?.textContent).toBe('+50 %');
    expect(startButton(host).disabled).toBe(false);
    await click(item(host, 'Art für wall.cost.gold', 'bereich'));
    expect(label(host, 'Von für wall.cost.gold').value).toBe('2');
    expect(startButton(host).textContent).toBe('Abstimmung starten');
    await click(item(host, 'Art für wall.cost.gold', 'fest'));
    await click(button(host, 'Als Text bearbeiten'));
    await type(label(host, 'Varianten als Text'), 'buildings.json:tower.cost.gld=3');
    expect(startButton(host).disabled).toBe(true);
    await click(button(host, 'Als Tabelle bearbeiten'));
    expect(host.textContent).toContain('Meinten Sie tower.cost.gold (4)?');
    await click(button(host, 'Übernehmen'));
    expect(startButton(host).disabled).toBe(false);
    await click(startButton(host));
    expect(onStart).toHaveBeenCalledWith({ ...REPORT, vary: ['buildings.json:tower.cost.gold=3'] });
  });

  it('erzeugt denselben Auftrag wie das alte Formular, auch bei einem Bereich', async () => {
    const example = EXAMPLES.find((e) => e.name === 'Mauerkosten abstimmen')!;
    const form = formFromExample(example, EMPTY_FORM);
    const onStart = vi.fn(async () => {});
    const host = await mount({ initial: form, origin: { kind: 'beispiel', label: example.name }, onStart });
    expect(host.textContent).toContain('Abstimmung');
    await click(startButton(host));
    expect(onStart).toHaveBeenCalledWith(specFromForm(form).spec);
  });

  it('„je Variante“ nur mit festen Varianten: n + 1 Läufe, perVariant im Auftrag', async () => {
    const onStart = vi.fn(async () => {});
    const host = await mount({ onStart });
    await addValue(host, 'wall.cost.gold', 'feste Variante');
    await addValue(host, 'purse.startGold', 'feste Variante');
    await click(host.querySelector<HTMLElement>('button[role="checkbox"]')!);
    expect(startButton(host).textContent).toBe('3 Läufe starten');
    await click(startButton(host));
    expect((onStart.mock.calls as unknown as [BalanceSpec][])[0][0]).toMatchObject({ perVariant: true });
  });

  it('Ablehnung beim Start erscheint in der Fußleiste', async () => {
    const onClose = vi.fn();
    const host = await mount({ onStart: async () => { throw new Error('lauf abgelehnt: test'); }, onClose });
    await click(startButton(host));
    expect(host.querySelector('footer')!.textContent).toContain('lauf abgelehnt: test');
    expect(onClose).not.toHaveBeenCalled();
  });

  it('im Textmodus bleibt eine Zeile aus dem Daten-Browser beim nächsten Tippen erhalten', async () => {
    const host = await mount();
    await click(button(host, 'Als Text bearbeiten'));
    await type(label(host, 'Varianten als Text'), 'economy.json:purse.startGold=20');
    await addValue(host, 'wall.cost.gold', 'feste Variante');
    expect((label(host, 'Varianten als Text') as unknown as HTMLTextAreaElement).value).toBe('economy.json:purse.startGold=20\nbuildings.json:wall.cost.gold=2');
    await type(label(host, 'Varianten als Text'), 'economy.json:purse.startGold=21\nbuildings.json:wall.cost.gold=2');
    await click(button(host, 'Als Tabelle bearbeiten'));
    expect(host.querySelectorAll('table[aria-label="Varianten"] tbody tr')).toHaveLength(2);
  });
});
