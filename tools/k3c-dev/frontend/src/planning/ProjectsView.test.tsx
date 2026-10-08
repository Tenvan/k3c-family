import { Theme } from '@radix-ui/themes';
import type { ReactElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import type { PlanSprint } from '../api';
import { mockPlanningData } from '../api/mockPlanning';
import { ProjectCard } from './ProjectCard';
import { ProjectSections } from './ProjectsView';
import { groupProjects, rankedCount } from './projects';

vi.mock('../api', () => ({ backend: {} })); // statisch gerendert, ohne Fenster: kein Backend nötig

const d = mockPlanningData();
const g = groupProjects(d);
const render = (el: ReactElement) => renderToStaticMarkup(<Theme>{el}</Theme>);
const sprint = (s: PlanSprint) => <div key={s.id} data-sprint={s.id} />;
const props = { data: d, sel: '', renderSprint: sprint };
const html = render(<ProjectSections groups={g} count={rankedCount(d.projects)} error={null} onMove={() => {}} {...props} />);
const order = (...xs: string[]) => xs.map((x) => html.indexOf(x));

describe('ProjectSections mit Mock-Daten', () => {
  it('Projekte nach Rang mit Sprints, Fortschritt und nächster Session (AC-06)', () => {
    const at = order('pl-pj-BET', 'pl-pj-SKL', 'pl-pj-RGW');
    expect(at.every((x, i) => x >= 0 && (i === 0 || x > at[i - 1]))).toBe(true);
    expect(html).toContain('SP11 · aktiv · 1/4');
    expect(html).toContain('Sprints 0/2');
    expect(html).toMatch(/Nächste Session: <strong>SP11\.3<\/strong>/);
    expect(html).toContain('data-sprint="SP11"');
  });

  it('ABN, „Ohne Projekt“ und eingeklappte ruhende/erledigte als eigene Bereiche (AC-08)', () => {
    const [abn, ohne, folded] = order('data-area="abn"', 'data-area="ohne"', 'data-area="eingeklappt"');
    expect(abn).toBeGreaterThan(html.indexOf('pl-pj-RGW'));
    expect(ohne).toBeGreaterThan(abn);
    expect(folded).toBeGreaterThan(ohne);
    expect(html).toContain('X1.1 · Gamepad am TV');
    expect(html).toContain('data-sprint="M5"');
    expect(html).toContain('<details data-area="eingeklappt">'); // ohne open: eingeklappt
    expect(html.slice(folded)).toContain('pl-pj-GRA');
    expect(html.slice(folded)).toContain('pl-pj-FND');
    expect(html.slice(folded)).not.toContain('>hoch<'); // ohne Rang keine Knöpfe
  });
});

describe('ProjectCard Rang-Knöpfe (AC-07)', () => {
  const buttons = (i: number, error?: string) =>
    render(<ProjectCard view={g.active[i]} count={3} error={error} {...props} />).match(/<button[^>]*>(hoch|runter)<\/button>/g) ?? [];

  it('am Rand deaktiviert', () => {
    const [up, down] = buttons(0);
    expect(up).toContain('disabled');
    expect(down).not.toContain('disabled');
    expect(buttons(2)[1]).toContain('disabled');
  });

  it('Ablehnung zeigt den Grund', () => {
    const html = render(<ProjectCard view={g.active[2]} count={3} error="Rang 4 außerhalb 1 … 3" {...props} />);
    expect(html).toContain('Rang nicht geändert');
    expect(html).toContain('Rang 4 außerhalb 1 … 3');
  });
});
