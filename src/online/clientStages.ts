import { BIOMES, type BiomeConfig } from '../model/biome';
import type { LevelLayout } from '../model/types';
import { clientLog } from '../core/clientLog';
import { applyDelta } from './clientDelta';
import type { SlotSeat, WorldState } from './clientProtocol';

/**
 * Ströme je Stufe (B-176, docs/protocol.md › Mehrere Stufen je Gerät): je `stage` Level, Zustand und empfangene Frames.
 * `level` beginnt einen Strom neu, `snap` setzt den Zustand, `delta` gilt nur mit Zustand seiner Stufe.
 */

/** Höchstens so viele ungelesene Frames je Stufe (4 s bei 30 Hz); ältere fallen weg, auch für Stufen, die niemand liest. */
const MAX_FRAMES = 120;

/** Ein Tick: voller Zustand (nicht verändern, Teile werden mit früheren Frames geteilt). */
export interface Frame {
  tick: number;
  ack: number;
  /** `env.now()` beim Empfang, Grundlage der Interpolation */
  receivedAt: number;
  state: WorldState;
}

export interface LevelInfo {
  depth: number;
  layout: LevelLayout;
  biome: BiomeConfig;
}

/** `snap` oder `delta` einer Stufe. */
export interface StateMessage {
  t: 'snap' | 'delta';
  stage: number;
  tick: number;
  ack: number;
  s: WorldState | Record<string, unknown>;
}

interface Stream {
  level: LevelInfo | null;
  state: Record<string, unknown> | null;
  frames: Frame[];
  received: number;
}

export class StageStreams {
  private readonly streams = new Map<number, Stream>();

  /** Stufen mit Strom, aufsteigend. */
  stages(): number[] {
    return [...this.streams.keys()].sort((a, b) => a - b);
  }

  levelOf(stage: number): LevelInfo | null {
    return this.streams.get(stage)?.level ?? null;
  }

  /** Alle seit dem letzten Aufruf empfangenen Ticks der Stufe, älteste zuerst. */
  takeFramesOf(stage: number): Frame[] {
    const stream = this.streams.get(stage);
    if (!stream) return [];
    const frames = stream.frames;
    stream.frames = [];
    return frames;
  }

  /** `level`: Der Strom der Stufe beginnt neu. false: Biom unbekannt (der Strom hat dann kein Level). */
  level(stage: number, depth: number, layout: LevelLayout): boolean {
    const biome = BIOMES.find((b) => b.id === layout.biomeId);
    this.streams.set(stage, { level: biome ? { depth, layout, biome } : null, state: null, frames: [], received: 0 });
    if (!biome) {
      clientLog('error', `💥 Unbekanntes Biom ${layout.biomeId}`, { stage, depth });
      return false;
    }
    clientLog('info', '📂 Level empfangen', { stage, depth, biome: biome.id, width: layout.widthUnits });
    return true;
  }

  /** `snap` oder `delta`: neuer Frame der Stufe; ein `delta` ohne Zustand seiner Stufe wird verworfen (null). */
  state(msg: StateMessage, receivedAt: number): Frame | null {
    let stream = this.streams.get(msg.stage);
    if (msg.t === 'delta' && !stream?.state) {
      console.warn(`delta ohne snap verworfen (Stufe ${msg.stage})`);
      return null;
    }
    if (!stream) {
      stream = { level: null, state: null, frames: [], received: 0 };
      this.streams.set(msg.stage, stream);
    }
    stream.state = msg.t === 'snap' ? (msg.s as Record<string, unknown>) : applyDelta(stream.state!, msg.s as Record<string, unknown>);
    const frame: Frame = { tick: msg.tick, ack: msg.ack, receivedAt, state: stream.state as unknown as WorldState };
    if (stream.received++ === 0) clientLog('info', '✅ Erster Zustand nach Level', { stage: msg.stage, tick: msg.tick, sites: frame.state.sites?.map((s) => s.kind) });
    stream.frames.push(frame);
    if (stream.frames.length > MAX_FRAMES) stream.frames.shift();
    return frame;
  }

  /** Nur Ströme der Stufen behalten, in denen ein eigener Slot steht (`seats`): Die anderen sind beendet. */
  keep(you: SlotSeat[]): void {
    for (const stage of this.streams.keys()) if (!you.some((s) => s.stage === stage)) this.streams.delete(stage);
  }

  clear(): void {
    this.streams.clear();
  }
}
