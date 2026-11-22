// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import { buttons, chip, click, mount, tippe, unmount, until } from '../testing';
import { SourcesPage } from './SourcesPage';

// WB1.4/AC-02 (D-03): Katalog mit Lizenz-Kennzeichen und Recherche-Treffer je Slot; Recherche, Download und Import
// bleiben MCP (assets_research, assets_hits, assets_import).

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

describe('Quellen (WB1.4)', () => {
  it('zeigt den Katalog mit Lizenz und Kennzeichen', async () => {
    const host = await mount(<SourcesPage />);
    const row = await until('Zeile kampf.ogg', () => host.querySelector('[data-datei="audio/kampf.ogg"]'));
    expect(row.textContent).toContain('Kampfmusik');
    expect(row.textContent).toContain('CC BY-NC 4.0');
    expect(row.textContent).toContain('NC');
  });

  it('zeigt Treffer je Slot; Öffnen öffnet den Link im Browser', async () => {
    const open = vi.spyOn(backend, 'openUrl').mockImplementation(() => undefined);
    const host = await mount(<SourcesPage />);
    const hit = await until('Treffer h1', () => host.querySelector('[data-treffer="h1"]'));
    await until('Öffnen-Knopf', () => buttons(hit, 'Öffnen')[0]);
    expect(hit.textContent).toContain('ambient:wald');
    expect(hit.textContent).toContain('Wald bei Tag');
    expect(hit.textContent).toContain('erkannt');
    await click(buttons(hit, 'Öffnen')[0]);
    expect(open).toHaveBeenCalledWith('https://opengameart.org/');
  });

  it('keine Recherche-, Download- oder Import-Knöpfe', async () => {
    const host = await mount(<SourcesPage />);
    await until('Katalog geladen', () => host.querySelector('[data-datei]'));
    for (const t of ['Recherche', 'Download', 'Herunterladen', 'Importieren', 'Verwerfen', 'Lizenz bestätigen']) expect(buttons(host, t), t).toHaveLength(0);
  });
});

const zeilen = (host: HTMLElement) => [...host.querySelectorAll('[data-datei], [data-treffer]')].map((e) => e.getAttribute('data-datei') ?? e.getAttribute('data-treffer'));

describe('Quellen-Filter und -Prompts (B-729)', () => {
  it('AC-01: Filter nach Lizenz, Pack und Art für Katalog und Treffer', async () => {
    const host = await mount(<SourcesPage />);
    await until('alle Einträge', () => zeilen(host).length === 3);
    await click(chip(host, 'Lizenz', 'CC0'));
    await until('Lizenz CC0', () => zeilen(host).join() === 'h1,audio/tag.ogg');
    await click(chip(host, 'Lizenz', 'CC0'));
    await click(chip(host, 'Pack', 'Kampfmusik'));
    await until('Pack', () => zeilen(host).join() === 'audio/kampf.ogg');
    await click(chip(host, 'Pack', 'Kampfmusik'));
    await click(chip(host, 'Art', 'sound'));
    await until('Art', () => zeilen(host).join() === 'audio/tag.ogg,audio/kampf.ogg' || zeilen(host).join() === 'audio/kampf.ogg,audio/tag.ogg');
    await click(chip(host, 'Art', 'sound'));
    await tippe(host, 'wald');
    await until('Suche', () => zeilen(host).join() === 'h1');
  });

  it('AC-02: Prompt-Buttons je Datei und je Treffer mit Datei, Pack und Lizenz', async () => {
    const writeText = vi.fn(async (text: string) => void text);
    Object.defineProperty(navigator, 'clipboard', { value: { writeText }, configurable: true });
    const host = await mount(<SourcesPage />);
    const datei = await until('Zeile kampf.ogg', () => host.querySelector<HTMLElement>('[data-datei="audio/kampf.ogg"]'));
    expect(buttons(host, 'Credit nachtragen')).toHaveLength(0);
    await click(datei.querySelector('[data-wahl="datei"]')!);
    await until('Datei-Prompts', () => ['Lizenz prüfen', 'Credit nachtragen'].every((t) => buttons(datei, t).length === 1));
    await click(buttons(datei, 'Credit nachtragen')[0]);
    await until('kopiert', () => writeText.mock.calls.length === 1);
    expect(writeText.mock.calls[0][0]).toBe('Quellen: Credit für `audio/kampf.ogg` nachtragen (Pack Kampfmusik, CC BY-NC 4.0)');

    const hit = host.querySelector<HTMLElement>('[data-treffer="h1"]')!;
    await click(hit.querySelector('[data-wahl="treffer"]')!);
    await until('Treffer-Prompts', () => ['Lizenz prüfen', 'Treffer übernehmen'].every((t) => buttons(hit, t).length === 1));
    expect(buttons(datei, 'Credit nachtragen')).toHaveLength(0);
    await click(buttons(hit, 'Treffer übernehmen')[0]);
    await until('kopiert', () => writeText.mock.calls.length === 2);
    expect(writeText.mock.calls[1][0]).toBe('Quellen: Treffer `h1` für Slot `ambient:wald` übernehmen (https://opengameart.org/, OpenGameArt, CC0)');
  });

  it('ohne Prompt-Vorlagen ein Hinweis statt leerer Fläche', async () => {
    vi.spyOn(backend, 'assetsPrompts').mockResolvedValue([]);
    const host = await mount(<SourcesPage />);
    const datei = await until('Katalog geladen', () => host.querySelector<HTMLElement>('[data-datei="audio/tag.ogg"]'));
    await click(datei.querySelector('[data-wahl="datei"]')!);
    await until('Hinweis', () => datei.textContent?.includes('Keine Prompt-Vorlagen'));
  });
});
