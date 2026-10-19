import type { AssetChoice, AssetHit, AssetSaved, AssetsView, Backend, CatalogAsset, EventName, Events, HubPlanView, HubRulesView, HubSaveResult, HubStand, OpenGameResult, PlanCheckView, PlanObjectsView, PlanSaved, PlanView, PlannedChunk, ResearchResult, WorldLevelView, WorldPreviewArgs, WorldPreviewView, WorldSource } from './types';

// Das Studio ist eine Seite des Go-Servers (engine/studio): jede Methode von App ist ein POST auf /api/studio/rpc/<Methode>
// mit den Argumenten als JSON-Array, die Antwort ist {result} oder {error}. Die Kopfzeile hält fremde Seiten fern (CORS-Preflight).
const API = '/api/studio/';
const HEADERS = { 'Content-Type': 'application/json', 'X-K3C-Studio': '1' };

interface Answer<T> {
  result?: T;
  error?: string;
}

async function answer<T>(res: Response): Promise<T> {
  const body = (await res.json().catch(() => ({ error: `Fehler ${res.status} vom Server` }))) as Answer<T>;
  if (!res.ok || body.error) throw new Error(body.error || `Fehler ${res.status} vom Server`);
  return body.result as T;
}

function rpc<T>(method: string, ...args: unknown[]): Promise<T> {
  return fetch(`${API}rpc/${method}`, { method: 'POST', headers: HEADERS, body: JSON.stringify(args) }).then((res) => answer<T>(res));
}

/** Dateiwahl des Browsers (Ersatz des Dateidialogs der EXE); Abbruch = null. */
function pickFile(): Promise<File | null> {
  return new Promise((resolve) => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.png,.ogg,.mp3,.wav';
    input.addEventListener('change', () => resolve(input.files?.[0] ?? null));
    input.addEventListener('cancel', () => resolve(null));
    input.click();
  });
}

async function importFile(id: string, hit: string): Promise<string> {
  const file = await pickFile();
  if (!file) return '';
  const query = new URLSearchParams({ id, hit, name: file.name });
  const res = await fetch(`${API}import?${query}`, { method: 'POST', headers: { 'X-K3C-Studio': '1' }, body: file });
  return answer<string>(res);
}

/** Meldet „Im Spiel öffnen“ im Browser; die Adresse prüft und liefert der Server. */
async function openGame(base: string, seed: string, biome: string, mode: string): Promise<OpenGameResult> {
  const res = await rpc<OpenGameResult>('WorldOpenGame', base, seed, biome, mode);
  if (!res.error && res.url) window.open(res.url, '_blank', 'noopener');
  return res;
}

let source: EventSource | null = null;
const listeners = new Map<EventName, Set<(data: null) => void>>();

function subscribe<E extends EventName>(event: E, fn: (data: Events[E]) => void): () => void {
  let set = listeners.get(event);
  if (!set) {
    set = new Set();
    listeners.set(event, set);
    source ??= new EventSource(`${API}events`);
    source.addEventListener(event, () => listeners.get(event)?.forEach((f) => f(null)));
  }
  set.add(fn);
  return () => {
    set.delete(fn);
  };
}

export function httpBackend(): Backend {
  return {
    mock: false,
    assetsList: () => rpc<AssetsView>('AssetsList'),
    assetsSet: (id: string, wahl: AssetChoice[], stand: string) => rpc<AssetSaved>('AssetsSet', id, wahl, stand),
    assetsCatalog: (query: string, art: string, limit: number) => rpc<CatalogAsset[]>('AssetsCatalog', query, art, limit),
    assetsHits: (id: string) => rpc<AssetHit[]>('AssetsHits', id),
    assetsResearch: (id: string, query: string) => rpc<ResearchResult>('AssetsResearch', id, query),
    assetsHitDrop: (id: string, hit: string) => rpc<void>('AssetsHitDrop', id, hit),
    assetsImport: importFile,
    assetsDownload: (id: string, hit: string) => rpc<string>('AssetsDownload', id, hit),
    assetsLicenseOk: (id: string, hit: string) => rpc<void>('AssetsLicenseOK', id, hit),
    worldLevel: (seed: string, src: WorldSource) => rpc<WorldLevelView>('WorldLevel', seed, src),
    worldPreview: (args: WorldPreviewArgs) => rpc<WorldPreviewView>('WorldPreview', args),
    worldOpenGame: openGame,
    planLoad: () => rpc<PlanView>('PlanLoad'),
    planCheck: (island: string, depth: number, chunks: PlannedChunk[], count: number) => rpc<PlanCheckView>('PlanCheck', island, depth, chunks, count),
    planSave: (island: string, depth: number, chunks: PlannedChunk[], count: number, stand: string) => rpc<PlanSaved>('PlanSave', island, depth, chunks, count, stand),
    planIslandAdd: (stand: string) => rpc<PlanSaved>('PlanIslandAdd', stand),
    planIslandRemove: (id: string, stand: string) => rpc<PlanSaved>('PlanIslandRemove', id, stand),
    planChunkObjects: (seed: string, src: WorldSource, slot: number) => rpc<PlanObjectsView>('PlanChunkObjects', seed, src, slot),
    hubPlan: () => rpc<HubPlanView>('HubPlan'),
    hubRules: (hubJSON: string) => rpc<HubRulesView>('HubRules', hubJSON),
    hubCheck: (candidate: string) => rpc<HubPlanView>('HubCheck', candidate),
    hubSave: (candidate: string, stand: HubStand) => rpc<HubSaveResult>('HubSave', candidate, stand),
    openUrl: (url: string) => void window.open(url, '_blank', 'noopener'),
    on: subscribe,
  };
}

/** Der Mock läuft im Test und mit ?mock=1 (Oberfläche ohne Server, z. B. `npm run dev` im Studio-Ordner). */
export function useMock(): boolean {
  return import.meta.env.MODE === 'test' || new URLSearchParams(window.location.search).has('mock');
}
