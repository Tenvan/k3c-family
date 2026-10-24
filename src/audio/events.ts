/**
 * Ereignis → Ton (B-011, SO1.3): reine Zuordnung und Positions-Dämpfung. Audio rechnet keine Spielregel,
 * es reagiert nur auf Ereignisse und Kamera-Ausschnitte. Welche Ereignisse klingen, klärt SO2 (Q15, B-167).
 */
import type { GameEvent } from '../model/types';
import assets from '../../data/assets.json';

/** Sichtbarer Ausschnitt eines lokalen Spielers in Units (Mitte und Breite der Kamera). */
export interface Listener {
  center: number;
  span: number;
}

export interface SoundCue {
  /** Sound-Slot im Register `data/assets.json` (`sfx:coin` …), Katalog `docs/assets/sounds.md` */
  slot: string;
  /** Warnung: überall gleich laut, keine Dämpfung */
  global: boolean;
}

/** Faktor direkt am Rand des sichtbaren Bereichs; darüber hinaus fällt er bis auf 0 (Abstand = eine Bildschirmbreite). */
export const EDGE_GAIN = 0.5;

/** Nacht leise (Q15, B-167/AC-06): Effekte klingen nachts gedämpft, kindgerecht statt erschreckend. */
export const NIGHT_GAIN = 0.6;

/** Rein: Pegelfaktor der Effekte zur Tageszeit (`world.cycle.phase`). */
export const nightGain = (phase?: string): number => (phase === 'night' ? NIGHT_GAIN : 1);

/** Ereignis-Typ → Sound-Slot; Warnungen (`global`) klingen überall gleich laut. Katalog `docs/assets/sounds.md`. */
const CUES: Readonly<Partial<Record<GameEvent['type'], SoundCue>>> = {
  coinPickup: { slot: 'sfx:coin', global: false },
  hit: { slot: 'sfx:hit', global: false },
  kill: { slot: 'sfx:kill', global: false },
  arrow: { slot: 'sfx:arrow', global: false },
  strike: { slot: 'sfx:strike', global: false },
  built: { slot: 'sfx:build', global: false },
  playerDown: { slot: 'sfx:death', global: false },
  revive: { slot: 'sfx:revive', global: false },
  skillPoint: { slot: 'sfx:skill', global: false },
  arrived: { slot: 'sfx:portal', global: false },
  coinGive: { slot: 'sfx:coinGive', global: false },
  buildProgress: { slot: 'sfx:buildProgress', global: false },
  revived: { slot: 'sfx:revived', global: false },
  bossSpawned: { slot: 'sfx:boss', global: true },
  dusk: { slot: 'sfx:warn', global: true },
  wave: { slot: 'sfx:warn', global: true },
  castleFallen: { slot: 'sfx:warn', global: true },
};

/** Rein: Ereignis → Sound-Slot. Nur lesen, kein Spielzustand (B-167/AC-04); Lücken des Katalogs und unbekannte Typen → `null` (stumm). */
export function cueFor(e: GameEvent): SoundCue | null {
  return CUES[e.type] ?? null;
}

/** Rein: Hub-Ausbau (SO6.4) = die Hub-Stufe im Snapshot steigt; ohne vorigen oder neuen Wert kein Ausbau. */
export const hubRaised = (prev: number | undefined, next: number | undefined): boolean => prev !== undefined && next !== undefined && next > prev;

/** Rein: Ort der Quelle in Units: `x` des Ereignisses, sonst `fallback` (z. B. Ort des Effekts); ohne beides `undefined` (keine Dämpfung). */
export const eventX = (e: GameEvent, fallback?: number): number | undefined => ('x' in e && typeof e.x === 'number' ? e.x : fallback);

/** Ab dieser Geschwindigkeit (Units/s) hört man den Hufschlag (B-574); darunter steht das Pferd. */
export const RIDE_MIN_SPEED = 0.3;
/** Pegel der Hufschlag-Schleife: ein dezenter Teppich unter den Effekten. */
export const RIDE_GAIN = 0.5;

/**
 * Rein: Hufschlag je reitendem Spieler (B-574). Reitend = lebt und bewegt sich (`vx` im Snapshot; der Monarch ist immer beritten,
 * `data/monarch.json` › `mount`). Schlüssel = `index` des Spielers, Pegel nach Abstand zur Kamera und Tageszeit wie die Effekte.
 */
export function rideLoops(players: readonly { index: number; x: number; vx: number; respawnIn: number }[], listeners: readonly Listener[], phase?: string): { key: number; volume: number }[] {
  return players
    .filter((p) => p.respawnIn <= 0 && Math.abs(p.vx) >= RIDE_MIN_SPEED)
    .map((p) => ({ key: p.index, volume: attenuation(p.x, listeners, false) * nightGain(phase) * RIDE_GAIN }))
    .filter((l) => l.volume > 0);
}

/** Sound-Slot des Registers (nur was der Client braucht) */
export interface SfxSlot {
  id: string;
  art: string;
  bus?: string;
  wahl: readonly { datei: string; sprite?: string }[];
}

/** Rein: Slot → Datei (erste Präferenz ohne Atlas-Sprite) für alle Effekt-Slots; Slots ohne Datei fehlen. */
export function sfxUrls(slots: readonly SfxSlot[]): Readonly<Record<string, string>> {
  const out: Record<string, string> = {};
  for (const s of slots) {
    const file = s.art === 'sound' && s.bus === 'sfx' ? s.wahl.find((w) => !w.sprite)?.datei : undefined;
    if (file) out[s.id] = file;
  }
  return out;
}

/** Lautstärke 0–1 einer Quelle bei `x` (Units): im Bereich eines Listeners 1, sonst leiser; der lauteste Listener gilt. */
export function attenuation(x: number | undefined, listeners: readonly Listener[], global: boolean): number {
  if (global || listeners.length === 0 || x === undefined) return 1;
  const best = Math.min(...listeners.map((l) => Math.max(0, Math.abs(x - l.center) - l.span / 2) / Math.max(l.span, 1)));
  return best === 0 ? 1 : Math.max(0, EDGE_GAIN * (1 - best));
}

/** Die Dateien der Effekte laut Slot-Register (erste Präferenz je Slot). */
export const SFX_URLS: Readonly<Record<string, string>> = sfxUrls(assets.slots as SfxSlot[]);
