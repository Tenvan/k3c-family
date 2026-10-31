// @vitest-environment happy-dom
import { Theme } from '@webkit/templates';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { resetMockSources } from '../api/mockSources';
import { SceneView } from './SceneView';
import { ToolsPanel } from './ToolsPanel';

// B-677 (AC-08): Werkzeuge-Fläche und Sicht Szene im DOM mit dem Mock. credits.ts liest public/**/CREDITS.md?raw, das
// Vitest des Studios nicht ausliefert; die Szenen-Ansichten sind hier Marken, ihr Inhalt ist anderswo geprüft.
vi.mock('@game/tools/credits', () => ({ CREDITS: [] }));
vi.mock('../graphics/ScenesView', () => ({ ScenesView: () => <div data-testid="grafik-szenen" /> }));
vi.mock('../sound/SoundScenes', () => ({ SoundScenes: () => <div data-testid="sound-szenen" /> }));

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

async function mount(node: React.ReactNode): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme>{node}</Theme>));
  await flush();
  return host;
}

const click = (el: Element | null) => act(async () => { (el as HTMLElement).click(); });
const byText = (scope: ParentNode, selector: string, text: string) => [...scope.querySelectorAll(selector)].find((e) => e.textContent?.includes(text)) ?? null;

beforeEach(() => { resetMockSources(); localStorage.clear(); });
afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  document.body.innerHTML = '';
  vi.restoreAllMocks();
  root = null;
  host = null;
});

describe('Werkzeuge-Fläche (B-677/AC-08)', () => {
  it('zeigt Proben und die beiden Vorrat-Werkzeuge', async () => {
    const h = await mount(<ToolsPanel onClose={() => undefined} onChanged={() => undefined} offline={false} />);
    const text = h.textContent ?? '';
    expect(text).toContain('Mischprobe');
    expect(text).toContain('Lautheitsvergleich');
    expect(text).toContain('Abstandsprobe');
    expect(byText(h, 'button', 'Vorrat aufräumen …')).not.toBeNull();
    expect((byText(h, 'button', 'Aus Recherche übernehmen …') as HTMLButtonElement).disabled).toBe(false);
  });

  it('Vorrat aufräumen öffnet den Dialog; nach dem Entfernen meldet das Panel die Änderung', async () => {
    const preview = vi.spyOn(backend, 'sourceCleanupPreview').mockResolvedValue({ dateien: [{ id: 'x', dateiname: 'a.png', groesse: 10 }], groesse: 10 } as never);
    const run = vi.spyOn(backend, 'sourceCleanup').mockResolvedValue({ dateien: [], groesse: 0 });
    const changed = vi.fn();
    const h = await mount(<ToolsPanel onClose={() => undefined} onChanged={changed} offline={false} />);
    expect(preview).not.toHaveBeenCalled();
    await click(byText(h, 'button', 'Vorrat aufräumen …'));
    await flush();
    expect(document.body.textContent).toContain('1 unverwendete Datei');
    await click(byText(document.body, 'button', 'Entfernen'));
    expect(run).toHaveBeenCalledOnce();
    expect(changed).toHaveBeenCalledOnce();
  });

  it('Aus Recherche übernehmen ist offline gesperrt', async () => {
    const h = await mount(<ToolsPanel onClose={() => undefined} onChanged={() => undefined} offline />);
    expect((byText(h, 'button', 'Aus Recherche übernehmen …') as HTMLButtonElement).disabled).toBe(true);
    expect(h.textContent).toContain('Offline');
  });

  it('Übernahme aus der Recherche nennt übernommene und übersprungene Treffer und meldet die Änderung', async () => {
    const changed = vi.fn();
    const h = await mount(<ToolsPanel onClose={() => undefined} onChanged={changed} offline={false} />);
    await click(byText(h, 'button', 'Aus Recherche übernehmen …'));
    await flush();
    await click(byText(document.body, 'button', 'Übernehmen'));
    await flush();
    expect(h.querySelector('[data-testid="research-done"]')?.textContent).toBe('Aus Recherche: 2 übernommen, 3 übersprungen.');
    expect(changed).toHaveBeenCalledOnce();
  });

  it('Schließen ruft onClose', async () => {
    const close = vi.fn();
    const h = await mount(<ToolsPanel onClose={close} onChanged={() => undefined} offline={false} />);
    await click(h.querySelector('[aria-label="Werkzeuge schließen"]'));
    expect(close).toHaveBeenCalledOnce();
  });
});

describe('Sicht Szene (B-677)', () => {
  it.each([['grafik', 'grafik-szenen', 'sound-szenen'], ['sound', 'sound-szenen', 'grafik-szenen']] as const)('art=%s zeigt nur die passende Ansicht, ohne Schalter', async (art, yes, no) => {
    const h = await mount(<SceneView art={art} />);
    expect(h.querySelector(`[data-testid="${yes}"]`)).not.toBeNull();
    expect(h.querySelector(`[data-testid="${no}"]`)).toBeNull();
    expect(h.querySelector('.ap-art')).toBeNull();
  });

  it('bei Fokus Alles wählt der Schalter die Art und merkt sie', async () => {
    const h = await mount(<SceneView art="" />);
    expect(h.querySelector('[data-testid="grafik-szenen"]')).not.toBeNull();
    await click(byText(h.querySelector('.ap-art')!, 'button', 'Sound'));
    expect(h.querySelector('[data-testid="sound-szenen"]')).not.toBeNull();
    expect(h.querySelector('[data-testid="grafik-szenen"]')).toBeNull();
    expect(localStorage.getItem('k3c-studio.assets.szenenart')).toBe('sound');
  });

  it('startet mit der gemerkten Art', async () => {
    localStorage.setItem('k3c-studio.assets.szenenart', 'sound');
    const h = await mount(<SceneView art="" />);
    expect(h.querySelector('[data-testid="sound-szenen"]')).not.toBeNull();
  });
});
