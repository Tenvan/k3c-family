import fixture from './previewFixture.json';
import { hubLevelChoices } from '../stage/stageLogic';
import type { Site } from '@game/model/types';
import type { GameServersView, OpenGameResult, WorldLevelView, WorldPreviewArgs, WorldPreviewView, WorldSource } from './types';

// Erfundenes Level für den Mock (Browser, Tests): immer gleich zu gleichem Seed, 1 Abschnitt = 30 Units. Der Seed
// „fehler“ liefert den Hinweis einer abgelehnten Anfrage.

const KINDS = ['edge', 'normal', 'normal', 'recruitCamp', 'normal', 'hub', 'normal', 'chest', 'portal', 'normal', 'normal', 'exit'];
/** Abschnitte: ohne Chunk-Anzahl die festen zwölf, mit Anzahl n genau n (Rand, Hub in der Mitte, Ausgang; B-485/AC-04). */
const kindsOf = (n?: number | null): string[] =>
  (n ? Array.from({ length: n }, (_, i) => (i === 0 ? 'edge' : i === n - 1 ? 'exit' : i === n / 2 ? 'hub' : 'normal')) : KINDS);

export function mockWorldLevel(seed: string, source: WorldSource): WorldLevelView {
  const empty = { seed, biomeId: source.biome ?? 'forest', widthUnits: 0, chunkWidthUnits: 30, hubCenterUnits: 0, chunks: [], entities: [], warnings: [] };
  if (seed === 'fehler') return { ...empty, error: 'Seed braucht 1 bis 64 Zeichen' };
  const kinds = kindsOf(source.chunkCount);
  const chunks = kinds.map((kind, i) => ({ index: i, kind, startUnits: i * 30 })) as WorldLevelView['chunks'];
  const entities = [
    { kind: 'castle', x: 165 }, { kind: 'portal', x: 255 }, { kind: 'exit', x: 345 }, { kind: 'tree', x: 40 }, { kind: 'tree', x: 70 },
    { kind: 'rock', x: 100 }, { kind: 'chest', x: 225 }, { kind: 'recruitCamp', x: 105 },
  ];
  return { ...empty, widthUnits: kinds.length * 30, hubCenterUnits: kinds.indexOf('hub') * 30 + 15, chunks, entities, warnings: seed === 'warnung' ? ['Level zu kurz'] : [], error: '' };
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

/** Ruinen des Entwurfs als Plätze der Vorschau-Welt (WZ14.7): Lage aus dem Platz im Mock-Level (Abschnitte wie
 *  `mockWorldLevel`, 30 Units, Hub ± n), gebaut mit einem Viertel der HP und Kennung `ruin` wie im Server. Ohne Prüfung:
 *  die macht das Backend. */
function mockRuinSites(source: WorldSource): Site[] {
  const hub = kindsOf(source.chunkCount).indexOf('hub');
  return (source.planned ?? []).flatMap((c, n) => (c.buildings ?? []).map((b): Site => {
    const x = (hub + c.slot) * 30 + b.at;
    return { id: 9000 + n, kind: b.kind as Site['kind'], x, state: 'built', paidGold: 0, buildProgress: 1, hp: 50, maxHp: 200, workerId: null, bows: 0, bowPaidGold: 0, ruin: true };
  }));
}

// Vorschau-Welt für den Mock: ein von der Vorschau-Funktion des Servers erzeugter Zustand (Seed k3c, Wald, Hub-Stufe 5,
// ohne Bürger und Gegner). Hub-Stufe, Tageszeit und Gebäude-Auswahl wirken hier nur auf die Felder, nicht auf den Ausbau.
export function mockWorldPreview(args: WorldPreviewArgs): WorldPreviewView {
  if (args.seed === 'fehler') return { level: fixture.level as never, state: null, error: 'Seed braucht 1 bis 64 Zeichen' };
  const state = structuredClone(fixture.state) as unknown as NonNullable<WorldPreviewView['state']>;
  state.sites = [...state.sites, ...mockRuinSites(args.source)].filter((s) => args.buildings[`${s.kind}@${s.x}`] !== false);
  const top = levelsOf(args.hubCandidate);
  if (top === null) return { level: fixture.level as never, state: null, error: 'vorschau: Kandidaten-hub.json nicht lesbar' };
  state.hubLevel = args.hubLevel > 0 ? Math.min(args.hubLevel, top) : top;
  state.cycle = { ...(state.cycle as object), phase: args.phase || 'day', progress: args.progress };
  return { level: { ...(fixture.level as object), seed: args.seed, biomeId: args.source.biome ?? 'forest' } as never, state, error: '' };
}

/** „Im Spiel öffnen“ im Mock: öffnet nichts, prüft wie Go nur den Seed und meldet die Adresse (ohne Wails kein Browser-Start);
 *  eine andere als die Standard-Welt hat den Stand `<welt>-<seed>` (WZ18.6, B-499). */
export function mockOpenGame(base: string, seed: string, biome: string, mode: 'new' | 'full', world = 'standard'): OpenGameResult {
  if (mode === 'new' && biome !== 'forest') return { url: '', save: '', error: 'Start nur für den Wald (Tiefe 0)' };
  if (mode === 'full' && world !== 'standard') return { url: '', save: '', error: 'Voll ausgebaut gibt es nur für die Standard-Welt' };
  const max = mode === 'full' ? 27 : 32;
  if (!new RegExp(`^[a-z0-9-]{1,${max}}$`).test(seed)) return { url: '', save: '', error: `Seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens ${max} Zeichen)` };
  if (mode === 'new' && world !== 'standard' && `${world}-${seed}`.length > 32) return { url: '', save: '', error: 'Welt-ID und Seed ergeben zusammen höchstens 31 Zeichen (Spielstand `<welt>-<seed>`)' };
  // wie Go (B-499): eine eigene Welt hat den Stand `<welt>-<seed>`, ohne `fresh` (die Lobby lädt ihn oder legt ihn an)
  const save = mode === 'full' ? `voll-${seed}` : world === 'standard' ? seed : `${world}-${seed}`;
  const query = world === 'standard' ? `autostart=1&save=${save}` : `autostart=1&save=${save}&seed=${seed}&world=${world}`;
  return { url: `${base}/#${encodeURIComponent(`game.html?${query}`)}`, save, error: '' };
}

/** Spielserver im Mock (B-513): einer auf dem Standardport, erreichbar, im Dev-Mode, mit erfundener Heimnetz-Adresse. */
export function mockGameServers(): GameServersView {
  return { servers: [{ base: 'http://127.0.0.1:8080', lan: '192.168.1.20:8080', host: 'studio-pc', reachable: true, dev: true, version: 'dev' }] };
}
