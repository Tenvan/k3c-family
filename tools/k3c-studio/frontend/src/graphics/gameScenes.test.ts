import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { SKILL_LOOKS, skillFrame } from '@game/scenes/effects/skillLooks';
import { drawBuildSign } from '@game/scenes/render/buildSignView';
import { drawBar, drawWarn } from '@game/scenes/kampfView';
import { FONTS } from '@game/scenes/render/fontRules';
import { MONARCH } from '@game/model/data';
import { canvasGraphics } from './canvasGraphics';
import { GAME_HEIGHT } from '@game/core/constants';
import { BIOMES } from '@game/model/biome';
import { BAR_HEIGHT, WORLD_TOP } from '@game/scenes/layout';
import { ABLAUF_SEKUNDEN, ablauf, BOSSES, kampfBild, MIN_SCHRIFT_PX, mockWorld, MOCK_START, phaseAt, TOP_STRIP, ZELLEN, zellenZoom } from './combatLogic';
import { biomeTags, sceneFilter } from './scenesLogic';
import { bossKarten, gegnerKarten, GAME_SCENES, reittierKarten, SIGN_SAMPLES, skillRows } from './gameScenesLogic';

// B-598 / B-602: Aufbereitung und Mock-Zustände der Spiel-Szenen; der Import-Test belegt, dass das Studio mit den Spiel-Funktionen zeichnet.

const read = (f: string) => readFileSync(new URL(f, import.meta.url), 'utf8');

describe('Skill-Effekte (B-598 AC-01)', () => {
  it('zeigt alle Skills, Rückfall nur ohne eigenen Look', () => {
    const rows = skillRows();
    expect(rows.map((r) => r.id).sort()).toEqual(Object.keys(MONARCH.skills).sort());
    for (const r of rows) {
      expect(r.fallback).toBe(!(r.id in SKILL_LOOKS));
      expect(r.look).toBeDefined();
    }
  });

  it('skillFrame: Ring wächst und blendet aus, Partikel wandern zum Ziel', () => {
    const look = SKILL_LOOKS.backstab!;
    const [start, ...endParts] = [skillFrame(look, 0), skillFrame(look, 1)];
    expect(start![0]!.alpha).toBeCloseTo(0.8);
    expect(endParts[0]![0]!.alpha).toBe(0);
    expect(endParts[0]![0]!.scale).toBeCloseTo(1.8);
    expect(endParts[0]).toHaveLength(1 + look.particles);
  });
});

describe('Schilder, Gegner, Reittiere, Bosse (B-598 AC-02)', () => {
  it('Schilder: alle drei Varianten', () => {
    expect(SIGN_SAMPLES.map((s) => s.variant).sort()).toEqual(['defense', 'settlement', 'stairs']);
  });
  it('Gegner je Biom kommen aus den Biom-Daten', () => {
    expect(gegnerKarten('forest').map((c) => c.title)).toEqual(['greed', 'goblin', 'goblinArcher', 'wolf']);
    expect(gegnerKarten('crystal').length).toBeGreaterThan(0);
  });
  it('Reittiere tragen den Monarchen, Bosse haben je eine Karte mit Phasen-Notiz', () => {
    expect(reittierKarten().every((c) => c.mount && c.spec.sheet)).toBe(true);
    const bosse = bossKarten();
    expect(bosse).toHaveLength(BOSSES.length);
    expect(bosse.find((c) => c.title.startsWith('Kristallherz'))?.note).toContain('Phase 3 ab < 25 % Leben');
  });
  it('kennt die sechs Szenen', () => {
    expect(GAME_SCENES).toEqual(['skills', 'schilder', 'gegner', 'reittiere', 'bosse', 'kampf']);
  });
});

describe('Kampf-Anzeigen: Mock-Zustände (B-602 AC-01)', () => {
  it('Boss Phase 2, 40 % Leben, Warnkreis 1,5 s: Leiste, Phasen-Text, Kreis wie im Spiel', () => {
    const b = kampfBild(MOCK_START);
    expect(b.boss).toMatchObject({ share: 0.4, name: 'Kristallherz-Wächter', phase: 'Phase 2 von 3' });
    expect(b.circles).toHaveLength(1);
    expect(b.circles[0]!.r).toBe(5 * 32);
    expect(b.circles[0]!.urgency).toBeCloseTo(0.5);
    expect(b.banner).toBe('Vollmond  ·  1:35');
  });
  it('Phasen folgen den Schwellen aus bosses.json, Miniboss ohne Phasen-Text', () => {
    const end = BOSSES.find((b) => b.kind === 'end')!;
    expect([100, 59, 24].map((hp) => phaseAt(end, hp))).toEqual([1, 2, 3]);
    const mini = BOSSES.find((b) => b.kind === 'mini')!;
    expect(phaseAt(mini, 10)).toBeUndefined();
    expect(kampfBild({ ...MOCK_START, boss: mini.id }).boss?.phase).toBe('');
  });
  it('Events: Blutmond, Händler-Überfall ohne Restzeit, ohne Event kein Banner, kein Warnkreis ohne Schlag', () => {
    expect(kampfBild({ ...MOCK_START, events: ['bloodMoon', 'merchantRaid'] }).banner).toBe('Blutmond  ·  1:35\nHändler-Überfall');
    expect(kampfBild({ ...MOCK_START, events: [], warn: null })).toMatchObject({ banner: null, circles: [] });
    expect(mockWorld({ ...MOCK_START, boss: 'gibtEsNicht' })).toBeNull();
  });
  it('Ablauf: Leiste tritt auf, Phasen wechseln, Warnkreis läuft ab, Banner erscheint und verschwindet', () => {
    const phasen = new Set<string>();
    for (let t = 0; t <= ABLAUF_SEKUNDEN; t += 0.5) phasen.add(kampfBild(ablauf(MOCK_START, t).mock).boss!.phase);
    expect(phasen.size).toBe(3);
    expect(ablauf(MOCK_START, 0).leiste).toBe(0);
    expect(ablauf(MOCK_START, 2).leiste).toBe(1);
    expect([0, 5, 7, 12].map((t) => ablauf(MOCK_START, t).banner)).toEqual([0, 1, 1, 0]);
    expect(ablauf(MOCK_START, 1).mock.warn!.seconds).toBeCloseTo(3);
    expect(ablauf(MOCK_START, 3.9).mock.warn!.seconds).toBeCloseTo(0.1);
  });
  it('Schrift und freie Fläche laut Regeln des Spiels', () => {
    expect(FONTS.bossBar.px).toBeGreaterThanOrEqual(MIN_SCHRIFT_PX);
    expect(FONTS.banner.px).toBeGreaterThanOrEqual(MIN_SCHRIFT_PX);
    expect(TOP_STRIP).toBe(70);
  });
  it('Zellengröße: Zoom der Welt aus Layout und viewZoom des Spiels, kleinere Zelle zoomt weniger (WZ29.4)', () => {
    const zoom = (h: number) => h / (GAME_HEIGHT - WORLD_TOP);
    expect(zellenZoom('voll')).toBeCloseTo(zoom(GAME_HEIGHT - BAR_HEIGHT));
    expect(zellenZoom('partner')).toBeCloseTo(zoom(((GAME_HEIGHT - BAR_HEIGHT) * 2) / 3));
    expect(zellenZoom('streifen')).toBeCloseTo(zoom((GAME_HEIGHT - BAR_HEIGHT) / 2));
    expect(ZELLEN.map(zellenZoom)).toEqual([...ZELLEN.map(zellenZoom)].sort((a, b) => b - a));
  });
});

describe('Filter Biom/Tiefe (WZ29.4, B-599)', () => {
  it('Hintergrund gehört zu seinem Biom, Gegner zu jedem Biom, das ihn spawnen lässt; sonst kein Biom', () => {
    const forest = BIOMES.find((b) => b.id === 'forest')!;
    expect(biomeTags('bg:forest:2')).toEqual([`Tiefe ${forest.depth} · ${forest.name}`]);
    const goblin = BIOMES.filter((b) => [...b.enemies.portal, ...b.enemies.night].includes('goblin'));
    expect(goblin.length).toBeGreaterThan(0);
    expect(biomeTags('goblin')).toEqual(goblin.map((b) => `Tiefe ${b.depth} · ${b.name}`));
    expect(biomeTags('castle')).toBeUndefined();
    const dim = sceneFilter([], []).dimensions.find((d) => d.key === 'biom')!;
    expect(dim.label).toBe('Biom/Tiefe');
    expect(dim.order).toEqual(BIOMES.map((b) => `Tiefe ${b.depth} · ${b.name}`));
  });
});

describe('Spiel-Funktionen statt zweiter Darstellung (B-598 / B-602 AC-02)', () => {
  const quellen = ['SkillScene.tsx', 'SignScene.tsx', 'combatDraw.ts', 'combatLogic.ts', 'gameScenesLogic.ts', 'FigureScenes.tsx'].map((f) => [f, read(f)] as const);
  const alle = quellen.map(([, s]) => s).join('\n');

  it('importiert die Zeichen- und Anzeige-Funktionen aus dem Spielcode', () => {
    for (const name of ['@game/scenes/kampfView', '@game/scenes/kampfAnzeige', '@game/scenes/layout', '@game/scenes/render/buildSignView', '@game/scenes/effects/skillLooks', '@game/tools/spriteReference']) expect(alle).toContain(`from '${name}'`);
    expect(typeof drawBar).toBe('function');
    expect(typeof drawWarn).toBe('function');
    expect(typeof drawBuildSign).toBe('function');
  });

  it('definiert keine eigene Fassung dieser Funktionen', () => {
    for (const name of ['bossBar', 'warnCircles', 'eventBanner', 'phaseText', 'drawBuildSign', 'drawBar', 'drawWarn', 'skillLook', 'particleOffset', 'signVariant', 'computeLayout', 'viewZoom'])
      expect(alle, name).not.toMatch(new RegExp(`(function|const)\\s+${name}\\b`));
  });

  it('die Zeichenfläche reicht die Aufrufe an den 2D-Kontext weiter', () => {
    const calls: string[] = [];
    const ctx = new Proxy({}, { get: (_t, k) => (typeof k === 'string' && /^(fillRect|strokeRect|beginPath|arc|fill|stroke)$/.test(k) ? () => calls.push(k) : undefined), set: () => true }) as unknown as CanvasRenderingContext2D;
    drawBuildSign(canvasGraphics(ctx), 'wall');
    drawWarn(canvasGraphics(ctx), [{ x: 10, r: 5, urgency: 1 }], 0);
    expect(calls).toEqual(expect.arrayContaining(['fillRect', 'strokeRect', 'arc', 'fill', 'stroke']));
  });
});
