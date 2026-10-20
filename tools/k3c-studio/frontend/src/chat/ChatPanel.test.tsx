// @vitest-environment happy-dom
import { act } from 'react';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { fillChat, setChatOpen } from './chatBridge';
import { answered, button, flush, input, mount, panel, send, unmount, waitFor } from './chatTestKit';

// WZ19.2 / AC-03 (Chat): auf jeder Seite da, ein- und ausblendbar, Antwort fortlaufend, Abbruch, Fehlermeldung,
// Eingabefeld von außen füllbar. Die Oberfläche läuft gegen den Mock (api/mockChat.ts).

beforeEach(() => { localStorage.clear(); setChatOpen(true); });
afterEach(unmount);

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
    const text = () => h.querySelector('.chat-msg[data-role="assistant"] .chat-text')?.textContent ?? '';
    await waitFor(() => text().length > 0);
    const mid = text();
    await waitFor(() => answered(h));
    const done = h.querySelector('.chat-msg[data-role="assistant"] .chat-text')!.textContent!;
    expect(done.trim()).toBe('Antwort auf: Hallo Welt');
    expect(mid.length).toBeLessThanOrEqual(done.length);
    expect(h.querySelector('.chat-msg[data-role="assistant"]')!.getAttribute('data-status')).toBe('complete');
  });

  it('bricht eine laufende Antwort ab und markiert sie', async () => {
    const h = await mount();
    await send(h, '!lang');
    await waitFor(() => !!button(h, 'Abbrechen'));
    await act(async () => button(h, 'Abbrechen').click());
    await waitFor(() => answered(h));
    const note = h.querySelector('.chat-msg[data-role="assistant"] .chat-note');
    expect(note?.textContent).toBe('Abgebrochen');
    expect(button(h, 'Senden')).toBeTruthy();
  });

  it('zeigt Fehler der CLI als Meldung an der Antwort', async () => {
    const h = await mount();
    await send(h, '!fehler', true);
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
