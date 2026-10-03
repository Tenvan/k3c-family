import { describe, expect, it } from 'vitest';
import { audioRows, buildAudioReport, decodeFormats, type AudioProbeInput } from './audioProbe';

const input: AudioProbeInput = {
  context: { baseLatency: 0.01, outputLatency: 0.045 },
  beforeGesture: 'suspended',
  gestures: [
    { via: 'controller', state: 'suspended' },
    { via: 'klick', state: 'running' },
  ],
  autoplay: 'blockiert',
  formats: { ogg: false, m4a: true, mp3: true, wav: true },
};

describe('Audio-Probe der Gamepad-Testseite (B-166)', () => {
  it('ohne AudioContext ist das Feld audio null, ohne Fehler', () => {
    expect(buildAudioReport({ ...input, context: null })).toBeNull();
  });

  it('baut das Feld audio in der festen Form', () => {
    expect(buildAudioReport(input)).toEqual({
      contextState: { beforeGesture: 'suspended', afterFirstGesture: 'suspended' },
      gestures: input.gestures,
      autoplayWithoutGesture: 'blockiert',
      formats: { ogg: false, m4a: true, mp3: true, wav: true },
      baseLatency: 0.01,
      outputLatency: 0.045,
    });
  });

  it('fehlende Latenz und fehlende Geste ergeben null', () => {
    const audio = buildAudioReport({ ...input, context: {}, gestures: [] });
    expect(audio?.baseLatency).toBeNull();
    expect(audio?.outputLatency).toBeNull();
    expect(audio?.contextState.afterFirstGesture).toBeNull();
  });

  it('ein Format, das nicht dekodiert, ist false; die anderen werden weiter geprüft', async () => {
    const decoded: string[] = [];
    const formats = await decodeFormats(
      async (f) => new TextEncoder().encode(f).buffer as ArrayBuffer,
      async (b) => {
        const f = new TextDecoder().decode(b);
        if (f === 'ogg') throw new Error('EncodingError');
        decoded.push(f);
      },
    );
    expect(formats).toEqual({ ogg: false, m4a: true, mp3: true, wav: true });
    expect(decoded).toEqual(['m4a', 'mp3', 'wav']);
  });

  it('ein Ladefehler ergibt ebenfalls false', async () => {
    const formats = await decodeFormats(
      async (f) => {
        if (f === 'm4a') throw new Error('HTTP 404');
        return new ArrayBuffer(1);
      },
      async () => undefined,
    );
    expect(formats).toEqual({ ogg: true, m4a: false, mp3: true, wav: true });
  });

  it('Anzeige: Zustand vor und nach Geste, Abspielversuch, vier Formate, Latenz', () => {
    const rows = audioRows(buildAudioReport(input));
    const value = (label: string) => rows.find((r) => r[0] === label);
    expect(value('Zustand vor Geste')).toEqual(['Zustand vor Geste', 'suspended', false]);
    expect(value('Zustand nach Geste')).toEqual(['Zustand nach Geste', 'suspended', false]);
    expect(value('Gesten')?.[1]).toBe('controller: suspended, klick: running');
    expect(value('Abspielen ohne Geste')?.[1]).toBe('blockiert');
    expect(value('Format ogg')).toEqual(['Format ogg', 'nein', false]);
    expect(value('Format wav')).toEqual(['Format wav', 'ja', true]);
    expect(value('Latenz')?.[1]).toBe('Basis 10 ms · Ausgabe 45 ms');
  });

  it('Anzeige ohne Geste und ohne Latenz, und ohne Audio', () => {
    const rows = audioRows(buildAudioReport({ ...input, context: {}, gestures: [] }));
    expect(rows.find((r) => r[0] === 'Zustand nach Geste')).toEqual(['Zustand nach Geste', 'noch keine Geste', undefined]);
    expect(rows.find((r) => r[0] === 'Latenz')?.[1]).toBe('Basis unbekannt · Ausgabe unbekannt');
    expect(audioRows(null)).toEqual([['Audio', 'kein Audio (kein AudioContext)', false]]);
  });
});
