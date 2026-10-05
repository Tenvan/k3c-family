import { describe, expect, it } from 'vitest';
import { DEFAULT_SETTINGS, type Settings } from '../core/settings';
import { MenuPress, OPTION_IDS, OPTION_PAD_KEYS, applyOption, idleCommands, moveOption, optionLabel, tapDir } from './optionsLogic';

const base: Settings = { ...DEFAULT_SETTINGS };

describe('Einträge', () => {
  it('stehen in fester Reihenfolge', () => {
    expect(OPTION_IDS).toEqual(['music', 'sfx', 'screenshake', 'flash', 'colorblind', 'language', 'resume', 'leave']);
  });

  it('jeder Eintrag hat einen Text', () => {
    for (const id of OPTION_IDS) expect(optionLabel(base, id).length).toBeGreaterThan(0);
  });
});

describe('Auswahl', () => {
  it('springt nicht über die Enden', () => {
    expect(moveOption(0, -1)).toBe(0);
    expect(moveOption(OPTION_IDS.length - 1, 1)).toBe(OPTION_IDS.length - 1);
    expect(moveOption(2, 1)).toBe(3);
    expect(moveOption(2, 0)).toBe(2);
  });
});

describe('Werte', () => {
  it('Regler gehen in 10-%-Schritten und bleiben in 0–100', () => {
    expect(applyOption({ ...base, musicVolume: 50 }, 'music', 'left').settings.musicVolume).toBe(40);
    expect(applyOption({ ...base, sfxVolume: 50 }, 'sfx', 'right').settings.sfxVolume).toBe(60);
    expect(applyOption({ ...base, musicVolume: 0 }, 'music', 'left').settings.musicVolume).toBe(0);
    expect(applyOption({ ...base, sfxVolume: 100 }, 'sfx', 'right').settings.sfxVolume).toBe(100);
  });

  it('Bestätigen ändert einen Regler nicht', () => {
    expect(applyOption(base, 'music', 'confirm').settings).toBe(base);
  });

  it('Schalter kippen (links, rechts, bestätigen), getrennt voneinander', () => {
    const a = applyOption(base, 'screenshake', 'confirm').settings;
    expect(a).toEqual({ ...base, screenshake: false });
    expect(applyOption(a, 'screenshake', 'right').settings.screenshake).toBe(true);
    expect(applyOption(base, 'flash', 'left').settings).toEqual({ ...base, flash: false });
    expect(applyOption(base, 'colorblind', 'confirm').settings).toEqual({ ...base, colorblindSymbols: false });
  });

  it('Sprache wechselt zwischen Deutsch und English, in jede Richtung', () => {
    const en = applyOption(base, 'language', 'right').settings;
    expect(en.language).toBe('en');
    expect(applyOption(en, 'language', 'left').settings.language).toBe('de');
    expect(applyOption(base, 'language', 'confirm').close).toBe(false);
    expect(optionLabel(en, 'language')).toContain('English');
  });

  it('nur „Weiter“ mit Bestätigen schließt', () => {
    expect(applyOption(base, 'resume', 'confirm').close).toBe(true);
    expect(applyOption(base, 'resume', 'left').close).toBe(false);
    expect(applyOption(base, 'flash', 'confirm').close).toBe(false);
  });

  it('nur „Spiel verlassen“ mit Bestätigen verlässt (B-293)', () => {
    expect(applyOption(base, 'leave', 'confirm')).toEqual({ settings: base, close: false, leave: true });
    expect(applyOption(base, 'leave', 'left').leave).toBe(false);
    expect(applyOption(base, 'leave', 'right').leave).toBe(false);
    for (const id of OPTION_IDS.filter((i) => i !== 'leave')) {
      for (const dir of ['left', 'right', 'confirm'] as const) expect(applyOption(base, id, dir).leave).toBe(false);
    }
    expect(optionLabel(base, 'leave')).toBe('Spiel verlassen');
  });

  it('Antippen: Drittel links, Mitte, rechts', () => {
    expect(tapDir(100, 1920)).toBe('left');
    expect(tapDir(960, 1920)).toBe('confirm');
    expect(tapDir(1800, 1920)).toBe('right');
  });
});

describe('Menu kurz', () => {
  it('Loslassen nach 300 ms öffnet', () => {
    const m = new MenuPress();
    expect(m.update(true, false, 1000)).toBe(false);
    expect(m.update(true, false, 1300)).toBe(false);
    expect(m.update(false, false, 1300)).toBe(true);
    expect(m.update(false, false, 1316)).toBe(false);
  });

  it('Loslassen nach 700 ms öffnet nicht', () => {
    const m = new MenuPress();
    m.update(true, false, 0);
    expect(m.update(false, false, 700)).toBe(false);
  });

  it('mit gehaltener View (Kombi) öffnet nicht, auch wenn kurz', () => {
    const m = new MenuPress();
    m.update(true, true, 0);
    expect(m.update(false, false, 100)).toBe(false);
    m.update(true, false, 500); // danach gilt wieder ein frischer Druck
    expect(m.update(false, false, 600)).toBe(true);
  });
});

describe('B-Taste und Stillstand', () => {
  it('die Szene belegt Pad-Taste B (Index 1) nicht', () => {
    expect(Object.values(OPTION_PAD_KEYS)).not.toContain(1);
  });

  it('idleCommands: alle Sitzplätze stehen still', () => {
    expect(idleCommands([0, 2])).toEqual([
      { slot: 0, moveX: 0, sprint: false, pay: false },
      { slot: 2, moveX: 0, sprint: false, pay: false },
    ]);
  });
});
