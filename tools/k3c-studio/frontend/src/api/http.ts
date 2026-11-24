import { devEvents, devRpc } from '@webkit/lib/devRpc';
import type { Backend, Events } from './types';
import type { AssetsBackend } from './typesAssets';
import type { WorldBackend } from './typesWorld';
import type { HubBackend } from './typesHub';
import type { BalanceBackend } from './typesBalance';

// Client-Server-Betrieb (k3c-studio -headless/-dev, Go: uiapi.go über tools/workkit/uikit): Transport in
// tools/webkit/lib/devRpc.ts, Methoden wie die Wails-Bindings (wails.ts).
const rpc = devRpc('k3c-studio');

function httpAssets(): AssetsBackend {
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

function httpWorld(): WorldBackend {
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
    worldIslandEtappe: (id, island, etappe) => rpc('WorldIslandEtappe', id, island, etappe),
    worldNatureSet: (id, island, stage, key, value, seed) => rpc('WorldNatureSet', id, island, stage, key, value, seed),
  };
}

function httpHub(): HubBackend {
  return {
    hubPlan: () => rpc('HubPlan'),
    hubCheck: (candidate) => rpc('HubCheck', candidate),
    hubRules: (hubJSON) => rpc('HubRules', hubJSON),
    hubSave: (candidate, stand) => rpc('HubSave', candidate, stand),
  };
}

function httpBalance(): BalanceBackend {
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

export function httpBackend(): Backend {
  return {
    mock: false,
    info: () => rpc('Info'),
    ...httpAssets(),
    ...httpWorld(),
    ...httpHub(),
    ...httpBalance(),
    openUrl: (url) => {
      window.open(url, '_blank', 'noopener');
    },
    on: devEvents<Events>(),
  };
}
