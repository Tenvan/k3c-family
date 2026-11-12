// Hook PreToolUse (.claude/settings.json): Planungsdateien ändert nur die k3c-workbench (MCP plan_*). Läuft die Workbench,
// lehnt der Hook Edit/Write und schreibende Shell-Befehle auf docs/backlog, docs/sprints und docs/projekte ab; läuft sie
// nicht, gilt die Ausnahme aus CLAUDE.md (von Hand erlaubt). Lesen bleibt immer erlaubt.
import { connect } from 'node:net';

export const PLAN_DIRS = /(^|[\s\\/"'=])docs[\\/](backlog|sprints|projekte)([\\/]|$)/i;

// Schreibende Befehle in Bash und PowerShell; Lesen (cat, grep, git log/diff/show/add/commit) bleibt frei.
const WRITES = [
  /\bsed\s+(-[a-z]*i|--in-place)/i, /(^|[^>&0-9])>>?\s*["']?[^\s|&>]*docs[\\/]/i, /\b(rm|mv|cp|touch|tee|truncate|mkdir|rmdir)\b/i,
  /\b(Set-Content|Add-Content|Out-File|Remove-Item|Move-Item|Copy-Item|New-Item|Rename-Item)\b/i,
  /\bgit\s+(checkout|restore|rm|mv|reset|stash|apply|am)\b/i, /\b(python|python3|node|bun|perl|ruby)\b/i,
];

export type HookInput = { tool_name?: string; tool_input?: Record<string, unknown> };

/** Grund der Ablehnung oder null, wenn der Aufruf keine Planungsdatei ändert (ohne Blick auf die Workbench). */
export function planWrite(input: HookInput): string | null {
  const tool = input.tool_name ?? '';
  const ti = input.tool_input ?? {};
  if (['Edit', 'Write', 'MultiEdit', 'NotebookEdit'].includes(tool)) {
    const file = String(ti.file_path ?? ti.notebook_path ?? '');
    return PLAN_DIRS.test(file) ? `${tool} auf ${file}` : null;
  }
  if (tool === 'Bash' || tool === 'PowerShell') {
    const cmd = String(ti.command ?? '');
    return PLAN_DIRS.test(cmd) && WRITES.some((w) => w.test(cmd)) ? `${tool}-Befehl schreibt in Planungsdateien` : null;
  }
  return null;
}

/** Läuft die Workbench (MCP-Port 127.0.0.1:K3C_DEV_PORT, Standard 5180)? */
export function workbenchUp(port = Number(process.env.K3C_DEV_PORT ?? 5180), timeoutMs = 500): Promise<boolean> {
  return new Promise((resolve) => {
    const s = connect({ host: '127.0.0.1', port });
    const done = (ok: boolean) => { s.destroy(); resolve(ok); };
    s.setTimeout(timeoutMs, () => done(false));
    s.once('connect', () => done(true));
    s.once('error', () => done(false));
  });
}

async function main(): Promise<void> {
  const raw = await new Response(Bun.stdin.stream()).text();
  let input: HookInput = {};
  try { input = JSON.parse(raw) as HookInput; } catch { return; }
  const what = planWrite(input);
  if (!what || !(await workbenchUp())) return;
  const reason = `🚫 Planung nur über die k3c-workbench: ${what} ist abgelehnt. Tickets, Sprints, Sessions und Projekte ` +
    'mit mcp__k3c-workbench__plan_create, plan_set, plan_section(s), plan_delete ändern (lesen: plan_list, plan_get, plan_next).';
  process.stdout.write(JSON.stringify({ hookSpecificOutput: { hookEventName: 'PreToolUse', permissionDecision: 'deny', permissionDecisionReason: reason } }));
}

if (import.meta.main) await main();
