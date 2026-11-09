import { describe, expect, it } from 'vitest';
import { freshHref, shouldRefresh, withVersion } from './fresh';

const client = { version: 'v0.22.2', built: '' };
const server = { version: 'v0.22.3', built: '' };

describe('frisch laden', () => {
  it('nur bei erreichbarem Server mit anderer Version und ohne früheren Versuch', () => {
    expect(shouldRefresh(client, server, null)).toBe(true);
    expect(shouldRefresh(client, null, null)).toBe(false);
    expect(shouldRefresh(client, client, null)).toBe(false);
    expect(shouldRefresh(client, server, 'v0.22.3')).toBe(false); // schon versucht: keine Schleife
    expect(shouldRefresh(client, server, 'v0.22.1')).toBe(true); // Versuch für eine ältere Version
  });

  it('Adresse behält Abfrage und Hash der geöffneten Seite', () => {
    expect(freshHref('/index.html', '?a=1', '#game.html?stress=typisch', 'v0.22.3')).toBe('/index.html?a=1&v=v0.22.3#game.html?stress=typisch');
    expect(freshHref('/', '', '', 'v1')).toBe('/?v=v1');
  });

  it('ein altes v wird ersetzt', () => {
    expect(freshHref('/', '?v=alt', '', 'neu')).toBe('/?v=neu');
  });

  it('Seiten im iframe bekommen die Version, ein vorhandenes v bleibt', () => {
    expect(withVersion('game.html', 'v1')).toBe('game.html?v=v1');
    expect(withVersion('game.html?dev=1&stress=typisch', 'v1')).toBe('game.html?dev=1&stress=typisch&v=v1');
    expect(withVersion('game.html?v=x', 'v1')).toBe('game.html?v=x');
    expect(withVersion('x.html#a', 'v1')).toBe('x.html?v=v1#a'); // der Hash bleibt hinten
    expect(withVersion('x.html?q=1#a', 'v1')).toBe('x.html?q=1&v=v1#a');
  });
});
