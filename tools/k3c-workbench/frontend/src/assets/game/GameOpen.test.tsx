// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import { buttons, click, mount, unmount, until } from '../testing';
import { CatalogPage } from '../catalog/CatalogPage';

// WB1.4 (005 › 5, D-04): „Im Spiel ansehen“ im Katalog legt den Testspielstand an und öffnet die Adresse im Browser.

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

describe('Im Spiel ansehen (WB1.4)', () => {
  it('ruft studioGameOpen mit festem Seed und öffnet die Adresse', async () => {
    const game = vi.spyOn(backend, 'studioGameOpen');
    const open = vi.spyOn(backend, 'openUrl').mockImplementation(() => undefined);
    const host = await mount(<CatalogPage />);
    await until('Knopf Im Spiel ansehen', () => buttons(host, 'Im Spiel ansehen')[0]);
    await click(buttons(host, 'Im Spiel ansehen')[0]);
    await until('Adresse geöffnet', () => open.mock.calls.length > 0);
    expect(game).toHaveBeenCalledWith('ansicht');
    expect(open.mock.calls[0][0]).toMatch(/^http:\/\/127\.0\.0\.1:\d+\/index\.html#game\.html/);
  });

  it('zeigt den Fehler, öffnet nichts', async () => {
    vi.spyOn(backend, 'studioGameOpen').mockRejectedValue(new Error('Spiel-Client fehlt (dist/index.html) – erst bauen: task build'));
    const open = vi.spyOn(backend, 'openUrl').mockImplementation(() => undefined);
    const host = await mount(<CatalogPage />);
    await until('Knopf Im Spiel ansehen', () => buttons(host, 'Im Spiel ansehen')[0]);
    await click(buttons(host, 'Im Spiel ansehen')[0]);
    await until('Fehlertext', () => host.textContent?.includes('task build'));
    expect(open).not.toHaveBeenCalled();
  });
});
