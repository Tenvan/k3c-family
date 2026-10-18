import { describe, expect, it } from 'vitest';
import { mockWorldPreview } from '../api/mockWorld';
import { backgroundToggles, buildingGroups, buildingToggles, hubLevelChoices, previewArgsOf, siteKey, worldOf } from './stageLogic';

describe('Hub-Stufen (B-460/AC-04)', () => {
  it('aus data/hub.json › levels: 1 bis Anzahl der Einträge', () => {
    expect(hubLevelChoices()).toEqual([1, 2, 3, 4, 5]);
  });

  it('ein zusätzlicher Eintrag in einer Fixture erscheint ohne Code-Änderung', () => {
    expect(hubLevelChoices([{}, {}, {}, {}, {}, {}])).toEqual([1, 2, 3, 4, 5, 6]);
    expect(hubLevelChoices([])).toEqual([]);
  });
});

describe('Gebäude-Liste aus dem Zustand (B-460/AC-04)', () => {
  const sites = [{ kind: 'wall', x: 90 }, { kind: 'farm', x: 12.5 }, { kind: 'tower', x: 90 }];

  it('von links nach rechts, bei gleicher Position nach Art, Schlüssel kind@x wie in Go', () => {
    expect(buildingToggles(sites)).toEqual([
      { key: 'farm@12.5', kind: 'farm', x: 12.5 },
      { key: 'tower@90', kind: 'tower', x: 90 },
      { key: 'wall@90', kind: 'wall', x: 90 },
    ]);
    expect(siteKey('farm', 12.5)).toBe('farm@12.5');
  });

  it('die Liste des Mocks (vom Server erzeugter Zustand) hat zu jedem Platz einen Schalter', () => {
    const state = mockWorldPreview(previewArgsOf('k3c', 'forest', 5, new Set())).state!;
    expect(buildingToggles(state.sites)).toHaveLength(state.sites.length);
  });
});

describe('Gebäude-Filter nach Art (B-477/AC-05)', () => {
  const toggles = buildingToggles([
    { kind: 'tower', x: 330 }, { kind: 'wall', x: 322 }, { kind: 'zelt', x: 10 }, { kind: 'wall', x: 345 }, { kind: 'gate', x: 318 }, { kind: 'altar', x: 5 },
  ]);

  it('Gruppen in fester Reihenfolge (Mauern, Türme, Tore), unbekannte Arten danach alphabetisch; Plätze von links nach rechts', () => {
    const groups = buildingGroups(toggles, new Set());
    expect(groups.map((g) => g.kind)).toEqual(['wall', 'tower', 'gate', 'altar', 'zelt']);
    expect(groups.map((g) => g.label)).toEqual(['Mauern', 'Türme', 'Tore', 'altar', 'zelt']);
    expect(groups[0]!.items.map((t) => t.key)).toEqual(['wall@322', 'wall@345']);
  });

  it('Zustand des Gruppenschalters: alle, keiner, gemischt', () => {
    const state = (off: string[]) => Object.fromEntries(buildingGroups(toggles, new Set(off)).map((g) => [g.kind, [g.state, g.on]]));
    expect(state([]).wall).toEqual(['all', 2]);
    expect(state(['wall@322']).wall).toEqual(['some', 1]);
    expect(state(['wall@322', 'wall@345']).wall).toEqual(['none', 0]);
    expect(state(['wall@322']).tower).toEqual(['all', 1]);
  });

  it('jeder Platz steht in genau einer Gruppe; ohne Plätze keine Gruppen', () => {
    expect(buildingGroups(toggles, new Set()).flatMap((g) => g.items)).toHaveLength(toggles.length);
    expect(buildingGroups([], new Set())).toEqual([]);
  });

  it('die Gruppen des Mocks (vom Server erzeugter Zustand) enthalten jeden Platz', () => {
    const state = mockWorldPreview(previewArgsOf('k3c', 'forest', 5, new Set())).state!;
    const groups = buildingGroups(buildingToggles(state.sites), new Set());
    expect(groups.reduce((n, g) => n + g.items.length, 0)).toBe(state.sites.length);
  });
});

describe('previewArgsOf', () => {
  it('abgewählte Gebäude als false, sortiert; Tag mit Fortschritt 0', () => {
    expect(previewArgsOf('k3c', 'forest', 3, new Set(['wall@90', 'farm@12.5']))).toEqual({
      seed: 'k3c', source: { biome: 'forest' }, hubLevel: 3, phase: 'day', progress: 0, buildings: { 'farm@12.5': false, 'wall@90': false },
    });
    expect(previewArgsOf('k3c', 'forest', 5, new Set()).buildings).toEqual({});
  });
});

describe('Welt für den Renderer (B-460/AC-04)', () => {
  it('gleiche Eingaben → gleiche Welt', () => {
    const args = previewArgsOf('k3c', 'forest', 5, new Set());
    expect(JSON.stringify(worldOf(mockWorldPreview(args)))).toBe(JSON.stringify(worldOf(mockWorldPreview(args))));
  });

  it('Welt mit Biom, Level und Zustand; abgewählte Plätze fehlen', () => {
    const all = worldOf(mockWorldPreview(previewArgsOf('k3c', 'forest', 5, new Set())))!;
    expect(all.biome.id).toBe('forest');
    expect(all.level.biomeId).toBe('forest');
    expect(all.widthUnits).toBe(all.level.widthUnits);
    expect(all.hubLevel).toBe(5);
    expect(all.troops).toEqual([]);
    expect(all.enemies).toEqual([]);
    const first = all.sites[0]!;
    const part = worldOf(mockWorldPreview(previewArgsOf('k3c', 'forest', 5, new Set([siteKey(first.kind, first.x)]))))!;
    expect(part.sites).toHaveLength(all.sites.length - 1);
  });

  it('Hinweis, fehlender Zustand und unbekanntes Biom ergeben keine Welt', () => {
    expect(worldOf(null)).toBeNull();
    expect(worldOf(mockWorldPreview(previewArgsOf('fehler', 'forest', 5, new Set())))).toBeNull();
    const view = mockWorldPreview(previewArgsOf('k3c', 'forest', 5, new Set()));
    expect(worldOf({ ...view, level: { ...view.level, biomeId: 'nirgendwo' } })).toBeNull();
  });
});

describe('Hintergrund-Schichten (B-460/AC-05)', () => {
  it('jedes Biom hat drei Schichten von fern nach nah, Scroll-Faktoren steigen und liegen unter 1 (Parallaxe)', () => {
    for (const biome of ['forest', 'cave', 'mine', 'ironhold', 'crystal']) {
      const layers = backgroundToggles(biome);
      expect(layers.map((l) => l.key), biome).toEqual([1, 2, 3].map((n) => `bg:${biome}:${n}`));
      const scrolls = layers.map((l) => l.scroll);
      expect(scrolls, biome).toEqual([...scrolls].sort((a, b) => a - b));
      expect(scrolls.every((s) => s > 0 && s < 1), biome).toBe(true);
    }
    expect(backgroundToggles('forest').map((l) => l.label)).toEqual(['Schicht 1 (fern)', 'Schicht 2 (mittel)', 'Schicht 3 (nah)']);
  });

  it('ein Biom ohne Zuordnung hat keine Schichten', () => {
    expect(backgroundToggles('nirgendwo')).toEqual([]);
  });
});

describe('Optionen mit Tageszeit', () => {
  it('Sim-Phase und Fortschritt gehen in die Optionen', () => {
    expect(previewArgsOf('k3c', 'forest', 5, new Set(), { phase: 'dusk', progress: 0.25 })).toMatchObject({ phase: 'dusk', progress: 0.25 });
  });

  it('Mock: Stufen aus dem Planer-Stand bzw. aus data/hub.json, ein unlesbarer Kandidat wird zum Hinweis', () => {
    expect(mockWorldPreview(previewArgsOf('k3c', 'forest', 0, new Set())).state!.hubLevel).toBe(hubLevelChoices().length);
    expect(mockWorldPreview(previewArgsOf('k3c', 'forest', 0, new Set(), undefined, '{"levels":[{},{},{},{},{},{},{}]}')).state!.hubLevel).toBe(7);
    const broken = mockWorldPreview(previewArgsOf('k3c', 'forest', 3, new Set(), undefined, '{kaputt'));
    expect(broken.state).toBeNull();
    expect(broken.error).toContain('nicht lesbar');
  });

  it('der Planer-Stand geht nur mit, wenn er gesetzt ist (Gebäude-Planer)', () => {
    expect(previewArgsOf('k3c', 'forest', 3, new Set(), undefined, '{"levels":[]}').hubCandidate).toBe('{"levels":[]}');
    expect(previewArgsOf('k3c', 'forest', 3, new Set())).not.toHaveProperty('hubCandidate');
  });
});
