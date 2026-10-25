import type { Backend, BalanceBackend, EventName, Events } from './types';

// Client-Server-Betrieb (k3c-workbench -headless/-dev, Go: uiapi.go): jede Methode von App ist ein POST auf
// /api/dev/rpc/<Methode> mit den Argumenten als JSON-Array, die Antwort ist {result} oder {error}. Ereignisse kommen
// als Server-Sent Events von /api/dev/events. Die Kopfzeile hält fremde Seiten fern (CORS-Preflight).
const API = '/api/dev/';
const HEADERS = { 'Content-Type': 'application/json', 'X-K3C-Dev': '1' };

interface Answer<T> {
  result?: T;
  error?: string;
}

async function answer<T>(res: Response): Promise<T> {
  const body = (await res.json().catch(() => ({ error: `Fehler ${res.status} von k3c-workbench` }))) as Answer<T>;
  if (!res.ok || body.error) throw new Error(body.error || `Fehler ${res.status} von k3c-workbench`);
  return body.result as T;
}

function rpc<T>(method: string, ...args: unknown[]): Promise<T> {
  return fetch(`${API}rpc/${method}`, { method: 'POST', headers: HEADERS, body: JSON.stringify(args) }).then((res) =>
    answer<T>(res),
  );
}

/** Ein Strom für alle Ereignisse; je Name ein Listener, der die Abonnenten bedient. */
function eventHub() {
  let source: EventSource | null = null;
  const listeners = new Map<string, Set<(data: unknown) => void>>();
  return <E extends EventName>(event: E, fn: (data: Events[E]) => void): (() => void) => {
    let set = listeners.get(event);
    if (!set) {
      const subs = new Set<(data: unknown) => void>();
      listeners.set(event, subs);
      set = subs;
      source ??= new EventSource(`${API}events`);
      source.addEventListener(event, (e) => {
        const data: unknown = JSON.parse((e as MessageEvent<string>).data);
        subs.forEach((f) => f(data));
      });
    }
    const f = fn as (data: unknown) => void;
    set.add(f);
    return () => {
      set.delete(f);
    };
  };
}

function httpBalance(): BalanceBackend {
  return {
    balanceStart: (spec) => rpc('BalanceStart', spec),
    balanceRuns: () => rpc('BalanceRuns'),
    balanceStop: (id) => rpc('BalanceStop', id),
    balanceReports: () => rpc('BalanceReports'),
    balanceReport: (path) => rpc('BalanceReport', path),
    balanceTuning: (path) => rpc('BalanceTuning', path),
    balanceDataFiles: () => rpc('BalanceDataFiles'),
    balanceDataDoc: (name) => rpc('BalanceDataDoc', name),
    balanceDataValues: (name) => rpc('BalanceDataValues', name),
    balanceReportRate: (path, rating) => rpc('BalanceReportRate', path, rating),
    balanceReportDelete: (path) => rpc('BalanceReportDelete', path),
    balanceInsights: (path) => rpc('BalanceInsights', path),
    balanceReportHTML: (path) => rpc('BalanceReportHTML', path),
  };
}

export function httpBackend(): Backend {
  return {
    mock: false,
    info: () => rpc('Info'),
    services: () => rpc('Services'),
    servicesReload: () => rpc('ServicesReload'),
    serviceStart: (name) => rpc('ServiceStart', name),
    serviceStop: (name, force) => rpc('ServiceStop', name, force),
    serviceRestart: (name) => rpc('ServiceRestart', name),
    servicesStartAll: () => rpc('ServicesStartAll'),
    servicesStopAll: () => rpc('ServicesStopAll'),
    serviceLogLevels: (name) => rpc('ServiceLogLevels', name),
    sources: () => rpc('Sources'),
    consoleTail: (source) => rpc('ConsoleTail', source),
    logsQuery: (source, q) => rpc('LogsQuery', source, q),
    logsErrors: (source, level) => rpc('LogsErrors', source, level),
    mcpOverview: () => rpc('McpOverview'),
    mcpRestart: () => rpc('McpRestart'),
    mcpInstructions: () => rpc('McpInstructions'),
    mcpCalls: () => rpc('McpCalls'),
    mcpUsage: () => rpc('McpUsage'),
    resources: (windowMin) => rpc('Resources', windowMin),
    tasks: () => rpc('Tasks'),
    tasksReload: () => rpc('TasksReload'),
    taskStart: (name, args) => rpc('TaskStart', name, args),
    taskStop: (name) => rpc('TaskStop', name),
    taskRuns: () => rpc('TaskRuns'),
    taskGates: () => rpc('TaskGatesView'),
    taskGateToggle: (name) => rpc('TaskGateToggle', name),
    taskGateSave: () => rpc('TaskGateSave'),
    planningData: () => rpc('PlanningData'),
    planningDocs: () => rpc('PlanningDocs'),
    planningSet: (id, field, value) => rpc('PlanningSet', id, field, value),
    planningDoc: (name) => rpc('PlanningDoc', name),
    planningGet: (id) => rpc('PlanningGet', id),
    githubStatus: (force) => rpc('GitHubStatus', force),
    worktreeCheck: () => rpc('WorktreeCheck'),
    openInClaude: (prompt) => rpc('OpenInClaude', prompt),
    studioOpen: () => rpc('StudioOpen'),
    gitOverview: () => rpc('GitOverview'),
    gitAction: (action) => rpc('GitAction', action),
    gitDeleteBranches: (names, remote) => rpc('GitDeleteBranches', names, remote),
    gitPRs: () => rpc('GitPRs'),
    gitPRCreate: (title, body) => rpc('GitPRCreate', title, body),
    gitPRAction: (number, action) => rpc('GitPRAction', number, action),
    gitCheckMerged: (names) => rpc('GitCheckMerged', names),
    gitStamp: () => rpc('GitStamp'),
    ticketCreate: (input) => rpc('TicketCreate', input),
    ticketRecent: (n) => rpc('TicketRecent', n),
    ...httpBalance(),
    openUrl: (url) => {
      window.open(url, '_blank', 'noopener');
    },
    on: eventHub(),
  };
}
