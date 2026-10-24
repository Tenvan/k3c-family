/**
 * Musik (SO4.2, B-168, Beschluss Q16): ein reiner Zustands-Automat leitet aus Snapshot-Auszug und Ereignissen einen von acht
 * Musikzuständen ab, eine Tabelle bildet jeden auf genau ein Stück ab, der Player blendet per Crossfade über den Bus „Musik“.
 * Der Client rechnet keinen Spielzustand: er liest nur, was der Server schon geliefert hat.
 */
import type { Phase, World } from '../model/types';
import { FORMAT_ORDER, pickFile } from './atlas';
import { audioCore, type AudioCore } from './audioCore';
import { CROSSFADE_SECONDS } from './crossfade';
import assets from '../../data/assets.json';

export const MUSIC_STATES = ['day', 'dusk', 'night', 'battle', 'depth', 'boss', 'lobby', 'defeat', 'ending'] as const;
export type MusicState = (typeof MUSIC_STATES)[number];

/** Auszug des Snapshots, den der Automat braucht. `boss` und `victory` liefert die Simulation erst mit K2/K4 (bis dahin nie gesetzt). */
export interface MusicView {
  /** Die Lobby-Szene läuft (kein Spiel) */
  lobby?: boolean;
  phase: Phase;
  /** Anzahl lebender Gegner der Stufe (`world.enemies.length`) */
  enemies: number;
  /** `world.biome.depth`: 0 = Oberfläche */
  depth: number;
  boss?: boolean;
  victory?: boolean;
}

/** Rein: Auszug aus der Welt der eigenen Stufe. */
export const musicViewOf = (w: Pick<World, 'cycle' | 'enemies' | 'biome'>): MusicView => ({ phase: w.cycle.phase, enemies: w.enemies.length, depth: w.biome.depth });

/**
 * Rein: nächster Zustand. Priorität: Lobby > Niederlage > Sieg > Boss > Kampf > Tiefe > Tageszeit.
 * „Niederlage“ (`castleFallen`) und „Sieg“ (`ending`) bleiben, bis die Lobby wieder läuft (`prev`); Niederlage und Sieg haben je
 * ein eigenes Stück (B-573), die Niederlage gewinnt, wenn beides zusammentrifft.
 */
export function nextMusicState(prev: MusicState | undefined, view: MusicView, events: readonly { type: string }[] = []): MusicState {
  if (view.lobby) return 'lobby';
  if (prev === 'defeat' || events.some((e) => e.type === 'castleFallen')) return 'defeat';
  if (prev === 'ending' || view.victory) return 'ending';
  if (view.boss) return 'boss';
  if (view.enemies > 0) return 'battle';
  if (view.depth > 0) return 'depth';
  return view.phase;
}

/**
 * Zustand → Sound-Slot im Register `data/assets.json` (Auswahl SO4.1, Katalog `docs/assets/sounds.md` › Musik); `null` = Lücke.
 * Die Niederlage (`castleFallen`) hat ein eigenes Stück, das Siegstück `music:ende` bleibt dem Sieg vorbehalten (B-573, B-555).
 */
export const MUSIC_SLOTS: Readonly<Record<MusicState, string | null>> = {
  day: 'music:tag',
  dusk: 'music:abend',
  night: 'music:nacht',
  battle: 'music:kampf',
  depth: 'music:tiefe',
  boss: 'music:boss',
  lobby: 'music:lobby',
  defeat: 'music:niederlage',
  ending: 'music:ende',
};

/** Sound-Slot des Registers (nur was die Musik braucht) */
export interface MusicSlot {
  id: string;
  art: string;
  bus?: string;
  wahl: readonly { datei: string }[];
}

/** Rein: Zustand → Datei (erste Präferenz des Slots, Pfad unter `public/`) oder `null` (Lücke, Slot fehlt oder leer). */
export function musicTracks(slots: readonly MusicSlot[], map: Readonly<Record<MusicState, string | null>> = MUSIC_SLOTS): Record<MusicState, string | null> {
  const out = {} as Record<MusicState, string | null>;
  for (const state of MUSIC_STATES) {
    const slot = slots.find((s) => s.id === map[state] && s.art === 'sound' && s.bus === 'music');
    out[state] = slot?.wahl[0]?.datei ?? null;
  }
  return out;
}

/**
 * Zustand → genau ein Stück (Datei mit Endung, z. B. `audio/kandidaten/x.ogg`) oder `null` = Lücke.
 * Ersatzverhalten bei Lücke: Stille (das laufende Stück blendet aus). Geladen wird erst beim Zustandswechsel (kein Vorladen
 * aller acht Stücke beim Start, der Ladebalken bleibt kurz).
 */
export const MUSIC_TRACKS: Readonly<Record<MusicState, string | null>> = musicTracks(assets.slots as MusicSlot[]);

const EXT = /\.(ogg|mp3|wav)$/;

/**
 * Rein: abspielbare URL. Datei mit Endung → sie selbst, wenn der Browser das Format kann; Name ohne Endung → erstes abspielbares
 * Format unter `audio/` (Reihenfolge wie im Audio-Kern). Lücke oder kein Format → `undefined`.
 */
export function trackUrl(base: string | null, canPlay: (mime: string) => boolean): string | undefined {
  if (!base) return undefined;
  const ext = EXT.exec(base)?.[1] as (typeof FORMAT_ORDER)[number] | undefined;
  if (ext) return pickFile({ [ext]: base }, canPlay);
  const files: Partial<Record<(typeof FORMAT_ORDER)[number], string>> = {};
  for (const f of FORMAT_ORDER) files[f] = `audio/${base}.${f}`;
  return pickFile(files, canPlay);
}

type Core = Pick<AudioCore, 'preload' | 'playFile' | 'stop' | 'running'>;

/** Leitet den Zustand ab und spielt sein Stück, eine Musik je Gerät. Alle Methoden sind billig und werfen nie. */
export class MusicDirector {
  private state?: MusicState;
  /** Zustand, dessen Stück gerade läuft (oder dessen Lücke gilt); `undefined` = noch nichts erledigt */
  private settled?: MusicState;
  private busy = false;

  constructor(
    private readonly core: () => Core = audioCore,
    private readonly canPlay: (mime: string) => boolean = (mime) => new Audio().canPlayType(mime) !== '',
    private readonly tracks: Readonly<Record<MusicState, string | null>> = MUSIC_TRACKS,
  ) {}

  get current(): MusicState | undefined {
    return this.state;
  }

  /** Neues Spiel oder neue Szene: Zustand vergessen (auch „Ende“). */
  reset(): void {
    this.state = undefined;
  }

  lobby(): void {
    this.apply({ lobby: true, phase: 'day', enemies: 0, depth: 0 }, []);
  }

  /** Je Frame mit dem Auszug der Welt der eigenen Stufe und den Ereignissen dieses Frames. */
  update(view: MusicView, events: readonly { type: string }[] = []): void {
    this.apply(view, events);
  }

  private apply(view: MusicView, events: readonly { type: string }[]): void {
    this.state = nextMusicState(this.state, view, events);
    if (this.state !== this.settled && !this.busy) void this.play(this.state);
  }

  private async play(state: MusicState): Promise<void> {
    this.busy = true;
    try {
      const core = this.core();
      const url = trackUrl(this.tracks[state], this.canPlay);
      if (!url) {
        core.stop('music', CROSSFADE_SECONDS); // Lücke: Stille
        this.settled = state;
      } else if (core.running) {
        await core.preload(url); // erst laden: ein später fertiges altes Stück überholt das neue nicht
        // Neuer Zustand während des Ladens: nichts starten, der nächste Aufruf holt ihn. Lädt die Datei nicht, bleibt das alte
        // Stück (der Kern loggt einmal je Datei); der Zustand gilt als erledigt, damit nicht jeden Frame neu versucht wird.
        if (this.state === state) {
          await core.playFile(url, { bus: 'music', loop: true, fade: CROSSFADE_SECONDS });
          this.settled = state;
        }
      } // gesperrt (noch keine Eingabe): nichts erledigt, der nächste Aufruf versucht es wieder
    } catch {
      this.settled = state; // nie ein Fehler nach außen
    } finally {
      this.busy = false;
    }
  }
}

let shared: MusicDirector | undefined;

/** Der Musik-Regisseur der Seite (lazy wie `audioCore()`). */
export function musicDirector(): MusicDirector {
  return (shared ??= new MusicDirector());
}
