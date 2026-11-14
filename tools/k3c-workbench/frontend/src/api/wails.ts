import type {
  Backend, ConsoleLine, ErrorsView, EventName, Events, Info, LevelCounts, LogQuery, LogView, McpCall, McpOverview,
  McpState, McpUsage, ResourceView, ServicesView, ServiceStatus, Source, TaskCatalog, TaskGates, TaskRun, PlanDoc, PlanningData, GitHubData, WorktreeDiff,
  BalanceBackend, BalanceBaseline, BalanceDataFile, BalanceDataFileValue, BalanceInsights, BalanceDataValue, BalanceReportInfo, BalanceRun, BalanceSpec, BalanceSummary, Tuning, GitBranch, GitPRList, GitView, TicketCreated, TicketInput,
} from './types';
import type { WorldBackend, WorldDetail, WorldInfo, WorldLevel, WorldValuesView } from './typesWorld';
import type { AssetEintrag, AssetsBackend, Frame, Groesse, GroesseBasis, Hintergrund, PixelDiff, Quellen, Review, SoundEintrag, Spec, Stil, Vorlage } from './typesAssets';

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
  TaskGatesView(): Promise<TaskGates>;
  TaskGateToggle(name: string): Promise<TaskGates>;
  TaskGateSave(): Promise<TaskGates>;
  PlanningData(): Promise<PlanningData>;
  PlanningDocs(): Promise<PlanDoc[]>;
  PlanningSet(id: string, field: string, value: string): Promise<string>;
  PlanningDoc(name: PlanDoc): Promise<string>;
  PlanningGet(id: string): Promise<string>;
  GitHubStatus(force: boolean): Promise<GitHubData>;
  WorktreeCheck(): Promise<WorktreeDiff[]>;
  OpenInClaude(prompt: string): Promise<void>;
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
  // Balancing-Seite (WZ28.4, WZ28.5)
  BalanceStart(spec: BalanceSpec): Promise<BalanceRun>;
  BalanceRuns(): Promise<BalanceRun[]>;
  BalanceStop(id: string): Promise<void>;
  BalanceReports(): Promise<BalanceReportInfo[]>;
  BalanceBaselines(): Promise<BalanceBaseline[]>;
  BalanceReport(path: string): Promise<BalanceSummary>;
  BalanceTuning(path: string): Promise<Tuning>;
  BalanceDataFiles(): Promise<BalanceDataFile[]>;
  BalanceDataDoc(name: string): Promise<string>;
  BalanceDataValues(name: string): Promise<BalanceDataValue[]>;
  BalanceDataValuesAll(): Promise<BalanceDataFileValue[]>;
  BalanceReportRate(path: string, rating: string): Promise<void>;
  BalanceReportDelete(path: string): Promise<void>;
  BalanceReportsRate(paths: string[], rating: string): Promise<void>;
  BalanceReportsDelete(paths: string[]): Promise<void>;
  BalanceSaveBaseline(path: string, name: string): Promise<BalanceBaseline>;
  BalanceInsights(path: string): Promise<BalanceInsights>;
  BalanceReportHTML(path: string): Promise<string>;
  // Asset-Oberfläche (docs/standards/paper-mcp.md › Oberfläche; Typen in typesAssets.ts)
  AssetsList(): Promise<AssetEintrag[]>;
  AssetsIndex(): Promise<AssetEintrag[]>;
  AssetsCheck(pfade: string[]): Promise<AssetEintrag[]>;
  AssetsGroesse(id: string): Promise<Groesse>;
  AssetsGroesseBasis(id: string, basis: string): Promise<GroesseBasis>;
  AssetsFrameOhne(id: string, animation: string, frame: number, ohne: string[]): Promise<Frame>;
  AssetsHintergruende(): Promise<string[]>;
  AssetsHintergrund(teil: string): Promise<Hintergrund>;
  AssetsGet(id: string): Promise<Spec>;
  AssetsStyle(): Promise<Stil>;
  AssetsPrompts(): Promise<Vorlage[]>;
  AssetsReview(basis: string): Promise<Review>;
  AssetsFrame(id: string, animation: string, frame: number, variante: string, basis: string): Promise<Frame>;
  AssetsPixelDiff(id: string, animation: string, frame: number, basis: string): Promise<PixelDiff>;
  // Lesende Teile des früheren Asset-Studios (WB1.4, internal/studioview)
  StudioSounds(): Promise<SoundEintrag[]>;
  StudioSoundData(pfad: string): Promise<string>;
  StudioSources(): Promise<Quellen>;
  StudioGameOpen(seed: string): Promise<string>;
  // Welten-Editor (B-879, app_world.go)
  WorldList(): Promise<WorldInfo[]>;
  WorldDetail(id: string): Promise<WorldDetail>;
  WorldLevel(id: string, island: string, stage: number, seed: string): Promise<WorldLevel>;
  WorldValues(id: string): Promise<WorldValuesView>;
  WorldOverrideSet(id: string, file: string, path: string[], value: unknown): Promise<WorldValuesView>;
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

function balanceWails(app: GoApp): BalanceBackend {
  return {
    balanceStart: (spec) => app.BalanceStart(spec),
    balanceRuns: () => app.BalanceRuns(),
    balanceStop: (id) => app.BalanceStop(id),
    balanceReports: () => app.BalanceReports(),
    balanceBaselines: () => app.BalanceBaselines(),
    balanceReport: (path) => app.BalanceReport(path),
    balanceTuning: (path) => app.BalanceTuning(path),
    balanceDataFiles: () => app.BalanceDataFiles(),
    balanceDataDoc: (name) => app.BalanceDataDoc(name),
    balanceDataValues: (name) => app.BalanceDataValues(name),
    balanceDataValuesAll: () => app.BalanceDataValuesAll(),
    balanceReportRate: (path, rating) => app.BalanceReportRate(path, rating),
    balanceReportDelete: (path) => app.BalanceReportDelete(path),
    balanceReportsRate: (paths, rating) => app.BalanceReportsRate(paths, rating),
    balanceReportsDelete: (paths) => app.BalanceReportsDelete(paths),
    balanceSaveBaseline: (path, name) => app.BalanceSaveBaseline(path, name),
    balanceInsights: (path) => app.BalanceInsights(path),
    balanceReportHTML: (path) => app.BalanceReportHTML(path),
  };
}

function assetsWails(app: GoApp): AssetsBackend {
  return {
    assetsList: () => app.AssetsList(),
    assetsIndex: () => app.AssetsIndex(),
    assetsCheck: (pfade) => app.AssetsCheck(pfade),
    assetsGroesse: (id) => app.AssetsGroesse(id),
    assetsGroesseBasis: (id, basis) => app.AssetsGroesseBasis(id, basis),
    assetsFrameOhne: (id, animation, frame, ohne) => app.AssetsFrameOhne(id, animation, frame, ohne),
    assetsHintergruende: () => app.AssetsHintergruende(),
    assetsHintergrund: (teil) => app.AssetsHintergrund(teil),
    assetsGet: (id) => app.AssetsGet(id),
    assetsStyle: () => app.AssetsStyle(),
    assetsPrompts: () => app.AssetsPrompts(),
    assetsReview: (basis) => app.AssetsReview(basis),
    assetsFrame: (id, animation, frame, variante, basis) => app.AssetsFrame(id, animation, frame, variante, basis),
    assetsPixelDiff: (id, animation, frame, basis) => app.AssetsPixelDiff(id, animation, frame, basis),
    studioSounds: () => app.StudioSounds(),
    studioSoundData: (pfad) => app.StudioSoundData(pfad),
    studioSources: () => app.StudioSources(),
    studioGameOpen: (seed) => app.StudioGameOpen(seed),
  };
}

function worldWails(app: GoApp): WorldBackend {
  return {
    worldList: () => app.WorldList(),
    worldDetail: (id) => app.WorldDetail(id),
    worldLevel: (id, island, stage, seed) => app.WorldLevel(id, island, stage, seed),
    worldValues: (id) => app.WorldValues(id),
    worldOverrideSet: (id, file, path, value) => app.WorldOverrideSet(id, file, path, value),
  };
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
    taskGates: () => app.TaskGatesView(),
    taskGateToggle: (name) => app.TaskGateToggle(name),
    taskGateSave: () => app.TaskGateSave(),
    planningData: () => app.PlanningData(),
    planningDocs: () => app.PlanningDocs(),
    planningSet: (id, field, value) => app.PlanningSet(id, field, value),
    planningDoc: (name) => app.PlanningDoc(name),
    planningGet: (id) => app.PlanningGet(id),
    githubStatus: (force) => app.GitHubStatus(force),
    worktreeCheck: () => app.WorktreeCheck(),
    openInClaude: (prompt) => app.OpenInClaude(prompt),
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
    ...balanceWails(app),
    ...assetsWails(app),
    ...worldWails(app),
    openUrl: (url) => runtime.BrowserOpenURL(url),
    on: <E extends EventName>(event: E, fn: (data: Events[E]) => void) =>
      runtime.EventsOn(event, (data) => fn(data as Events[E])),
  };
}
