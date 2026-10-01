import { describe, expect, it } from 'vitest';
import { isOpenable } from './shell';

describe('isOpenable (openPage)', () => {
  it.each(['game.html', 'game.html?autostart=1&fresh=1&save=test-ab12&mock=3', 'testing.html#top', 'gamepad-test.html'])('erlaubt %s', (href) => {
    expect(isOpenable(href)).toBe(true);
  });

  it.each([
    'https://example.com/x.html',
    'http://localhost:8080/game.html',
    '//example.com/x.html',
    '/game.html',
    '../x.html',
    'sub/x.html',
    String.raw`sub\x.html`,
    '..html',
    'x.js',
    'game.html.js',
    'javascript:alert(1)',
    'index.html',
    '',
    '?game.html',
  ])('verweigert %s', (href) => {
    expect(isOpenable(href)).toBe(false);
  });
});
