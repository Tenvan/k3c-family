import { describe, expect, it } from 'vitest';
import { tempWrite } from '../tools/hooks/tempGuard';

const sh = (command: string) => tempWrite({ tool_name: 'Bash', tool_input: { command } });

describe('tempGuard: Go-Temp nur im Repo', () => {
  it('lehnt direkte go-Aufrufe ohne GOTMPDIR ab', () => {
    expect(sh('go run ./tools/checkdiff')).not.toBeNull();
    expect(sh('cd tools/k3c-workbench && go test ./internal/playtest')).not.toBeNull();
    expect(tempWrite({ tool_name: 'PowerShell', tool_input: { command: 'go build ./...; go vet ./...' } })).not.toBeNull();
  });

  it('lässt task, check_run, gesetztes GOTMPDIR und andere Befehle frei', () => {
    expect(sh('task check:diff')).toBeNull();
    expect(sh('GOTMPDIR=$PWD/.work/tmp go test ./pkg')).toBeNull();
    expect(tempWrite({ tool_name: 'PowerShell', tool_input: { command: '$env:GOTMPDIR="x"; go test ./a' } })).toBeNull();
    expect(sh('git log --oneline')).toBeNull();
    expect(tempWrite({ tool_name: 'Edit', tool_input: { command: 'go test' } })).toBeNull();
  });
});
