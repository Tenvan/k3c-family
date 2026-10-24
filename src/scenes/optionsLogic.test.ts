import { afterEach, describe, expect, it } from 'vitest';
import { DEFAULT_SETTINGS, type Settings } from '../core/settings';
import { setLanguage } from '../core/texts';
import { MenuPress, OPTION_IDS, OPTION_PAD_KEYS, applyOption, idleCommands, moveOption, optionEnabled, optionLabel, removeLabel, stepChoice, syncVolumes, tapDir, tapOption, worldLine } from './optionsLogic';

const base: Settings = { ...DEFAULT_SETTINGS };

describe('Einträge', () => {
  it('stehen in fester Reihenfolge', () => {
    expect(OPTION_IDS).toEqual(['music', 'sfx', 'ambient', 'screenshake', 'flash', 'colorblind', 'guide', 'language', 'resume', 'removePlayer', 'leave']);
  });

  it('jeder Eintrag hat einen Text', () => {
    for (const id of OPTION_IDS) expect(optionLabel(base, id).length).toBeGreaterThan(0);
  });
});

describe('Auswahl', () => {
  it('läuft am Ende zum Anfang und umgekehrt', () => {
    expect(moveOption(0, -1)).toBe(OPTION_IDS.length - 1);
    expect(moveOption(OPTION_IDS.length - 1, 1)).toBe(0);
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
    // AC-09: Ambient wie die anderen Regler in 10er-Schritten, ohne die anderen Busse zu berühren
    const up = applyOption({ ...base, ambientVolume: 50 }, 'ambient', 'right').settings;
    expect(up.ambientVolume).toBe(60);
    expect([up.musicVolume, up.sfxVolume]).toEqual([base.musicVolume, base.sfxVolume]);
    expect(applyOption({ ...base, ambientVolume: 50 }, 'ambient', 'left').settings.ambientVolume).toBe(40);
    expect(applyOption({ ...base, ambientVolume: 0 }, 'ambient', 'left').settings.ambientVolume).toBe(0);
    expect(applyOption({ ...base, ambientVolume: 100 }, 'ambient', 'right').settings.ambientVolume).toBe(100);
    expect(applyOption({ ...base, ambientVolume: 50 }, 'ambient', 'confirm').settings.ambientVolume).toBe(50);
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
    expect(applyOption(base, 'guide', 'confirm').settings).toEqual({ ...base, guideHints: false });
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
    expect(applyOption(base, 'leave', 'confirm')).toEqual({ settings: base, close: false, leave: true, removePlayer: false });
    expect(applyOption(base, 'leave', 'left').leave).toBe(false);
    expect(applyOption(base, 'leave', 'right').leave).toBe(false);
    for (const id of OPTION_IDS.filter((i) => i !== 'leave')) {
      for (const dir of ['left', 'right', 'confirm'] as const) expect(applyOption(base, id, dir).leave).toBe(false);
    }
    expect(optionLabel(base, 'leave')).toBe('Spiel verlassen');
  });

  it('nur „Spieler abmelden“ mit Bestätigen meldet ab, schließt und verlässt nicht (B-411)', () => {
    expect(applyOption(base, 'removePlayer', 'confirm')).toEqual({ settings: base, close: false, leave: false, removePlayer: true });
    expect(applyOption(base, 'removePlayer', 'left').removePlayer).toBe(false);
    for (const id of OPTION_IDS.filter((i) => i !== 'removePlayer')) {
      for (const dir of ['left', 'right', 'confirm'] as const) expect(applyOption(base, id, dir).removePlayer).toBe(false);
    }
    expect(optionLabel(base, 'removePlayer')).toBe('Spieler abmelden');
    for (const x of [100, 960, 1800]) expect(tapOption('removePlayer', x, 1920)).toBe('confirm');
  });

  it('„Spieler abmelden“ ist ohne Ziel ausgegraut, mit zwei Zielen wählbar (B-411/AC-03, AC-04)', () => {
    expect(optionEnabled('removePlayer', [])).toBe(false);
    expect(optionEnabled('removePlayer', [1])).toBe(true);
    expect(OPTION_IDS.filter((i) => i !== 'removePlayer').every((i) => optionEnabled(i, []))).toBe(true);
    expect(removeLabel(base, [1], 0)).toBe('Spieler abmelden');
    expect(removeLabel(base, [0, 1], 1)).toBe('Spieler abmelden  ◀ Spieler 2 ▶');
    expect(stepChoice(0, 'right', 2)).toBe(1);
    expect(stepChoice(1, 'right', 2)).toBe(1);
    expect(stepChoice(1, 'left', 2)).toBe(0);
    expect(stepChoice(1, 'confirm', 2)).toBe(1);
  });

  describe('„Spiel verlassen“ auf Englisch (B-293/AC-01)', () => {
    afterEach(() => setLanguage('de'));

    it('der Eintrag heißt „Leave game“', () => {
      setLanguage('en');
      expect(optionLabel({ ...base, language: 'en' }, 'leave')).toBe('Leave game');
    });
  });

  it('Antippen: Drittel links, Mitte, rechts', () => {
    expect(tapDir(100, 1920)).toBe('left');
    expect(tapDir(960, 1920)).toBe('confirm');
    expect(tapDir(1800, 1920)).toBe('right');
  });

  it('Antippen von „Weiter“ und „Spiel verlassen“ bestätigt in jedem Drittel (B-336)', () => {
    for (const x of [100, 960, 1800]) {
      expect(applyOption(base, 'resume', tapOption('resume', x, 1920)).close).toBe(true);
      expect(applyOption(base, 'leave', tapOption('leave', x, 1920)).leave).toBe(true);
    }
    expect(tapOption('music', 100, 1920)).toBe('left');
    expect(tapOption('music', 1800, 1920)).toBe('right');
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

describe('Welt im Pausenmenü (BD5.2, B-506/AC-02)', () => {
  it('nennt die Welt des Raums, leer = standard, ohne Angabe keine Zeile', () => {
    setLanguage('de');
    expect(worldLine('nordmeer')).toBe('Welt: nordmeer');
    expect(worldLine('')).toBe('Welt: standard');
    expect(worldLine(undefined)).toBe('');
  });
});

describe('Lautstärke wirkt im Spiel', () => {
  it('syncVolumes setzt alle drei Busse aus den Einstellungen', () => {
    const calls: [string, number][] = [];
    syncVolumes({ ...DEFAULT_SETTINGS, musicVolume: 30, sfxVolume: 60, ambientVolume: 10 }, { setVolume: (bus, percent) => calls.push([bus, percent]) });
    expect(calls).toEqual([['music', 30], ['sfx', 60], ['ambient', 10]]);
  });
});
