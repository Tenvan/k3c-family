/**
 * Misch-, Lautheits- und Abstandsprobe der Seite Sound (WZ30.2, B-597/AC-03): reine Aufbereitung mit den Werten des Audio-Kerns
 * (`DUCK`, `attenuation`, `EDGE_GAIN`), damit die Proben dem Spiel entsprechen und nichts davon hier doppelt steht.
 */
import { DUCK } from '@game/audio/ducking';
import { attenuation, type Listener } from '@game/audio/events';
import type { AssetChoice, AssetSlot } from '../api';

export interface Pick {
  slot: AssetSlot;
  choice: AssetChoice;
}

/** Slot mit Präferenz 1, falls zugeordnet. */
export function pickOf(slots: readonly AssetSlot[], id: string): Pick | undefined {
  const slot = slots.find((s) => s.id === id);
  const choice = slot?.wahl[0];
  return slot && choice ? { slot, choice } : undefined;
}

/** Slots eines Busses mit Zuordnung, in Register-Reihenfolge (Musik-Zustände für die Mischprobe, Effekte für die Lautheit). */
export function playable(slots: readonly AssetSlot[], bus: 'music' | 'sfx'): Pick[] {
  return slots.filter((s) => s.art === 'sound' && s.bus === bus).flatMap((s) => (s.wahl[0] ? [{ slot: s, choice: s.wahl[0] }] : []));
}

/** Kampf-Mischung (B-597): Treffer, Pfeil und Tod gleichzeitig; fehlende Slots entfallen. */
export const COMBAT_SLOTS = ['sfx:hit', 'sfx:arrow', 'sfx:kill'] as const;
export const combatMix = (slots: readonly AssetSlot[]): Pick[] => COMBAT_SLOTS.flatMap((id) => pickOf(slots, id) ?? []);

/** Lautheitsvergleich: gewählte Effekte in Auswahl-Reihenfolge, alle mit Pegel 1 auf dem Bus „Effekte“; ohne Zuordnung entfallen sie. */
export const loudnessQueue = (slots: readonly AssetSlot[], ids: readonly string[]): Pick[] => ids.flatMap((id) => pickOf(slots, id) ?? []);

/** Pause zwischen zwei Tönen des Lautheitsvergleichs in ms (die Töne sind kurz, die Dauer kennt der Audio-Kern nicht). */
export const LOUDNESS_GAP_MS = 1500;

/** Beschreibung der Absenkung beim Ducking mit den Werten aus `DUCK`. */
export const duckSummary = (): string =>
  `Musik fällt in ${DUCK.attack} s auf ${Math.round(DUCK.level * 100)} %, bleibt ${DUCK.hold} s unten und kehrt in ${DUCK.release} s zurück.`;

/** Der eigene Bildausschnitt der Abstandsprobe: Mitte 20, Breite 20 Units (wie `FAR_GAIN` in playback.ts). */
export const PROBE_VIEW: Listener = { center: 20, span: 20 };

export interface DistanceStep {
  label: string;
  /** Ort der Quelle in Units */
  x: number;
  /** Faktor wie im Spiel (`attenuation`) */
  gain: number;
}

/** Stufen von „nah“ bis „stumm“: im Bild, am Rand, eine halbe und eine ganze Bildschirmbreite daneben. */
export function distanceSteps(view: Listener = PROBE_VIEW): DistanceStep[] {
  const half = view.span / 2;
  const at = (label: string, x: number): DistanceStep => ({ label, x, gain: attenuation(x, [view], false) });
  return [
    at('nah (im Bild)', view.center),
    at('am Rand', view.center + half),
    at('halbe Breite daneben', view.center + half + view.span / 2),
    at('eine Breite daneben', view.center + half + view.span),
  ];
}
