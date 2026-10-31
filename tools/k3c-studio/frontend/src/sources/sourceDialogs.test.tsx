// @vitest-environment happy-dom
import { Theme } from '@webkit/templates';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { resetMockSources } from '../api/mockSources';
import { ResearchDialog } from './SourceDialogs';

// B-673/AC-04 im DOM mit dem Mock der Vorschlagsliste: Übernahme aus der Recherche (seit WZ33 aus dem Werkzeuge-Panel der Assets-Seite).
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { await new Promise((r) => setTimeout(r, 0)); });

async function mount(node: React.ReactNode): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme>{node}</Theme>));
  await flush();
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

describe('Übernahme aus Recherche (B-673/AC-04)', () => {
  it('nennt übernommene und übersprungene Treffer mit Grund und schreibt erst nach Bestätigung', async () => {
    const preview = vi.spyOn(backend, 'sourceResearchPreview');
    const imp = vi.spyOn(backend, 'sourceResearchImport');
    const done = vi.fn();
    await mount(<ResearchDialog open source="" onClose={() => undefined} onDone={done} />);
    const text = document.body.textContent ?? '';
    expect(preview).toHaveBeenCalledWith('');
    expect(text).toContain('2 Treffer werden als Vorschlag „offen“ übernommen, zusammen 228 kB; 3 übersprungen');
    expect(text).toContain('keine direkte Datei-URL');
    expect(text).toContain('Quelle nicht erlaubt');
    expect(text).toContain('Lizenz nicht geklärt');
    expect(imp).not.toHaveBeenCalled();
    await click(byText(document.body, 'button', 'Übernehmen'));
    expect(imp).toHaveBeenCalledWith('', ['https://kenney.nl/media/medieval.zip', 'https://kenney.nl/media/tor.png']);
    expect(done).toHaveBeenCalledOnce();
    expect((await backend.sources()).vorschlaege.filter((p) => p.quelle === 'Kenney')).toHaveLength(4);
  });

  it('zeigt offline den Fehler und übernimmt nichts', async () => {
    vi.spyOn(backend, 'sourceResearchPreview').mockRejectedValue(new Error('offline: Größe per HEAD nicht möglich'));
    await mount(<ResearchDialog open source="Kenney" onClose={() => undefined} onDone={() => undefined} />);
    expect(document.body.textContent).toContain('offline');
    expect((byText(document.body, 'button', 'Übernehmen') as HTMLButtonElement).disabled).toBe(true);
  });
});
