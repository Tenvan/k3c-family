import hubData from '@data/hub.json';
import { BIOMES } from '@game/model/biome';
import type { World } from '@game/model/types';
import { createViewWorld } from '@game/online/clientWorld';
import { biomeLayers } from '@game/scenes/render/worldSprites';
import type { WorldState } from '@game/online/clientProtocol';
import type { PlannedChunk, SiteLook, WorldPreviewArgs, WorldPreviewView } from '../api';
import type { SimPhase } from './timeOfDay';

// Reine Logik der Live-Vorschau (B-460, WZ7.3): Auswahl der Hub-Stufen, Gebäude-Liste, Welt für den Renderer. Kein
// Phaser, kein DOM.

/** Hub-Stufen zur Auswahl: 1 bis Anzahl der Einträge von `data/hub.json` › `levels` (mehr Stufen erscheinen ohne Code-Änderung). */
export function hubLevelChoices(levels: readonly unknown[] = (hubData as { levels: unknown[] }).levels): number[] {
  return levels.map((_, i) => i + 1);
}

/** Schlüssel eines Bauplatzes in der Gebäude-Auswahl: Art und X in Units, wie `room.PreviewSiteKey` (Go). */
export const siteKey = (kind: string, x: number): string => `${kind}@${x}`;

export interface BuildingToggle {
  key: string;
  kind: string;
  x: number;
}

/** Gebäude-Liste aus dem Zustand der Vorschau, von links nach rechts (bei gleicher Position nach Art). */
export function buildingToggles(sites: readonly { kind: string; x: number }[]): BuildingToggle[] {
  return sites
    .map((s) => ({ key: siteKey(s.kind, s.x), kind: s.kind, x: s.x }))
    .sort((a, b) => a.x - b.x || a.kind.localeCompare(b.kind));
}

/** Gewählter Platz des Gebäude-Planers: Art der Matrix-Zeile, Platz zur Hub-Mitte (null = unbekannt) und ob er je Seite gilt. */
export interface RowFocus {
  id: string;
  kind: string;
  offset: number | null;
  bothSides: boolean;
}

/** Plätze der Welt (x in Units), die zur gewählten Zeile gehören: gleiche Art und Abstand zur Hub-Mitte gleich dem Platz
 *  (± bei beiden Seiten, Toleranz für die Streuung je Seed); passt keiner, alle der Art. Der Händler ist kein Platz. */
export function focusXs(focus: RowFocus | undefined, hubX: number, sites: readonly { kind: string; x: number }[]): number[] {
  if (!focus || focus.kind.startsWith('merchant')) return [];
  const ofKind = sites.filter((s) => s.kind === focus.kind);
  if (focus.offset === null) return ofKind.map((s) => s.x);
  const offset = focus.offset;
  const exact = ofKind.filter((s) => (focus.bothSides ? Math.abs(Math.abs(s.x - hubX) - Math.abs(offset)) : Math.abs(s.x - (hubX + offset))) <= 0.5);
  return (exact.length > 0 ? exact : ofKind).map((s) => s.x);
}

/** Quelle Insel + Stufe mit dem Entwurf der geplanten Chunks und der Chunk-Anzahl (0 = Biom-Länge; gleiche Quelle wie
 *  die Chunk-Vorschau, B-461/AC-11, B-485/AC-04). */
export interface PlanSource {
  island: string;
  stage: number;
  planned: PlannedChunk[];
  chunkCount?: number;
}

/** Reihenfolge und Namen der Gruppen im Gebäude-Filter; unbekannte Arten folgen alphabetisch, mit ihrem Namen. */
const GROUPS: readonly [kind: string, label: string][] = [
  ['wall', 'Mauern'], ['tower', 'Türme'], ['gate', 'Tore'], ['farm', 'Farmen'], ['barracks', 'Kasernen'], ['workshop', 'Werkstätten'],
  ['tavern', 'Tavernen'], ['healer', 'Heilplätze'], ['smithy', 'Schmieden'], ['armory', 'Rüstkammern'], ['stairsDown', 'Treppen runter'],
  ['stairsUp', 'Treppen hoch'],
];

export interface BuildingGroup {
  kind: string;
  label: string;
  items: BuildingToggle[];
  /** Zahl der eingeschalteten Plätze */
  on: number;
  /** alle, keiner oder gemischt: Zustand des Gruppenschalters */
  state: 'all' | 'none' | 'some';
}

/** Gebäude-Schalter nach Art gruppiert (B-477/AC-05): Gruppen in fester Reihenfolge, Plätze von links nach rechts; `off` sind die abgewählten Schlüssel. */
export function buildingGroups(toggles: readonly BuildingToggle[], off: ReadonlySet<string>): BuildingGroup[] {
  const byKind = new Map<string, BuildingToggle[]>();
  for (const t of toggles) byKind.set(t.kind, [...(byKind.get(t.kind) ?? []), t]);
  const order = (kind: string) => { const i = GROUPS.findIndex(([k]) => k === kind); return i < 0 ? GROUPS.length : i; };
  return [...byKind.entries()]
    .sort(([a], [b]) => order(a) - order(b) || a.localeCompare(b))
    .map(([kind, items]) => {
      const on = items.filter((t) => !off.has(t.key)).length;
      return { kind, label: GROUPS.find(([k]) => k === kind)?.[1] ?? kind, items, on, state: on === items.length ? 'all' : on === 0 ? 'none' : 'some' };
    });
}

/** Optionen der Vorschau: abgewählte Gebäude als `false`, die übrigen fehlen (= an). Ein Planer-Stand (`hubCandidate`,
 *  Gebäude-Planer) geht nur mit, wenn er gesetzt ist. */
export function previewArgsOf(
  seed: string, biome: string, hubLevel: number, off: ReadonlySet<string>, time: { phase: SimPhase; progress: number } = { phase: 'day', progress: 0 },
  plan?: PlanSource,
  hubCandidate = '',
  looks: Readonly<Record<string, SiteLook>> = {},
): WorldPreviewArgs {
  return {
    seed, source: plan ? { biome, island: plan.island, stage: plan.stage, planned: plan.planned, chunkCount: plan.chunkCount } : { biome }, hubLevel, phase: time.phase, progress: time.progress,
    buildings: Object.fromEntries([...off].sort().map((k) => [k, false])),
    ...(hubCandidate ? { hubCandidate } : {}),
    ...(Object.keys(looks).length > 0 ? { sites: looks } : {}),
  };
}

export interface BackgroundToggle {
  /** Textur-Schlüssel der Schicht (`bg:<biom>:<n>`), auch der Spiegelung im Fluss */
  key: string;
  label: string;
  /** Scroll-Faktor: Parallaxe beim Scrollen, kleiner = ferner */
  scroll: number;
}

const DEPTH_NAMES = ['fern', 'mittel', 'nah'];

/** Hintergrund-Schichten eines Bioms von fern nach nah; leer bei einem Biom ohne Schichten (dann Silhouetten, Schalter ausgegraut). */
export function backgroundToggles(biomeId: string): BackgroundToggle[] {
  const layers = biomeLayers(biomeId) ?? [];
  return layers.map((l, i) => ({ key: l.key, scroll: l.scroll, label: `Schicht ${i + 1}${layers.length === DEPTH_NAMES.length ? ` (${DEPTH_NAMES[i]})` : ''}` }));
}

/** Die Welt, die der Renderer zeichnet; null bei Hinweis, unbekanntem Biom oder fehlendem Zustand. */
export function worldOf(view: WorldPreviewView | null): World | null {
  if (!view || view.error || !view.state || !view.level) return null;
  const biome = BIOMES.find((b) => b.id === view.level.biomeId);
  if (!biome) return null;
  return createViewWorld({ depth: biome.depth, layout: view.level, biome }, view.state as unknown as WorldState);
}
