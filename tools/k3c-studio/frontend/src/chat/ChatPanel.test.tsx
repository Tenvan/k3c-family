// @vitest-environment happy-dom
import { act } from 'react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { registerDraftLoader, setChatOpen } from './chatBridge';
import { answered, button, input, mount, panel, send, unmount, waitFor } from './chatTestKit';

// WZ19.2 / AC-03 (Chat): auf jeder Seite da, ein- und ausblendbar, Antwort fortlaufend, Abbruch, Fehlermeldung;
// Welt-Auftrag mit Welten Editor (B-516). Die Oberfläche läuft gegen den Mock (api/mockChat.ts).

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

  it('schickt Welt-Aufträge, solange der Welten Editor einen Entwurfs-Lader angemeldet hat (B-516)', async () => {
    const stream = vi.spyOn(backend, 'chatStream');
    const h = await mount();
    const hint = () => h.querySelector('.chat-task');
    expect(hint()).toBeNull();
    let off = () => {};
    await act(async () => { off = registerDraftLoader(async () => {}); });
    expect(hint()?.textContent).toContain('Welt-Auftrag');
    expect(input(h).placeholder).toContain('Erstelle eine Welt Nordsee');
    await send(h, 'Erstelle eine Welt Nordsee', true);
    expect(stream.mock.calls.at(-1)?.[2].task).toBe('welt');
    await act(async () => off());
    expect(hint()).toBeNull();
    await send(h, 'Hallo', true);
    expect(stream.mock.calls.at(-1)?.[2].task).toBeUndefined();
    stream.mockRestore();
  });
});
