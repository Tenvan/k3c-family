import { describe, expect, it } from 'vitest';
import { CREDITS, isCcBy, parseCredits, renderCredits } from './credits';

/** Verzeichnisse unter public/<root>/, abgeleitet aus den vorhandenen Dateien (Vite kennt keine leeren Ordner) */
const GLOBS = {
  grafik: Object.keys(import.meta.glob('../../public/grafik/*/*', { query: '?url' })),
  sprites: Object.keys(import.meta.glob('../../public/sprites/*/*', { query: '?url' })),
};
const dirsOf = (root: keyof typeof GLOBS) => [...new Set(GLOBS[root].map((p) => p.split('/')[4]!))];

describe('Credits (B-165)', () => {
  it.each(['grafik', 'sprites'] as const)('jedes Verzeichnis unter public/%s hat einen Eintrag (AC-01)', (root) => {
    const listed = new Set(CREDITS.filter((c) => c.root === root).flatMap((c) => c.folders));
    expect(dirsOf(root).filter((d) => !listed.has(d))).toEqual([]);
  });

  it.each(['grafik', 'sprites'] as const)('jeder Eintrag in %s zeigt auf ein vorhandenes Verzeichnis', (root) => {
    const dirs = new Set(dirsOf(root));
    const missing = CREDITS.filter((c) => c.root === root).flatMap((c) => c.folders).filter((f) => !dirs.has(f));
    expect(missing).toEqual([]);
  });

  it('jeder Eintrag hat Titel, Urheber, Lizenz und Quelle', () => {
    expect(CREDITS.length).toBeGreaterThan(0);
    for (const c of CREDITS) {
      expect(c.folders.length, c.title).toBeGreaterThan(0);
      expect(c.title && c.author && c.license && c.source, c.folders.join()).toBeTruthy();
    }
  });

  it('die Seite zeigt jeden Eintrag aus den CREDITS-Dateien (AC-02)', () => {
    const html = renderCredits(CREDITS);
    expect(html.match(/<tr[^>]*><td>/g)).toHaveLength(CREDITS.length);
    for (const c of CREDITS) {
      expect(html, c.title).toContain(c.title);
      expect(html, c.title).toContain(c.author.replace(/&/g, '&amp;'));
      expect(html, c.title).toContain(c.license);
      expect(html, c.title).toContain(c.source.replace(/^https?:\/\//, ''));
    }
  });

  it('jeder CC-BY-Eintrag zeigt Urheber, Lizenz und Quelle als Link (AC-03)', () => {
    const by = CREDITS.filter(isCcBy);
    expect(by.map((c) => c.folders[0])).toEqual(expect.arrayContaining(['warped-caves-pixel-art-pack', 'horse', 'elephant', 'lpc-wolf']));
    const rows = renderCredits(CREDITS).split('<tr').filter((r) => r.startsWith(' class="by"'));
    expect(rows).toHaveLength(by.length);
    by.forEach((c, i) => {
      expect(rows[i], c.title).toContain(c.author.replace(/&/g, '&amp;'));
      expect(rows[i], c.title).toContain(c.license);
      expect(rows[i], c.title).toContain(`<a href="${c.source}" target="_blank" rel="noopener">`);
    });
  });

  it('CC0 und CC BY-NC zählen nicht als CC-BY', () => {
    const c = { root: 'grafik' as const, folders: ['x'], title: 't', author: 'a', source: 's' };
    expect(isCcBy({ ...c, license: 'CC0 1.0' })).toBe(false);
    expect(isCcBy({ ...c, license: 'CC BY-NC 4.0' })).toBe(false);
    expect(isCcBy({ ...c, license: 'CC-BY 3.0 (auch OGA-BY 3.0 / GPL)' })).toBe(true);
  });

  it('der Parser ordnet Spalten über die Kopfzeile zu und trennt mehrere Ordner', () => {
    const md = '| Ordner | Werk | Autor:innen | Lizenz | Quelle |\n|---|---|---|---|---|\n| a, b | W | X | CC0 | https://q |';
    expect(parseCredits(md, 'sprites')).toEqual([
      { root: 'sprites', folders: ['a', 'b'], title: 'W', author: 'X', license: 'CC0', source: 'https://q' },
    ]);
  });
});
