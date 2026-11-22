import type { Backend, EventName, Events, Info } from './types';
import type { WorldBackend, WorldCreateView, WorldDetail, WorldInfo, WorldLevel, WorldValuesView } from './typesWorld';
import type { HubBackend, HubPlanView, HubRulesView, HubSaveResult, HubStand } from './typesHub';
import type { BalanceBackend, BalanceBaseline, BalanceDataFile, BalanceDataFileValue, BalanceDataValue, BalanceInsights, BalanceReportInfo, BalanceRun, BalanceSpec, BalanceSummary, Tuning } from './typesBalance';
import type { AssetEintrag, AssetsBackend, Frame, Groesse, GroesseBasis, Hintergrund, PixelDiff, Quellen, Review, SoundEintrag, Spec, Stil, Stufen, Vorlage, WeltSicht } from './typesAssets';

// Die Wails-Laufzeit legt window.go (Bindings der App) und window.runtime an. Die generierten Dateien unter
// frontend/wailsjs/ bindet die Oberfläche bewusst nicht ein: so braucht der Typecheck keine Wails-CLI.
// GoApp ist genau die Freigabeliste uiMethods in uiapi.go (TestUIMethodsWieWailsTS).
interface GoApp {
  Info(): Promise<Info>;
  OpenInClaude(prompt: string): Promise<void>;
  // Asset-Oberfläche (docs/standards/paper-mcp.md › Oberfläche; Typen in typesAssets.ts)
  AssetsList(): Promise<AssetEintrag[]>;
  AssetsIndex(): Promise<AssetEintrag[]>;
  AssetsCheck(pfade: string[]): Promise<AssetEintrag[]>;
  AssetsGroesse(id: string): Promise<Groesse>;
  AssetsGroesseBasis(id: string, basis: string): Promise<GroesseBasis>;
  AssetsFrameOhne(id: string, animation: string, frame: number, ohne: string[]): Promise<Frame>;
  AssetsStufen(): Promise<Stufen>;
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
  StudioWeltSicht(welt: string): Promise<WeltSicht>;
  StudioSoundData(pfad: string): Promise<string>;
  StudioSources(): Promise<Quellen>;
  StudioGameOpen(seed: string): Promise<string>;
  // Welten-Editor (B-879, app_world.go)
  WorldList(): Promise<WorldInfo[]>;
  WorldDetail(id: string): Promise<WorldDetail>;
  WorldLevel(id: string, island: string, stage: number, seed: string): Promise<WorldLevel>;
  WorldValues(id: string): Promise<WorldValuesView>;
  WorldCreate(id: string, name: string, extendsId: string): Promise<WorldCreateView>;
  WorldDelete(id: string): Promise<WorldCreateView>;
  WorldOverrideSet(id: string, file: string, path: string[], value: unknown): Promise<WorldValuesView>;
  WorldIslandEtappe(id: string, island: string, etappe: number | null): Promise<WorldCreateView>;
  WorldSetKind(id: string, kind: string): Promise<WorldCreateView>;
  WorldSetExtends(id: string, parent: string): Promise<WorldCreateView>;
  WorldNatureSet(id: string, island: string, stage: number, key: string, value: number | null, seed: string): Promise<WorldLevel>;
  // Gebäude-Planer (B-923, app_hub.go)
  HubPlan(): Promise<HubPlanView>;
  HubCheck(candidate: string): Promise<HubPlanView>;
  HubRules(hubJSON: string): Promise<HubRulesView>;
  HubSave(candidate: string, stand: HubStand): Promise<HubSaveResult>;
  // Balancing-Seite (WZ28.4, seit WZ63.5/WZ63.6 im Studio, app_balance.go; Typen in typesBalance.ts)
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

function assetsWails(app: GoApp): AssetsBackend {
  return {
    assetsList: () => app.AssetsList(),
    assetsIndex: () => app.AssetsIndex(),
    assetsCheck: (pfade) => app.AssetsCheck(pfade),
    assetsGroesse: (id) => app.AssetsGroesse(id),
    assetsStufen: () => app.AssetsStufen(),
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
    studioWeltSicht: (welt) => app.StudioWeltSicht(welt),
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
    worldCreate: (id, name, extendsId) => app.WorldCreate(id, name, extendsId),
    worldDelete: (id) => app.WorldDelete(id),
    worldOverrideSet: (id, file, path, value) => app.WorldOverrideSet(id, file, path, value),
    worldSetExtends: (id, parent) => app.WorldSetExtends(id, parent) as Promise<WorldCreateView>,
    worldSetKind: (id, kind) => app.WorldSetKind(id, kind) as Promise<WorldCreateView>,
    worldIslandEtappe: (id, island, etappe) => app.WorldIslandEtappe(id, island, etappe),
    worldNatureSet: (id, island, stage, key, value, seed) => app.WorldNatureSet(id, island, stage, key, value, seed),
  };
}

function hubWails(app: GoApp): HubBackend {
  return {
    hubPlan: () => app.HubPlan(),
    hubCheck: (candidate) => app.HubCheck(candidate),
    hubRules: (hubJSON) => app.HubRules(hubJSON),
    hubSave: (candidate, stand) => app.HubSave(candidate, stand),
  };
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

export function wailsBackend(): Backend {
  const app = window.go!.main!.App!;
  const runtime = window.runtime!;
  return {
    mock: false,
    info: () => app.Info(),
    openInClaude: (prompt) => app.OpenInClaude(prompt),
    ...assetsWails(app),
    ...worldWails(app),
    ...hubWails(app),
    ...balanceWails(app),
    openUrl: (url) => runtime.BrowserOpenURL(url),
    on: <E extends EventName>(event: E, fn: (data: Events[E]) => void) =>
      runtime.EventsOn(event, (data) => fn(data as Events[E])),
  };
}
