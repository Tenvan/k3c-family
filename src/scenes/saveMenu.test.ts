import { describe, expect, it } from 'vitest';
import { DeleteDialog, closeNotice, deleteNotice, nextSaveName, worldOf } from './saveMenu';

describe('Spielstände im Spielmenü (BD5.2, B-506/AC-02)', () => {
  it('neuer Spielstand bekommt den nächsten freien Namen spiel-N', () => {
    expect(nextSaveName([])).toBe('spiel-1');
    expect(nextSaveName(['familie', 'spiel-1', 'spiel-3'])).toBe('spiel-2');
    expect(nextSaveName(['spiel-1', 'spiel-2'])).toBe('spiel-3');
  });

  it('Welt ohne Angabe ist standard', () => {
    expect(worldOf(undefined)).toBe('standard');
    expect(worldOf('')).toBe('standard');
    expect(worldOf('nordmeer')).toBe('nordmeer');
  });

  it('Rückfrage: Abbrechen ist vorgewählt, nur Löschen + Bestätigen liefert den Namen', () => {
    const d = new DeleteDialog();
    expect(d.open).toBe(false);
    d.toggle('nord');
    expect(d.open && d.name === 'nord' && d.selected === 0).toBe(true);
    expect(d.labels()).toEqual(['Abbrechen', 'Löschen']);
    expect(d.confirm()).toBeNull(); // A sofort: nichts gelöscht
    expect(d.open).toBe(false);

    d.toggle('nord');
    d.move(1);
    expect(d.selected).toBe(1);
    d.move(1); // läuft zum Anfang zurück
    expect(d.selected).toBe(0);
    d.move(-1); // und umgekehrt
    expect(d.selected).toBe(1);
    expect(d.confirm()).toBe('nord');
    expect(d.open).toBe(false);
    expect(d.confirm()).toBeNull(); // ohne offene Rückfrage nie ein Name
  });

  it('Rückfrage: X/Entf erneut bricht ab, Tippen wählt die Zeile', () => {
    const d = new DeleteDialog();
    d.toggle('nord');
    d.move(1);
    d.toggle('nord');
    expect(d.open).toBe(false);
    d.toggle('burg');
    expect(d.selected).toBe(0); // neu geöffnet wieder auf Abbrechen
    expect(d.confirm(1)).toBe('burg');
    d.toggle('burg');
    expect(d.confirm(0)).toBeNull();
  });

  it('jede Server-Antwort ergibt einen Hinweis', () => {
    expect(deleteNotice('nord', 'ok')).toBe('Spielstand nord gelöscht');
    expect(deleteNotice('nord', 'in_use')).toBe('Spielstand nord ist in einem Raum offen und bleibt');
    expect(deleteNotice('nord', 'not_found')).toBe('Spielstand nord gibt es nicht mehr');
    expect(deleteNotice('nord', 'failed')).toBe('Spielstand nord: Löschen fehlgeschlagen');
  });
});

describe('Raum schließen (BD9.2, B-581/AC-03)', () => {
  it('Rückfrage für einen Raum: Abbrechen vorgewählt, nennt die belegten Plätze, nur „Raum schließen“ liefert den Namen', () => {
    const d = new DeleteDialog();
    d.toggle('nord', 'room', 2);
    expect(d.kind).toBe('room');
    expect(d.labels()).toEqual(['Abbrechen', 'Raum schließen']);
    expect(d.ask()).toContain('Belegte Plätze: 2');
    expect(d.confirm()).toBeNull();
    d.toggle('nord', 'room', 0);
    expect(d.ask()).toBe('Raum nord schließen? Der Stand wird gespeichert.');
    d.move(1);
    expect(d.confirm()).toBe('nord');
  });

  it('Rückfrage für einen Spielstand bleibt Löschen', () => {
    const d = new DeleteDialog();
    d.toggle('nord');
    expect(d.kind).toBe('save');
    expect(d.labels()).toEqual(['Abbrechen', 'Löschen']);
    expect(d.ask()).toBe('Spielstand nord löschen? Er bleibt als Sicherung erhalten.');
  });

  it('Hinweis nach dem Schließen', () => {
    expect(closeNotice('nord', 'ok')).toBe('Raum nord geschlossen');
    expect(closeNotice('nord', 'not_found')).toBe('Raum nord gibt es nicht mehr');
    expect(closeNotice('nord', 'failed')).toBe('Raum nord: Schließen fehlgeschlagen');
  });
});
