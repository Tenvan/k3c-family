/**
 * Audio-Mixer (B-011): ein Master-Gain, daran je ein Bus für Musik, Effekte und Ambient mit eigener Lautstärke.
 * Die Lautstärken liegen je Gerät in den Einstellungen (`core/settings.ts`, `localStorage`); kaputter Speicher → Standard.
 * Der `AudioContext` wird von außen gereicht (SO1.2 entsperrt ihn), damit der Mixer ohne echtes Audio testbar bleibt.
 */
import { clientLog } from '../core/clientLog';
import { clampVolume, loadSettings, saveSettings, type Settings } from '../core/settings';

export type Bus = 'music' | 'sfx' | 'ambient';
export const BUSES: readonly Bus[] = ['music', 'sfx', 'ambient'];

const FIELD = { music: 'musicVolume', sfx: 'sfxVolume', ambient: 'ambientVolume' } as const satisfies Record<Bus, keyof Settings>;

/** Der Teil des `AudioContext`, den der Mixer braucht. */
export interface GainNodeLike {
  gain: { value: number };
  connect(target: unknown): unknown;
}
export interface AudioContextLike {
  destination: unknown;
  createGain(): GainNodeLike;
}
type VolumeStorage = Pick<Storage, 'getItem' | 'setItem'>;

/** Rein: Prozent 0–100 → Faktor 0–1 (keine Zahl → 1). */
export const volumeToGain = (percent: number): number => clampVolume(percent, 100) / 100;

export class Mixer {
  readonly master: GainNodeLike;
  private readonly buses: Record<Bus, GainNodeLike>;
  private settings: Settings;

  constructor(
    ctx: AudioContextLike,
    private readonly storage?: VolumeStorage,
  ) {
    this.settings = loadSettings(storage);
    this.master = ctx.createGain();
    this.master.connect(ctx.destination);
    const make = (bus: Bus): GainNodeLike => {
      const node = ctx.createGain();
      node.gain.value = volumeToGain(this.settings[FIELD[bus]]);
      node.connect(this.master);
      return node;
    };
    this.buses = { music: make('music'), sfx: make('sfx'), ambient: make('ambient') };
    clientLog('info', '🔊 Mixer bereit', { music: this.getVolume('music'), sfx: this.getVolume('sfx'), ambient: this.getVolume('ambient') });
  }

  /** Eingang eines Busses: Quellen verbinden sich hiermit. */
  node(bus: Bus): GainNodeLike {
    return this.buses[bus];
  }

  getVolume(bus: Bus): number {
    return this.settings[FIELD[bus]];
  }

  /** Setzt einen Bus (Prozent, begrenzt auf 0–100) und speichert; die anderen Busse bleiben unverändert. */
  setVolume(bus: Bus, percent: number): void {
    const value = clampVolume(percent, this.getVolume(bus));
    this.settings = { ...this.settings, [FIELD[bus]]: value };
    this.buses[bus].gain.value = volumeToGain(value);
    saveSettings(this.settings, this.storage);
  }
}
