import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { fitZoom, parseZoom, stepZoom, zoomLabel } from './stageLogic';

const css = readFileSync(new URL('../styles/assetpage.css', import.meta.url), 'utf8');

describe('Zoom der Bühne', () => {
  it('Stufen hoch und runter, Zwischenwerte springen auf die nächste Stufe, Ränder bleiben', () => {
    expect(stepZoom(1, 1)).toBe(1.5);
    expect(stepZoom(1, -1)).toBe(0.75);
    expect(stepZoom(1.2, 1)).toBe(1.5);
    expect(stepZoom(1.2, -1)).toBe(1);
    expect(stepZoom(4, 1)).toBe(4);
    expect(stepZoom(0.25, -1)).toBe(0.25);
  });

  it('Anpassen: die ganze Szene passt in die Bühne, geklemmt; ohne Maße 1', () => {
    expect(fitZoom({ w: 400, h: 100 }, { w: 816, h: 1000 })).toBe(2);
    expect(fitZoom({ w: 4000, h: 200 }, { w: 516, h: 300 })).toBe(0.25);
    expect(fitZoom({ w: 10, h: 10 }, { w: 2000, h: 2000 })).toBe(4);
    expect(fitZoom({ w: 0, h: 0 }, { w: 500, h: 500 })).toBe(1);
  });

  it('Anzeige in Prozent, gemerkter Wert nur in den Grenzen', () => {
    expect(zoomLabel(1.5)).toBe('150 %');
    expect([parseZoom('2'), parseZoom(''), parseZoom('9'), parseZoom('x')]).toEqual([2, 1, 1, 1]);
  });
});

describe('assetpage.css', () => {
  it('Facetten-Spalte ohne Mindestbreite: minmax(0, …) mit kleinem Richtwert, Texte mit Ellipsis', () => {
    const body = css.match(/\.ap-body\s*\{[^}]*\}/)?.[0] ?? '';
    expect(body).toMatch(/grid-template-columns:\s*minmax\(0,\s*clamp\(\d+px,\s*\d+%,\s*\d+px\)\)/);
    expect(body).not.toMatch(/minmax\([1-9]\d*px/);
    expect(css).toMatch(/\.wk-facet-label\s*\{[^}]*text-overflow:\s*ellipsis/);
  });

  it('Bühne nur unter dem Raster (Splitter dazwischen, Mindesthöhen), Leiste über die volle Höhe, Vollbild nur als Klasse', () => {
    expect(css).toMatch(/\.ap-list\s*\{\s*grid-column:\s*1;\s*grid-row:\s*1;/);
    expect(css).toMatch(/\.ap-stagesplit\s*\{\s*grid-column:\s*1;\s*grid-row:\s*2;/);
    expect(css).toMatch(/\.ap-stage\s*\{\s*grid-column:\s*1;\s*grid-row:\s*3;/);
    expect(css).toMatch(/\.ap-side\s*\{\s*grid-column:\s*3;\s*grid-row:\s*1\s*\/\s*-1;/);
    expect(css).toMatch(/grid-template-rows:\s*minmax\([1-9]\d*px,\s*1fr\)\s*8px\s*minmax\([1-9]\d*px,\s*var\(--ap-stg,\s*38%\)\)/);
    expect(css).toMatch(/\.ap-stage-full\s*\{[^}]*position:\s*absolute/);
  });
});