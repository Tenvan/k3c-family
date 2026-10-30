import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { UNIT_PX } from '@game/core/constants';
import { EFFECT_CONFIG } from '@game/scenes/effects/effects';
import { skillFrame, skillLook } from '@game/scenes/effects/skillLooks';
import { DEFAULT_CONTROL, figureIndexAt, pause } from './figure';
import {
  PROJECTILE, advance, appearAlpha, burstPieces, configLook, cycleTime, durationOf, frameProgress, projectileMs, projectileOffset, warnIn, type TimedEffect,
} from './timeline';

// B-671/AC-07, B-672/AC-08: Zeitachse der abgespielten Effekte; Schleife, Tempo und Einzelbild wirken darauf.

const root = new URL('../../../../../', import.meta.url);
const source = (path: string) => readFileSync(new URL(path, root), 'utf8');

const fireball = skillLook('fireball')!;
const burst: TimedEffect = { kind: 'burst', look: fireball, cycleMs: fireball.durationMs + 700 };

describe('Zeit der Wiedergabe', () => {
  it('Tempo skaliert die Laufzeit, Einzelbild hält sie an', () => {
    expect(advance(100, 50, DEFAULT_CONTROL)).toBe(150);
    expect(advance(100, 50, { ...DEFAULT_CONTROL, tempo: 0.5 })).toBe(125);
    expect(advance(100, 50, { ...DEFAULT_CONTROL, tempo: 2 })).toBe(200);
    expect(advance(100, 50, { ...DEFAULT_CONTROL, frame: 2 })).toBe(100);
  });

  it('Schleife wiederholt den Zyklus, ohne Schleife bleibt nach dem ersten nichts zu sehen', () => {
    expect(cycleTime(burst, DEFAULT_CONTROL, burst.cycleMs + 100, 6)).toBe(100);
    const once = { ...DEFAULT_CONTROL, loop: false };
    expect(cycleTime(burst, once, 100, 6)).toBe(100);
    expect(burstPieces(fireball, cycleTime(burst, once, burst.cycleMs + 100, 6))).toEqual([]);
  });

  it('Einzelbild steht auf dem Fortschritt des Bilds, nie auf 0, und zeigt skillFrame des Spiels', () => {
    expect(frameProgress(0, 6)).toBeGreaterThan(0);
    expect(frameProgress(5, 6)).toBeLessThan(1);
    const c = { ...DEFAULT_CONTROL, frame: 2 };
    const t = cycleTime(burst, c, 12345, 6);
    expect(t).toBeCloseTo(frameProgress(2, 6) * fireball.durationMs);
    expect(burstPieces(fireball, t)).toEqual(skillFrame(fireball, frameProgress(2, 6)));
  });

  it('pause hält beim aktuellen Bild an, ein angehaltenes Bild bleibt', () => {
    expect(pause(DEFAULT_CONTROL, 3).frame).toBe(3);
    expect(pause({ ...DEFAULT_CONTROL, frame: 1 }, 3).frame).toBe(1);
  });
});

describe('Effekte', () => {
  it('Burst rechnet mit skillFrame des Spiels, mit Verzögerung erst danach', () => {
    expect(burstPieces(fireball, 100)).toEqual(skillFrame(fireball, 100 / fireball.durationMs));
    expect(burstPieces(fireball, 50, 100)).toEqual([]);
    expect(burstPieces(fireball, fireball.durationMs)).toEqual([]);
    expect(configLook('hit')).toMatchObject({ ...EFFECT_CONFIG.hit, motion: 'burst' });
  });

  it('Geschoss fliegt mit dem Tempo des Spiels bis zur Reichweite und verschwindet dort', () => {
    const p = { kind: 'projectile', range: 10, dir: -1, speed: PROJECTILE.speed, color: PROJECTILE.enemyColor, cycleMs: 1500 } as const;
    expect(projectileMs(10, 20)).toBe(500);
    expect(durationOf(p)).toBe(500);
    expect(projectileOffset(p, 250)).toBe(-5 * UNIT_PX);
    expect(projectileOffset(p, 500)).toBeNull();
  });

  it('Warnkreis zählt bis zur Auslösung herunter; Tempo 2× halbiert den Abstand der Auslösungen in Bildschirmzeit', () => {
    const warn: TimedEffect = { kind: 'warn', radius: 3, cycleMs: 4000 };
    expect(warnIn(4000, 1000)).toBe(3);
    let elapsed = 0;
    for (let i = 0; i < 100; i++) elapsed = advance(elapsed, 20, { ...DEFAULT_CONTROL, tempo: 2 });
    expect(cycleTime(warn, DEFAULT_CONTROL, elapsed, 1)).toBe(0); // 2 s Bildschirmzeit = ein ganzer Zyklus von 4 s
  });

  it('beschworene Figuren blenden nach der Auslösung ein und vor der nächsten aus', () => {
    expect(appearAlpha(10000, 0)).toBe(0);
    expect(appearAlpha(10000, 5000)).toBe(1);
    expect(appearAlpha(10000, 9999)).toBeLessThan(0.01);
  });
});

describe('Werte stammen aus dem Spiel', () => {
  it('Geschoss wie worldRenderer, Tempo wie engine/sim', () => {
    expect(source('src/scenes/render/worldRenderer.ts')).toContain(`G - ${PROJECTILE.y}, [scene.add.rectangle(0, 0, ${PROJECTILE.width}, ${PROJECTILE.height}, p.team === 'player' ? 0xffffff : 0x${PROJECTILE.enemyColor.toString(16)})`);
    expect(source('engine/sim/enemies.go')).toContain(`Speed: ${PROJECTILE.speed},`);
    expect(source('engine/sim/boss_abilities.go')).toContain(`const shardSpeed = ${PROJECTILE.speed}`);
  });

  it('die Zeitachse importiert skillFrame aus dem Spiel, statt es nachzubauen (B-671)', () => {
    const own = readFileSync(new URL('./timeline.ts', import.meta.url), 'utf8');
    expect(own).toMatch(/import \{[^}]*skillFrame[^}]*\} from '@game\/scenes\/effects\/skillLooks'/);
    expect(own).not.toMatch(/function skillFrame/);
  });
});

describe('Treffer-Test (B-672/AC-03)', () => {
  const box = (left: number, right: number) => ({ left, right, top: -100, bottom: 0 });
  it('findet die Figur unter dem Punkt, bei Überlappung die nächste Mitte; fehlende Figuren zählen im Index mit', () => {
    const boxes = [box(0, 40), null, box(30, 90)];
    expect(figureIndexAt(boxes, 10, -50)).toBe(0);
    expect(figureIndexAt(boxes, 38, -50)).toBe(0);
    expect(figureIndexAt(boxes, 70, -50)).toBe(2);
    expect(figureIndexAt(boxes, 200, -50)).toBeNull();
    expect(figureIndexAt(boxes, 95, -50, 8)).toBe(2);
  });
});
