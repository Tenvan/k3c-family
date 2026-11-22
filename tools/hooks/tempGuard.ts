// Hook PreToolUse (.claude/settings.json): Regel "Schreiben nur im Repo" (CLAUDE.md). `go run`, `go test`, `go build` und
// `go vet` legen ihre EXE (go-build…\checkdiff.exe, *.test.exe) sonst in %TEMP% ab. Über `task` und `check_run` setzt das
// Taskfile GOTMPDIR, TMP und TEMP; ein direkter Aufruf in der Shell (auch von Agenten) muss sie selbst setzen.
import { Buffer } from 'node:buffer';
import process from 'node:process';

export type HookInput = { tool_name?: string; tool_input?: Record<string, unknown> };

const GO_BUILDS = /(^|[\s;&|(])go\s+(run|test|build|vet|install|generate)\b/i;
const TEMP_SET = /\b(GOTMPDIR|K3C_TMP)\b/i;

/** Grund der Ablehnung oder null, wenn der Aufruf kein Go-Temp außerhalb des Repos erzeugen kann. */
export function tempWrite(input: HookInput): string | null {
  const tool = input.tool_name ?? '';
  if (tool !== 'Bash' && tool !== 'PowerShell') return null;
  const cmd = String(input.tool_input?.command ?? '');
  return GO_BUILDS.test(cmd) && !TEMP_SET.test(cmd) ? `${tool}-Befehl baut mit go ohne GOTMPDIR (EXE landet in %TEMP%)` : null;
}

async function main(): Promise<void> {
  const chunks: Buffer[] = [];
  for await (const c of process.stdin) chunks.push(c as Buffer);
  let input: HookInput = {};
  try { input = JSON.parse(Buffer.concat(chunks).toString('utf8')) as HookInput; } catch { return; }
  const what = tempWrite(input);
  if (!what) return;
  const reason = `🚫 Schreiben nur im Repo: ${what}. Prüfen über check_run oder task (setzen GOTMPDIR selbst). Direkter go-Aufruf nur ` +
    'mit Temp im Repo: Bash `GOTMPDIR=$PWD/.work/tmp TMP=$PWD/.work/tmp TEMP=$PWD/.work/tmp go test …` (Repo-Wurzel, .work/tmp anlegen), ' +
    'PowerShell `$env:GOTMPDIR=…; $env:TMP=…; $env:TEMP=…` im selben Befehl; Reste danach löschen.';
  process.stdout.write(JSON.stringify({ hookSpecificOutput: { hookEventName: 'PreToolUse', permissionDecision: 'deny', permissionDecisionReason: reason } }));
}

// Nur als Hook-Skript ausführen, nicht beim Import im Test.
if (/tempGuard\.ts$/.test(process.argv[1] ?? '')) await main();
