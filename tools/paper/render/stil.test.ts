// RD1.1: Farben aus der Stil-Datei auflösen, Material-Tausch für Varianten und Stufen.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { farbe, ladeStil, tausche } from './stil';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const stil = ladeStil();
const monarch = () => JSON.parse(readFileSync(join(SPECS, 'monarch.paper.json'), 'utf8')) as Spec;
const materialien = (s: Spec) => s.teile.flatMap((t) => t.lagen.map((l) => l.material));

describe('Stil', () => {
  it('farbe: Material mit Ton, ohne Ton basis', () => {
    expect(farbe(stil, 'stein', 'licht')).toBe('#c2bdb1');
    expect(farbe(stil, 'stein')).toBe('#9a9386');
  });

  it('farbe: Einzelton ohne Ton', () => {
    expect(farbe(stil, 'huf')).toBe('#3a2c2a');
    expect(farbe(stil, 'huf', 'basis')).toBe('#3a2c2a');
  });

  it('farbe: unbekanntes Material wirft', () => {
    expect(() => farbe(stil, 'gibts-nicht')).toThrow(/gibts-nicht/);
  });

  it('tausche ersetzt Materialien in Lagen und Umhang', () => {
    const original = monarch();
    const vorher = JSON.stringify(original);
    const neu = tausche(original, { koenigsblau: 'smaragd', koenigsrot: 'purpur' });
    expect(materialien(neu)).not.toContain('koenigsblau');
    expect(materialien(neu)).toContain('smaragd');
    expect(neu.umhang?.material).toBe('purpur');
    expect(JSON.stringify(original)).toBe(vorher);
  });
});
