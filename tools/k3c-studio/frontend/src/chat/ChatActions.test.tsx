// @vitest-environment happy-dom
import { act } from 'react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { setChatOpen } from './chatBridge';
import { mount, send, unmount, waitFor } from './chatTestKit';

// Aktionskarten des Chats (B-679/AC-06): Die Seite geht mit jeder Nachricht mit; Karten erscheinen unter dem Chat,
// gesperrte ohne „Ausführen“, ausgeführt wird nur per Knopf und höchstens einmal. Gegen den Mock (api/mockChat.ts).

beforeEach(() => { localStorage.clear(); setChatOpen(true); });
afterEach(() => { unmount(); vi.restoreAllMocks(); });

const cards = (h: HTMLElement) => [...h.querySelectorAll<HTMLElement>('[data-testid="action"]')];
const cardButton = (card: HTMLElement, name: string) => [...card.querySelectorAll('button')].find((b) => b.textContent === name);

describe('Aktionskarten', () => {
  it('schickt die offene Seite mit', async () => {
    const h = await mount();
    const stream = vi.spyOn(backend, 'chatStream');
    await send(h, 'Hallo', true);
    expect(stream.mock.calls[0][2].seite).toBe('assets');
  });

  it('zeigt ohne Aktionen keinen Bereich', async () => {
    const h = await mount();
    await send(h, 'Hallo', true);
    expect(h.querySelector('[data-testid="actions"]')).toBeNull();
  });

  it('zeigt Karten, sperrt ungültige und führt per Knopf genau einmal aus', async () => {
    const h = await mount();
    await send(h, '!aktion Welt Nordsee', true);
    await waitFor(() => cards(h).length === 2);
    const [ok, blocked] = cards(h);
    expect(blocked.textContent).toContain('Gesperrt: unbekannte Art');
    expect(cardButton(blocked, 'Ausführen')).toBeUndefined();
    await act(async () => cardButton(ok, 'Ausführen')!.click());
    await waitFor(() => cards(h)[0].dataset.status === 'ausgefuehrt');
    expect(cards(h)[0].textContent).toContain('gewählt: nordsee');
    expect(cardButton(cards(h)[0], 'Ausführen')).toBeUndefined();
    await act(async () => cardButton(cards(h)[1], 'Verwerfen')!.click());
    await waitFor(() => cards(h)[1].dataset.status === 'verworfen');
  });
});
