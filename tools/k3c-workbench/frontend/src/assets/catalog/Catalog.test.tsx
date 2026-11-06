// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import { buttons, click, mount, unmount, wait } from '../testing';
import { CatalogPage } from './CatalogPage';

// WB1.2/AC-02: Katalog mit Animations-Vorschau, Abzeichen „im Diff“ und rotem Eintrag für ungültige Specs.

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

const entry = (host: ParentNode, id: string) => host.querySelector<HTMLElement>(`[data-asset="${id}"]`)!;

describe('Katalog (WB1.2)', () => {
  it('listet alle Specs nach Gruppe mit Abzeichen „im Diff“', async () => {
    const host = await mount(<CatalogPage />);
    for (const id of ['monarch', 'pferd', 'mauer', 'banner']) expect(entry(host, id), id).toBeTruthy();
    for (const g of ['figuren', 'reittiere', 'gebaeude', 'requisiten']) expect(host.textContent).toContain(g);
    // Review gegen HEAD (Fixture): monarch geändert → im Diff
    expect(entry(host, 'monarch').textContent).toContain('im Diff');
  });

  it('zeigt ungültige Specs rot mit Feld, Regel und Text', async () => {
    const host = await mount(<CatalogPage />);
    const banner = entry(host, 'banner');
    expect(banner.getAttribute('data-invalid')).toBe('true');
    expect(banner.textContent).toContain('ungültig');
    expect(banner.textContent).toContain('teile[0].lagen[0].material · material-unbekannt');
    expect(banner.textContent).toContain('Material „scharlach“ fehlt in der Stil-Datei');
    expect(entry(host, 'monarch').getAttribute('data-invalid')).toBe('false');
  });

  it('Auswahl zeigt Animationen und das Bild des Frames', async () => {
    const frame = vi.spyOn(backend, 'assetsFrame');
    const host = await mount(<CatalogPage />);
    await click(entry(host, 'monarch'));
    for (const a of ['idle', 'lauf', 'reiten']) expect(buttons(host, a), a).toHaveLength(1);
    expect(frame).toHaveBeenCalledWith('monarch', 'idle', 0, '', '');
    expect(host.querySelector<HTMLImageElement>('img[alt="monarch idle Frame 1"]')?.src).toMatch(/^data:image\/png;base64,/);
    await click(buttons(host, 'lauf')[0]);
    expect(frame).toHaveBeenLastCalledWith('monarch', 'lauf', 0, '', '');
  });

  it('Abspielen läuft durch die Frames, Anhalten stoppt', async () => {
    const frame = vi.spyOn(backend, 'assetsFrame');
    const host = await mount(<CatalogPage />);
    await click(entry(host, 'monarch'));
    await click(buttons(host, 'Abspielen')[0]);
    await wait(300);
    const frames = frame.mock.calls.map((c) => c[2]);
    expect(Math.max(...frames)).toBeGreaterThan(0);
    await click(buttons(host, 'Anhalten')[0]);
    const n = frame.mock.calls.length;
    await wait(300);
    expect(frame.mock.calls.length).toBe(n);
  });

  it('ein Asset ohne Daten zeigt den Fehler, die Liste bleibt bedienbar', async () => {
    const host = await mount(<CatalogPage />);
    await click(entry(host, 'pferd'));
    expect(host.textContent).toContain('Asset „pferd“ fehlt');
    await click(entry(host, 'monarch'));
    expect(host.querySelector('img[alt^="monarch"]')).toBeTruthy();
  });
});
