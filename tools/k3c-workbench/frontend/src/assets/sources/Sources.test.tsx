// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import { buttons, click, mount, unmount } from '../testing';
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
    const row = host.querySelector('[data-datei="audio/kampf.ogg"]')!;
    expect(row.textContent).toContain('Kampfmusik');
    expect(row.textContent).toContain('CC BY-NC 4.0');
    expect(row.textContent).toContain('NC');
  });

  it('zeigt Treffer je Slot; Öffnen öffnet den Link im Browser', async () => {
    const open = vi.spyOn(backend, 'openUrl').mockImplementation(() => undefined);
    const host = await mount(<SourcesPage />);
    const hit = host.querySelector('[data-treffer="h1"]')!;
    expect(hit.textContent).toContain('ambient:wald');
    expect(hit.textContent).toContain('Wald bei Tag');
    expect(hit.textContent).toContain('erkannt');
    await click(buttons(hit, 'Öffnen')[0]);
    expect(open).toHaveBeenCalledWith('https://opengameart.org/');
  });

  it('keine Recherche-, Download- oder Import-Knöpfe', async () => {
    const host = await mount(<SourcesPage />);
    for (const t of ['Recherche', 'Download', 'Herunterladen', 'Importieren', 'Verwerfen', 'Lizenz bestätigen']) expect(buttons(host, t), t).toHaveLength(0);
  });
});
