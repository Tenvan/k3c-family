// Dev-Mode (`/cockpit-dev`): Mockdaten für das Panel, damit jede Karte in allen Zuständen sichtbar ist,
// ohne dass eine Session sie erst erzeugen muss. Rein, nur von `now` abhängig; echte Atome bleiben unberührt.
import type { Agent, BgTask, Breakdown, Decision, Gate, LogEntry, McpSchema, Mr, MrMine, Repo, Turn, TurnRecord, Usage } from '../types'

export type Mock = {
  usage: Usage
  turn: Turn
  gate: Gate
  agents: Agent[]
  log: LogEntry[]
  lastTurnAt: number
  cacheHit: number
  bgTasks: BgTask[]
  turnLog: TurnRecord[]
  repo: Repo
  breakdown: Breakdown
  mcpTools: McpSchema[]
  usedMcp: string[]
  summary: string
  mr: Mr
  mrMine: MrMine[]
}

const MIN = 60_000
const TRAIL: Decision[] = ['allow', 'allow', 'you', 'allow', 'you', 'deny', 'allow', 'you', 'allow', 'ask', 'allow', 'you', 'you', 'allow', 'allow', 'you', 'allow', 'ask']

export function mockData(now: number): Mock {
  const iso = (ms: number) => new Date(now + ms).toISOString()
  return {
    usage: {
      contextTokens: 412_000,
      contextWindow: 1_000_000,
      contextPercent: 41.2,
      limits: [
        { kind: 'five_hour', percentUsed: 62, resetsAt: iso(118 * MIN) },
        { kind: 'seven_day', percentUsed: 91, resetsAt: iso(30 * 60 * MIN) },
      ],
      usd: 7.84,
    },
    // Laufender Turn: zeigt die Karte „Agent arbeitet“.
    turn: { startedAt: now - 83_000, tools: 14, edits: 3, errors: 1, usdDelta: 0.42 },
    gate: { allow: 9, you: 6, ask: 2, deny: 1, file: 7, shell: 8, other: 3, trail: TRAIL, asked: [] },
    agents: [
      { id: 'm1', type: 'Explore', description: 'Aufrufer von loadRepo finden', status: 'completed', since: now - 9 * MIN, durationMs: 41_000, model: 'haiku', tokensIn: 38_000, tokensOut: 1_900, steps: 7 },
      { id: 'm2', type: 'oh-my-claudecode:executor', description: 'Mock-Modus verdrahten', status: 'running', since: now - 6 * MIN, model: 'sonnet', tokensIn: 121_000, tokensOut: 8_400, steps: 42, background: true },
      { id: 'm3', type: 'general-purpose', description: 'Changelog prüfen', status: 'failed', since: now - 4 * MIN, durationMs: 12_000, model: 'opus', steps: 3 },
    ],
    log: [
      { at: now - 5 * MIN, src: 'you', text: 'Bitte die Panel-Karten farbig absetzen' },
      { at: now - 5 * MIN + 4_000, src: 'main', text: '', tool: 'Grep', target: 'backgroundColor' },
      { at: now - 4 * MIN, src: 'gate', text: 'Bash freigegeben (von dir)' },
      { at: now - 4 * MIN + 9_000, src: 'main', text: '', tool: 'Bash', target: 'claude plugin test .', isError: true },
      { at: now - 3 * MIN, src: 'agent', text: 'executor gestartet: Mock-Modus verdrahten' },
      { at: now - 2 * MIN, src: 'main', text: '', tool: 'mcp__serena__find_symbol', target: 'mockData' },
      { at: now - MIN, src: 'main', text: '', tool: 'Edit', target: 'hooks/register.tsx' },
    ],
    lastTurnAt: now - 4 * MIN,
    cacheHit: 87,
    bgTasks: [
      { id: 'b1', description: 'Tests im Watch-Modus', command: 'claude plugin test . --watch', status: 'running', since: now - 7 * MIN },
      { id: 'b0', description: 'Abhängigkeiten installieren', status: 'completed', since: now - 20 * MIN, durationMs: 64_000 },
      {
        id: 'b3',
        description: 'Build und Release-Check',
        // Zehn Zeilen Befehl plus Ergebnis: zeigt die Kappung auf TASK_LINES.
        command: ['npm ci', 'npm run build', 'claude plugin validate .', 'claude plugin test .', 'node scripts/build-registry.js', 'git status --short', 'git diff --stat', 'towncrier build --draft', 'npm pack --dry-run'].join('\n'),
        status: 'failed',
        since: now - 3 * MIN,
        durationMs: 52_000,
        summary: 'claude plugin test: 1 fail',
      },
      { id: 'b2', description: 'Typecheck', command: 'tsc -p . --noEmit', status: 'completed', since: now - 12 * MIN, durationMs: 18_000, summary: '0 Fehler' },
    ],
    turnLog: [
      { usd: 0.08, tools: 3, edits: 1, durationMs: 21_000 },
      { usd: 0.31, tools: 9, edits: 0, durationMs: 74_000 },
      { usd: 0.12, tools: 5, edits: 2, durationMs: 33_000 },
      { usd: 0.44, tools: 17, edits: 0, durationMs: 128_000 },
      { usd: 0.19, tools: 6, edits: 3, durationMs: 41_000 },
      { usd: 0.42, tools: 14, edits: 3, durationMs: 83_000 },
    ],
    repo: { name: 'k3c', isLinked: true, branch: 'feature/dev-mock', sha: 'a1b2c3d', dirty: 4, ahead: 2, behind: 1, path: 'C:/WORKSPACE/demo/k3c', subdir: 'tools/cockpit' },
    breakdown: {
      total: 412_000,
      window: 1_000_000,
      percent: 41.2,
      rows: [
        { name: 'System prompt', tokens: 9_800, percent: 1, kind: 'used' },
        { name: 'System tools', tokens: 21_400, percent: 2.1, kind: 'used' },
        { name: 'MCP tools', tokens: 18_200, percent: 1.8, kind: 'used' },
        { name: 'Memory files', tokens: 6_100, percent: 0.6, kind: 'used' },
        { name: 'Messages', tokens: 356_500, percent: 35.7, kind: 'used' },
        { name: 'Free space', tokens: 555_000, percent: 55.5, kind: 'free' },
        { name: 'Autocompact buffer', tokens: 33_000, percent: 3.3, kind: 'buffer' },
      ],
    },
    // Ein nie genutzter Server, damit die Kontext-Diät erscheint.
    mcpTools: [
      { name: 'mcp__jira__search', serverName: 'jira', tokens: 3_900, isLoaded: true },
      { name: 'mcp__jira__create_issue', serverName: 'jira', tokens: 2_700, isLoaded: true },
      { name: 'mcp__serena__find_symbol', serverName: 'serena', tokens: 1_200, isLoaded: true },
    ],
    usedMcp: ['serena'],
    // Offener MR mit laufender Pipeline, halber Freigabe und zwei offenen Threads: zeigt alle Statusteile und Aktionen.
    mr: {
      forge: 'gitlab',
      iid: 42,
      title: 'Cockpit: Merge Requests im Band und Panel',
      state: 'opened',
      draft: false,
      url: 'https://gitlab.example/consulting/demo/-/merge_requests/42',
      source: 'feature/mr-cockpit',
      target: 'main',
      mergeStatus: 'ci_still_running',
      conflicts: false,
      notes: 5,
      threads: 2,
      pipeline: { id: 9001, status: 'running', url: 'https://gitlab.example/consulting/demo/-/pipelines/9001' },
      approvals: { required: 2, left: 1, by: ['kollegin'] },
      description: '## Was\n- MR-Zeile im Band, eigenes Panel, Knopf unter dem Prompt\n\n## Prüfen\n- `/cockpit-dev` und `/cockpit-mr`',
      author: 'rstich',
      updatedAt: new Date(now - 25 * MIN).toISOString(),
      changes: '9',
      threadList: [
        { path: 'hooks/mr.ts', line: 42, author: 'kollegin', body: 'Bitte auch geschlossene MRs im Watch berücksichtigen.' },
        { path: 'hooks/register.tsx', line: 815, author: 'kollegin', body: 'Polling bei inaktiver Session aussetzen?' },
      ],
    },
    mrMine: [
      { ref: 'consulting/demo!42', title: 'Cockpit: Merge Requests im Band und Panel', url: 'https://gitlab.example/consulting/demo/-/merge_requests/42', draft: false },
      { ref: 'team/report-tool!17', title: 'Report-Export vereinfachen', url: 'https://gitlab.example/team/report-tool/-/merge_requests/17', draft: true },
    ],
    summary: [
      'Ralf baut das Tenvan CC weiter aus; Schwerpunkt heute ist die Optik des Panels.',
      '## Ziel',
      '- Panel-Karten ohne Rahmen, mit gedimmtem Grund und Abstand',
      '- Dev-Mode mit Mockdaten für die Weiterentwicklung',
      '## Erledigt',
      '- Kartenfarben im Wechsel kühl/warm, Gate nach Zustand gefärbt',
      '- Hintergrundaufgaben als Kacheln, Befehl auf 8 Zeilen gekappt',
      '- /cockpit-dev schaltet Mockdaten ein und aus',
      '## Offen',
      '- Band im Dev-Mode noch mit echten Werten',
      '- Lesbarkeit der Zusammenfassung prüfen',
    ].join('\n'),
  }
}
