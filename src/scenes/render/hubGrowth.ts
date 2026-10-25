import type { World } from '../../model/types';

/**
 * Wachstumsreihe der Burg (GEB11, GEB17.6, docs/rules/materialien-gebaeude.md § 2.1): ein Look je Hub-Stufe und Ausbauschritt
 * (`hubStep` 1–3, docs/rules/stufen.md § 8). Nur Darstellung, Hub-Stufen und Freischaltungen bleiben. Reine Logik ohne Phaser und
 * ohne Texte; das Asset-Studio zeigt dieselbe Reihe mit Namen und Bedingung (`tools/k3c-studio/frontend/src/graphics/scenesLogic.ts`
 * › `hubWachstum`).
 */

/** Grafik-Id und Stufe unter `public/grafik/gebaeude/` (`<id>-<stufe>-<dichte>.png`) */
export interface HubGrowthLook {
  id: string;
  stufe: number;
}

/** Ausbauschritte je Hub-Stufe (`hubStep`, B-592) */
export const HUB_STEPS = 3;

/** Höchste Hub-Stufe der Reihe */
export const HUB_LEVELS = 5;

/**
 * Burg-Stufe der Hub-Stufe `level` (2–5) im Ausbauschritt `step`: Schritt 1 = Burg der Hub-Stufe (`burg-<level>`), Schritte 2 und 3
 * bauen darauf auf (`burg.json` › Stufen 6–13 mit `basis`: Wappenbanner und Fackeln, dann Wehrtürmchen mit Wimpeln).
 */
function burgStufe(level: number, step: number): number {
  return step === 1 ? level : 6 + (level - 2) * 2 + (step - 2);
}

/**
 * Spielreihenfolge, Index (Hub-Stufe − 1) × 3 + (Schritt − 1): Hub-Stufe 1 Lagerfeuer, Zelt, Holzhütte (`lagerfeuer` 1–3), danach
 * je Hub-Stufe 2–5 die Burg der Stufe und ihre zwei Ausbauschritte.
 */
export const HUB_GROWTH: readonly HubGrowthLook[] = [
  ...[1, 2, 3].map((stufe) => ({ id: 'lagerfeuer', stufe })),
  ...[2, 3, 4, 5].flatMap((level) => [1, 2, 3].map((step) => ({ id: 'burg', stufe: burgStufe(level, step) }))),
];

/** Bild vor dem Look, solange ein Bauer den Hub-Ausbau baut (`hubUpgrade.state` = `waitingWorker`, auch bei `danger`, B-572) */
export const HUB_SCAFFOLD = 'hub-geruest';

/** Look der Burg im Spiel: Grafik-Id und Stufe, Hub-Stufe (Rückfall auf `burg-<level>`), `scaffold` = Baugerüst davor */
export interface HubLook {
  id: string;
  stufe: number;
  level: number;
  scaffold: boolean;
}

/**
 * Look allein aus dem Server-Zustand (§ 2.1): Hub-Stufe und laufender Ausbauschritt (`hubStep`, fehlt = 1) wählen den Eintrag der
 * Reihe; das bezahlte Gold zählt nicht mehr (GEB17.4 begann sonst je Schritt wieder beim Lagerfeuer). Das Gerüst steht bei jedem
 * `waitingWorker`, unabhängig von `danger` (B-572).
 */
export function hubLookOf(world: Pick<World, 'hubLevel' | 'hubStep' | 'hubUpgrade' | 'danger'>): HubLook {
  const level = Math.min(HUB_LEVELS, Math.max(1, Math.trunc(world.hubLevel ?? 1) || 1));
  const step = Math.min(HUB_STEPS, Math.max(1, Math.trunc(world.hubStep ?? 1) || 1));
  const look = HUB_GROWTH[(level - 1) * HUB_STEPS + step - 1]!;
  return { ...look, level, scaffold: world.hubUpgrade?.state === 'waitingWorker' };
}
