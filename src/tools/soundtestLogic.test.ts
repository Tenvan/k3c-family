import { describe, expect, it } from 'vitest';
import { fileFor, groupCandidates, keyAction, moveSelection, padActions, parseCandidates, stepVolume, type Candidate } from './soundtestLogic';

const c = (gruppe: string, name = gruppe): Candidate => ({ gruppe, name, datei: 'audio/x', bus: 'music', quelle: 'q', lizenz: 'l' });

describe('Hörprobe-Logik', () => {
  it('parseCandidates verwirft kaputte Einträge', () => {
    expect(parseCandidates([c('Tag'), { gruppe: 'Tag' }, null, { ...c('Tag'), bus: 'x' }])).toHaveLength(1);
    expect(parseCandidates('kaputt')).toEqual([]);
  });

  it('groupCandidates: Zustände vor Ereignissen, leere Gruppen fehlen, Unbekanntes am Ende', () => {
    const g = groupCandidates([c('Münze'), c('Nacht'), c('Tag'), c('Sonstiges')]);
    expect(g.map((x) => x.title)).toEqual(['Tag', 'Nacht', 'Münze', 'Sonstiges']);
    expect(g.map((x) => x.kind)).toEqual(['Zustand', 'Zustand', 'Ereignis', 'Ereignis']);
  });

  it('fileFor nimmt das erste abspielbare Format, sonst nichts', () => {
    expect(fileFor(c('Tag'), () => true)).toBe('audio/x.ogg');
    expect(fileFor(c('Tag'), (m) => m === 'audio/mpeg')).toBe('audio/x.mp3');
    expect(fileFor(c('Tag'), () => false)).toBeUndefined();
  });

  it('moveSelection und stepVolume halten die Grenzen', () => {
    expect([moveSelection(0, 3, -1), moveSelection(1, 3, 1), moveSelection(2, 3, 1), moveSelection(0, 0, 1)]).toEqual([0, 2, 2, 0]);
    expect([stepVolume(100, 1), stepVolume(0, -1), stepVolume(55, 1)]).toEqual([100, 0, 70]);
  });

  it('Tastatur und Pad: B (Taste 1) löst nichts aus, Flanke statt Dauerfeuer', () => {
    expect(keyAction('Enter')).toBe('play');
    expect(keyAction('Escape')).toBeUndefined();
    const none = { buttons: Array<boolean>(17).fill(false), stickY: 0 };
    const press = (i: number) => ({ buttons: none.buttons.map((_, k) => k === i), stickY: 0 });
    expect(padActions(none, press(0))).toEqual(['play']);
    expect(padActions(press(0), press(0))).toEqual([]);
    expect(padActions(none, press(1))).toEqual([]);
    expect(padActions(none, press(3))).toEqual(['fade']);
    expect(padActions(none, { ...none, stickY: -1 })).toEqual(['up']);
    expect(padActions({ ...none, stickY: -1 }, { ...none, stickY: -1 })).toEqual([]);
  });
});
