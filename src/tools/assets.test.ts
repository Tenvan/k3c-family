import { describe, expect, it } from 'vitest';
import register from '../../data/assets.json';

/** Slot-Register (B-298, M10.1): Erstbefüllung aus den Zuordnungstabellen, die als Historie bleiben. */
const docs = import.meta.glob('../../docs/assets/zuordnung*.md', { query: '?raw', import: 'default', eager: true }) as Record<string, string>;

/** Alle Bilder und Sounds unter public/, als Pfad relativ zu public/ */
const publicFiles = new Set(Object.keys(import.meta.glob('../../public/**/*.{png,ogg,mp3,wav,m4a}')).map((p) => p.replace('../../public/', '')));

const tableIds = Object.values(docs).flatMap((text) => {
  const ids: string[] = [];
  let inTable = false;
  for (const line of text.split(/\r?\n/)) {
    if (!line.startsWith('|')) inTable = false;
    else if (line.startsWith('| Objekt |')) inTable = true;
    else if (inTable && !line.startsWith('|---')) ids.push(line.split('|')[1].trim().replace(/`/g, ''));
  }
  return ids;
});

const CATEGORIES: Record<string, readonly string[]> = {
  grafik: ['figuren', 'gebaeude', 'umgebung', 'objekte', 'hud'],
  sound: ['sfx', 'musik', 'ambient'],
};

describe('Slot-Register data/assets.json', () => {
  const ids = register.slots.map((s) => s.id);

  it('hat für jede Objekt-ID der Zuordnungstabellen einen Asset-Slot', () => {
    expect(tableIds.length).toBeGreaterThan(50);
    expect(tableIds.filter((id) => !ids.includes(id))).toEqual([]);
  });

  it('hat eindeutige IDs und gültige Kategorien je Art', () => {
    expect(new Set(ids).size).toBe(ids.length);
    for (const s of register.slots) expect(CATEGORIES[s.art], s.id).toContain(s.kategorie);
  });

  it('nennt nur Dateien, die unter public/ liegen', () => {
    const missing = register.slots.flatMap((s) => s.wahl.map((w) => w.datei)).filter((f) => !publicFiles.has(f));
    expect(missing).toEqual([]);
  });

  it('enthält die Sound-Slots der Hörprobe', () => {
    expect(ids).toEqual(expect.arrayContaining(['music:tag', 'music:nacht', 'sfx:coin', 'sfx:hit', 'sfx:warn']));
  });
});
