import { describe, expect, it } from 'vitest';
import { assignContext, assignContextLine, filterText } from './assignContext';

// B-600/AC-01 (WZ29.7): Der Chat erhält Seite, Slot und Filter als Kontext.
const none = { kategorie: '', typ: '', status: '', suche: '' };

describe('Kontext für den Zuordnungs-Auftrag', () => {
  it('baut Seite, Slot und Filter für Sound und Grafik', () => {
    expect(assignContext('sound', 'sfx:skill:healer', { ...none, status: 'luecke', suche: ' heil ' })).toEqual({
      seite: 'sound', slot: 'sfx:skill:healer', filter: 'Status luecke · Suche „heil“',
    });
    expect(assignContext('grafik', '', none)).toEqual({ seite: 'grafik', slot: '', filter: '' });
  });

  it('hat auf anderen Seiten keinen Kontext', () => {
    expect(assignContext('welt', 'nursery', none)).toBeNull();
  });

  it('nennt alle Filter in fester Reihenfolge', () => {
    expect(filterText({ kategorie: 'gebaeude', typ: 'nursery', status: 'fehlt', suche: '' })).toBe('Kategorie gebaeude · Typ nursery · Status fehlt');
  });

  it('zeigt den Kontext als Zeile', () => {
    expect(assignContextLine({ seite: 'grafik', slot: '', filter: '' })).toBe('Grafik · kein Slot gewählt');
    expect(assignContextLine({ seite: 'sound', slot: 'sfx:coin', filter: 'Status luecke' })).toBe('Sound · Slot sfx:coin · Status luecke');
  });
});
