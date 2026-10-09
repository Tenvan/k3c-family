import { afterEach, describe, expect, it, vi } from 'vitest';
import { unfold } from './fold';

// Ohne DOM: localStorage und window als kleine Attrappen (fold.tsx liest und schreibt nur darüber).
const store = new Map<string, string>();
vi.stubGlobal('localStorage', {
  getItem: (k: string) => store.get(k) ?? null,
  setItem: (k: string, v: string) => void store.set(k, v),
});
vi.stubGlobal('window', { dispatchEvent: () => true });

afterEach(() => store.clear());

describe('unfold (Workbench-Spec § 1 › Auswahl von außen)', () => {
  it('klappt alle genannten Karten auf, nicht nur die erste', () => {
    store.set('k3c-dev.planning.fold', JSON.stringify(['BET', 'SP11', 'F2']));
    unfold(['BET', 'SP11']);
    expect(JSON.parse(store.get('k3c-dev.planning.fold')!)).toEqual(['F2']);
  });
});
