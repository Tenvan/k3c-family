/**
 * Audio-Kern (B-011): erzeugt den `AudioContext` erst bei der ersten Eingabe (davor kein Audio, keine Browser-Warnung) und ruft
 * `resume()`, bis er läuft (Gamepad-Tasten zählen in Chromium nicht immer als Geste: dann bleibt es stumm, ohne Fehler).
 * Danach: Mixer und Sound-Atlas laden (2 Requests: `atlas.json` + eine Audiodatei im gewählten Format).
 */
import { clientLog } from '../core/clientLog';
import { parseAtlas, pickFile, type Atlas } from './atlas';
import { Mixer, type AudioContextLike, type Bus, type GainNodeLike } from './mixer';

export interface SourceLike {
  buffer: unknown;
  connect(target: unknown): unknown;
  start(when: number, offset: number, duration: number): void;
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

export class AudioCore {
  private ctx?: FullContext;
  private mixer?: Mixer;
  private atlas?: Atlas;
  private buffer?: unknown;

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
  play(name: string, bus: Bus = 'sfx'): boolean {
    const sprite = this.atlas?.sprites[name];
    if (!sprite || !this.ctx || !this.mixer || !this.buffer || !this.running) return false;
    const src = this.ctx.createBufferSource();
    src.buffer = this.buffer;
    src.connect(this.mixer.node(bus) as GainNodeLike);
    src.start(this.ctx.currentTime, sprite.start, sprite.duration);
    return true;
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
