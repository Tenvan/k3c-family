import fixture from './previewFixture.json';
import { hubLevelChoices } from '../stage/stageLogic';
import type { OpenGameResult, WorldLevelView, WorldPreviewArgs, WorldPreviewView, WorldSource } from './types';

// Erfundenes Level für den Mock (Browser, Tests): immer gleich zu gleichem Seed, 1 Abschnitt = 30 Units. Der Seed
// „fehler“ liefert den Hinweis einer abgelehnten Anfrage.

const KINDS = ['edge', 'normal', 'normal', 'recruitCamp', 'normal', 'hub', 'normal', 'chest', 'portal', 'normal', 'normal', 'exit'];

export function mockWorldLevel(seed: string, source: WorldSource): WorldLevelView {
  const empty = { seed, biomeId: source.biome ?? '', widthUnits: 0, chunkWidthUnits: 30, hubCenterUnits: 0, chunks: [], entities: [], warnings: [] };
  if (seed === 'fehler') return { ...empty, error: 'Seed braucht 1 bis 64 Zeichen' };
  const chunks = KINDS.map((kind, i) => ({ index: i, kind, startUnits: i * 30 })) as WorldLevelView['chunks'];
  const entities = [
    { kind: 'castle', x: 165 }, { kind: 'portal', x: 255 }, { kind: 'exit', x: 345 }, { kind: 'tree', x: 40 }, { kind: 'tree', x: 70 },
    { kind: 'rock', x: 100 }, { kind: 'chest', x: 225 }, { kind: 'recruitCamp', x: 105 },
  ];
  return { ...empty, widthUnits: KINDS.length * 30, hubCenterUnits: 165, chunks, entities, warnings: seed === 'warnung' ? ['Level zu kurz'] : [], error: '' };
}

/** Hub-Stufen im Mock: die des Planer-Stands, sonst die von data/hub.json; null = Kandidat kein JSON mit levels. */
function levelsOf(candidate: string | undefined): number | null {
  if (!candidate) return hubLevelChoices().length;
  try {
    const levels = (JSON.parse(candidate) as { levels?: unknown }).levels;
    return Array.isArray(levels) ? levels.length : null;
  } catch {
    return null;
  }
}

// Vorschau-Welt für den Mock: ein von der Vorschau-Funktion des Servers erzeugter Zustand (Seed k3c, Wald, Hub-Stufe 5,
// ohne Bürger und Gegner). Hub-Stufe, Tageszeit und Gebäude-Auswahl wirken hier nur auf die Felder, nicht auf den Ausbau.
export function mockWorldPreview(args: WorldPreviewArgs): WorldPreviewView {
  if (args.seed === 'fehler') return { level: fixture.level as never, state: null, error: 'Seed braucht 1 bis 64 Zeichen' };
  const state = structuredClone(fixture.state) as unknown as NonNullable<WorldPreviewView['state']>;
  state.sites = state.sites.filter((s) => args.buildings[`${s.kind}@${s.x}`] !== false);
  const top = levelsOf(args.hubCandidate);
  if (top === null) return { level: fixture.level as never, state: null, error: 'vorschau: Kandidaten-hub.json nicht lesbar' };
  state.hubLevel = args.hubLevel > 0 ? Math.min(args.hubLevel, top) : top;
  state.cycle = { ...(state.cycle as object), phase: args.phase || 'day', progress: args.progress };
  return { level: { ...(fixture.level as object), seed: args.seed, biomeId: args.source.biome ?? 'forest' } as never, state, error: '' };
}

/** „Im Spiel öffnen“ im Mock: öffnet nichts, prüft wie Go nur den Seed und meldet die Adresse (ohne Wails kein Browser-Start). */
export function mockOpenGame(base: string, seed: string, biome: string, mode: 'new' | 'full'): OpenGameResult {
  if (mode === 'new' && biome !== 'forest') return { url: '', save: '', error: 'Start nur für den Wald (Tiefe 0)' };
  const max = mode === 'full' ? 27 : 32;
  if (!new RegExp(`^[a-z0-9-]{1,${max}}$`).test(seed)) return { url: '', save: '', error: `Seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens ${max} Zeichen)` };
  const save = mode === 'full' ? `voll-${seed}` : seed;
  return { url: `${base}/#${encodeURIComponent(`game.html?autostart=1&save=${save}`)}`, save, error: '' };
}
