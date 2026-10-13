/**
 * Audio-Kern (B-011): erzeugt den `AudioContext` erst bei der ersten Eingabe (davor kein Audio, keine Browser-Warnung) und ruft
 * `resume()`, bis er läuft (Gamepad-Tasten zählen in Chromium nicht immer als Geste: dann bleibt es stumm, ohne Fehler).
 * Danach: Mixer und Sound-Atlas laden (2 Requests: `atlas.json` + eine Audiodatei im gewählten Format).
 */
import { clientLog } from '../core/clientLog';
import { parseAtlas, pickFile, type Atlas } from './atlas';
import type { GameEvent } from '../model/types';
import { attenuation, cueFor, type Listener } from './events';
import { Mixer, type AudioContextLike, type Bus, type GainNodeLike } from './mixer';
import { QUICK_FADE_SECONDS, crossfadeCurves, fadeOutFrom } from './crossfade';

export interface SourceLike {
  buffer: unknown;
  connect(target: unknown): unknown;
  start(when: number, offset: number, duration?: number): void;
  loop?: boolean;
  stop?(when?: number): void;
  onended?: (() => void) | null;
}
export interface FullContext extends AudioContextLike {
  state: string;
  currentTime: number;
  resume(): Promise<void>;
  decodeAudioData(data: ArrayBuffer): Promise<unknown>;
  createBufferSource(): SourceLike;
}
export interface CoreDeps {
  createContext(): FullContext;
  canPlay(mime: string): boolean;
  fetchJson(url: string): Promise<unknown>;
  fetchBytes(url: string): Promise<ArrayBuffer>;
  storage?: Pick<Storage, 'getItem' | 'setItem'>;
}

const ATLAS_URL = 'audio/atlas.json';

/** Wiedergabe einer ganzen Datei (Musik, Hörprobe) auf einem Bus. */
export interface FileOptions {
  /** Standard `music` */
  bus?: Bus;
  /** Standard: Schleife auf dem Bus `music`, sonst einmal */
  loop?: boolean;
  /** Einblenden und das vorige Stück desselben Busses ausblenden in Sekunden; 0 = kurzer Schnitt */
  fade?: number;
  /** Das Stück ist zu Ende, ohne dass es ersetzt oder gestoppt wurde */
  onEnded?: () => void;
}
interface Voice {
  url: string;
  src: SourceLike;
  gain: GainNodeLike;
}

export class AudioCore {
  private ctx?: FullContext;
  private mixer?: Mixer;
  private atlas?: Atlas;
  private buffer?: unknown;
  private readonly files = new Map<string, Promise<unknown>>();
  private readonly voices: Partial<Record<Bus, Voice>> = {};

  constructor(private readonly deps: CoreDeps) {}

  get running(): boolean {
    return this.ctx?.state === 'running';
  }

  /** Erste (und jede weitere, solange gesperrt) Eingabe. Wirft nie. */
  async onInput(): Promise<void> {
    if (this.running) return;
    try {
      if (!this.ctx) {
        this.ctx = this.deps.createContext();
        this.mixer = new Mixer(this.ctx, this.deps.storage);
        void this.loadAtlas();
      }
      await this.ctx.resume();
    } catch (e) {
      clientLog('warn', '🔇 Audio nicht startbar', { error: String(e) });
    }
  }

  /** Spielt einen Sprite des Atlas auf einem Bus; noch gesperrt oder unbekannt → nichts. */
  play(name: string, bus: Bus = 'sfx', volume = 1): boolean {
    const sprite = this.atlas?.sprites[name];
    if (!sprite || !this.ctx || !this.mixer || !this.buffer || !this.running) return false;
    if (volume <= 0) return false;
    const src = this.ctx.createBufferSource();
    src.buffer = this.buffer;
    const target = this.mixer.node(bus) as GainNodeLike;
    if (volume < 1) {
      const g = this.ctx.createGain(); // Dämpfung je Ton (Position), der Bus-Regler bleibt unberührt
      g.gain.value = volume;
      g.connect(target);
      src.connect(g);
    } else src.connect(target);
    src.start(this.ctx.currentTime, sprite.start, sprite.duration);
    return true;
  }

  /** Spiel-Ereignis → Ton, je nach Position gedämpft; `x` in Units (Quelle), `listeners` = Kameras der lokalen Spieler. */
  onEvent(e: GameEvent, x: number | undefined, listeners: readonly Listener[]): boolean {
    const cue = cueFor(e);
    return !!cue && this.play(cue.sprite, 'sfx', attenuation(x, listeners, cue.global));
  }

  /** Lädt und dekodiert eine Datei einmal je URL; `false` bei Fehler (geloggt) oder ohne laufenden Kontext. Wirft nie. */
  async preload(url: string): Promise<boolean> {
    return (await this.loadBuffer(url)) !== undefined;
  }

  /**
   * Spielt eine ganze Datei auf einem Bus, je Bus ein Stück: ein neues ersetzt das alte, mit `fade` > 0 per Crossfade
   * (das alte wird vom aktuellen Pegel aus ausgeblendet, auch mitten im Übergang). Dasselbe Stück läuft einfach weiter.
   * `false`, wenn gesperrt oder die Datei nicht lädt; dann bleibt das bisherige Stück unberührt.
   */
  async playFile(url: string, opts: FileOptions = {}): Promise<boolean> {
    const bus = opts.bus ?? 'music';
    const buf = await this.loadBuffer(url);
    const { ctx, mixer } = this;
    if (buf === undefined || !ctx || !mixer || !this.running) return false;
    const old = this.voices[bus];
    if (old?.url === url) return true;
    const fade = Math.max(0, opts.fade ?? 0);
    const src = ctx.createBufferSource();
    src.buffer = buf;
    src.loop = opts.loop ?? bus === 'music';
    const gain = ctx.createGain();
    if (fade > 0) gain.gain.setValueCurveAtTime?.(crossfadeCurves().in, ctx.currentTime, fade);
    src.connect(gain);
    gain.connect(mixer.node(bus));
    src.start(ctx.currentTime, 0);
    const voice: Voice = { url, src, gain };
    src.onended = () => {
      if (this.voices[bus] !== voice) return;
      delete this.voices[bus];
      opts.onEnded?.();
    };
    this.voices[bus] = voice;
    if (old) this.release(old, fade > 0 ? fade : QUICK_FADE_SECONDS);
    return true;
  }

  /** Beendet das Stück eines Busses mit kurzem oder langem Ausblenden. */
  stop(bus: Bus, fade = QUICK_FADE_SECONDS): void {
    const v = this.voices[bus];
    delete this.voices[bus];
    if (v) this.release(v, fade);
  }

  /** Läuft auf dem Bus gerade ein Stück (`url`: dieses)? */
  isPlaying(bus: Bus, url?: string): boolean {
    const v = this.voices[bus];
    return !!v && (url === undefined || v.url === url);
  }

  /** Lautstärke eines Busses in Prozent; vor der Entsperrung der Standard. */
  getVolume(bus: Bus): number {
    return this.mixer?.getVolume(bus) ?? 100;
  }

  setVolume(bus: Bus, percent: number): void {
    this.mixer?.setVolume(bus, percent);
  }

  /** Ausblenden vom aktuellen Pegel, danach beenden. Scheitert die Kurve, wird hart gestoppt (nie ein Fehler nach außen). */
  private release(v: Voice, seconds: number): void {
    const ctx = this.ctx;
    if (!ctx) return;
    const now = ctx.currentTime;
    try {
      v.gain.gain.cancelScheduledValues?.(now);
      v.gain.gain.setValueCurveAtTime?.(fadeOutFrom(v.gain.gain.value), now, seconds);
      v.src.stop?.(now + seconds);
    } catch {
      try {
        v.src.stop?.(now);
      } catch {
        /* schon beendet */
      }
    }
  }

  private loadBuffer(url: string): Promise<unknown> {
    if (!this.ctx) return Promise.resolve(undefined);
    let p = this.files.get(url);
    if (!p) {
      p = this.deps
        .fetchBytes(url)
        .then((bytes) => this.ctx!.decodeAudioData(bytes))
        .catch((e) => {
          clientLog('warn', '🔇 Audiodatei nicht ladbar', { url, error: String(e) });
          return undefined;
        });
      this.files.set(url, p);
    }
    return p;
  }

  private async loadAtlas(): Promise<void> {
    try {
      const atlas = parseAtlas(await this.deps.fetchJson(ATLAS_URL));
      const file = atlas && pickFile(atlas.files, this.deps.canPlay);
      if (!atlas || !file) return clientLog('warn', '🔇 Kein Audioformat im Atlas abspielbar');
      this.buffer = await this.ctx!.decodeAudioData(await this.deps.fetchBytes(`audio/${file}`));
      this.atlas = atlas;
      clientLog('info', '🔊 Sound-Atlas geladen', { file, sounds: Object.keys(atlas.sprites).length });
    } catch (e) {
      clientLog('warn', '🔇 Sound-Atlas nicht ladbar', { error: String(e) });
    }
  }
}

let shared: AudioCore | undefined;

/** Der Audio-Kern der Seite (Browser-Anbindung; lazy, damit Tests und Seiten ohne Audio nichts anlegen). */
export function audioCore(): AudioCore {
  return (shared ??= new AudioCore({
    createContext: () => new AudioContext() as unknown as FullContext,
    canPlay: (mime) => new Audio().canPlayType(mime) !== '',
    fetchJson: async (url) => (await fetch(url)).json(),
    fetchBytes: async (url) => (await fetch(url)).arrayBuffer(),
    storage: (() => {
      try {
        return window.localStorage;
      } catch {
        return undefined;
      }
    })(),
  }));
}
