// WZ63.4: Die MCP-Server k3c-workbench und k3c-studio teilen sich kein Tool.
// Die Tool-Namen stehen in den instructions.md beider Server (ab der ersten `## `-Überschrift; der Kopf nennt nur die Zuständigkeit).
import { readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const ROOT = resolve(__dirname, '..');
const WORKBENCH = 'tools/k3c-workbench/internal/mcpsrv/instructions.md';
const STUDIO = 'tools/k3c-studio/internal/mcpsrv/instructions.md';

const read = (file: string) => readFileSync(join(ROOT, file), 'utf8');

function toolNames(file: string): Set<string> {
  const text = read(file);
  const body = text.slice(text.indexOf('\n## '));
  const names = new Set<string>();
  for (const m of body.matchAll(/`([a-z0-9]+_[a-z0-9_]+)[ `{]/g)) names.add(m[1]);
  return names;
}

describe('MCP-Server Workbench und Studio', () => {
  const wb = toolNames(WORKBENCH);
  const st = toolNames(STUDIO);

  it('jeder Server nennt Tools', () => {
    expect(wb.size).toBeGreaterThan(20);
    expect(st.size).toBeGreaterThan(10);
  });

  it('Tool-Namen sind disjunkt', () => {
    const both = [...wb].filter((n) => st.has(n));
    expect(both).toEqual([]);
  });

  it('Studio-Bereiche stehen nicht in den Workbench-Instructions und umgekehrt', () => {
    for (const n of wb) expect(n).not.toMatch(/^(world|assets|hub)_|^level_generate$/);
    for (const n of st) expect(n).toMatch(/^(world|assets|hub)_|^level_generate$/);
  });

  it('jeder Kopf verweist auf den anderen Server', () => {
    const head = (f: string) => read(f).split('\n## ')[0];
    expect(head(WORKBENCH)).toContain('k3c-studio');
    expect(head(STUDIO)).toContain('k3c-workbench');
  });
});
