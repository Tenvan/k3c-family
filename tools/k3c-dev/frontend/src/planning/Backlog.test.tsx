import { Theme } from '@radix-ui/themes';
import type { ReactElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { mockPlanningData } from '../api/mockPlanning';
import { BacklogView, projectOptions, setField } from './Backlog';
import { PlanDetail } from './Detail';

const planningSet = vi.fn();
vi.mock('../api', () => ({ backend: { planningSet: (...a: unknown[]) => planningSet(...a) } }));

const d = mockPlanningData();
const render = (el: ReactElement) => renderToStaticMarkup(<Theme>{el}</Theme>);
const sessions = d.sprints.flatMap((s) => s.sessions);
const count = (html: string, label: string) => html.split(`aria-label="${label}`).length - 1;

describe('Planungsseite: Flags bearbeiten (B-363)', () => {
  beforeEach(() => { planningSet.mockClear(); });

  it('AC-01: jedes Ticket bietet Prio, Umgebung und Projekt als Menü an', () => {
    const html = render(<BacklogView data={d} tickets={d.tickets} sel="" onPick={() => {}} target={{ kind: 'new' }} goal="__new__" choose={() => {}} />);
    expect(d.tickets.length).toBeGreaterThan(0);
    for (const label of ['Prio: ', 'Umgebung: ', 'Projekt: ']) expect(count(html, label)).toBe(d.tickets.length);
  });

  it('AC-01: die Projekt-Werte sind „–“ und die Kürzel der Projekte', () => {
    expect(projectOptions(d)).toEqual(['–', ...(d.projects ?? []).map((p) => p.id)]);
  });

  it('AC-02: Agent und Umgebung nur bei offenen Sessions', () => {
    const open = sessions.find((x) => x.status === 'offen');
    const done = sessions.find((x) => x.status === 'fertig');
    expect(open && done).toBeTruthy();
    const openHtml = render(<PlanDetail data={d} sel={open!.nr} onSelect={() => {}} onClose={() => {}} target={{ kind: 'new' }} />);
    const doneHtml = render(<PlanDetail data={d} sel={done!.nr} onSelect={() => {}} onClose={() => {}} target={{ kind: 'new' }} />);
    expect(count(openHtml, 'Agent: ')).toBe(1);
    expect(count(openHtml, 'Umgebung: ')).toBe(1);
    expect(count(doneHtml, 'Agent: ')).toBe(0);
    expect(count(doneHtml, 'Umgebung: ')).toBe(0);
  });

  it('AC-03: ein abgelehnter Wert zeigt die Meldung des Tools', async () => {
    const alert = vi.fn();
    vi.stubGlobal('window', { alert });
    planningSet.mockImplementation(() => Promise.reject(new Error('Projekt "ZZZ" gibt es nicht')));
    await setField('B-001', 'Projekt', 'ZZZ');
    expect(planningSet).toHaveBeenCalledWith('B-001', 'Projekt', 'ZZZ');
    expect(alert).toHaveBeenCalledWith('Projekt "ZZZ" gibt es nicht');
    vi.unstubAllGlobals();
  });
});
