import { describe, expect, it } from 'vitest';
import { GuideSeen } from './guideSeen';

function fakeStorage(initial: Record<string, string> = {}) {
  const data = { ...initial };
  return { data, getItem: (key: string) => data[key] ?? null, setItem: (key: string, value: string) => void (data[key] = value) };
}

const throwing = {
  getItem: (): string | null => {
    throw new Error('gesperrt');
  },
  setItem: (): void => {
    throw new Error('voll');
  },
};

describe('guideSeen', () => {
  it('merkt gesehene Hinweise je Gerät und liest sie wieder', () => {
    const s = fakeStorage();
    new GuideSeen(s).mark('coin');
    expect([...new GuideSeen(s).ids]).toEqual(['coin']);
  });

  it('Rücksetzen zeigt alle Hinweise wieder', () => {
    const s = fakeStorage({ 'k3c-guide-seen': '["coin","pay"]' });
    const seen = new GuideSeen(s);
    seen.reset();
    expect(seen.ids.size).toBe(0);
    expect(new GuideSeen(s).ids.size).toBe(0);
  });

  it('kaputter Speicher → nichts gesehen', () => {
    for (const raw of ['{kaputt', '"text"', '{"a":1}', '[1,"dusk"]']) expect([...new GuideSeen(fakeStorage({ 'k3c-guide-seen': raw })).ids]).toEqual(raw.includes('dusk') ? ['dusk'] : []);
  });

  it('gesperrter Speicher: kein Absturz, Merkung bis zum Neuladen', () => {
    const seen = new GuideSeen(throwing);
    seen.mark('dusk');
    expect(seen.ids.has('dusk')).toBe(true);
    expect(new GuideSeen(throwing).ids.size).toBe(0);
  });
});
