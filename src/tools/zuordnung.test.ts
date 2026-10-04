import { describe, expect, it } from 'vitest';
import buildings from '../../data/buildings.json';
import enemies from '../../data/enemies.json';
import troops from '../../data/troops.json';
import grafikCredits from '../../public/grafik/CREDITS.md?raw';
import spriteCredits from '../../public/sprites/CREDITS.md?raw';

/** Alle Teile der Zuordnungstabelle (B-161): docs/assets/zuordnung*.md */
const docs = import.meta.glob('../../docs/assets/zuordnung*.md', { query: '?raw', import: 'default', eager: true }) as Record<string, string>;

/** Ticket-Nummern, die als Datei in docs/backlog/ (inkl. archiv/) liegen */
const tickets = new Set(Object.keys(import.meta.glob('../../docs/backlog/**/B-*.md')).map((p) => /B-\d{3}/.exec(p)![0]));

const cells = (line: string) => line.split('|').slice(1, -1).map((c) => c.trim());

/** Zeilen aller Objekt-Tabellen (Kopfzeile beginnt mit „| Objekt |“); andere Tabellen (Pack-Tabelle) zählen nicht */
function objectRows() {
  const rows: { file: string; id: string; pack: string; style: string; license: string; status: string }[] = [];
  for (const [file, text] of Object.entries(docs)) {
    let inTable = false;
    for (const line of text.split('\n')) {
      if (!line.startsWith('|')) inTable = false;
      else if (line.startsWith('| Objekt |')) inTable = true;
      else if (inTable && !line.startsWith('|---')) {
        const [id = '', , pack = '', , style = '', license = '', status = ''] = cells(line);
        rows.push({ file, id: id.replace(/`/g, ''), pack, style, license, status });
      }
    }
  }
  return rows;
}

/** Ordner mit Credit-Eintrag, als `grafik/<ordner>` bzw. `sprites/<ordner>` */
function creditedFolders() {
  const folders = new Set<string>();
  for (const [root, text] of [['grafik', grafikCredits], ['sprites', spriteCredits]] as const) {
    for (const line of text.split('\n').filter((l) => l.startsWith('| ') && !l.startsWith('| Ordner'))) {
      for (const folder of cells(line)[0]!.split(',')) folders.add(`${root}/${folder.trim()}`);
    }
  }
  return folders;
}

const rows = objectRows();
const ids = [...Object.keys(buildings), ...Object.keys(enemies), ...Object.keys(troops)].filter((id) => !id.startsWith('_'));

describe('Grafik-Zuordnung (B-161)', () => {
  it('liest mindestens eine Objekt-Tabelle', () => {
    expect(rows.length).toBeGreaterThan(0);
  });

  it.each(ids)('%s hat eine Zeile mit Status (AC-01)', (id) => {
    const row = rows.find((r) => r.id === id);
    expect(row, `Zeile für ${id}`).toBeDefined();
    expect(row!.status).toMatch(/^(zugeordnet|\*\*Lücke)/);
  });

  it('jede Zeile nennt Stil und Lizenz (AC-05)', () => {
    const bad = rows.filter((r) => !r.style || r.style === '–' || !r.license || r.license === '–');
    expect(bad.map((r) => r.id)).toEqual([]);
  });

  it('jedes zugeordnete Asset hat einen Credit-Eintrag (AC-03)', () => {
    const credited = creditedFolders();
    const missing = rows
      .filter((r) => r.status === 'zugeordnet')
      .flatMap((r) => r.pack.replace(/`/g, '').split(',').map((p) => p.trim()))
      .filter((p) => !credited.has(p));
    expect(missing).toEqual([]);
  });

  it('jede Lücke ist fett und nennt ein vorhandenes Ticket (AC-04)', () => {
    for (const r of rows.filter((x) => x.status !== 'zugeordnet')) {
      expect(r.status, r.id).toMatch(/^\*\*Lücke: [^*]*B-\d{3}[^*]*\*\*$/);
      for (const t of r.status.match(/B-\d{3}/g)!) expect(tickets.has(t), `${r.id}: ${t}`).toBe(true);
    }
  });
});
