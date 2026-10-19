// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { mockHubPlan, mockHubRules } from '../api/mockHub';
import { PlannerPage } from './PlannerPage';

// B-462/AC-01, AC-03, AC-04, AC-08 im DOM: der Mock liefert die Antworten des Backends für die 7-Stufen-Fixture
// (api/hubFixture.json). Die Phaser-Bühne braucht ein Canvas; ihr Zusammenspiel prüft LivePreview.test.tsx.
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

const levelButtons = (h: HTMLElement) => [...h.querySelectorAll<HTMLButtonElement>('.pl-matrix thead button.pl-level')];
const row = (h: HTMLElement, key: string) => h.querySelector<HTMLTableRowElement>(`.pl-matrix tr[data-key="${key}"]`)!;
const cell = (h: HTMLElement, key: string, level: number) => row(h, key).querySelector<HTMLElement>(`td[data-level="${level}"]`)!;

afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Gebäude-Planer: Matrix (B-462/AC-01, AC-08)', () => {
  it('zeigt mit der 7-Stufen-Fixture 7 Spalten und je Bauplatz eine Zeile', async () => {
    const h = await mount();
    const plan = mockHubPlan();
    expect(levelButtons(h).map((b) => b.textContent)).toEqual([1, 2, 3, 4, 5, 6, 7].map((n) => `Hub-Stufe ${n}`));
    expect(h.querySelectorAll('.pl-matrix tbody tr')).toHaveLength(plan.matrix.rows.length);
    expect(h.textContent).toContain('7 Hub-Stufen');
    expect(h.textContent).toContain('testdata/hub/hub7.json');
  });

  it('Zelle mit Platz und Tiefe ab der Stufe des Bauplatzes, davor leer', async () => {
    const h = await mount();
    expect(cell(h, 'armory@40', 6).classList.contains('pl-off')).toBe(true);
    expect(cell(h, 'armory@40', 6).textContent).toBe('');
    expect(cell(h, 'armory@40', 7).classList.contains('pl-from')).toBe(true);
    expect(cell(h, 'armory@40', 7).textContent).toBe('+40');
    expect(cell(h, 'stairsUp@16', 2).textContent).toBe('+16ab Tiefe 1');
    expect(cell(h, 'stairsDown@8', 3).textContent).toContain('nur mit tieferer Stufe');
    expect(cell(h, 'gate@1', 2).textContent).toBe('±56');
    expect(row(h, 'line@2').querySelector('th')!.textContent).toBe('Mauerlinie 2Turm ±60');
    expect(cell(h, 'merchantBuy@16', 1).textContent).toBe('+16nur Tiefe 0');
    expect(h.textContent).toContain('Prüfung ohne Befund');
  });

  it('nennt Befunde der Prüfung und eine nicht lesbare Datei mit Pfad', async () => {
    const plan = mockHubPlan();
    const spy = vi.spyOn(backend, 'hubPlan').mockResolvedValue({ ...plan, findings: [{ rule: 'pflichtplatz', site: 'workshop', detail: 'fehlt' }] });
    const h = await mount();
    expect(h.querySelector('.pl-findings li')!.textContent).toBe('pflichtplatz: workshop: fehlt');
    spy.mockResolvedValue({ ...plan, raw: '', matrix: { levels: 0, rows: [] }, error: 'data/hub.json nicht lesbar: weg' });
    await act(async () => [...h.querySelectorAll('button')].find((b) => b.textContent === 'Neu laden')!.click());
    await flush();
    expect(h.querySelector('[role="alert"]')!.textContent).toBe('data/hub.json nicht lesbar: weg');
    expect(h.querySelector('.pl-matrix')).toBeNull();
  });
});

describe('Gebäude-Planer: Ablehnung des Backends', () => {
  it('lehnt hubPlan ab → Fehlertext sichtbar statt leerer Seite', async () => {
    vi.spyOn(backend, 'hubPlan').mockRejectedValue('Binding HubPlan abgestürzt');
    const h = await mount();
    expect(h.querySelector('[role="note"]')!.textContent).toContain('Gebäude-Planer nicht geladen');
    expect(h.textContent).toContain('Binding HubPlan abgestürzt');
    expect(h.querySelector('.pl-matrix')).toBeNull();
  });

  it('lehnt hubRules ab → Fehlertext an der Regeltabelle, die Matrix bleibt', async () => {
    vi.spyOn(backend, 'hubRules').mockRejectedValue(new Error('Regeltabelle: Absturz'));
    const h = await mount();
    expect(h.querySelector('[role="note"]')!.textContent).toContain('Regeltabelle nicht verglichen');
    expect(h.textContent).toContain('Regeltabelle: Absturz');
    expect(h.querySelector('.pl-matrix')).not.toBeNull();
  });
});

describe('Gebäude-Planer: Abweichung der Regeltabelle (B-462/AC-03)', () => {
  it('zeigt die abweichenden Zeilen mit Ist, Soll und dem Task', async () => {
    const spy = vi.spyOn(backend, 'hubRules');
    const h = await mount();
    const rules = mockHubRules('');
    expect(spy).toHaveBeenCalledWith(mockHubPlan().raw);
    expect(h.querySelector('.pl-diff [role="status"]')!.textContent).toContain('Regeltabelle weicht ab');
    expect(h.querySelector('.pl-diff')!.textContent).toContain('task rules:update');
    const lines = h.querySelectorAll('.pl-diff-table tbody tr');
    expect(lines).toHaveLength(rules.lines.length);
    expect(lines[0]!.querySelector('.pl-ist')!.textContent).toBe(rules.lines[0]!.ist);
    expect(lines[0]!.querySelector('.pl-soll')!.textContent).toBe(rules.lines[0]!.soll);
  });

  it('ohne Abweichung und mit Fehler des Vergleichs', async () => {
    const rules = mockHubRules('');
    const spy = vi.spyOn(backend, 'hubRules').mockResolvedValue({ ...rules, same: true, lines: [] });
    const h = await mount();
    expect(h.querySelector('.pl-diff')).toBeNull();
    expect(h.textContent).toContain('Regeltabelle stimmt mit dem Planer-Stand überein');
    spy.mockResolvedValue({ ...rules, error: 'Regeltabelle nicht vergleichbar: marken fehlen' });
    await act(async () => [...h.querySelectorAll('button')].find((b) => b.textContent === 'Neu laden')!.click());
    await flush();
    expect(h.querySelector('[role="alert"]')!.textContent).toContain('nicht vergleichbar');
  });
});

describe('Gebäude-Planer: Live-Vorschau (B-462/AC-04)', () => {
  it('fragt die Vorschau mit dem Planer-Stand in der gewählten Hub-Stufe ab', async () => {
    const spy = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    const raw = mockHubPlan().raw;
    expect(spy).toHaveBeenLastCalledWith(expect.objectContaining({ hubCandidate: raw, hubLevel: 7, buildings: {} }));
    await act(async () => levelButtons(h)[2]!.click());
    await flush();
    expect(spy).toHaveBeenLastCalledWith(expect.objectContaining({ hubCandidate: raw, hubLevel: 3 }));
    expect(levelButtons(h)[2]!.getAttribute('aria-pressed')).toBe('true');
    expect(cell(h, 'armory@40', 3).classList.contains('pl-chosen')).toBe(true);
  });
});
