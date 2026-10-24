// @vitest-environment happy-dom
import { Theme } from '@webkit/templates';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { resetMockSuggestions, setMockSuggestionsOffline } from '../api/mockSuggestions';
import { ChatPanel } from './ChatPanel';
import { chatContext, chatTask, setChatOpen, useAssignContext } from './chatBridge';
import type { AssignContext } from '../api/chatTypes';
import { button, flush, waitFor } from './chatTestKit';
import { SuggestionList } from './SuggestionList';
import { formatSize } from './SuggestionCard';

// WZ30.5 / AC-05 (B-600): Vorschlagskarten im Chat auf Sound- und Grafik-Seite, gegen den Mock (api/mockSuggestions.ts).
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

async function mountNode(node: React.ReactNode): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme>{node}</Theme>));
  await flush(10);
  return host;
}
const cards = (h: HTMLElement) => [...h.querySelectorAll<HTMLElement>('[data-testid="suggestion"]')];
const inCard = (c: HTMLElement, name: string) => [...c.querySelectorAll('button')].find((b) => b.textContent === name)!;

beforeEach(() => { resetMockSuggestions(); localStorage.clear(); setChatOpen(true); });
afterEach(() => { act(() => root?.unmount()); host?.remove(); root = null; host = null; });

describe('Vorschlagskarten (WZ30.5/AC-05)', () => {
  it('listet mehrere Vorschläge der Art mit Vorschau bzw. Hörprobe', async () => {
    const sound = await mountNode(<SuggestionList kind="sound" />);
    expect(cards(sound).map((c) => c.dataset.slot)).toEqual(['music:abend', 'sfx:skill:healer', 'sfx:skill:shield']);
    expect(inCard(cards(sound)[0]!, 'Hörprobe')).toBeTruthy();
    expect(cards(sound)[0]!.textContent).toContain('CC0 1.0');
    expect(cards(sound)[0]!.textContent).toContain('1,5 MB');
    act(() => root!.unmount());
    host!.remove();
    const grafik = await mountNode(<SuggestionList kind="grafik" />);
    expect(cards(grafik)).toHaveLength(3);
    expect(cards(grafik)[0]!.querySelector('img')?.getAttribute('src')).toContain('wagon.png');
  });

  it('übernimmt einen Vorschlag einzeln, die anderen bleiben', async () => {
    const h = await mountNode(<SuggestionList kind="sound" />);
    await act(async () => inCard(cards(h)[0]!, 'Übernehmen').click());
    await waitFor(() => cards(h).length === 2);
    expect(h.textContent).toContain('Übernommen: music:abend');
  });

  it('verwirft einen Vorschlag', async () => {
    const h = await mountNode(<SuggestionList kind="sound" />);
    await act(async () => inCard(cards(h)[1]!, 'Verwerfen').click());
    await waitFor(() => cards(h).length === 2);
    expect(cards(h).map((c) => c.dataset.slot)).not.toContain('sfx:skill:healer');
  });

  it('sperrt „Übernehmen“ bei Lizenzverstoß und nennt den Grund', async () => {
    const h = await mountNode(<SuggestionList kind="sound" />);
    const blocked = cards(h).find((c) => c.dataset.allowed === 'false')!;
    expect(inCard(blocked, 'Übernehmen').disabled).toBe(true);
    expect(blocked.textContent).toContain('Gesperrt: Lizenz ohne Weitergabe der Rohdateien (Q70)');
    expect(inCard(blocked, 'Verwerfen').disabled).toBe(false);
  });

  it('zeigt offline, ohne Karten, und erholt sich beim Aktualisieren', async () => {
    setMockSuggestionsOffline(true);
    const h = await mountNode(<SuggestionList kind="sound" />);
    expect(h.textContent).toContain('offline');
    expect(cards(h)).toHaveLength(0);
    setMockSuggestionsOffline(false);
    await act(async () => button(h, 'Aktualisieren').click());
    await waitFor(() => cards(h).length === 3);
    expect(h.textContent).not.toContain('Chat und Internet sind nicht erreichbar');
  });

  it('erscheint im Chat nur auf Sound- und Grafik-Seite', async () => {
    const welt = await mountNode(<ChatPanel page="welt" />);
    expect(welt.querySelector('[data-testid="suggestions"]')).toBeNull();
    act(() => root!.unmount());
    host!.remove();
    const sound = await mountNode(<ChatPanel page="sound" />);
    expect(sound.querySelector('[data-testid="suggestions"]')).not.toBeNull();
  });

  it('sperrt „Übernehmen“, solange die Datei nicht im Vorrat liegt (Download nur über Quellen)', async () => {
    const h = await mountNode(<SuggestionList kind="grafik" />);
    const offen = cards(h)[1]!;
    expect(inCard(offen, 'Übernehmen').disabled).toBe(true);
    expect(offen.textContent).toContain('erst auf der Seite Quellen laden');
    expect(offen.querySelector('img')).toBeNull();
  });

  it('nimmt den Slot aus dem Seitenkontext und macht Nachrichten zu Zuordnungs-Aufträgen (WZ29.7)', async () => {
    const ctx: AssignContext = { seite: 'grafik', slot: 'tavern', filter: 'Status luecke' };
    function Page() { useAssignContext(ctx); return <ChatPanel page="grafik" />; }
    const h = await mountNode(<Page />);
    await waitFor(() => cards(h).length === 2);
    expect(cards(h).map((c) => c.dataset.slot)).toEqual(['tavern', 'tavern']);
    expect(chatTask()).toBe('zuordnung');
    expect(chatContext()).toEqual(ctx);
    expect(h.querySelector('[data-testid="chat-assign"]')?.textContent).toContain('Grafik · Slot tavern · Status luecke');
    act(() => root!.unmount());
    root = null;
    expect(chatTask()).toBeUndefined();
  });

  it('formatiert Größen', () => {
    expect(formatSize(86_000)).toBe('86 kB');
    expect(formatSize(1_480_000)).toBe('1,5 MB');
  });
});
