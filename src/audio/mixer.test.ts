import { describe, expect, it } from 'vitest';
import { loadSettings } from '../core/settings';
import { BUSES, Mixer, volumeToGain, type AudioContextLike, type GainNodeLike } from './mixer';

function fakeCtx() {
  const nodes: (GainNodeLike & { to?: unknown })[] = [];
  const destination = { name: 'destination' };
  const ctx: AudioContextLike = {
    destination,
    createGain: () => {
      const node: GainNodeLike & { to?: unknown } = { gain: { value: 1 }, connect: (t) => void (node.to = t) };
      nodes.push(node);
      return node;
    },
  };
  return { ctx, nodes, destination };
}

function fakeStorage(initial: Record<string, string> = {}) {
  const data = { ...initial };
  return { data, getItem: (k: string) => data[k] ?? null, setItem: (k: string, v: string) => void (data[k] = v) };
}

const throwing = {
  getItem: (): string | null => {
    throw new Error('gesperrt');
  },
  setItem: (): void => {
    throw new Error('voll');
  },
};

describe('Mixer', () => {
  it('AC-01: drei Busse hängen am Master, Master am Ausgang', () => {
    const { ctx, destination } = fakeCtx();
    const mixer = new Mixer(ctx, fakeStorage());
    expect((mixer.master as { to?: unknown }).to).toBe(destination);
    for (const bus of BUSES) expect((mixer.node(bus) as { to?: unknown }).to).toBe(mixer.master);
    expect(new Set(BUSES.map((b) => mixer.node(b))).size).toBe(3);
  });

  it('AC-01: Effekte 0 lässt Musik und Ambient unverändert', () => {
    const mixer = new Mixer(fakeCtx().ctx, fakeStorage());
    mixer.setVolume('music', 70);
    mixer.setVolume('ambient', 30);
    mixer.setVolume('sfx', 0);
    expect([mixer.node('sfx').gain.value, mixer.node('music').gain.value, mixer.node('ambient').gain.value]).toEqual([0, 0.7, 0.3]);
    expect([mixer.getVolume('sfx'), mixer.getVolume('music'), mixer.getVolume('ambient')]).toEqual([0, 70, 30]);
  });

  it('begrenzt auf 0–100 %, ungültige Werte behalten den alten', () => {
    const mixer = new Mixer(fakeCtx().ctx, fakeStorage());
    mixer.setVolume('music', 250);
    expect(mixer.getVolume('music')).toBe(100);
    mixer.setVolume('music', -3);
    expect(mixer.getVolume('music')).toBe(0);
    mixer.setVolume('music', 40);
    mixer.setVolume('music', Number.NaN);
    expect(mixer.getVolume('music')).toBe(40);
    expect(volumeToGain(50)).toBe(0.5);
  });

  it('AC-02: Speichern, neuer Mixer liest die Werte beim Start', () => {
    const storage = fakeStorage();
    new Mixer(fakeCtx().ctx, storage).setVolume('sfx', 20);
    const again = new Mixer(fakeCtx().ctx, storage);
    expect(again.getVolume('sfx')).toBe(20);
    expect(again.node('sfx').gain.value).toBe(0.2);
    expect(loadSettings(storage).sfxVolume).toBe(20);
  });

  it('AC-02: gesperrter oder kaputter Speicher → Standard, kein Fehler', () => {
    const mixer = new Mixer(fakeCtx().ctx, throwing);
    expect(BUSES.map((b) => mixer.getVolume(b))).toEqual([100, 100, 100]);
    expect(() => mixer.setVolume('music', 10)).not.toThrow();
    expect(mixer.node('music').gain.value).toBe(0.1);
    const broken = new Mixer(fakeCtx().ctx, fakeStorage({ 'k3c-settings': '{kaputt' }));
    expect(broken.getVolume('ambient')).toBe(100);
  });
});
