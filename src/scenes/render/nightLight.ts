import { LIGHT, type LightSource, type LightSourceKind } from '../../model/data';
import type { CycleInfo, World } from '../../model/types';
import { hubLookOf } from './hubGrowth';
import { daylight } from './viewRules';

/**
 * Nacht und Licht der Darstellung (B-472, W13): Kamerahelligkeit aus `daylight()` mit der Nacht-Untergrenze aus `data/light.json`
 * und die Lichtquellen der Welt. Reine Logik ohne Phaser; Spiel (`GameScene`) und Welt-Betrachter des Studios nutzen dieselben
 * Funktionen, es gibt keine zweite Kopie der Untergrenze.
 */

/** Kamerahelligkeit in tiefster Nacht (0 = schwarz, 1 = Tag) */
export const NIGHT_BRIGHTNESS = LIGHT.nightBrightness;

type Cycle = Pick<CycleInfo, 'phase' | 'progress'>;

/** Helligkeitsfaktor der Kamera: Nacht = Untergrenze, Tag = 1, dazwischen nach `daylight()` (Dämmerung, Morgengrauen linear) */
export function brightnessOf(cycle: Cycle, night = NIGHT_BRIGHTNESS): number {
  return night + (1 - night) * daylight(cycle as CycleInfo);
}

/** Lichtquelle in der Welt: Art, Mitte in Units, Werte aus `data/light.json` */
export interface LitSource extends LightSource {
  kind: LightSourceKind;
  x: number;
}

/**
 * Lichtquellen der Welt: das Lagerfeuer ist die Burg im Look `lagerfeuer` (Hub-Stufe 1 vor dem Ausbau, `hubLookOf`). Fackeln gibt es
 * als Spielobjekt noch nicht (B-472 › Nicht-Ziele), sie leuchten erst mit eigenem Ticket. Arten ohne Werte in `data/` geben kein
 * Licht und stehen in `missing` (der Aufrufer warnt).
 */
export function lightSources(
  world: Pick<World, 'castle' | 'hubLevel' | 'hubUpgrade' | 'danger'>,
  data: Partial<Record<LightSourceKind, LightSource>> = LIGHT.sources,
): { lit: LitSource[]; missing: LightSourceKind[] } {
  const wanted: { kind: LightSourceKind; x: number }[] = hubLookOf(world).id === 'lagerfeuer' ? [{ kind: 'campfire', x: world.castle.x }] : [];
  const lit: LitSource[] = [];
  const missing: LightSourceKind[] = [];
  for (const w of wanted) {
    const d = data[w.kind];
    if (d && d.radius > 0 && d.strength > 0) lit.push({ ...w, radius: d.radius, strength: d.strength });
    else missing.push(w.kind);
  }
  return { lit, missing };
}

/** Deckkraft des Lichtscheins: Stärke × (1 − daylight), am Tag 0, in tiefster Nacht die volle Stärke (höchstens 1) */
export function glowAlpha(strength: number, cycle: Cycle): number {
  return Math.min(1, Math.max(0, strength)) * (1 - daylight(cycle as CycleInfo));
}

