// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import { buttons, click, mount, unmount } from '../testing';
import { SoundPage } from './SoundPage';

// WB1.4/AC-02 (D-03): Sound-Slots nur ansehen und anhören, kein Zuordnen.

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

describe('Sound (WB1.4)', () => {
  it('listet die Sound-Slots nach Kategorie mit Status', async () => {
    const host = await mount(<SoundPage />);
    for (const id of ['music:tag', 'sfx:muenze', 'ambient:wald']) expect(host.querySelector(`[data-sound="${id}"]`), id).toBeTruthy();
    expect(host.querySelector('[data-sound="music:tag"]')!.textContent).toContain('zugeordnet');
    expect(host.querySelector('[data-sound="sfx:muenze"]')!.textContent).toContain('fehlt');
    expect(host.querySelector('[data-sound="ambient:wald"]')!.textContent).toContain('Lücke');
    for (const k of ['musik', 'sfx', 'ambient']) expect(host.textContent).toContain(k);
  });

  it('Anhören lädt die Datei als Data-URL in einen Player', async () => {
    const data = vi.spyOn(backend, 'studioSoundData');
    const host = await mount(<SoundPage />);
    await click(buttons(host.querySelector('[data-sound="music:tag"]')!, 'Anhören')[0]);
    expect(data).toHaveBeenCalledWith('audio/tag.ogg');
    expect(host.querySelector('audio')?.getAttribute('src')).toMatch(/^data:audio\/ogg;base64,/);
  });

  it('eine fehlende Datei zeigt den Fehler, kein Zuordnen-Knopf', async () => {
    const host = await mount(<SoundPage />);
    await click(buttons(host.querySelector('[data-sound="sfx:muenze"]')!, 'Anhören')[0]);
    expect(host.textContent).toContain('audio/muenze.ogg fehlt unter public/');
    for (const t of ['Zuordnen', 'Übernehmen', 'Speichern', 'Löschen']) expect(buttons(host, t), t).toHaveLength(0);
  });
});
