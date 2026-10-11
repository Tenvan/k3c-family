import { Theme } from '@radix-ui/themes';
import type { ReactElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { mockPlanningData } from '../api/mockPlanning';
import { Splitter } from '../tasks/TaskBars';
import { ProjectSections } from './ProjectsView';
import { groupProjects, rankedCount } from './projects';

vi.mock('../api', () => ({ backend: {} }));

const render = (el: ReactElement) => renderToStaticMarkup(<Theme>{el}</Theme>);

describe('Splitter der Planungsseite (B-400)', () => {
  it('AC-01: waagerechter Balken mit Anteil, Grenzen und Klasse', () => {
    const html = render(<Splitter horizontal className="pl-split" value={35} onChange={() => {}} onReset={() => {}} />);
    expect(html).toContain('role="separator"');
    expect(html).toContain('aria-orientation="horizontal"');
    expect(html).toContain('aria-valuenow="35"');
    expect(html).toContain('class="pl-split"');
  });

  it('AC-01: der Balken der Tasks-Seite bleibt senkrecht', () => {
    const html = render(<Splitter value={40} onChange={() => {}} onReset={() => {}} />);
    expect(html).toContain('aria-orientation="vertical"');
    expect(html).toContain('class="tk-split"');
  });

  it('AC-01: die Projekt-Ansicht hat keinen Splitter', () => {
    const d = mockPlanningData();
    const html = render(<ProjectSections groups={groupProjects(d)} count={rankedCount(d.projects)} error={null} onMove={() => {}}
      data={d} sel="" renderSprint={() => null} />);
    expect(html).not.toContain('role="separator"');
  });
});
