import type { Backend, EventName, Events } from './types';

// Das Fenster der Rust-Fassung (Tauri 2, withGlobalTauri, kein npm-Paket): jede Methode läuft über den Command `rpc`
// mit derselben Aufruf-Tabelle wie die HTTP-API (Namen und Argumente wie in http.ts), Ereignisse kommen über
// `event.listen`. Ein Ereignis vor dem Listener geht verloren; die Oberfläche holt den Stand beim Start per `info`.

interface TauriGlobal {
  core: { invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> };
  event: { listen<T>(name: string, fn: (e: { payload: T }) => void): Promise<() => void> };
}

const tauri = (): TauriGlobal | undefined => (window as unknown as { __TAURI__?: TauriGlobal }).__TAURI__;

export function hasTauri(): boolean {
  return tauri() !== undefined;
}

/** Ruft eine Methode der Aufruf-Tabelle. Ein abgelehnter Command liefert den Fehlertext; die Seiten erwarten wie bei den anderen Transporten ein `Error`. */
export function tauriRpc<T>(method: string, ...args: unknown[]): Promise<T> {
  return tauri()!
    .core.invoke<T>('rpc', { method, args })
    .catch((e: unknown) => {
      throw new Error(String(e));
    });
}

function on<E extends EventName>(event: E, fn: (data: Events[E]) => void): () => void {
  let off: (() => void) | undefined;
  let dead = false;
  void tauri()!
    .event.listen<Events[E]>(event, (e) => {
      if (!dead) fn(e.payload);
    })
    .then((unlisten) => {
      if (dead) unlisten();
      else off = unlisten;
    });
  return () => {
    dead = true;
    off?.();
    off = undefined;
  };
}

export function tauriBackend(): Backend {
  const rpc = tauriRpc;
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
    planningData: () => rpc('PlanningData'),
    planningList: () => rpc('PlanningList'),
    planningItems: (ids) => rpc('PlanningItems', ids),
    planningDocs: () => rpc('PlanningDocs'),
    planningSet: (id, field, value) => rpc('PlanningSet', id, field, value),
    planningDoc: (name) => rpc('PlanningDoc', name),
    planningGet: (id) => rpc('PlanningGet', id),
    githubStatus: (force) => rpc('GitHubStatus', force),
    worktreeCheck: () => rpc('WorktreeCheck'),
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
    openUrl: (url) => {
      void tauri()!
        .core.invoke('open_url', { url })
        .catch((e: unknown) => console.error('open_url', e));
    },
    on,
  };
}
