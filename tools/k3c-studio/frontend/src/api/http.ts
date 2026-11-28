import { devEvents, devRpc } from '@webkit/lib/devRpc';
import type { Backend, Events } from './types';
import type { AssetsBackend } from './typesAssets';
import type { WorldBackend } from './typesWorld';
import type { HubBackend } from './typesHub';
import type { BalanceBackend } from './typesBalance';

// Methodenzuordnung für alle Transporte mit Aufruf-Tabelle: Client-Server-Betrieb (k3c-studio -headless/-dev über
// tools/webkit/lib/devRpc.ts) und das Tauri-Fenster der Rust-Fassung (tauri.ts).
export type Rpc = <T>(method: string, ...args: unknown[]) => Promise<T>;

function rpcAssets(rpc: Rpc): AssetsBackend {
  return {
    assetsList: () => rpc('AssetsList'),
    assetsIndex: () => rpc('AssetsIndex'),
    assetsCheck: (pfade) => rpc('AssetsCheck', pfade),
    assetsGroesse: (id) => rpc('AssetsGroesse', id),
    assetsStufen: () => rpc('AssetsStufen'),
    assetsGroesseBasis: (id, basis) => rpc('AssetsGroesseBasis', id, basis),
    assetsFrameOhne: (id, animation, frame, ohne) => rpc('AssetsFrameOhne', id, animation, frame, ohne),
    assetsHintergruende: () => rpc('AssetsHintergruende'),
    assetsHintergrund: (teil) => rpc('AssetsHintergrund', teil),
    assetsGet: (id) => rpc('AssetsGet', id),
    assetsStyle: () => rpc('AssetsStyle'),
    assetsPrompts: () => rpc('AssetsPrompts'),
    assetsReview: (basis) => rpc('AssetsReview', basis),
    assetsFrame: (id, animation, frame, variante, basis) => rpc('AssetsFrame', id, animation, frame, variante, basis),
    assetsPixelDiff: (id, animation, frame, basis) => rpc('AssetsPixelDiff', id, animation, frame, basis),
    studioSounds: () => rpc('StudioSounds'),
    studioWeltSicht: (welt) => rpc('StudioWeltSicht', welt),
    studioSoundData: (pfad) => rpc('StudioSoundData', pfad),
    studioSources: () => rpc('StudioSources'),
    studioGameOpen: (seed) => rpc('StudioGameOpen', seed),
  };
}

function rpcWorld(rpc: Rpc): WorldBackend {
  return {
    worldList: () => rpc('WorldList'),
    worldDetail: (id) => rpc('WorldDetail', id),
    worldLevel: (id, island, stage, seed) => rpc('WorldLevel', id, island, stage, seed),
    worldValues: (id) => rpc('WorldValues', id),
    worldCreate: (id, name, extendsId) => rpc('WorldCreate', id, name, extendsId),
    worldDelete: (id) => rpc('WorldDelete', id),
    worldOverrideSet: (id, file, path, value) => rpc('WorldOverrideSet', id, file, path, value),
    worldSetExtends: (id, parent) => rpc('WorldSetExtends', id, parent),
    worldSetKind: (id, kind) => rpc('WorldSetKind', id, kind),
    worldPicks: (id) => rpc('WorldPicks', id),
    worldSetPick: (id, key, value) => rpc('WorldSetPick', id, key, value),
    worldIslandEndBoss: (id, island, boss) => rpc('WorldIslandEndBoss', id, island, boss),
    worldSkins: (id, skins) => rpc('WorldSkins', id, skins),
    worldIslandEtappe: (id, island, etappe) => rpc('WorldIslandEtappe', id, island, etappe),
    worldNatureSet: (id, island, stage, key, value, seed) => rpc('WorldNatureSet', id, island, stage, key, value, seed),
  };
}

function rpcHub(rpc: Rpc): HubBackend {
  return {
    hubPlan: () => rpc('HubPlan'),
    hubCheck: (candidate) => rpc('HubCheck', candidate),
    hubRules: (hubJSON) => rpc('HubRules', hubJSON),
    hubSave: (candidate, stand) => rpc('HubSave', candidate, stand),
  };
}

function rpcBalance(rpc: Rpc): BalanceBackend {
  return {
    balanceStart: (spec) => rpc('BalanceStart', spec),
    balanceRuns: () => rpc('BalanceRuns'),
    balanceStop: (id) => rpc('BalanceStop', id),
    balanceReports: () => rpc('BalanceReports'),
    balanceBaselines: () => rpc('BalanceBaselines'),
    balanceReport: (path) => rpc('BalanceReport', path),
    balanceTuning: (path) => rpc('BalanceTuning', path),
    balanceDataFiles: () => rpc('BalanceDataFiles'),
    balanceDataDoc: (name) => rpc('BalanceDataDoc', name),
    balanceDataValues: (name) => rpc('BalanceDataValues', name),
    balanceDataValuesAll: () => rpc('BalanceDataValuesAll'),
    balanceReportRate: (path, rating) => rpc('BalanceReportRate', path, rating),
    balanceReportDelete: (path) => rpc('BalanceReportDelete', path),
    balanceReportsRate: (paths, rating) => rpc('BalanceReportsRate', paths, rating),
    balanceReportsDelete: (paths) => rpc('BalanceReportsDelete', paths),
    balanceSaveBaseline: (path, name) => rpc('BalanceSaveBaseline', path, name),
    balanceInsights: (path) => rpc('BalanceInsights', path),
    balanceReportHTML: (path) => rpc('BalanceReportHTML', path),
  };
}

/** Das Backend aus den drei Teilen eines Transports: Aufrufe, Ereignisse, Adressen öffnen. */
export function rpcBackend(rpc: Rpc, on: Backend['on'], openUrl: Backend['openUrl']): Backend {
  return {
    mock: false,
    info: () => rpc('Info'),
    ...rpcAssets(rpc),
    ...rpcWorld(rpc),
    ...rpcHub(rpc),
    ...rpcBalance(rpc),
    openUrl,
    on,
  };
}

export function httpBackend(): Backend {
  return rpcBackend(devRpc('k3c-studio'), devEvents<Events>(), (url) => {
    window.open(url, '_blank', 'noopener');
  });
}
