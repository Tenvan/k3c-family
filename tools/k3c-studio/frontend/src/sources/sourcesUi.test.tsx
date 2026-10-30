// @vitest-environment happy-dom
import { Theme } from '@webkit/templates';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { resetMockSources } from '../api/mockSources';
import { ResearchDialog } from './SourceDialogs';
import { SourceEmpty } from './SourceList';
import { SourcesPage } from './SourcesPage';

// B-673 (AC-02, AC-03, AC-04, AC-06): Seite Quellen im DOM mit dem Mock der Vorschlagsliste.
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

describe('Seite Quellen (B-673)', () => {
  it('zeigt Reiter mit Anzahl und Gruppen mit Lücke oder belegt; der Reiter wirkt mit dem Filter', async () => {
    const h = await mount(<SourcesPage />);
    const tabs = h.querySelector('[data-testid="source-tabs"]')!;
    expect(tabs.textContent).toMatch(/Alle\s*10/);
    expect(tabs.textContent).toMatch(/Kenney\s*2/);
    const head = (slot: string) => h.querySelector(`[data-slot="${slot}"] .src-head`)?.textContent ?? '';
    expect(head('tavern')).toContain('Lücke');
    expect(head('sfx:skill:healer')).toContain('belegt');
    await click(byText(tabs, 'button', 'Kenney'));
    expect([...h.querySelectorAll('[data-testid="proposal"]')].map((r) => r.getAttribute('data-id'))).toEqual(['q08', 'q05']);
    expect(h.querySelector('[data-testid="source-rules"]')?.textContent).toContain('CC0');
  });

  it('zeigt die Aktionsleiste nur bei Auswahl mit Anzahl und Größe; Herunterladen öffnet den Bestätigungsdialog', async () => {
    const h = await mount(<SourcesPage />);
    expect(h.querySelector('[data-testid="selection-bar"]')).toBeNull();
    await click(h.querySelector('[aria-label="Taverne (Pixel) wählen"]'));
    await click(h.querySelector('[aria-label="Abendlied (Flöte) wählen"]'));
    expect(h.querySelector('[data-testid="selection-sum"]')?.textContent).toBe('2 gewählt · 1,5 MB');
    await click(byText(h, 'button', 'Herunterladen'));
    expect(document.body.textContent).toContain('Herunterladen bestätigen');
    expect(document.body.textContent).toContain('2 Dateien, zusammen 1,5 MB');
    await click(byText(h, 'button', 'Auswahl leeren'));
    expect(h.querySelector('[data-testid="selection-bar"]')).toBeNull();
  });

  it('zeigt bei einer Quelle ohne Vorschläge den Hinweis mit beiden Knöpfen', async () => {
    const real = await backend.sources();
    vi.spyOn(backend, 'sources').mockResolvedValue({ ...real, vorschlaege: real.vorschlaege.filter((p) => p.quelle !== 'Kenney'), quellen: real.quellen.map((q) => (q.name === 'Kenney' ? { ...q, vorgeschlagen: 0 } : q)) });
    const h = await mount(<SourcesPage />);
    await click(byText(h.querySelector('[data-testid="source-tabs"]')!, 'button', 'Kenney'));
    const empty = h.querySelector('[data-testid="source-empty"]')!;
    expect(empty.textContent).toContain('Noch keine Vorschläge von Kenney');
    expect(byText(empty, 'button', 'Claude in Kenney suchen lassen')).not.toBeNull();
    expect(byText(empty, 'button', 'Aus Recherche übernehmen')).not.toBeNull();
  });

  it('SourceEmpty: leere Quelle bietet beide Wege an, gesperrt offline die Übernahme', async () => {
    const h = await mount(<SourceEmpty source="freesound" filtered={false} offline onSearch={() => undefined} onResearch={() => undefined} onReset={() => undefined} />);
    expect(byText(h, 'button', 'Claude in freesound suchen lassen')).not.toBeNull();
    expect((byText(h, 'button', 'Aus Recherche übernehmen') as HTMLButtonElement).disabled).toBe(true);
  });
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
