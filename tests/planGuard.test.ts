import { describe, expect, it } from 'vitest';
import { planWrite } from '../tools/hooks/planGuard';

const call = (tool_name: string, tool_input: Record<string, unknown>) => planWrite({ tool_name, tool_input });

describe('planGuard: Planungsdateien nur über die Workbench', () => {
  it('lehnt Edit und Write auf Tickets, Sprints und Projekte ab', () => {
    expect(call('Edit', { file_path: 'C:\\WORKSPACE\\k3c\\docs\\backlog\\B-001-x.md' })).toContain('Edit');
    expect(call('Write', { file_path: '/repo/docs/sprints/aktiv/X1/README.md' })).toContain('Write');
    expect(call('Write', { file_path: '/repo/docs/projekte/NAT.md' })).not.toBeNull();
  });

  it('lässt andere Dateien und Doku außerhalb der Planung frei', () => {
    expect(call('Edit', { file_path: '/repo/docs/decisions/009.md' })).toBeNull();
    expect(call('Write', { file_path: '/repo/docs/glossar.md' })).toBeNull();
    expect(call('Read', { file_path: '/repo/docs/backlog/B-001-x.md' })).toBeNull();
  });

  it('lehnt schreibende Shell-Befehle ab, Lesen und git add/commit bleiben frei', () => {
    expect(call('Bash', { command: "sed -i 's/a/b/' docs/backlog/B-001-x.md" })).not.toBeNull();
    expect(call('Bash', { command: 'rm docs/sprints/geplant/X1/README.md' })).not.toBeNull();
    expect(call('Bash', { command: 'echo x > docs/backlog/README.md' })).not.toBeNull();
    expect(call('PowerShell', { command: 'Set-Content docs\\backlog\\B-1.md "x"' })).not.toBeNull();
    expect(call('Bash', { command: 'grep -n Status docs/backlog/B-001-x.md' })).toBeNull();
    expect(call('Bash', { command: 'git add docs && git commit -m "docs: x"' })).toBeNull();
    expect(call('Bash', { command: 'cat docs/sprints/README.md | head' })).toBeNull();
  });
});
