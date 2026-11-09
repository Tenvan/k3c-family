import { describe, expect, it } from 'vitest';
import { FullscreenKeeper } from './fullscreenKeeper';

describe('Vollbild halten (B-768/AC-05)', () => {
  it('fordert bei der ersten Eingabe an, nicht wenn Vollbild schon an ist', () => {
    const keeper = new FullscreenKeeper();
    expect(keeper.wants(0, true)).toBe(false);
    expect(keeper.wants(0, false)).toBe(true);
  });

  it('wartet zwischen zwei Versuchen (kein Dauerfeuer bei jeder Taste)', () => {
    const keeper = new FullscreenKeeper(1500);
    keeper.tried(1000);
    expect(keeper.wants(1200, false)).toBe(false);
    expect(keeper.wants(2499, false)).toBe(false);
    expect(keeper.wants(2500, false)).toBe(true);
  });

  it('nach einem Verlust ist der nächste Versuch wieder erlaubt', () => {
    const keeper = new FullscreenKeeper();
    expect(keeper.result(null)).toBe('ok');
    keeper.lost();
    expect(keeper.wants(10_000, false)).toBe(true);
  });

  it('lehnt der Browser mehrfach ab, hört der Halter mit einem Hinweis auf', () => {
    const keeper = new FullscreenKeeper(0, 3);
    expect(keeper.result('Permissions check failed')).toBe('denied');
    expect(keeper.result('Permissions check failed')).toBe('denied');
    expect(keeper.result('Permissions check failed')).toBe('giveup');
    expect(keeper.wants(99_999, false)).toBe(false);
  });

  it('ein Erfolg setzt die Fehler zurück; nach einem Verlust gilt auch ein Aufgeben nicht mehr', () => {
    const keeper = new FullscreenKeeper(0, 2);
    expect(keeper.result('x')).toBe('denied');
    expect(keeper.result(null)).toBe('ok');
    expect(keeper.result('x')).toBe('denied'); // wieder von vorn
    expect(keeper.result('x')).toBe('giveup');
    keeper.lost();
    expect(keeper.wants(5, false)).toBe(true);
    expect(keeper.result('x')).toBe('denied');
  });
});
