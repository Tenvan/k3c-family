import { describe, expect, it } from 'vitest';
import createExample from '../../testdata/protocol/c2s-create.json';
import {
  DIALOG_ROWS,
  createOptions,
  defaultDefeat,
  devAllowed,
  dialogStep,
  gradeChoices,
  initialDialog,
  initialOptions,
  setOption,
  type DialogState,
} from './lobbyOptions';

const idle = { dir: 0, side: 0, confirm: false };
const row = (name: (typeof DIALOG_ROWS)[number]) => DIALOG_ROWS.indexOf(name);

describe('Standards je Grad (B-105/AC-01)', () => {
  it.each([
    ['dev', 'resources'],
    ['easy', 'resources'],
    ['normal', 'stage'],
    ['hard', 'stage'],
    ['ultra', 'lost'],
  ] as const)('%s → %s', (grade, defeat) => {
    expect(defaultDefeat(grade)).toBe(defeat);
  });

  it('Standard: Normal, Endboss, Stufenverlust', () => {
    expect(createOptions(initialOptions())).toEqual({ grade: 'normal', goal: 'endboss', defeat: 'stage' });
  });
});

describe('Wechselregel', () => {
  it('Grad-Wechsel setzt den Niederlage-Modus auf den Standard des Grads', () => {
    const ultra = setOption(initialOptions(), 'grade', gradeChoices(false).indexOf('ultra'), false);
    expect(createOptions(ultra)).toMatchObject({ grade: 'ultra', defeat: 'lost' });
  });

  it('eigene Wahl des Modus bleibt beim Grad-Wechsel', () => {
    const own = setOption(initialOptions(), 'defeat', 2, false);
    const easy = setOption(own, 'grade', gradeChoices(false).indexOf('easy'), false);
    expect(createOptions(easy)).toMatchObject({ grade: 'easy', defeat: 'lost' });
  });

  it('ungültige Indizes werden begrenzt', () => {
    expect(createOptions(setOption(initialOptions(), 'goal', 99, false)).goal).toBe('buildAll');
    expect(createOptions(setOption(initialOptions(), 'goal', -3, false)).goal).toBe('endboss');
    expect(createOptions(setOption(initialOptions(), 'grade', -1, false)).grade).toBe('easy');
  });
});

describe('Dev-Sperre', () => {
  it('Dev nur bei devAllowed', () => {
    expect(gradeChoices(false)).toEqual(['easy', 'normal', 'hard', 'ultra']);
    expect(gradeChoices(true)).toEqual(['dev', 'easy', 'normal', 'hard', 'ultra']);
    expect(createOptions(setOption(initialOptions(), 'grade', 0, true)).grade).toBe('dev');
  });

  it('devAllowed nur mit ?dev=1', () => {
    expect(devAllowed('')).toBe(false);
    expect(devAllowed('?dev=0')).toBe(false);
    expect(devAllowed('?room=KRNZ&dev=1')).toBe(true);
  });
});

describe('Dialog-Eingaben', () => {
  it('Pfeile wählen die Zeile, links/rechts ändern den Wert', () => {
    let d: DialogState = initialDialog();
    d = dialogStep(d, { ...idle, side: 1 }, false).dialog;
    expect(d.options.grade).toBe('hard');
    d = dialogStep(d, { ...idle, dir: 1 }, false).dialog;
    expect(d.row).toBe(row('goal'));
  });

  it('A auf einer Option schaltet weiter und beginnt nach dem letzten Wert von vorn', () => {
    let d: DialogState = { ...initialDialog(), row: row('grade') };
    d = dialogStep(d, { ...idle, confirm: true }, false).dialog;
    d = dialogStep(d, { ...idle, confirm: true }, false).dialog;
    expect(d.options.grade).toBe('ultra');
    d = dialogStep(d, { ...idle, confirm: true }, false).dialog;
    expect(d.options.grade).toBe('easy');
  });

  it('„Raum anlegen“ und „Abbrechen“ liefern ihre Aktion', () => {
    expect(dialogStep({ ...initialDialog(), row: row('create') }, { ...idle, confirm: true }, false).action).toBe('create');
    expect(dialogStep({ ...initialDialog(), row: row('cancel') }, { ...idle, confirm: true }, false).action).toBe('cancel');
    expect(dialogStep(initialDialog(), idle, false).action).toBeNull();
  });
});

it('gesendete Auswahl hat die Feldnamen des Protokoll-Beispiels', () => {
  const { grade, goal, defeat } = createExample;
  expect(createOptions(initialOptions())).toEqual({ grade, goal, defeat });
});
