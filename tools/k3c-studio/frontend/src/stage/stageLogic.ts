import hubData from '@data/hub.json';
import { BIOMES } from '@game/model/biome';
import { GAME_HEIGHT } from '@game/core/constants';
import type { World } from '@game/model/types';
import { createViewWorld } from '@game/online/clientWorld';
import type { WorldState } from '@game/online/clientProtocol';
import type { WorldPreviewArgs, WorldPreviewView } from '../api';

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

/** Optionen der Vorschau: abgewählte Gebäude als `false`, die übrigen fehlen (= an). */
export function previewArgsOf(seed: string, biome: string, hubLevel: number, off: ReadonlySet<string>): WorldPreviewArgs {
  return {
    seed, source: { biome }, hubLevel, phase: 'day', progress: 0,
    buildings: Object.fromEntries([...off].sort().map((k) => [k, false])),
  };
}

/** Die Welt, die der Renderer zeichnet; null bei Hinweis, unbekanntem Biom oder fehlendem Zustand. */
export function worldOf(view: WorldPreviewView | null): World | null {
  if (!view || view.error || !view.state || !view.level) return null;
  const biome = BIOMES.find((b) => b.id === view.level.biomeId);
  if (!biome) return null;
  return createViewWorld({ depth: biome.depth, layout: view.level, biome }, view.state as unknown as WorldState);
}

/** Kamera-Zoom, bei dem die Spielhöhe in die Höhe der Bühne passt (Texte der Stufe gleicht `scaleTexts` aus). */
export function stageZoom(stageHeightPx: number): number {
  return stageHeightPx > 0 ? stageHeightPx / GAME_HEIGHT : 1;
}

/** Ob die Kamera neu auf die Burg gesetzt wird: nur bei einem anderen Level (Seed oder Biom), nicht bei Hub-Stufe oder Gebäude-Auswahl. */
export function levelKey(world: Pick<World, 'seed' | 'biome'>): string {
  return `${world.seed}:${world.biome.id}`;
}
