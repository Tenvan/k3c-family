import type {
  Backend, ConsoleLine, ErrorsView, EventName, Events, Info, LevelCounts, LogQuery, LogView, McpCall, McpOverview,
  McpState, McpUsage, ServicesView, ServiceStatus, Source, TaskCatalog, TaskRun, PlanDoc,
} from './types';

// Die Wails-Laufzeit legt window.go (Bindings der App) und window.runtime an. Die generierten Dateien unter
// frontend/wailsjs/ bindet die Oberfläche bewusst nicht ein: so braucht der Typecheck keine Wails-CLI.
interface GoApp {
  Info(): Promise<Info>;
  Services(): Promise<ServicesView>;
  ServiceStart(name: string): Promise<ServiceStatus>;
  ServiceStop(name: string, force: boolean): Promise<ServiceStatus>;
  ServiceRestart(name: string): Promise<ServiceStatus>;
  ServicesStartAll(): Promise<void>;
  ServicesStopAll(): Promise<void>;
  ServiceLogLevels(name: string): Promise<LevelCounts>;
  Sources(): Promise<Source[]>;
  ConsoleTail(source: string): Promise<ConsoleLine[]>;
  LogsQuery(source: string, q: LogQuery): Promise<LogView>;
  LogsErrors(source: string, level: string): Promise<ErrorsView>;
  McpOverview(): Promise<McpOverview>;
  McpRestart(): Promise<McpState>;
  McpInstructions(): Promise<string>;
  McpCalls(): Promise<McpCall[]>;
  McpUsage(): Promise<McpUsage>;
  Tasks(): Promise<TaskCatalog>;
  TasksReload(): Promise<TaskCatalog>;
  TaskStart(name: string, args: string[]): Promise<TaskRun>;
  TaskStop(name: string): Promise<TaskRun>;
  TaskRuns(): Promise<TaskRun[]>;
  PlanningPage(): Promise<string>;
  PlanningDoc(name: PlanDoc): Promise<string>;
}

interface WailsRuntime {
  EventsOn(name: string, fn: (...data: unknown[]) => void): () => void;
}

declare global {
  interface Window {
    go?: { main?: { App?: GoApp } };
    runtime?: WailsRuntime;
  }
}

export function hasWails(): boolean {
  return window.go?.main?.App !== undefined && window.runtime !== undefined;
}

export function wailsBackend(): Backend {
  const app = window.go!.main!.App!;
  const runtime = window.runtime!;
  return {
    mock: false,
    info: () => app.Info(),
    services: () => app.Services(),
    serviceStart: (name) => app.ServiceStart(name),
    serviceStop: (name, force) => app.ServiceStop(name, force),
    serviceRestart: (name) => app.ServiceRestart(name),
    servicesStartAll: () => app.ServicesStartAll(),
    servicesStopAll: () => app.ServicesStopAll(),
    serviceLogLevels: (name) => app.ServiceLogLevels(name),
    sources: () => app.Sources(),
    consoleTail: (source) => app.ConsoleTail(source),
    logsQuery: (source, q) => app.LogsQuery(source, q),
    logsErrors: (source, level) => app.LogsErrors(source, level),
    mcpOverview: () => app.McpOverview(),
    mcpRestart: () => app.McpRestart(),
    mcpInstructions: () => app.McpInstructions(),
    mcpCalls: () => app.McpCalls(),
    mcpUsage: () => app.McpUsage(),
    tasks: () => app.Tasks(),
    tasksReload: () => app.TasksReload(),
    taskStart: (name, args) => app.TaskStart(name, args),
    taskStop: (name) => app.TaskStop(name),
    taskRuns: () => app.TaskRuns(),
    planningPage: () => app.PlanningPage(),
    planningDoc: (name) => app.PlanningDoc(name),
    on: <E extends EventName>(event: E, fn: (data: Events[E]) => void) =>
      runtime.EventsOn(event, (data) => fn(data as Events[E])),
  };
}
