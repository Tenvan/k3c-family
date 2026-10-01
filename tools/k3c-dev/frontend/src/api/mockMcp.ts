import type {
  McpCall, McpOverview, McpState, McpUsage, SlowCall, ToolStats, ToolUsage, UsageBucket, UsageCount, UsageScope,
} from './types';

// Erfundene MCP-Daten für den Mock (B-065): Zähler, Aufruf-Log mit parallelen und laufenden Aufrufen, eine
// Minuten-Zeitreihe über 7 Tage und beide Statistik-Bereiche. Die Regeln rechnet der Mock grob wie Go (usage), exakt
// müssen sie hier nicht sein; deterministisch über einen eigenen Zufallsgenerator.

interface Tool {
  name: string;
  description: string;
  ms: [number, number]; // Spanne einer normalen Dauer
  args: string[];
  errors: string[];
}

const TOOLS: Tool[] = [
  { name: 'check_run', description: 'Prüfung aus dem Katalog, verdichtet', ms: [1400, 9000],
    args: ['{"target":"task:test"}', '{"target":"go:test"}', '{"target":"task:lint"}'], errors: ['task:test · exit 1'] },
  { name: 'logs_query', description: 'Gefilterte Log-Einträge', ms: [6, 60],
    args: ['{"source":"k3c-dev","minLevel":"WARN"}', '{"source":"k3c-dev"}'], errors: ['unbekannte Log-Quelle "x"'] },
  { name: 'logs_errors', description: 'Warnungen und Fehler, verdichtet', ms: [10, 90],
    args: ['{"source":"k3c-dev"}'], errors: [] },
  { name: 'console_tail', description: 'Letzte Zeilen einer Konsolen-Quelle', ms: [1, 6],
    args: ['{"source":"check:task:test"}', '{"source":"Vite"}'], errors: ['unbekannte Quelle "check:x"'] },
  { name: 'svc_status', description: 'Zustand aller Dienste', ms: [2, 12], args: ['{}'], errors: [] },
  { name: 'svc_start', description: 'Dienst starten und auf gesund warten', ms: [1500, 3200],
    args: ['{"service":"Vite"}'], errors: ['Vite läuft bereits (läuft)'] },
  { name: 'workbench_status', description: 'Überblick über Werkzeug, Dienste und Logs', ms: [3, 16], args: ['{}'],
    errors: [] },
  { name: 'reports_list', description: 'Berichte der Gamepad-Testseite', ms: [2, 9], args: ['{}'], errors: [] },
];

interface Event {
  at: number;
  tool: string;
  args: string;
  ms: number;
  ok: boolean;
  error: string;
}

function rng(seed: number) {
  let s = seed;
  return () => {
    s = (s * 1664525 + 1013904223) % 4294967296;
    return s / 4294967296;
  };
}

const iso = (t: number) => new Date(t).toISOString();
const pick = <T>(r: () => number, list: T[]) => list[Math.floor(r() * list.length)];

function sampleEvent(r: () => number, at: number): Event {
  const tool = pick(r, TOOLS.slice(0, r() < 0.5 ? 2 : TOOLS.length));
  const [lo, hi] = tool.ms;
  let ms = lo + (hi - lo) * r() * r();
  if (tool.name === 'check_run' && r() < 0.05) ms *= 5; // gelegentlicher Ausreißer
  const failed = tool.errors.length > 0 && r() < 0.06;
  return { at, tool: tool.name, args: pick(r, tool.args), ms, ok: !failed, error: failed ? pick(r, tool.errors) : '' };
}

function percentile(sorted: number[], q: number): number {
  return sorted.length === 0 ? 0 : sorted[Math.floor(q * (sorted.length - 1))];
}

function top(counts: Map<string, UsageCount>): UsageCount[] {
  return [...counts.values()].sort((a, b) => b.count - a.count || a.value.localeCompare(b.value)).slice(0, 10);
}

function bump(m: Map<string, UsageCount>, key: string, init: () => UsageCount) {
  const c = m.get(key) ?? init();
  c.count++;
  m.set(key, c);
}

/** p95 je Vergleichsgruppe: Tool und Argumente ab 8 Aufrufen, sonst das Tool. */
function baselines(events: Event[]) {
  const groups = new Map<string, number[]>();
  for (const e of events) {
    for (const key of [e.tool, `${e.tool} ${e.args}`]) groups.set(key, [...(groups.get(key) ?? []), e.ms]);
  }
  const p95 = new Map([...groups].map(([k, v]) => [k, percentile([...v].sort((a, b) => a - b), 0.95)]));
  return (e: Event) => ((groups.get(`${e.tool} ${e.args}`)?.length ?? 0) >= 8 ? p95.get(`${e.tool} ${e.args}`)! : p95.get(e.tool)!);
}

function toolUsage(name: string, list: Event[], isOutlier: (e: Event) => boolean): ToolUsage {
  const ms = list.map((e) => e.ms).sort((a, b) => a - b);
  const args = new Map<string, UsageCount>();
  const errors = new Map<string, UsageCount>();
  for (const e of list) {
    const argMs = list.filter((x) => x.args === e.args).map((x) => x.ms).sort((a, b) => a - b);
    bump(args, e.args, () => ({ value: e.args, count: 0, avgMs: argMs.reduce((a, b) => a + b, 0) / argMs.length,
      p95Ms: percentile(argMs, 0.95), maxMs: argMs.at(-1) ?? 0 }));
    if (!e.ok) bump(errors, e.error, () => ({ value: e.error, count: 0 }));
  }
  const sum = ms.reduce((a, b) => a + b, 0);
  return { name, calls: list.length, errors: list.filter((e) => !e.ok).length, avgMs: sum / list.length,
    p50Ms: percentile(ms, 0.5), p95Ms: percentile(ms, 0.95), maxMs: ms.at(-1) ?? 0, sumMs: sum,
    outliers: list.filter(isOutlier).length, args: top(args), topErrors: top(errors) };
}

function scopeOf(events: Event[], since: number): UsageScope {
  const base = baselines(events);
  const isOutlier = (e: Event) => e.ms >= 1000 && e.ms > 2 * base(e);
  const slow = (e: Event): SlowCall => ({ at: iso(e.at), tool: e.tool, args: e.args, durationMs: e.ms, ok: e.ok,
    baselineMs: base(e) });
  const byTool = new Map<string, Event[]>();
  for (const e of events) byTool.set(e.tool, [...(byTool.get(e.tool) ?? []), e]);
  const tools = [...byTool].map(([n, l]) => toolUsage(n, l, isOutlier))
    .sort((a, b) => b.calls - a.calls || a.name.localeCompare(b.name));
  const ms = events.map((e) => e.ms).sort((a, b) => a - b);
  const errors = new Map<string, UsageCount>();
  for (const e of events) if (!e.ok) bump(errors, `${e.tool} ${e.error}`, () => ({ tool: e.tool, value: e.error, count: 0 }));
  const sum = ms.reduce((a, b) => a + b, 0);
  return { since: iso(since), calls: events.length, errors: events.filter((e) => !e.ok).length,
    avgMs: events.length ? sum / events.length : 0, p50Ms: percentile(ms, 0.5), p95Ms: percentile(ms, 0.95),
    maxMs: ms.at(-1) ?? 0, sumMs: sum, outliers: events.filter(isOutlier).length, tools, topErrors: top(errors),
    recentOutliers: events.filter(isOutlier).reverse().slice(0, 10).map(slow),
    slowest: [...events].sort((a, b) => b.ms - a.ms).slice(0, 10).map(slow) };
}

function minutesOf(events: Event[]): UsageBucket[] {
  const byMinute = new Map<number, UsageBucket>();
  for (const e of events) {
    const ts = Math.floor(e.at / 60_000) * 60_000;
    const b = byMinute.get(ts) ?? { ts, calls: {}, errors: 0, ms: {}, maxMs: {}, outliers: {} };
    b.calls[e.tool] = (b.calls[e.tool] ?? 0) + 1;
    b.ms[e.tool] = (b.ms[e.tool] ?? 0) + e.ms;
    b.maxMs[e.tool] = Math.max(b.maxMs[e.tool] ?? 0, e.ms);
    if (!e.ok) b.errors++;
    if (e.ms > 20_000) b.outliers[e.tool] = (b.outliers[e.tool] ?? 0) + 1;
    byMinute.set(ts, b);
  }
  return [...byMinute.values()].sort((a, b) => a.ts - b.ts);
}

type Emit = (event: 'mcp:start' | 'mcp:call', call: McpCall) => void;

export function mockMcp(emit: Emit, mcpState: () => McpState) {
  const r = rng(42);
  const started = Date.now();
  const history: Event[] = [];
  for (let t = started - 7 * 24 * 3_600_000; t < started; t += 60_000 * (1 + Math.floor(r() * 9))) {
    history.push(sampleEvent(r, t));
  }
  const session: Event[] = [];
  const log: McpCall[] = [];
  const running = new Map<number, McpCall>();
  let seq = 0;
  let id = 0;

  const begin = () => {
    const e = sampleEvent(r, Date.now());
    const call: McpCall = { id: ++id, ref: `#${id}`, startedAt: iso(e.at), ts: '', startSeq: ++seq, endSeq: 0,
      running: true, tool: e.tool, args: e.args, durationMs: 0, ok: false, summary: '' };
    running.set(call.id, call);
    emit('mcp:start', call);
    setTimeout(() => {
      running.delete(call.id);
      const done: McpCall = { ...call, running: false, ts: iso(Date.now()), endSeq: ++seq,
        durationMs: Math.min(e.ms, 8000), ok: e.ok, error: e.error || undefined,
        summary: e.ok ? `${e.tool} · ok` : e.error };
      session.push({ ...e, ms: done.durationMs, at: Date.now() });
      log.unshift(done);
      log.length = Math.min(log.length, 200);
      emit('mcp:call', done);
    }, Math.min(e.ms, 8000));
  };
  setInterval(() => {
    begin();
    if (r() < 0.3) begin(); // ab und zu zwei parallel
  }, 2500);

  const overview = async (): Promise<McpOverview> => {
    const tools: ToolStats[] = TOOLS.map((t) => {
      const list = session.filter((e) => e.tool === t.name);
      const last = list.at(-1);
      return { name: t.name, description: t.description, calls: list.length, errors: list.filter((e) => !e.ok).length,
        avgMs: list.length ? list.reduce((a, e) => a + e.ms, 0) / list.length : 0,
        lastCall: last ? `${new Date(last.at).toLocaleTimeString('de-DE')} ${last.args}` : '' };
    });
    return { mcp: mcpState(), stats: { startedAt: iso(started), clients: 2, peakClients: 3, inFlight: running.size,
      peakInFlight: 3, totalCalls: session.length, errors: session.filter((e) => !e.ok).length, tools } };
  };

  return {
    mcpOverview: overview,
    mcpCalls: async (): Promise<McpCall[]> => [...running.values()].reverse().concat(log),
    mcpUsage: async (): Promise<McpUsage> => ({
      session: scopeOf(session, started), allTime: scopeOf([...history, ...session], started - 7 * 24 * 3_600_000),
      minutes: minutesOf([...history, ...session]),
      rules: { outlierFactor: 2, outlierFloorMs: 1000, baselineCalls: 8, percentileErrorPct: 12 },
    }),
  };
}
