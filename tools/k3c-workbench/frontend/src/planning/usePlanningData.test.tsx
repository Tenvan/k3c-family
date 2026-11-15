// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { PlanItem, PlanningChanged } from '../api';
import { mockPlanningData } from '../api/mockPlanning';
import { PLAN_RELOAD_MS } from '../lib/useLiveReload';
import { usePlanningData } from './usePlanningData';

// B-909 (WZ61.2): nach planning:changed nur die genannten IDs nachladen, ohne IDs die Liste, Fließtext beim Öffnen.

const list = vi.fn();
const items = vi.fn();
const handlers: Record<string, (d?: PlanningChanged | null) => void> = {};
vi.mock('../api', () => ({
  backend: {
    planningList: () => list(),
    planningItems: (ids: string[]) => items(ids),
    on: (ev: string, fn: (d?: PlanningChanged | null) => void) => { handlers[ev] = fn; return () => {}; },
  },
}));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

afterEach(() => { list.mockReset(); items.mockReset(); vi.useRealTimers(); for (const k of Object.keys(handlers)) delete handlers[k]; });

const listData = () => {
  const d = mockPlanningData();
  for (const s of d.sprints) for (const x of s.sessions) delete x.text;
  return d;
};
const ticketText = (id: string, status: string) =>
  `# ${id} · Neuer Titel\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** hoch\n- **Umgebung:** offline\n- **Status:** ${status}\n- **Sprint:** SP11\n- **Spec:** Entwurf\n`;
const item = (id: string, kind: PlanItem['kind'], text: string): PlanItem => ({ id, kind, path: `docs/${id}.md`, text });

type State = ReturnType<typeof usePlanningData>;
async function mount(sel = ''): Promise<{ get: () => State; done: () => void }> {
  let state!: State;
  const Probe = () => { state = usePlanningData(sel); return null; };
  const root = createRoot(document.createElement('div'));
  await act(async () => { root.render(<Probe />); });
  return { get: () => state, done: () => act(() => root.unmount()) };
}
const fire = (payload: PlanningChanged | null) => act(async () => {
  handlers['planning:changed']!(payload);
  await vi.advanceTimersByTimeAsync(PLAN_RELOAD_MS);
});

describe('usePlanningData', () => {
  it('AC-02: nach planning:changed mit einer ID wird nur diese ID nachgeladen, nicht die Liste', async () => {
    list.mockResolvedValue(listData());
    const m = await mount();
    vi.useFakeTimers();
    const id = m.get().data!.tickets[0]!.nr;
    items.mockResolvedValue([item(id, 'ticket', ticketText(id, 'eingeplant'))]);
    await fire({ ids: [id] });
    expect(items).toHaveBeenCalledTimes(1);
    expect(items).toHaveBeenCalledWith([id]);
    expect(list).toHaveBeenCalledTimes(1); // nur der erste Aufruf
    const t = m.get().data!.tickets.find((x) => x.nr === id)!;
    expect([t.title, t.status, t.prio]).toEqual(['Neuer Titel', 'eingeplant', 'hoch']);
    m.done();
  });

  it('mehrere Ereignisse in einem Takt sammeln ihre IDs in einem Nachladen', async () => {
    list.mockResolvedValue(listData());
    const m = await mount();
    vi.useFakeTimers();
    const [a, b] = m.get().data!.tickets.map((t) => t.nr);
    items.mockResolvedValue([]);
    await act(async () => { handlers['planning:changed']!({ ids: [a!] }); handlers['planning:changed']!({ ids: [b!, a!] }); await vi.advanceTimersByTimeAsync(PLAN_RELOAD_MS); });
    expect(items).toHaveBeenCalledTimes(1);
    expect(items.mock.calls[0]![0]).toEqual([a, b]);
    m.done();
  });

  it('ohne IDs lädt die Seite einmal die ganze Liste', async () => {
    list.mockResolvedValue(listData());
    const m = await mount();
    vi.useFakeTimers();
    await fire(null);
    expect(list).toHaveBeenCalledTimes(2);
    expect(items).not.toHaveBeenCalled();
    m.done();
  });

  it('schlägt das Nachladen fehl, lädt die Seite einmal alles; die Dokumente plan/fragen/glossar fragt sie nicht an', async () => {
    list.mockResolvedValue(listData());
    const m = await mount();
    vi.useFakeTimers();
    await fire({ ids: ['plan', 'glossar'] });
    expect(items).not.toHaveBeenCalled();
    items.mockRejectedValue(new Error('x'));
    await fire({ ids: ['B-001'] });
    expect(list).toHaveBeenCalledTimes(2);
    m.done();
  });

  it('ein berührter Sprint lädt zusätzlich die Liste', async () => {
    list.mockResolvedValue(listData());
    const m = await mount();
    vi.useFakeTimers();
    items.mockResolvedValue([item('SP11', 'sprint', '# SP11 · x\n')]);
    await fire({ ids: ['SP11'] });
    expect(items).toHaveBeenCalledWith(['SP11']);
    expect(list).toHaveBeenCalledTimes(2);
    m.done();
  });
});

describe('usePlanningData: Fließtext', () => {
  it('den Text der gewählten Session lädt die Seite beim Öffnen und nach einer Änderung neu', async () => {
    const d = listData();
    list.mockResolvedValue(d);
    const nr = d.sprints[0]!.sessions[1]!.nr;
    items.mockResolvedValue([item(nr, 'session', '# S\n\nalt\n')]);
    const m = await mount(nr);
    expect(items).toHaveBeenCalledWith([nr]);
    expect(m.get().texts[nr]).toBe('# S\n\nalt\n');
    vi.useFakeTimers();
    items.mockResolvedValue([item(nr, 'session', '# S\n\nneu\n')]);
    await fire({ ids: [nr] });
    expect(m.get().texts[nr]).toBe('# S\n\nneu\n');
    expect(list).toHaveBeenCalledTimes(1);
    m.done();
  });
});

describe('usePlanningData: Fließtext', () => {
  it('den Text der gewählten Session lädt die Seite beim Öffnen und nach einer Änderung neu', async () => {
    const d = listData();
    list.mockResolvedValue(d);
    const nr = d.sprints[0]!.sessions[1]!.nr;
    items.mockResolvedValue([item(nr, 'session', '# S\n\nalt\n')]);
    const m = await mount(nr);
    expect(items).toHaveBeenCalledWith([nr]);
    expect(m.get().texts[nr]).toBe('# S\n\nalt\n');
    vi.useFakeTimers();
    items.mockResolvedValue([item(nr, 'session', '# S\n\nneu\n')]);
    await fire({ ids: [nr] });
    expect(m.get().texts[nr]).toBe('# S\n\nneu\n');
    expect(list).toHaveBeenCalledTimes(1);
    m.done();
  });
});
