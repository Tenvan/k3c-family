import type { AssetChoice, AssetHit, AssetSaved, AssetsView, Backend, CatalogAsset, EventName, Events, ResearchResult, OpenGameResult, WorldLevelView, WorldPreviewArgs, WorldPreviewView, WorldSource } from './types';

// Die Wails-Laufzeit legt window.go (Bindings der App) und window.runtime an. Die generierten Dateien unter
// frontend/wailsjs/ bindet die Oberfläche bewusst nicht ein (wie k3c-dev): so braucht der Typecheck keine Wails-CLI.
interface GoApp {
  AssetsList(): Promise<AssetsView>;
  AssetsSet(id: string, wahl: AssetChoice[], stand: string): Promise<AssetSaved>;
  AssetsCatalog(query: string, art: string, limit: number): Promise<CatalogAsset[]>;
  AssetsHits(id: string): Promise<AssetHit[]>;
  AssetsResearch(id: string, query: string): Promise<ResearchResult>;
  AssetsHitDrop(id: string, hit: string): Promise<void>;
  AssetsImport(id: string, hit: string): Promise<string>;
  AssetsDownload(id: string, hit: string): Promise<string>;
  AssetsLicenseOK(id: string, hit: string): Promise<void>;
  WorldLevel(seed: string, source: WorldSource): Promise<WorldLevelView>;
  WorldPreview(args: WorldPreviewArgs): Promise<WorldPreviewView>;
  WorldOpenGame(base: string, seed: string, biome: string, mode: string): Promise<OpenGameResult>;
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
    assetsList: () => app.AssetsList(),
    assetsSet: (id, wahl, stand) => app.AssetsSet(id, wahl, stand),
    assetsCatalog: (query, art, limit) => app.AssetsCatalog(query, art, limit),
    assetsHits: (id) => app.AssetsHits(id),
    assetsResearch: (id, query) => app.AssetsResearch(id, query),
    assetsHitDrop: (id, hit) => app.AssetsHitDrop(id, hit),
    assetsImport: (id, hit) => app.AssetsImport(id, hit),
    assetsDownload: (id, hit) => app.AssetsDownload(id, hit),
    assetsLicenseOk: (id, hit) => app.AssetsLicenseOK(id, hit),
    worldLevel: (seed, source) => app.WorldLevel(seed, source),
    worldPreview: (args) => app.WorldPreview(args),
    worldOpenGame: (base, seed, biome, mode) => app.WorldOpenGame(base, seed, biome, mode),
    openUrl: (url) => runtime.BrowserOpenURL(url),
    on: <E extends EventName>(event: E, fn: (data: Events[E]) => void) => runtime.EventsOn(event, (data) => fn(data as Events[E])),
  };
}
