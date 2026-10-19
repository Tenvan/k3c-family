import { describe, expect, it, vi } from 'vitest';
import { makeSprite, spriteTop } from '@game/scenes/render/sprites';
import spritesJson from '@data/sprites.json';
import {
  ANIMS, DEFAULT_CONTROL, MOUNTS, SHEETS, TEMPO_STEPS, animInfo, effectiveFps, figureTop, inAtlas, mountPoseOf, resolveAnim, resume,
  setTempo, sheetFiles, stepFrame, toggleLoop, unitTicks, unitsToPx,
} from './figure';

// Phaser selbst braucht Canvas; `sprites.ts` benutzt es nur für Farben und Filter, `makeSprite` rechnet reine Daten.
vi.mock('phaser', () => ({ default: { Display: { Color: { HexStringToColor: () => ({ color: 0 }) } }, Textures: { FilterMode: { NEAREST: 0 } } } }));

// B-463 (WZ10.1): reine Logik der Abspiel-Bausteine.

const DATA = spritesJson as unknown as { players: { sheet: string }[]; enemies: Record<string, { sheet: string; scale?: number; y?: number }> };

/** Attrappe einer Szene, die `makeSprite` genug liefert, um `spriteTop` des Spiels zu fragen */
function fakeScene() {
  const data = new Map<string, unknown>();
  const sprite: Record<string, unknown> = {};
  for (const m of ['setOrigin', 'setScale', 'setTint', 'setAlpha', 'play']) sprite[m] = () => sprite;
  sprite.setData = (k: string, v: unknown) => { data.set(k, v); return sprite; };
  sprite.getData = (k: string) => data.get(k);
  return { add: { sprite: () => sprite } } as never;
}

describe('Maßstab (B-463/AC-11)', () => {
  it('die Höhe der Figur ist gleich spriteTop des Spiels, für Spieler und alle Gegner mit scale und y', () => {
    const specs = [...DATA.players, ...Object.values(DATA.enemies), { sheet: DATA.players[0]!.sheet, scale: 1.5, y: 40 }];
    for (const spec of specs) {
      expect(figureTop(spec), spec.sheet).toBe(spriteTop(makeSprite(fakeScene(), spec)));
    }
  });

  it('Units in Pixel und die Skala an der Bodenlinie', () => {
    expect(unitsToPx(2)).toBe(64);
    expect(unitTicks(-2, 2).map((t) => t.unit)).toEqual([-2, -1, 0, 1, 2]);
    expect(unitTicks(0, 10, 5).map((t) => t.px)).toEqual([0, 160, 320]);
  });
});

describe('Animation auflösen (B-463/AC-02)', () => {
  it('liefert Bildzahl und Bildrate aus data/sprites.json', () => {
    for (const [name, d] of Object.entries(SHEETS)) {
      for (const a of ANIMS) expect(animInfo(name, a), `${name}/${a}`).toEqual(d.anims[a] ?? null);
    }
  });

  it('fehlt eine Animation, bleibt es bei idle mit der Lücke (Sheet ghost hat kein run)', () => {
    const ghost = Object.entries(SHEETS).find(([, d]) => d.anims.idle && !d.anims.run)?.[0];
    expect(ghost).toBeDefined();
    expect(resolveAnim(ghost!, 'run')).toEqual({ anim: 'idle', gap: { kind: 'anim', reason: 'Lücke: run fehlt' } });
    expect(resolveAnim(ghost!, 'idle')).toEqual({ anim: 'idle', gap: null });
  });

  it('ein unbekanntes Sheet ist eine Lücke, kein Absturz', () => {
    expect(resolveAnim('gibtsnicht', 'run').gap?.kind).toBe('sheet');
    expect(figureTop({ sheet: 'gibtsnicht' })).toBe(0);
  });
});

describe('Steuerung (B-463/AC-01)', () => {
  it('Tempo nur in den Stufen 0,5×, 1× und 2×', () => {
    expect([...TEMPO_STEPS]).toEqual([0.5, 1, 2]);
    expect(setTempo(DEFAULT_CONTROL, 2).tempo).toBe(2);
    expect(setTempo(DEFAULT_CONTROL, 3)).toBe(DEFAULT_CONTROL);
    expect(effectiveFps({ frames: 6, fps: 14 }, { ...DEFAULT_CONTROL, tempo: 0.5 })).toBe(7);
  });

  it('Schleife schaltet um, Einzelbild geht im Kreis und hält die Wiedergabe an, weiter nimmt sie wieder auf', () => {
    expect(toggleLoop(DEFAULT_CONTROL).loop).toBe(false);
    const c1 = stepFrame(DEFAULT_CONTROL, 6, 1, 5);
    expect(c1.frame).toBe(0);
    expect(stepFrame(c1, 6, -1).frame).toBe(5);
    expect(stepFrame({ ...DEFAULT_CONTROL, frame: 2 }, 6, 1).frame).toBe(3);
    expect(stepFrame(DEFAULT_CONTROL, 0, 1)).toBe(DEFAULT_CONTROL);
    expect(resume(c1).frame).toBeNull();
  });
});

describe('Bilder laden (B-463)', () => {
  it('Sheets mit Rolle liegen im Atlas, Reittiere als Einzel-PNG nach dem Muster von preloadSprites', () => {
    expect(inAtlas(DATA.players[0]!.sheet)).toBe(true);
    const mount = Object.values(MOUNTS)[0]!;
    const files = sheetFiles(mount.sheet);
    expect(files.length).toBeGreaterThan(0);
    for (const f of files) {
      expect(f.url).toBe(`sprites/${mount.sheet}/${f.anim}.png`);
      expect(f.key).toBe(`${mount.sheet}-${f.anim}`);
      expect(f.frameWidth).toBe(SHEETS[mount.sheet]!.frameWidth);
    }
    expect(sheetFiles('gibtsnicht')).toEqual([]);
  });
});

describe('Reittier (B-463)', () => {
  it('alle 13 Reittiere ergeben eine Pose für idle und run, ein unbekanntes eine Lücke', () => {
    const rider = { sheet: DATA.players[0]!.sheet };
    expect(Object.keys(MOUNTS)).toHaveLength(13);
    for (const key of Object.keys(MOUNTS)) {
      expect(mountPoseOf(key, rider, 5, 0).pose?.anim, key).toBe('idle');
      expect(mountPoseOf(key, rider, 5, 5).pose?.anim, key).toBe('run');
    }
    expect(mountPoseOf('gibtsnicht', rider, 5, 0)).toEqual({ pose: null, gap: { kind: 'mount', reason: 'Lücke: Reittier gibtsnicht unbekannt' } });
  });

  it('die Bausteine tragen keine Monarch-Annahme: ein Gegner-Sheet als Parameter geht genauso', () => {
    const enemy = Object.values(DATA.enemies)[0]!;
    expect(resolveAnim(enemy.sheet, 'idle').gap).toBeNull();
    expect(figureTop(enemy)).toBeLessThan(0);
  });
});
