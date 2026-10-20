// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { ChatPanel, ChatToggle } from './ChatPanel';
import { fillChat, setChatOpen } from './chatBridge';

// WZ19.2 / AC-03 (Chat): auf jeder Seite da, ein- und ausblendbar, Antwort fortlaufend, Abbruch, Fehlermeldung,
// Eingabefeld von außen füllbar. Die Oberfläche läuft gegen den Mock (api/mockChat.ts).
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = (ms = 0) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><ChatToggle /><ChatPanel /></Theme>));
  return host;
}

/** Wie eine Eingabe von Hand: Wert über den nativen Setter, dann input-Ereignis (React hört darauf). */
async function type(el: HTMLTextAreaElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => { el.dispatchEvent(new Event('input', { bubbles: true })); });
}

const input = (h: HTMLElement) => h.querySelector<HTMLTextAreaElement>('textarea')!;
const button = (h: HTMLElement, name: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === name)!;
const panel = (h: HTMLElement) => h.querySelector<HTMLElement>('[data-testid="chat-panel"]')!;

async function send(h: HTMLElement, text: string) {
  await type(input(h), text);
  await act(async () => button(h, 'Senden').click());
}

beforeEach(() => { localStorage.clear(); setChatOpen(true); });
afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
});

describe('Chat rechts im Studio (WZ19.2/AC-03)', () => {
  it('blendet sich ein und aus und merkt den Zustand', async () => {
    const h = await mount();
    expect(panel(h).hidden).toBe(false);
    await act(async () => h.querySelector<HTMLElement>('[data-testid="chat-toggle"]')!.click());
    expect(panel(h).hidden).toBe(true);
    expect(localStorage.getItem('k3c-studio.chat.state')).toBe('zu');
    await act(async () => h.querySelector<HTMLElement>('[data-testid="chat-toggle"]')!.click());
    expect(panel(h).hidden).toBe(false);
    await act(async () => button(h, 'Ausblenden').click());
    expect(panel(h).hidden).toBe(true);
  });

  it('zeigt die Antwort fortlaufend', async () => {
    const h = await mount();
    await send(h, 'Hallo Welt');
    expect(h.textContent).toContain('Hallo Welt');
    await flush(10);
    const mid = h.querySelector('.chat-msg[data-role="assistant"] .chat-text')?.textContent ?? '';
    expect(mid.length).toBeGreaterThan(0);
    await flush(100);
    const done = h.querySelector('.chat-msg[data-role="assistant"] .chat-text')!.textContent!;
    expect(done.trim()).toBe('Antwort auf: Hallo Welt');
    expect(mid.length).toBeLessThanOrEqual(done.length);
    expect(h.querySelector('.chat-msg[data-role="assistant"]')!.getAttribute('data-status')).toBe('complete');
  });

  it('bricht eine laufende Antwort ab und markiert sie', async () => {
    const h = await mount();
    await send(h, '!lang');
    await flush(20);
    await act(async () => button(h, 'Abbrechen').click());
    await flush(20);
    const note = h.querySelector('.chat-msg[data-role="assistant"] .chat-note');
    expect(note?.textContent).toBe('Abgebrochen');
    expect(button(h, 'Senden')).toBeTruthy();
  });

  it('zeigt Fehler der CLI als Meldung an der Antwort', async () => {
    const h = await mount();
    await send(h, '!fehler');
    await flush(20);
    expect(h.querySelector('.chat-msg[data-role="assistant"] .chat-note')?.textContent).toContain('Claude Code CLI nicht gefunden');
  });

  it('lässt das Eingabefeld von außen füllen und blendet den Chat dafür ein', async () => {
    const h = await mount();
    setChatOpen(false);
    await flush();
    await act(async () => { expect(fillChat('Prompt-Grundgerüst …')).toBe(true); });
    expect(input(h).value).toBe('Prompt-Grundgerüst …');
    expect(panel(h).hidden).toBe(false);
    expect(h.querySelector('.chat-msg')).toBeNull(); // gefüllt, nicht gesendet
  });
});
