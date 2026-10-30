import { describe, expect, it } from 'vitest';
import { mockWorldPreview } from '../api/mockWorld';
import { applyLook, backgroundToggles, buildingGroups, buildingToggles, commonOf, focusXs, hpOf, hubLevelChoices, kindColor, pickKeys, presetLooks, previewArgsOf, siteAt, siteKey, stripLanes, worldOf } from './stageLogic';

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
  const sites = [{ kind: 'wall', x: 90 }, { kind: 'nursery', x: 12.5 }, { kind: 'tower', x: 90 }];

  it('von links nach rechts, bei gleicher Position nach Art, Schlüssel kind@x wie in Go', () => {
    expect(buildingToggles(sites)).toEqual([
      { key: 'nursery@12.5', kind: 'nursery', x: 12.5 },
      { key: 'tower@90', kind: 'tower', x: 90 },
      { key: 'wall@90', kind: 'wall', x: 90 },
    ]);
    expect(siteKey('nursery', 12.5)).toBe('nursery@12.5');
  });

  it('die Liste des Mocks (vom Server erzeugter Zustand) hat zu jedem Platz einen Schalter', () => {
    const state = mockWorldPreview(previewArgsOf('k3c', 'forest', 5, new Set())).state!;
    expect(buildingToggles(state.sites)).toHaveLength(state.sites.length);
  });
});

describe('Gebäude-Filter nach Art (B-477/AC-05)', () => {
  const toggles = buildingToggles([
    { kind: 'tower', x: 330 }, { kind: 'wall', x: 322 }, { kind: 'zelt', x: 10 }, { kind: 'wall', x: 345 }, { kind: 'altar', x: 5 },
  ]);

  it('Gruppen in fester Reihenfolge (Mauern, Türme), unbekannte Arten danach alphabetisch; Plätze von links nach rechts', () => {
    const groups = buildingGroups(toggles, new Set());
    expect(groups.map((g) => g.kind)).toEqual(['wall', 'tower', 'altar', 'zelt']);
    expect(groups.map((g) => g.label)).toEqual(['Mauern', 'Türme', 'altar', 'zelt']);
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

describe('Lageleiste der Gebäude (B-670)', () => {
  it('jede bekannte Art hat eine eigene Farbe, unbekannte Arten eine feste aus dem Namen', () => {
    const kinds = ['wall', 'tower', 'nursery', 'barracks', 'workshop', 'tavern', 'healer', 'smithy', 'armory', 'stairsDown', 'stairsUp'];
    expect(new Set(kinds.map(kindColor)).size).toBe(kinds.length);
    expect(kindColor('harbor')).toBe(kindColor('harbor'));
  });

  it('nahe Plätze wandern in die nächste Spur, ferne bleiben in Spur 0', () => {
    expect(stripLanes([10, 12, 13, 30, 31], 4)).toEqual({ lanes: [0, 1, 2, 0, 1], count: 3 });
    expect(stripLanes([], 4)).toEqual({ lanes: [], count: 1 });
  });
});

describe('previewArgsOf', () => {
  it('abgewählte Gebäude als false, sortiert; Tag mit Fortschritt 0', () => {
    expect(previewArgsOf('k3c', 'forest', 3, new Set(['wall@90', 'nursery@12.5']))).toEqual({
      seed: 'k3c', source: { biome: 'forest' }, hubLevel: 3, phase: 'day', progress: 0, buildings: { 'nursery@12.5': false, 'wall@90': false },
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
    expect(mockWorldPreview(previewArgsOf('k3c', 'forest', 0, new Set(), undefined, undefined, '{"levels":[{},{},{},{},{},{},{}]}')).state!.hubLevel).toBe(7);
    const broken = mockWorldPreview(previewArgsOf('k3c', 'forest', 3, new Set(), undefined, undefined, '{kaputt'));
    expect(broken.state).toBeNull();
    expect(broken.error).toContain('nicht lesbar');
  });

  it('der Planer-Stand geht nur mit, wenn er gesetzt ist (Gebäude-Planer)', () => {
    expect(previewArgsOf('k3c', 'forest', 3, new Set(), undefined, undefined, '{"levels":[]}').hubCandidate).toBe('{"levels":[]}');
    expect(previewArgsOf('k3c', 'forest', 3, new Set())).not.toHaveProperty('hubCandidate');
  });
});

describe('focusXs (Gebäude-Planer: gewählte Zeile → Plätze der Welt)', () => {
  const sites = [{ kind: 'nursery', x: 48 }, { kind: 'nursery', x: 152 }, { kind: 'armory', x: 140 }, { kind: 'armory', x: 300 }];
  const f = (kind: string, offset: number | null, bothSides = false) => ({ id: 'x', kind, offset, bothSides });

  it('nimmt Art und Abstand zur Hub-Mitte, je Seite mit ±', () => {
    expect(focusXs(f('armory', 40), 100, sites)).toEqual([140]);
    expect(focusXs(f('nursery', 52, true), 100, sites)).toEqual([48, 152]);
  });
  it('Streuung bis 0,5 Units, sonst alle Plätze der Art', () => {
    expect(focusXs(f('armory', 40.4), 100, sites)).toEqual([140]);
    expect(focusXs(f('armory', 10), 100, sites)).toEqual([140, 300]);
    expect(focusXs(f('armory', null), 100, sites)).toEqual([140, 300]);
  });
  it('ohne Zeile oder für den Händler: nichts', () => {
    expect(focusXs(undefined, 100, sites)).toEqual([]);
    expect(focusXs(f('merchantBuy', 5), 100, sites)).toEqual([]);
  });
});

describe('Voreinstellungen der Gebäude-Zustände (B-666/AC-02)', () => {
  const toggles = buildingToggles([{ kind: 'wall', x: 191 }, { kind: 'wall', x: 192 }, { kind: 'wall', x: 193 }, { kind: 'wall', x: 194 }, { kind: 'barracks', x: 150 }, { kind: 'tavern', x: 160 }]);
  const levels = (kind: string) => (kind === 'tavern' ? 0 : 5);

  it('„Wie Hub-Stufe“: alles Auto, keine Einträge', () => {
    expect(presetLooks('hub', toggles, levels)).toEqual({});
  });
  it('„Voll ausgebaut“: höchste Baustufe je Art, Arten ohne Stufen bleiben Auto', () => {
    const looks = presetLooks('max', toggles, levels);
    expect(looks['wall@191']).toEqual({ level: 5 });
    expect(looks['barracks@150']).toEqual({ level: 5 });
    expect(looks['tavern@160']).toBeUndefined();
  });
  it('„Nichts gebaut“: jeder Platz unbebaut', () => {
    const looks = presetLooks('none', toggles, levels);
    expect(Object.keys(looks)).toHaveLength(toggles.length);
    expect(Object.values(looks).every((l) => l.unbuilt)).toBe(true);
  });
  it('„Nach Angriff“: gemischte Trefferpunkte aus der x-Stelle, bei gleichem Level immer gleich', () => {
    const a = presetLooks('attack', toggles, levels);
    expect(presetLooks('attack', toggles, levels)).toEqual(a);
    expect(new Set(Object.values(a).map((l) => l.hpPercent))).toEqual(new Set([75, 50, 25]));
    expect(a['wall@192']).toBeUndefined(); // 192 % 4 = 0 → 100 %: kein Eintrag
    expect(a['wall@193']).toEqual({ hpPercent: 50 }); // 193 % 4 = 1
  });
});

describe('Zustand einer Auswahl ändern (B-666/AC-03)', () => {
  it('setzt Baustufe und Trefferpunkte für alle Schlüssel; ein leerer Zustand entfernt den Eintrag', () => {
    const looks = applyLook({ 'wall@1': { level: 2 } }, ['wall@1', 'wall@2'], { hpPercent: 50 });
    expect(looks).toEqual({ 'wall@1': { level: 2, hpPercent: 50 }, 'wall@2': { hpPercent: 50 } });
    expect(applyLook(looks, ['wall@1', 'wall@2'], { level: undefined, hpPercent: undefined })).toEqual({});
  });
  it('„nicht gebaut“ ersetzt den Zustand, ein späterer Wert hebt es wieder auf', () => {
    const none = applyLook({ 'wall@1': { level: 3 } }, ['wall@1'], { unbuilt: true });
    expect(none['wall@1']).toEqual({ unbuilt: true });
    expect(applyLook(none, ['wall@1'], { hpPercent: 75 })['wall@1']).toEqual({ hpPercent: 75 });
  });
  it('commonOf liefert den gemeinsamen Wert oder undefined bei gemischten und leeren Auswahlen', () => {
    expect(commonOf(['a', 'b'], () => 1)).toBe(1);
    expect(commonOf(['a', 'b'], (k) => (k === 'a' ? 1 : 2))).toBeUndefined();
    expect(commonOf([], () => 1)).toBeUndefined();
    expect(hpOf(undefined)).toBe(100);
    expect(hpOf({ hpPercent: 25 })).toBe(25);
    expect(hpOf({ unbuilt: true })).toBe(0);
  });
});

describe('Klick auf einen Bauplatz der Bühne (B-667/AC-01 bis AC-03)', () => {
  const boxes = [{ kind: 'wall', x: 10, left: 100, right: 130, top: 0, bottom: 50 }, { kind: 'tower', x: 11, left: 120, right: 160, top: 0, bottom: 50 }];
  it('siteAt trifft den Umriss; bei Überlappung gewinnt die nächste Mitte; daneben nichts', () => {
    expect(siteAt(boxes, 105)?.kind).toBe('wall');
    expect(siteAt(boxes, 150)?.kind).toBe('tower');
    expect(siteAt(boxes, 126)?.kind).toBe('wall'); // Mitte 115 gegen 140
    expect(siteAt(boxes, 135)?.kind).toBe('tower');
    expect(siteAt(boxes, 90)).toBeUndefined();
    expect(siteAt(boxes, 95, 6)?.kind).toBe('wall'); // mit Rand
    expect(siteAt(boxes, 105, 0, 25)?.kind).toBe('wall');
    expect(siteAt(boxes, 105, 0, -40)).toBeUndefined(); // Himmel über dem Gebäude
    expect(siteAt(boxes, 105, 0, 90)).toBeUndefined(); // Boden darunter
  });
  const toggles = buildingToggles([{ kind: 'wall', x: 10 }, { kind: 'wall', x: 20 }, { kind: 'tower', x: 11 }]);
  const hit = (over: object = {}) => ({ kind: 'wall', x: 10, shift: false, double: false, ...over });
  it('Klick wählt den Platz, Umschalt erweitert und entfernt, Doppelklick wählt die Art, freie Fläche hebt auf', () => {
    expect(pickKeys(hit(), [], toggles, new Set())).toEqual(['wall@10']);
    expect(pickKeys(hit({ x: 20, shift: true }), ['wall@10'], toggles, new Set())).toEqual(['wall@10', 'wall@20']);
    expect(pickKeys(hit({ shift: true }), ['wall@10', 'wall@20'], toggles, new Set())).toEqual(['wall@20']);
    expect(pickKeys(hit({ double: true }), [], toggles, new Set())).toEqual(['wall@10', 'wall@20']);
    expect(pickKeys(null, ['wall@10'], toggles, new Set())).toEqual([]);
  });
  it('ausgeblendete Plätze sind nicht wählbar', () => {
    expect(pickKeys(hit(), ['tower@11'], toggles, new Set(['wall@10']))).toEqual(['tower@11']);
    expect(pickKeys(hit({ double: true }), [], toggles, new Set(['wall@20']))).toEqual(['wall@10']);
  });
});
