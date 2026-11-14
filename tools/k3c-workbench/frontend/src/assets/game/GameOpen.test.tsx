// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import { buttons, click, mount, unmount, until } from '../testing';
import { CatalogPage } from '../catalog/CatalogPage';

// B-847: „Im Spiel ansehen“ im Katalog legt den Testspielstand an und startet das native Spiel; kein Browser-Fenster.

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

describe('Im Spiel ansehen (B-847)', () => {
  it('ruft studioGameOpen mit festem Seed und zeigt, was gestartet ist', async () => {
    const game = vi.spyOn(backend, 'studioGameOpen').mockResolvedValue('Natives Spiel gestartet mit Spielstand voll-ansicht');
    const open = vi.spyOn(backend, 'openUrl').mockImplementation(() => undefined);
    const host = await mount(<CatalogPage />);
    await until('Knopf Im Spiel ansehen', () => buttons(host, 'Im Spiel ansehen')[0]);
    await click(buttons(host, 'Im Spiel ansehen')[0]);
    await until('Meldung', () => host.textContent?.includes('Natives Spiel gestartet'));
    expect(game).toHaveBeenCalledWith('ansicht');
    expect(open).not.toHaveBeenCalled();
  });

  it('zeigt den Fehler mit dem Build-Task', async () => {
    vi.spyOn(backend, 'studioGameOpen').mockRejectedValue(
      new Error('Natives Spiel fehlt (native/k3c-game/bin/k3c-game.exe) – erst bauen: task native:game:build'));
    const host = await mount(<CatalogPage />);
    await until('Knopf Im Spiel ansehen', () => buttons(host, 'Im Spiel ansehen')[0]);
    await click(buttons(host, 'Im Spiel ansehen')[0]);
    await until('Fehlertext', () => host.textContent?.includes('task native:game:build'));
  });
});
