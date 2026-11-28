// Block „Hub-Stufe → Gebäude“ (aus table/table_test.go).

import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';
import { block, compareBlock, END, replaceBlock, START } from './table';

const read = (rel: string) => readFileSync(fileURLToPath(new URL(`../../${rel}`, import.meta.url)), 'utf8');
const rows = (text: string) => text.split('\n| ').length - 1;

describe('block', () => {
  // WZ15.2/AC-01: Der Block in § 3 ist genau der aus data/hub.json erzeugte.
  it('ist gleich dem Block der Regeln', () => {
    const { same, actual, wanted } = compareBlock(read('docs/rules/materialien-gebaeude.md'), read('data/hub.json'));
    expect(actual).toBe(wanted);
    expect(same).toBe(true);
  });

  it('hat den Inhalt der heutigen fünf Stufen', () => {
    const text = block(read('data/hub.json'));
    for (const want of [
      '| 1 | Holz | Werkstatt −16; Bogner −28 | Zone 1: Baugrund Mauer ±44, Turm ±36, dazu 0…2 Plätze je Seed | Kaufen +16, Verkaufen +20 (nur Tiefe 0) |',
      '| 2 | Stein | Treppe hoch +16 (ab Tiefe 1); Treppe runter +8 (nur mit tieferer Stufe); Taverne −32; Kaserne +24; Lager −24 (je Insel) | Zone 2: 1…3 Mauern, 1…3 Türme je Seed | – |',
      '| 5 | Kristall | – | Zone 5: 1…3 Mauern, 1…3 Türme je Seed | – |',
    ]) {
      expect(text).toContain(want);
    }
    expect(rows(text)).toBe(5 + 1); // Kopf und fünf Stufen
  });

  // WZ15.2/AC-03: Mehr Hub-Stufen ohne Umbau, keine feste 5.
  it('trägt sieben Stufen', () => {
    const text = block(read('testdata/hub/hub7.json'));
    expect(text).toContain('| 6 | Stein | Schmiede +28 | Zone 6: 1…3 Mauern, 1…3 Türme je Seed | – |');
    expect(text).toContain('| 7 | Eisen | Rüstkammer +40 | Zone 7: 1…3 Mauern');
    expect(rows(text)).toBe(7 + 1);
  });

  it('lehnt kaputtes JSON, fehlende levels und falsche Typen ab', () => {
    expect(() => block('{')).toThrow('hub.json: ');
    expect(() => block('{"levels":[]}')).toThrow('keine levels');
    expect(() => block('{"levels":[{"cost":{"stone":1.5}}]}')).toThrow('falschen Typ');
    expect(() => block('{"levels":{}}')).toThrow('falschen Typ');
  });

  it('gibt Kommazahlen aus und prüft Stufen und Material', () => {
    const doc = (site: string, levels = '[{"cost":{}}]') =>
      `{"levels":${levels},"sites":[${site}],"sitePlacement":{"zone1WallUnits":44.5,"zone1TowerUnits":36.5,"zones":1},"merchant":{"buyOffsetUnits":16,"sellOffsetUnits":20,"onlyDepth":0}}`;
    const nursery = '{"kind":"nursery","offsetUnits":52,"hubLevel":1}';
    const half = block(doc('{"kind":"workshop","offsetUnits":-16.5}'));
    expect(half).toContain('Werkstatt −16.5');
    expect(half).toContain('Baugrund Mauer ±44.5, Turm ±36.5');
    // ein fehlender hubLevel heißt Stufe 1 wie in der Engine
    expect(block(doc('{"kind":"nursery","offsetUnits":52}'))).toContain('Baumschule +52');
    expect(() => block(doc('{"kind":"nursery","offsetUnits":52,"hubLevel":3}'))).toThrow('hubLevel 3');
    expect(() => block(doc(nursery, '[{"cost":{}},{"cost":{"stone":1,"iron":2}}]'))).toThrow('[iron stone]');
    expect(() => block(doc(nursery, '[{"cost":{}},{"cost":{"mithril":1}}]'))).toThrow('mithril');
    // Ausbauschritte (B-592): Material aus den Kosten der Schritte, das Material-Tor zählt nicht
    const steps = '[{"steps":[]},{"steps":[{"cost":{"stone":1,"gold":1}},{"cost":{"stone":2},"gate":{"copper":3}}]}]';
    expect(block(doc(nursery, steps))).toContain('| 2 | Stein |');
  });
});

describe('replaceBlock', () => {
  const doc = `Kopf\n${START}\nalt\n${END}\nFuß\n`;
  const wanted = `Kopf\n${START}\nneu\n${END}\nFuß\n`;

  it('lässt den Rest byte-gleich, auch mit CRLF', () => {
    expect(replaceBlock(doc, `${START}\nneu\n${END}`)).toBe(wanted);
    expect(replaceBlock(doc.replaceAll('\n', '\r\n'), `${START}\nneu\n${END}`)).toBe(wanted.replaceAll('\n', '\r\n'));
  });

  it.each([
    ['ohne Marken', 'nichts'],
    ['nur Start', `${START}\n`],
    ['Start doppelt', START + END + START],
    ['Ende vor Start', `${END}\n${START}`],
    ['Ende doppelt', START + END + END],
  ])('lehnt ab: %s', (_name, text) => {
    expect(() => replaceBlock(text, 'x')).toThrow();
  });

  it('normalisiert die Zeilenenden des Blocks', () => {
    const crlf = `${START}\r\nalt\r\n${END}\r\n`;
    expect(replaceBlock(crlf, `${START}\r\nneu\r\n${END}`)).toBe(`${START}\r\nneu\r\n${END}\r\n`);
  });
});
