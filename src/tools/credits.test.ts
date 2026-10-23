import { describe, expect, it } from 'vitest';
import { CREDITS, isCcBy, parseCredits, renderCredits } from './credits';

/** Verzeichnisse unter public/<root>/, abgeleitet aus den vorhandenen Dateien (Vite kennt keine leeren Ordner) */
const GLOBS = {
  grafik: Object.keys(import.meta.glob('../../public/grafik/*/*', { query: '?url' })),
  sprites: Object.keys(import.meta.glob('../../public/sprites/*/*', { query: '?url' })),
};
/** Eigene Grafik ohne Fremdlizenz (GEB1): Gebäude aus `task gen:gebaeude`, keine Credits nötig */
const OWN = new Set(['gebaeude']);
const dirsOf = (root: keyof typeof GLOBS) => [...new Set(GLOBS[root].map((p) => p.split('/')[4]!))].filter((d) => !OWN.has(d));
/** Import-Zeilen aus k3c-workbench nennen die Datei (kandidaten/<datei>, B-298); geprüft wird ihr Ordner. */
const dirOf = (folder: string) => folder.split('/')[0]!;

/** Sound-Dateien unter public/audio/ (Pfad ab audio/); selbst erzeugte Töne brauchen keinen Eintrag (B-493) */
const AUDIO = Object.keys(import.meta.glob('../../public/audio/**/*.{ogg,mp3,wav}', { query: '?url' })).map((p) => p.replace('../../public/audio/', ''));
const OWN_AUDIO = /^(atlas|probe-[ab])\.(ogg|mp3)$/;

describe('Credits der Sounds (B-493)', () => {
  const audio = CREDITS.filter((c) => c.root === 'audio');

  it('jede fremde Sound-Datei unter public/audio hat einen Eintrag (AC-02)', () => {
    const listed = new Set(audio.flatMap((c) => c.folders));
    expect(AUDIO.filter((f) => !OWN_AUDIO.test(f) && !listed.has(f))).toEqual([]);
  });

  it('jeder Sound-Eintrag zeigt auf eine vorhandene Datei', () => {
    expect(audio.length).toBeGreaterThan(0);
    expect(audio.flatMap((c) => c.folders).filter((f) => !AUDIO.includes(f))).toEqual([]);
  });

  it('die Seite zeigt die Sounds, CC-BY-Stücke hervorgehoben (AC-01)', () => {
    const html = renderCredits(CREDITS);
    for (const c of audio) expect(html, c.folders[0]).toContain(c.title.replace(/&/g, '&amp;').replace(/</g, '&lt;'));
    const by = audio.filter(isCcBy).map((c) => c.folders[0]);
    expect(by).toEqual(expect.arrayContaining(['kandidaten/a-big-adventure.ogg', 'kandidaten/summer-adventure-background-music.mp3']));
  });
});

describe('Credits (B-165)', () => {
  it.each(['grafik', 'sprites'] as const)('jedes Verzeichnis unter public/%s hat einen Eintrag (AC-01)', (root) => {
    const listed = new Set(CREDITS.filter((c) => c.root === root).flatMap((c) => c.folders.map(dirOf)));
    expect(dirsOf(root).filter((d) => !listed.has(d))).toEqual([]);
  });

  it.each(['grafik', 'sprites'] as const)('jeder Eintrag in %s zeigt auf ein vorhandenes Verzeichnis', (root) => {
    const dirs = new Set(dirsOf(root));
    const missing = CREDITS.filter((c) => c.root === root).flatMap((c) => c.folders).filter((f) => !dirs.has(dirOf(f)));
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
    expect(by.map((c) => c.folders[0])).toEqual(expect.arrayContaining(['horse', 'elephant', 'lpc-wolf']));
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
    // Import-Abschnitt aus k3c-workbench (B-298): Zeile je Datei, Spalte Status wird ignoriert
    const imported = [
      '| Ordner | Pack | Urheber | Lizenz | Quelle | Status |',
      '|---|---|---|---|---|---|',
      '| kandidaten/coin.png | Coin | A | CC0 1.0 | https://q | Lizenz prüfen |',
    ].join('\n');
    expect(parseCredits(imported, 'grafik')).toEqual([
      { root: 'grafik', folders: ['kandidaten/coin.png'], title: 'Coin', author: 'A', license: 'CC0 1.0', source: 'https://q' },
    ]);
  });

  it('ein Import unter kandidaten/ erscheint in der Lizenzliste mit Urheber, Lizenz und Quelle (B-420)', () => {
    const imported = [
      '| Ordner | Pack | Urheber | Lizenz | Quelle | Status |',
      '|---|---|---|---|---|---|',
      '| kandidaten/coin.png | Coin | Erika Muster | CC0 1.0 | https://example.org/coin | Lizenz prüfen |',
    ].join('\n');
    const html = renderCredits(parseCredits(imported, 'grafik'));
    for (const part of ['Coin', 'Erika Muster', 'CC0 1.0', 'example.org/coin']) expect(html, part).toContain(part);
  });
});
