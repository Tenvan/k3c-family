import type {
  Backend, ConsoleLine, ErrorsView, EventName, Events, Info, LevelCounts, LogQuery, LogView, McpCall, McpOverview,
  McpState, McpUsage, ResourceView, ServicesView, ServiceStatus, Source, TaskCatalog, TaskRun, PlanDoc, PlanItem, PlanningData, GitHubData, WorktreeDiff,
  GitBranch, GitPRList, GitView, TicketCreated, TicketInput,
} from './types';
import type { PlaytestModel, PlaytestTranscribeOptions, PlaytestCapability, PlaytestFinding, PlaytestFindingPatch, PlaytestJob, PlaytestTimeline, PlaytestDevices, PlaytestOptions, PlaytestPlan, PlaytestSession, PlaytestStatus } from '../playtest/types';

// Die Wails-Laufzeit legt window.go (Bindings der App) und window.runtime an. Die generierten Dateien unter
// frontend/wailsjs/ bindet die Oberfläche bewusst nicht ein: so braucht der Typecheck keine Wails-CLI.
interface GoApp {
  Info(): Promise<Info>;
  Services(): Promise<ServicesView>;
  ServicesReload(): Promise<ServicesView>;
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
  Resources(windowMin: number): Promise<ResourceView>;
  Tasks(): Promise<TaskCatalog>;
  TasksReload(): Promise<TaskCatalog>;
  TaskStart(name: string, args: string[]): Promise<TaskRun>;
  TaskStop(name: string): Promise<TaskRun>;
  TaskRuns(): Promise<TaskRun[]>;
  PlanningData(): Promise<PlanningData>;
  PlanningList(): Promise<PlanningData>;
  PlanningItems(ids: string[]): Promise<PlanItem[]>;
  PlanningDocs(): Promise<PlanDoc[]>;
  PlanningSet(id: string, field: string, value: string): Promise<string>;
  PlanningDoc(name: PlanDoc): Promise<string>;
  PlanningGet(id: string): Promise<string>;
  GitHubStatus(force: boolean): Promise<GitHubData>;
  WorktreeCheck(): Promise<WorktreeDiff[]>;
  GitOverview(): Promise<GitView>;
  GitAction(action: string): Promise<string>;
  GitDeleteBranches(names: string[], remote: boolean): Promise<string>;
  GitPRs(): Promise<GitPRList>;
  GitPRCreate(title: string, body: string): Promise<string>;
  GitPRAction(number: number, action: string): Promise<string>;
  GitCheckMerged(names: string[]): Promise<GitBranch[]>;
  GitStamp(): Promise<string>;
  TicketCreate(input: TicketInput): Promise<TicketCreated>;
  TicketRecent(n: number): Promise<TicketCreated[]>;
  PlaytestList(): Promise<PlaytestSession[]>;
  PlaytestPlan(action: string, id: string): Promise<PlaytestPlan>;
  PlaytestDelete(action: string, id: string, confirm: boolean): Promise<PlaytestPlan>;
  PlaytestDevices(): Promise<PlaytestDevices>;
  PlaytestStart(options: PlaytestOptions): Promise<PlaytestStatus>;
  PlaytestStop(): Promise<void>;
  PlaytestStatus(): Promise<PlaytestStatus>;
  PlaytestCheck(): Promise<PlaytestCapability>;
  PlaytestTranscribe(id: string, options: PlaytestTranscribeOptions): Promise<PlaytestJob>;
  PlaytestModels(): Promise<PlaytestModel[]>;
  PlaytestTranscribeStatus(): Promise<PlaytestJob>;
  PlaytestTranscribeCancel(): Promise<void>;
  PlaytestTimeline(id: string): Promise<PlaytestTimeline>;
  PlaytestFindings(id: string): Promise<PlaytestFinding[]>;
  PlaytestFindingUpdate(id: string, findingId: string, patch: PlaytestFindingPatch): Promise<PlaytestFinding>;
}

interface WailsRuntime {
  EventsOn(name: string, fn: (...data: unknown[]) => void): () => void;
  BrowserOpenURL(url: string): void;
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
    servicesReload: () => app.ServicesReload(),
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
    resources: (windowMin) => app.Resources(windowMin),
    tasks: () => app.Tasks(),
    tasksReload: () => app.TasksReload(),
    taskStart: (name, args) => app.TaskStart(name, args),
    taskStop: (name) => app.TaskStop(name),
    taskRuns: () => app.TaskRuns(),
    planningData: () => app.PlanningData(),
    planningList: () => app.PlanningList(), planningItems: (ids) => app.PlanningItems(ids),
    planningDocs: () => app.PlanningDocs(),
    planningSet: (id, field, value) => app.PlanningSet(id, field, value),
    planningDoc: (name) => app.PlanningDoc(name),
    planningGet: (id) => app.PlanningGet(id),
    githubStatus: (force) => app.GitHubStatus(force),
    worktreeCheck: () => app.WorktreeCheck(),
    gitOverview: () => app.GitOverview(),
    gitAction: (action) => app.GitAction(action),
    gitDeleteBranches: (names, remote) => app.GitDeleteBranches(names, remote),
    gitPRs: () => app.GitPRs(),
    gitPRCreate: (title, body) => app.GitPRCreate(title, body),
    gitPRAction: (number, action) => app.GitPRAction(number, action),
    gitCheckMerged: (names) => app.GitCheckMerged(names),
    gitStamp: () => app.GitStamp(),
    ticketCreate: (input) => app.TicketCreate(input),
    ticketRecent: (n) => app.TicketRecent(n),
    openUrl: (url) => runtime.BrowserOpenURL(url),
    on: <E extends EventName>(event: E, fn: (data: Events[E]) => void) =>
      runtime.EventsOn(event, (data) => fn(data as Events[E])),
  };
}
