import { describe, expect, it, vi } from 'vitest';
vi.mock('phaser', () => ({ default: { Textures: { FilterMode: { NEAREST: 0 } }, Display: {} } })); // nur Konstanten, keine Laufzeit

import { MOUNTS, SHEETS, createSheetAnims, preloadMount, preloadSheet } from './sprites';

// B-476/AC-01: beliebige Reittiere auf Anforderung laden und ihre Animationen anlegen (Mini-Szene ohne Phaser-Laufzeit).
function fakeScene(existing: string[] = []) {
  const textures = new Set(existing);
  const anims = new Set<string>();
  const loaded: string[] = [];
  return {
    loaded, anims,
    scene: {
      textures: { exists: (k: string) => textures.has(k), get: () => ({ setFilter: () => {} }) },
      anims: { exists: (k: string) => anims.has(k), create: (c: { key: string }) => { anims.add(c.key); }, generateFrameNumbers: () => [] },
      load: { spritesheet: (k: string) => { loaded.push(k); textures.add(k); } },
    } as never,
  };
}

const [mountKey, mount] = Object.entries(MOUNTS)[0]!;

describe('Reittiere auf Anforderung (B-476)', () => {
  it('preloadMount reiht die Animationen des Sheets ein, ein zweiter Aufruf nichts mehr', () => {
    const { scene, loaded } = fakeScene();
    const n = preloadMount(scene, mountKey);
    expect(n).toBe(Object.keys(SHEETS[mount.sheet]!.anims).length);
    expect(loaded.every((k) => k.startsWith(`${mount.sheet}-`))).toBe(true);
    expect(preloadMount(scene, mountKey)).toBe(0);
  });

  it('ein unbekanntes Reittier oder Sheet lädt nichts', () => {
    const { scene, loaded } = fakeScene();
    expect(preloadMount(scene, 'gibtsnicht')).toBe(0);
    expect(preloadSheet(scene, 'gibtsnicht')).toBe(0);
    expect(loaded).toEqual([]);
  });

  it('createSheetAnims legt je geladener Animation genau einmal an', () => {
    const { scene, anims } = fakeScene();
    preloadMount(scene, mountKey);
    createSheetAnims(scene, mount.sheet);
    createSheetAnims(scene, mount.sheet);
    expect([...anims].sort()).toEqual(Object.keys(SHEETS[mount.sheet]!.anims).map((a) => `${mount.sheet}-${a}`).sort());
  });
});
