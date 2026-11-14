// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import { buttons, chip, click, mount, tippe, unmount, until } from '../testing';
import { SoundPage } from './SoundPage';

// WB1.4/AC-02 (D-03): Sound-Slots nur ansehen und anhören, kein Zuordnen.

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

describe('Sound (WB1.4)', () => {
  it('listet die Sound-Slots nach Kategorie mit Status', async () => {
    const host = await mount(<SoundPage />);
    await until('Sound-Slots', () => ['music:tag', 'sfx:muenze', 'ambient:wald'].every((id) => host.querySelector(`[data-sound="${id}"]`)));
    expect(host.querySelector('[data-sound="music:tag"]')!.textContent).toContain('zugeordnet');
    expect(host.querySelector('[data-sound="sfx:muenze"]')!.textContent).toContain('fehlt');
    expect(host.querySelector('[data-sound="ambient:wald"]')!.textContent).toContain('Lücke');
    for (const k of ['musik', 'sfx', 'ambient']) expect(host.textContent).toContain(k);
  });

  it('Anhören lädt die Datei als Data-URL in einen Player', async () => {
    const data = vi.spyOn(backend, 'studioSoundData');
    const host = await mount(<SoundPage />);
    const slot = await until('Slot music:tag', () => host.querySelector('[data-sound="music:tag"]'));
    await until('Anhören-Knopf', () => buttons(slot, 'Anhören')[0]);
    await click(buttons(slot, 'Anhören')[0]);
    await until('Player mit Data-URL', () => host.querySelector('audio')?.getAttribute('src'));
    expect(data).toHaveBeenCalledWith('audio/tag.ogg');
    expect(host.querySelector('audio')?.getAttribute('src')).toMatch(/^data:audio\/ogg;base64,/);
  });

});

describe('Sound-Filter und -Prompts (B-728)', () => {
  it('B-728/AC-01: Filter nach Text, Kategorie und Status; kein Treffer lässt den Filter stehen', async () => {
    const host = await mount(<SoundPage />);
    await until('Sound-Slots', () => host.querySelectorAll('[data-sound]').length === 3);
    const slots = () => [...host.querySelectorAll('[data-sound]')].map((e) => e.getAttribute('data-sound'));
    await tippe(host, 'münze');
    await until('Text-Filter', () => slots().join() === 'sfx:muenze');
    await tippe(host, 'gibtsnicht');
    await until('kein Treffer', () => host.textContent?.includes('Kein Treffer.'));
    expect(host.querySelector('input[aria-label="Suchen"]')).not.toBeNull();
    await tippe(host, '');
    await click(chip(host, 'Kategorie', 'ambient'));
    await until('Kategorie-Chip', () => slots().join() === 'ambient:wald');
    await click(chip(host, 'Kategorie', 'ambient'));
    await click(chip(host, 'Status', 'zugeordnet'));
    await until('Status-Chip', () => slots().join() === 'music:tag');
  });

  it('B-728/AC-02: Klick auf den Slot zeigt Prompt-Buttons mit Slot-Id und Datei', async () => {
    const writeText = vi.fn(async (text: string) => void text);
    Object.defineProperty(navigator, 'clipboard', { value: { writeText }, configurable: true });
    const titel = (await backend.assetsPrompts()).filter((v) => v.ort === 'sound').map((v) => v.titel);
    expect(titel).toEqual(['Anderen Sound suchen', 'Kandidaten vergleichen', 'Lautstärke prüfen']);
    const host = await mount(<SoundPage />);
    const slot = await until('Slot music:tag', () => host.querySelector<HTMLElement>('[data-sound="music:tag"]'));
    expect(buttons(host, titel[0])).toHaveLength(0);
    await click(slot.querySelector('[data-wahl="sound"]')!);
    await until('Prompt-Buttons', () => titel.every((t) => buttons(slot, t).length === 1));
    await click(buttons(slot, 'Anderen Sound suchen')[0]);
    await until('kopiert', () => writeText.mock.calls.length === 1);
    expect(writeText.mock.calls[0][0]).toBe('Sound: anderen Sound für Slot `music:tag` suchen (aktuell audio/tag.ogg)');
    const wald = host.querySelector<HTMLElement>('[data-sound="ambient:wald"]')!;
    await click(wald.querySelector('[data-wahl="sound"]')!);
    await until('nur der gewählte Slot', () => buttons(host, titel[0]).length === 1 && buttons(wald, titel[0]).length === 1);
    await click(buttons(wald, 'Lautstärke prüfen')[0]);
    await until('kopiert', () => writeText.mock.calls.length === 2);
    expect(writeText.mock.calls[1][0]).toContain('`ambient:wald` prüfen (keine Datei)');
  });

  it('eine fehlende Datei zeigt den Fehler, kein Zuordnen-Knopf', async () => {
    const host = await mount(<SoundPage />);
    const slot = await until('Slot sfx:muenze', () => host.querySelector('[data-sound="sfx:muenze"]'));
    await until('Anhören-Knopf', () => buttons(slot, 'Anhören')[0]);
    await click(buttons(slot, 'Anhören')[0]);
    await until('Fehlertext', () => host.textContent?.includes('audio/muenze.ogg fehlt unter public/'));
    for (const t of ['Zuordnen', 'Übernehmen', 'Speichern', 'Löschen']) expect(buttons(host, t), t).toHaveLength(0);
  });
});
