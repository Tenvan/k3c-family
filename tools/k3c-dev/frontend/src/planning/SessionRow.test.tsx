import { Theme } from '@radix-ui/themes';
import type { ReactElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import type { PlanSession, PlanSprint } from '../api';
import { SessionRow } from './SprintCard';

vi.mock('../api', () => ({ backend: {} }));

const render = (el: ReactElement) => renderToStaticMarkup(<Theme>{el}</Theme>);
const sprint: PlanSprint = { id: 'WZ2', title: 'Planungs-Werkzeuge', domain: 'DEV', status: 'aktiv', reife: 'bereit', spec: 'freigegeben', tickets: [], sessions: [] };
const session = (titel: string): PlanSession => ({ nr: 'WZ2.1', typ: 'Umsetzung', agent: 'autonom', status: 'offen', titel });
const row = (x: PlanSession) => render(<SessionRow sprint={sprint} x={x} onSelect={() => {}} checked={false} onCheck={() => {}} />);

describe('Session-Zeile zeigt den Titel (B-402)', () => {
  it('AC-02: der Titel steht in der Zeile, Typ und Agent im Tooltip', () => {
    const html = row(session('Ticket-Nummern über Worktrees'));
    expect(html).toContain('>Ticket-Nummern über Worktrees<');
    expect(html).toContain('title="Ticket-Nummern über Worktrees · Umsetzung · autonom"');
  });

  it('AC-02: ohne Titel stehen Typ und Agent in der Zeile', () => {
    expect(row(session(''))).toContain('>Umsetzung · autonom<');
  });
});
