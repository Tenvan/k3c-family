// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend, type HubSaveResult } from '../api';
import { mockHubCheck, mockHubPlan, mockHubSave } from '../api/mockHub';
import { pickOption } from '../../../../webkit/testing/select';
import { PlannerPage } from './PlannerPage';

// WZ9.3 (B-462/AC-02, AC-05, AC-06) im DOM: Bearbeiten als Kandidat, Prüfung und Speichern über das Backend (Mock).
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class { show() {} refresh() {} setLook() {} setPosition() {} destroy() {} } }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><PlannerPage /></Theme>));
  await flush();
  return host;
}

afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

const button = (h: HTMLElement, text: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === text)!;
const click = async (el: HTMLElement) => { await act(async () => el.click()); await flush(); };
const select = (h: HTMLElement, key: string) => click(h.querySelector<HTMLButtonElement>(`tr[data-key="${key}"] button.pl-row`)!);
const field = (h: HTMLElement, label: string) => h.querySelector<HTMLInputElement>(`input[aria-label="${label}"]`)!;

async function type(el: HTMLInputElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('input', { bubbles: true })));
  await flush();
}

/** Letzter geprüfter Kandidat als Objekt. */
function lastCandidate(spy: { mock: { calls: unknown[][] } }) {
  return JSON.parse(spy.mock.calls.at(-1)![0] as string);
}

describe('Gebäude-Planer: Bearbeiten (B-462 › Bearbeitbar)', () => {
  it('Rüstkammer auf Stufe 5: Kandidat geprüft, Live-Vorschau folgt', async () => {
    const check = vi.spyOn(backend, 'hubCheck');
    const preview = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    await select(h, 'armory@40');
    await type(field(h, 'Hub-Stufe (hubLevel)'), '5');
    expect(lastCandidate(check).sites[8]).toEqual({ kind: 'armory', offsetUnits: 40, hubLevel: 5 });
    expect(JSON.parse(preview.mock.lastCall![0].hubCandidate!).sites[8].hubLevel).toBe(5);
    expect(h.textContent).toContain('geändert, Prüfung ohne Befund');
  });

  it('Platz, Tiefe und needsDeeper', async () => {
    const check = vi.spyOn(backend, 'hubCheck');
    const h = await mount();
    await select(h, 'barracks@24');
    await type(field(h, 'Platz (offsetUnits)'), '26');
    await type(field(h, 'ab Tiefe (fromDepth)'), '1');
    await click(h.querySelector<HTMLElement>('[role="checkbox"][aria-label="needsDeeper"]')!);
    expect(lastCandidate(check).sites[6]).toEqual({ kind: 'barracks', offsetUnits: 26, hubLevel: 2, fromDepth: 1, needsDeeper: true });
  });

  it('Baugrund der Zone 1 und Händler', async () => {
    const check = vi.spyOn(backend, 'hubCheck');
    const h = await mount();
    await select(h, 'zone1');
    await type(field(h, 'Mauer Zone 1 (zone1WallUnits)'), '62');
    await type(field(h, 'Turm Zone 1 (zone1TowerUnits)'), '54');
    await select(h, 'merchantSell@20');
    await type(field(h, 'Platz (sellOffsetUnits)'), '21');
    await type(field(h, 'nur Tiefe (onlyDepth)'), '1');
    const c = lastCandidate(check);
    expect(c.sitePlacement.zone1WallUnits).toBe(62);
    expect(c.sitePlacement.zone1TowerUnits).toBe(54);
    expect(c.merchant).toEqual({ buyOffsetUnits: 16, sellOffsetUnits: 21, onlyDepth: 1 });
  });

  it('Platz entfernen mit Hinweis auf alte Spielstände, Platz hinzufügen', async () => {
    const check = vi.spyOn(backend, 'hubCheck');
    const h = await mount();
    await select(h, 'healer@-40');
    expect(h.querySelector('.pl-editor')!.textContent).toContain('Alte Spielstände verlieren den entfernten Platz.');
    await click(button(h, 'Platz entfernen'));
    expect(lastCandidate(check).sites.map((s: { kind: string }) => s.kind)).not.toContain('healer');
    expect(mockHubPlan().kinds).toContain('healer');
    await type(field(h, 'neuer Platz (offsetUnits)'), '-41');
    await pickOption(h, 'Art', 'healer');
    await click(button(h, 'Platz hinzufügen'));
    expect(lastCandidate(check).sites.at(-1)).toEqual({ kind: 'healer', offsetUnits: -41, hubLevel: 7 });
  });
});

const changeArmory = async (h: HTMLElement) => {
  await select(h, 'armory@40');
  await type(field(h, 'Hub-Stufe (hubLevel)'), '5');
};

describe('Gebäude-Planer: Speichern (B-462/AC-02, AC-05, AC-06)', () => {
  it('nur mit Änderung; schickt Kandidat und Stand, zeigt den REG-Hinweis', async () => {
    const save = vi.spyOn(backend, 'hubSave');
    const h = await mount();
    expect(button(h, 'Speichern').disabled).toBe(true);
    await changeArmory(h);
    await click(button(h, 'Speichern'));
    expect(save).toHaveBeenCalledWith(expect.stringContaining('"hubLevel": 5'), mockHubPlan().stand);
    expect(h.textContent).toContain('REG-Änderung: Commit mit Begründung (data/hub.json und Regeltabelle), task go:golden nur bei gewollter Änderung');
  });

  it('Befund der Prüfung sperrt das Speichern', async () => {
    vi.spyOn(backend, 'hubCheck').mockImplementation(async (c) => ({ ...mockHubCheck(c), findings: [{ rule: 'stufe', site: 'armory@+40', detail: 'hubLevel 9 außerhalb 1…7' }] }));
    const save = vi.spyOn(backend, 'hubSave');
    const h = await mount();
    await changeArmory(h);
    expect(button(h, 'Speichern').disabled).toBe(true);
    expect(h.textContent).toContain('nicht speicherbar: 1 Befunde');
    expect(h.querySelector('.pl-findings li')!.textContent).toBe('stufe: armory@+40: hubLevel 9 außerhalb 1…7');
    expect(save).not.toHaveBeenCalled();
  });

  it('extern geänderte Datei: nicht gespeichert, Neuladen angeboten, Eingaben bleiben', async () => {
    const stale: HubSaveResult = { ...mockHubSave('', mockHubPlan().stand), saved: false, notice: '', stale: 'data/hub.json seit dem Laden geändert' };
    vi.spyOn(backend, 'hubSave').mockResolvedValue(stale);
    const h = await mount();
    await changeArmory(h);
    await click(button(h, 'Speichern'));
    expect(h.textContent).toContain('Datei extern geändert');
    expect(field(h, 'Hub-Stufe (hubLevel)').value).toBe('5');
    const load = vi.spyOn(backend, 'hubPlan');
    await click(button(h, 'Neu laden (Änderungen verwerfen)'));
    expect(load).toHaveBeenCalled();
    expect(field(h, 'Hub-Stufe (hubLevel)').value).toBe('7');
  });

  it('Fehler im Tabellenschritt: gespeichert, „Regeltabelle veraltet“', async () => {
    const result = { ...mockHubSave('', mockHubPlan().stand), rulesError: 'Regeltabelle veraltet, task gen:rules (marken fehlen)' };
    vi.spyOn(backend, 'hubSave').mockResolvedValue(result);
    const h = await mount();
    await changeArmory(h);
    await click(button(h, 'Speichern'));
    expect(h.textContent).toContain('Regeltabelle veraltet, task gen:rules');
    expect(h.textContent).toContain('REG-Änderung');
  });
});

describe('Gebäude-Planer: Review WZ9.3', () => {
  it('Kommazahl in einem ganzzahligen Feld wird nicht übernommen', async () => {
    const check = vi.spyOn(backend, 'hubCheck');
    const h = await mount();
    await changeArmory(h);
    await type(field(h, 'Hub-Stufe (hubLevel)'), '1.5');
    expect(lastCandidate(check).sites[8].hubLevel).toBe(5);
    await type(field(h, 'Hub-Stufe (hubLevel)'), '1e20');
    expect(lastCandidate(check).sites[8].hubLevel).toBe(5);
  });

  it('nicht lesbarer Kandidat: Fehler sichtbar, nicht speicherbar', async () => {
    vi.spyOn(backend, 'hubCheck').mockImplementation(async (c) => ({ ...mockHubCheck(c), matrix: { levels: 0, rows: [] }, error: 'data/hub.json nicht lesbar als hub.json: kaputt' }));
    const h = await mount();
    await changeArmory(h);
    expect(h.textContent).toContain('nicht speicherbar: data/hub.json nicht lesbar als hub.json: kaputt');
    expect(button(h, 'Speichern').disabled).toBe(true);
    expect(h.querySelectorAll('.pl-matrix tbody tr')).toHaveLength(mockHubPlan().matrix.rows.length);
  });

  it('unveränderter Stand: eigener Hinweis ohne REG-Änderung', async () => {
    vi.spyOn(backend, 'hubSave').mockResolvedValue({ ...mockHubSave('', mockHubPlan().stand), saved: false, unchanged: true, notice: '' });
    const h = await mount();
    await changeArmory(h);
    await click(button(h, 'Speichern'));
    expect(h.textContent).toContain('Unverändert');
    expect(h.textContent).not.toContain('REG-Änderung');
  });

  it('während des Speicherns sind die Felder gesperrt', async () => {
    let finish: (r: HubSaveResult) => void = () => {};
    vi.spyOn(backend, 'hubSave').mockImplementation(() => new Promise((resolve) => { finish = resolve; }));
    const check = vi.spyOn(backend, 'hubCheck');
    const h = await mount();
    await changeArmory(h);
    await click(button(h, 'Speichern'));
    expect(h.querySelector<HTMLFieldSetElement>('fieldset.pl-fieldset')!.disabled).toBe(true);
    await type(field(h, 'Hub-Stufe (hubLevel)'), '3');
    expect(lastCandidate(check).sites[8].hubLevel).toBe(5);
    await act(async () => finish(mockHubSave('', mockHubPlan().stand)));
    await flush();
    expect(h.querySelector<HTMLFieldSetElement>('fieldset.pl-fieldset')!.disabled).toBe(false);
  });

  it('zwischen zwei Änderungen bleibt der zuletzt geprüfte Stand in der Matrix', async () => {
    let calls = 0;
    vi.spyOn(backend, 'hubCheck').mockImplementation((c) => {
      calls++;
      if (calls > 1) return new Promise(() => {});
      const v = mockHubCheck(c);
      return Promise.resolve({ ...v, matrix: { ...v.matrix, rows: v.matrix.rows.slice(0, 12) } });
    });
    const h = await mount();
    await changeArmory(h);
    expect(h.querySelectorAll('.pl-matrix tbody tr')).toHaveLength(12);
    await select(h, 'workshop@-16');
    await type(field(h, 'Platz (offsetUnits)'), '-17');
    expect(calls).toBe(2);
    expect(h.querySelectorAll('.pl-matrix tbody tr')).toHaveLength(12);
  });
});
