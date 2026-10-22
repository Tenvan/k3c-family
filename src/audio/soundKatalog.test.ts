import { describe, expect, it } from 'vitest';
import assets from '../../data/assets.json';
import katalog from '../../docs/assets/sounds.md?raw';
import credits from '../../public/audio/README.md?raw';

// Sound-Katalog und Credits (B-167/AC-01, AC-02, SO2.2): jedes Ereignis steht im Katalog, jede fremde Sound-Datei hat einen
// Credit-Eintrag mit CC0 oder CC-BY. Die Credit-Tabelle der Sounds steht in public/audio/README.md (so schreibt sie k3c-workbench).

/** Ereignisse aus B-167 › Anforderungen (Spalte „Ereignis“ des Katalogs) */
const EREIGNISSE = ['Treffer', 'Gegner-Tod', 'Münze aufheben', 'Münze geben', 'Pfeil', 'Schlag', 'Bau-Fortschritt', 'Bau fertig', 'Tod', 'Skill',
  'Nacht naht', 'Portal öffnet', 'Respawn (`revive`)', 'Wiederbeleben durch Mitspieler (`revived`)', 'Hub-Ausbau', 'Skill je Klasse', 'Boss-Auftritt', 'UI-Klicks', 'Reiten (Hufschlag, Schleife)'];
const ZUSTAENDE = ['Tag', 'Abend', 'Nacht', 'Kampf', 'Tiefe/Höhle', 'Boss', 'Lobby', 'Niederlage/Sieg'];
/** Selbst erzeugt, kein Credit nötig (public/audio/README.md) */
const SELBST = /^audio\/(atlas|probe-[ab])\.(ogg|mp3)$/;
const FILES = Object.keys(import.meta.glob('../../public/audio/**/*.{ogg,mp3,wav}', { query: '?url' })).map((p) => p.replace('../../public/', ''));
const TICKETS = new Set(Object.keys(import.meta.glob('../../docs/backlog/B-*.md', { query: '?raw' })).map((p) => /B-\d+/.exec(p)![0]));

const cells = (line: string) => line.trim().replace(/^\||\|$/g, '').split('|').map((c) => c.trim());
const rows = (md: string, head: string) => md.replace(/\r\n/g, '\n').split('\n').filter((l) => l.startsWith('|') && !l.startsWith('| ' + head) && !l.startsWith('|---')).map(cells);
const lines = [...rows(katalog, 'Ereignis'), ...rows(katalog, 'Zustand')];
const rowOf = (name: string) => lines.find((c) => c[0] === name);
const creditRows = rows(credits, 'Ordner').filter((c) => c[0]!.startsWith('kandidaten/'));

describe('Sound-Katalog (B-167/AC-01)', () => {
  it.each([...EREIGNISSE, ...ZUSTAENDE])('%s steht im Katalog mit Sound, Quelle und Lizenz oder als Lücke mit Ticket', (name) => {
    const c = rowOf(name);
    expect(c, `Zeile ${name} fehlt`).toBeDefined();
    const [, slot, sound, quelle, urheber, lizenz, status] = c!;
    const luecke = /\*\*Lücke\*\* \((B-\d+)/.exec(status!);
    if (luecke && slot === '–') return expect(TICKETS.has(luecke[1]!), `Ticket ${luecke[1]}`).toBe(true);
    expect([sound, quelle, urheber, lizenz].every((x) => x && x !== '–'), name).toBe(true);
    expect(lizenz).toMatch(/^CC0|^CC[- ]BY/);
  });

  it('der aktive Sound jeder zugeordneten Zeile ist die erste Präferenz des Slots im Register', () => {
    for (const c of lines.filter((r) => r[1]!.startsWith('`'))) {
      const slot = (assets.slots as { id: string; wahl: { datei: string }[] }[]).find((s) => s.id === c[1]!.replace(/`/g, ''));
      expect(slot?.wahl[0]?.datei, c[0]).toBe(c[2]!.replace(/`/g, '').replace(/ \(\+\d+\)$/, ''));
    }
  });
});

describe('Credits der Sounds (B-167/AC-02)', () => {
  it('jede fremde Sound-Datei unter public/audio hat einen Credit-Eintrag', () => {
    const listed = new Set(creditRows.map((c) => 'audio/' + c[0]));
    expect(FILES.filter((f) => !SELBST.test(f) && !listed.has(f))).toEqual([]);
  });

  it('jeder Credit-Eintrag zeigt auf eine vorhandene Datei und nennt Urheber, CC0 oder CC-BY und Quelle', () => {
    expect(creditRows.length).toBeGreaterThan(0);
    for (const [datei, titel, urheber, lizenz, quelle] of creditRows) {
      expect(FILES, datei).toContain('audio/' + datei);
      expect(titel && urheber && quelle, datei).toBeTruthy();
      expect(lizenz, datei).toMatch(/^CC0|^CC[- ]BY/);
    }
  });

  it('jede Datei der Sound-Slots im Register liegt unter public/audio', () => {
    const slots = (assets.slots as { art: string; wahl: { datei: string; sprite?: string }[] }[]).filter((s) => s.art === 'sound');
    for (const w of slots.flatMap((s) => s.wahl)) expect(FILES, w.datei).toContain(w.datei);
  });
});
