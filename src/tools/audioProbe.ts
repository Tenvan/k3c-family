/**
 * Audio-Probe der Gamepad-Testseite (B-166, X1.1): Zustand des AudioContext vor und nach einer Geste, Abspielversuch
 * ohne Geste, Dekodierung je Format und Latenz. Das Ergebnis steht als Feld `audio` im Bericht (X1.3 liest es so).
 */

import { t, textOf } from './texts';

export const FORMATS = ['ogg', 'm4a', 'mp3', 'wav'] as const;
export type Format = (typeof FORMATS)[number];
export type GestureVia = 'controller' | 'taste' | 'klick';

export interface AudioReport {
  contextState: { beforeGesture: string; afterFirstGesture: string | null };
  gestures: { via: GestureVia; state: string }[];
  autoplayWithoutGesture: string; // 'gespielt' | 'blockiert' | 'fehler: <Name>'
  formats: Record<Format, boolean>;
  baseLatency: number | null;
  outputLatency: number | null;
}

/** Was der Probelauf gesammelt hat; `context` ist `null`, wenn der Browser keinen AudioContext hat. */
export interface AudioProbeInput {
  context: { baseLatency?: number; outputLatency?: number } | null;
  beforeGesture: string;
  gestures: { via: GestureVia; state: string }[];
  autoplay: string;
  formats: Record<Format, boolean>;
}

const AUDIO_DIR = 'audio-test/test.';
const MAX_GESTURES = 20;
const RESUME_WAIT_MS = 300; // resume() ohne Geste bleibt in manchen Browsern offen

/** Lädt und dekodiert jedes Format; ein Fehler ergibt `false`, die anderen Formate laufen weiter. */
export async function decodeFormats(
  load: (format: Format) => Promise<ArrayBuffer>,
  decode: (data: ArrayBuffer) => Promise<unknown>,
): Promise<Record<Format, boolean>> {
  const result = { ogg: false, m4a: false, mp3: false, wav: false };
  for (const f of FORMATS) {
    try {
      await decode(await load(f));
      result[f] = true;
    } catch {
      result[f] = false;
    }
  }
  return result;
}

/** Das Feld `audio` des Berichts, `null` ohne AudioContext. */
export function buildAudioReport(input: AudioProbeInput): AudioReport | null {
  if (!input.context) return null;
  const latency = (v: number | undefined) => (typeof v === 'number' && Number.isFinite(v) ? v : null);
  return {
    contextState: { beforeGesture: input.beforeGesture, afterFirstGesture: input.gestures[0]?.state ?? null },
    gestures: input.gestures.map((g) => ({ ...g })),
    autoplayWithoutGesture: input.autoplay,
    formats: { ...input.formats },
    baseLatency: latency(input.context.baseLatency),
    outputLatency: latency(input.context.outputLatency),
  };
}

/** Anzeigewert des Abspielversuchs: Der Bericht behält die Kennungen (`gespielt`, `blockiert`, `fehler: …`), nur die Anzeige wird übersetzt. */
function autoplayText(value: string): string {
  if (value === 'gespielt') return t('audio.played');
  if (value === 'blockiert') return t('audio.blocked');
  return value.startsWith('fehler: ') ? t('audio.error', { name: value.slice('fehler: '.length) }) : value;
}

/** Anzeigezeilen `[Bezeichnung, Wert, ok?]` wie in der Umgebung der Testseite. */
export function audioRows(audio: AudioReport | null): [string, string, boolean?][] {
  if (!audio) return [['Audio', t('audio.none'), false]];
  const ms = (v: number | null) => (v === null ? t('audio.unknown') : `${Math.round(v * 1000)} ms`);
  const after = audio.contextState.afterFirstGesture;
  return [
    [t('audio.beforeGesture'), audio.contextState.beforeGesture, audio.contextState.beforeGesture === 'running'],
    [t('audio.afterGesture'), after ?? t('audio.noGestureYet'), after === null ? undefined : after === 'running'],
    [t('audio.gestures'), audio.gestures.map((g) => `${textOf(`audio.via.${g.via}`, g.via)}: ${g.state}`).join(', ') || '–'],
    [t('audio.autoplay'), autoplayText(audio.autoplayWithoutGesture)],
    ...FORMATS.map((f): [string, string, boolean] => [t('audio.format', { format: f }), audio.formats[f] ? t('audio.yes') : t('audio.no'), audio.formats[f]]),
    [t('audio.latency'), t('audio.latencyValue', { base: ms(audio.baseLatency), out: ms(audio.outputLatency) })],
  ];
}

/** Abspielversuch ohne Geste über ein Audio-Element, sofort wieder angehalten. */
async function tryAutoplay(): Promise<string> {
  const el = new Audio(`${AUDIO_DIR}wav`);
  try {
    await el.play();
    el.pause();
    return 'gespielt';
  } catch (err) {
    const name = err instanceof Error ? err.name : String(err);
    return name === 'NotAllowedError' ? 'blockiert' : `fehler: ${name}`;
  }
}

async function load(format: Format): Promise<ArrayBuffer> {
  const res = await fetch(`${AUDIO_DIR}${format}`);
  if (!res.ok) throw new Error(`HTTP ${res.status}`);
  return res.arrayBuffer();
}

export interface AudioProbe {
  /** Das Feld `audio`; `null`, solange der Probelauf läuft oder es keinen AudioContext gibt. */
  report(): AudioReport | null;
  gesture(via: GestureVia): Promise<void>;
  /** Geste per Klick und ein kurzer Testton über den Context. */
  playTone(): Promise<void>;
}

/** Startet den Probelauf beim Laden der Seite (ohne Geste); `onChange` meldet jedes neue Ergebnis. */
export function startAudioProbe(onChange: () => void): AudioProbe {
  const Ctx = window.AudioContext as typeof AudioContext | undefined;
  let ctx: AudioContext | null = null;
  try {
    ctx = Ctx ? new Ctx() : null;
  } catch {
    ctx = null;
  }
  const input: AudioProbeInput = { context: ctx, beforeGesture: ctx?.state ?? 'kein', gestures: [], autoplay: '', formats: { ogg: false, m4a: false, mp3: false, wav: false } };
  let ready = false;
  if (ctx) {
    const c = ctx;
    void Promise.all([tryAutoplay(), decodeFormats(load, (b) => c.decodeAudioData(b))]).then(([autoplay, formats]) => {
      Object.assign(input, { autoplay, formats });
      ready = true;
      onChange();
    });
  }
  const gesture = async (via: GestureVia) => {
    if (!ctx || input.gestures.length >= MAX_GESTURES) return;
    await Promise.race([ctx.resume().catch(() => undefined), new Promise((r) => setTimeout(r, RESUME_WAIT_MS))]);
    input.gestures.push({ via, state: ctx.state });
    onChange();
  };
  return {
    report: () => (ready ? buildAudioReport(input) : null),
    gesture,
    async playTone() {
      await gesture('klick');
      if (!ctx) return;
      const src = ctx.createBufferSource();
      src.buffer = await ctx.decodeAudioData(await load('wav'));
      src.connect(ctx.destination);
      src.start();
    },
  };
}
