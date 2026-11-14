// @vitest-environment happy-dom
import { act } from 'react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import type { AssetEintrag } from '../api';
import { buttons, click, mount, unmount } from '../testing';
import { mountCatalog, select, until } from './catalogTesting';
import { CatalogPage } from './CatalogPage';

// B-732: Liste vor der Prüfung (AC-01), Cache über den Abbau der Seite hinweg und „Neu laden“ (AC-03).

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

const entry = (host: ParentNode, id: string) => host.querySelector<HTMLElement>(`[data-asset="${id}"]`)!;

/** Seite abbauen wie beim Tab-Wechsel, ohne den Cache zu leeren (unmount() aus testing.tsx setzt ihn zurück). */
function remount(host: HTMLElement) {
  const again = document.createElement('div');
  host.replaceWith(again);
  return again;
}

describe('Katalog: schrittweise Prüfung (B-732/AC-01)', () => {
  it('zeigt die Einträge, bevor die Prüfung fertig ist', async () => {
    let finish!: (v: AssetEintrag[]) => void;
    const real = backend.assetsCheck.bind(backend);
    const check = vi.spyOn(backend, 'assetsCheck').mockReturnValue(new Promise((r) => (finish = r)));
    const host = await mount(<CatalogPage />);
    await until('Liste', () => entry(host, 'banner'));
    expect(entry(host, 'banner').textContent).toContain('prüft …');
    expect(entry(host, 'banner').getAttribute('data-invalid')).toBe('false');
    expect(host.querySelector('[data-checking]')?.textContent).toMatch(/Prüft 0\/\d+/);
    await until('Prüfung angefragt', () => check.mock.calls.length > 0);
    finish(await real(check.mock.calls[0][0]));
    await until('Prüfung eingetragen', () => !host.querySelector('[data-checking]'));
    expect(entry(host, 'banner').textContent).not.toContain('prüft …');
    expect(entry(host, 'banner').getAttribute('data-invalid')).toBe('true');
  });

  it('ein Fehler der Prüfung markiert nur die Einträge des Häppchens', async () => {
    vi.spyOn(backend, 'assetsCheck').mockRejectedValue(new Error('bun fehlt'));
    const host = await mountCatalog();
    expect(entry(host, 'monarch').getAttribute('data-invalid')).toBe('true');
    expect(entry(host, 'monarch').textContent).toContain('cli: bun fehlt');
  });
});

describe('Katalog: Cache (B-732/AC-03)', () => {
  it('lädt nach dem Abbau der Seite weder Liste noch Frames neu', async () => {
    const index = vi.spyOn(backend, 'assetsIndex');
    const frame = vi.spyOn(backend, 'assetsFrame');
    const spec = vi.spyOn(backend, 'assetsGet');
    const { createRoot } = await import('react-dom/client');
    const host = await mountCatalog();
    await select(host, 'monarch');
    expect(frame).toHaveBeenCalledTimes(1);
    // zweite Seite mit eigener Wurzel: Cache auf Modul-Ebene bleibt
    const other = remount(host);
    document.body.appendChild(other);
    const root = createRoot(other);
    act(() => root.render(<CatalogPage />));
    await until('Liste aus dem Cache', () => entry(other, 'monarch'));
    await click(entry(other, 'monarch'));
    await until('Vorschau aus dem Cache', () => other.querySelector('img[alt="monarch idle Frame 1"]'));
    expect(index).toHaveBeenCalledTimes(1);
    expect(spec).toHaveBeenCalledTimes(1);
    expect(frame).toHaveBeenCalledTimes(1);
    act(() => root.unmount());
  });

  it('„Neu laden“ holt Liste und gezeigten Frame neu', async () => {
    const index = vi.spyOn(backend, 'assetsIndex');
    const frame = vi.spyOn(backend, 'assetsFrame');
    const host = await mountCatalog();
    await select(host, 'monarch');
    await click(buttons(host, 'Neu laden')[0]);
    await until('Frame neu geladen', () => frame.mock.calls.length === 2 && host.querySelector('img[alt="monarch idle Frame 1"]'));
    expect(index).toHaveBeenCalledTimes(2);
    expect(frame).toHaveBeenCalledTimes(2);
  });
});

describe('Katalog: Filter „nur geänderte“ (B-732)', () => {
  it('zeigt nur Assets im Diff', async () => {
    const real = backend.assetsReview.bind(backend);
    // gelöschte Assets (fass) zählen nicht, sie stehen nicht in der Liste
    vi.spyOn(backend, 'assetsReview').mockImplementation(async (b) => {
      const r = await real(b);
      return { ...r, assets: r.assets.filter((a) => a.id === 'monarch' || a.id === 'fass') };
    });
    const host = await mountCatalog();
    const box = host.querySelector<HTMLElement>('[role="checkbox"]')!;
    await until('Zähler im Filter', () => box.closest('label')?.textContent?.includes('nur geänderte (1)'));
    await click(box);
    await until('Filter an: banner weg', () => entry(host, 'monarch') && !entry(host, 'banner'));
    await click(box);
    await until('Filter aus: banner da', () => entry(host, 'banner'));
  });
});
