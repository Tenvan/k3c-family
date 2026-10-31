import register from '../../../data/assets.json';
import { BUILDING_TEXTURES, hubLookTexture, textureScale } from './buildingSprites';
import { HUB_GROWTH, HUB_STEPS } from './hubGrowth';
import { fitTarget, validTarget, type PxSize, type TargetFit } from './targetFit';

/**
 * Gebäude-Grafik aus dem Slot-Register `data/assets.json` (B-685): je Textur-Schlüssel der Slot (Gebäude je Stufe), daraus die
 * aktive Zuordnung (Präferenz 1, Ausschnitt) und das Zielmaß `ziel` in Units. Ohne Slot oder Zuordnung bleibt die Datei aus
 * `BUILDING_TEXTURES`, ohne Zielmaß der heutige Maßstab (`textureScale`). Rein, ohne Phaser.
 */

export interface CutRect extends PxSize {
  x: number;
  y: number;
}

interface RegisterSlot {
  id: string;
  art: string;
  ziel?: { w?: unknown; h?: unknown };
  wahl: { datei: string; ausschnitt?: CutRect }[];
}

const SLOTS = new Map((register as unknown as { slots: RegisterSlot[] }).slots.map((s) => [s.id, s]));

/** Wachstumsreihe der Burg (GEB17.6): Index (Hub-Stufe − 1) × 3 + (Schritt − 1) → `hub:camp:<schritt>`, `hub:<stufe>`, `hub:<stufe>:<schritt>` */
const GROWTH_SLOTS = new Map(HUB_GROWTH.map((look, i) => {
  const level = Math.floor(i / HUB_STEPS) + 1;
  const step = (i % HUB_STEPS) + 1;
  return [hubLookTexture(look), level === 1 ? `hub:camp:${step}` : step === 1 ? `hub:${level}` : `hub:${level}:${step}`];
}));

/** Slot eines Textur-Schlüssels: Burg-Reihe, `grafik:<kind>-<n>` → `<kind>:<n>`, `grafik:<kind>` → `<kind>`; Gerüste und Hintergrundmauer ohne Slot (`null`) */
export function buildingSlot(key: string): string | null {
  const growth = GROWTH_SLOTS.get(key);
  if (growth) return growth;
  const m = /^grafik:([A-Za-z]+)(?:-(\d+))?$/.exec(key);
  const id = m ? (m[2] ? `${m[1]}:${m[2]}` : m[1]!) : '';
  return SLOTS.get(id)?.art === 'grafik' ? id : null;
}

/** Datei unter `public/grafik/` und Ausschnitt der aktiven Zuordnung; ohne Slot oder Zuordnung die Datei aus `BUILDING_TEXTURES` */
export function buildingSource(key: string): { file: string; cut?: CutRect } | null {
  const first = SLOTS.get(buildingSlot(key) ?? '')?.wahl[0];
  if (first?.datei.startsWith('grafik/')) return { file: first.datei.slice('grafik/'.length), cut: first.ausschnitt };
  const file = BUILDING_TEXTURES[key];
  return file ? { file } : null;
}

/** Zielmaß in Units; fehlt es oder ist es 0, `null` (heutiges Maß) */
export function buildingTarget(key: string): PxSize | null {
  return validTarget(SLOTS.get(buildingSlot(key) ?? '')?.ziel);
}

/** Darstellung eines Schlüssels bei Quellgröße `source` (Ausschnitt oder ganzes Bild): eingepasst ins Zielmaß, sonst heutiger Maßstab */
export function buildingFit(key: string, source: PxSize): TargetFit {
  return fitTarget(source, buildingTarget(key), textureScale(key));
}
