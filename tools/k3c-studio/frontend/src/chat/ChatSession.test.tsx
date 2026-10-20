// @vitest-environment happy-dom
import { act } from 'react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { setChatOpen } from './chatBridge';
import { button, choose, flush, messages, mount, openChat, select, send, unmount, waitFor } from './chatTestKit';

// Chat-Kopf (Studio-Chat): Neuer Chat, gespeicherte Chats laden und fortsetzen, löschen sowie Anbieter, Modell und
// Denkstufe, die mit jeder Nachricht mitgehen. Gegen den Mock (api/mockChat.ts, Chats im Speicher).

beforeEach(() => { localStorage.clear(); setChatOpen(true); });
afterEach(() => { unmount(); vi.restoreAllMocks(); vi.unstubAllGlobals(); });

const savedIds = async () => (await backend.chatList()).map((c) => c.id);
const pick = (h: HTMLElement) => select(h, 'Gespeicherte Chats');

describe('Neuer Chat und gespeicherte Chats', () => {
  it('beginnt mit „Neuer Chat“ einen leeren Verlauf mit neuer Kennung', async () => {
    const h = await mount();
    await send(h, 'erste Frage', true);
    expect(messages(h)).toEqual(['erste Frage', 'Antwort auf: erste Frage']);
    const first = pick(h).value;
    expect(first).not.toBe('');
    await act(async () => button(h, 'Neuer Chat').click());
    expect(messages(h)).toEqual([]);
    expect(pick(h).value).toBe('');
    await send(h, 'zweite Frage', true);
    expect(pick(h).value).not.toBe(first);
    expect(await savedIds()).toContain(first);
  });

  it('listet gespeicherte Chats mit Titel, neuester zuerst', async () => {
    const h = await mount();
    await send(h, 'Frage Alt', true);
    await act(async () => button(h, 'Neuer Chat').click());
    await flush(5); // der Zeitstempel des zweiten Chats ist sicher jünger
    await send(h, 'Frage Neu', true);
    await waitFor(() => [...pick(h).options].some((o) => o.textContent?.startsWith('Frage Neu')));
    const labels = [...pick(h).options].map((o) => o.textContent ?? '');
    expect(labels[0]).toBe('Neuer Chat');
    const neu = labels.findIndex((l) => l.startsWith('Frage Neu'));
    const alt = labels.findIndex((l) => l.startsWith('Frage Alt'));
    expect(neu).toBeGreaterThan(0);
    expect(neu).toBeLessThan(alt);
  });

  it('lädt einen gespeicherten Chat in den Verlauf und setzt ihn fort', async () => {
    const h = await mount();
    await send(h, 'Frage A', true);
    const id = pick(h).value;
    await act(async () => button(h, 'Neuer Chat').click());
    await send(h, 'Frage B', true);
    await openChat(h, id, 'Frage A');
    expect(messages(h)).toEqual(['Frage A', 'Antwort auf: Frage A']);
    expect(pick(h).value).toBe(id);
    const stream = vi.spyOn(backend, 'chatStream');
    await send(h, 'Frage A2', true);
    expect(stream.mock.calls[0][0]).toBe(id);
    expect((await backend.chatGet(id)).messages.map((m) => m.text)).toEqual(['Frage A', 'Antwort auf: Frage A', 'Frage A2', 'Antwort auf: Frage A2']);
  });

  it('zeigt fehlgeschlagene Antworten des geladenen Chats unvollständig, die Meldung nur einmal', async () => {
    const h = await mount();
    await send(h, '!fehler', true);
    const id = pick(h).value;
    await act(async () => button(h, 'Neuer Chat').click());
    await openChat(h, id, '!fehler');
    const answer = h.querySelector('.chat-msg[data-role="assistant"]')!;
    expect(answer.getAttribute('data-status')).toBe('incomplete');
    expect(answer.querySelector('.chat-text')).toBeNull();
    expect(answer.querySelectorAll('.chat-note')).toHaveLength(1);
    expect(answer.querySelector('.chat-note')?.textContent).toContain('Claude Code CLI nicht gefunden');
  });

});

describe('Chat gelöscht, während er offen ist', () => {
  it('zeigt die Meldung des Servers und beginnt mit der nächsten Nachricht einen neuen Chat', async () => {
    const h = await mount();
    await send(h, 'bleibt offen', true);
    const id = pick(h).value;
    await backend.chatDelete(id); // woanders gelöscht, der Verlauf steht noch da
    await send(h, 'noch eine', true);
    expect(h.querySelector('.chat-msg[data-role="assistant"]:last-child .chat-note')?.textContent).toBe('Fehler: Der Chat wurde gelöscht');
    expect(pick(h).value).toBe('');
    expect((button(h, 'Chat löschen') as HTMLButtonElement).disabled).toBe(true);
    await send(h, 'neu angefangen', true);
    expect(pick(h).value).not.toBe('');
    expect(pick(h).value).not.toBe(id);
    expect((await backend.chatGet(pick(h).value)).messages.map((m) => m.text)).toEqual(['neu angefangen', 'Antwort auf: neu angefangen']);
  });
});

describe('Chat löschen', () => {
  it('löscht den aktuellen Chat nach Rückfrage und beginnt einen neuen', async () => {
    const h = await mount();
    await send(h, 'Wegwerf', true);
    const id = pick(h).value;
    const del = button(h, 'Chat löschen') as HTMLButtonElement;
    expect(del.disabled).toBe(false);
    const confirm = vi.fn().mockReturnValueOnce(false).mockReturnValueOnce(true);
    vi.stubGlobal('confirm', confirm);
    await act(async () => del.click());
    expect(await savedIds()).toContain(id); // abgelehnt
    await act(async () => del.click());
    await waitFor(() => pick(h).value === '' && messages(h).length === 0);
    expect(confirm).toHaveBeenCalledTimes(2);
    expect(await savedIds()).not.toContain(id);
    expect(messages(h)).toEqual([]);
    expect(pick(h).value).toBe('');
    expect([...pick(h).options].some((o) => o.value === id)).toBe(false);
    expect((button(h, 'Chat löschen') as HTMLButtonElement).disabled).toBe(true);
  });
});

describe('Anbieter, Modell und Denkstufe', () => {
  it('bietet die Werte des Backends an, Standard zuerst', async () => {
    const h = await mount();
    const labels = (name: string) => [...select(h, name).options].map((o) => o.textContent);
    expect(labels('Anbieter')).toEqual(['Standard', 'Claude Code CLI']);
    expect(labels('Modell')).toEqual(['Standard', 'Fable', 'Opus', 'Sonnet', 'Haiku']);
    expect(labels('Denkstufe')).toEqual(['Standard', 'Niedrig', 'Mittel', 'Hoch', 'Sehr hoch', 'Maximum']);
  });

  it('schickt die Wahl mit jeder Nachricht mit und merkt sie', async () => {
    const h = await mount();
    await choose(select(h, 'Anbieter'), 'claude-cli');
    await choose(select(h, 'Modell'), 'opus');
    await choose(select(h, 'Denkstufe'), 'high');
    const stream = vi.spyOn(backend, 'chatStream');
    await send(h, 'mit Wahl', true);
    expect(stream.mock.calls[0][2]).toEqual({ provider: 'claude-cli', model: 'opus', effort: 'high' });
    expect(['chat.provider', 'chat.model', 'chat.effort'].map((k) => localStorage.getItem(`k3c-studio.${k}`))).toEqual(['claude-cli', 'opus', 'high']);
    await choose(select(h, 'Modell'), '');
    await send(h, 'ohne Modell', true);
    expect(stream.mock.calls[1][2]).toEqual({ provider: 'claude-cli', model: '', effort: 'high' });
  });

  it('stellt die gemerkte Wahl beim nächsten Start wieder her, unbekannte Werte fallen auf Standard', async () => {
    localStorage.setItem('k3c-studio.chat.model', 'sonnet');
    localStorage.setItem('k3c-studio.chat.effort', 'gibt-es-nicht');
    const h = await mount();
    expect(select(h, 'Modell').value).toBe('sonnet');
    expect(select(h, 'Denkstufe').value).toBe('');
  });

  it('übernimmt Anbieter, Modell und Denkstufe eines geladenen Chats', async () => {
    const h = await mount();
    await choose(select(h, 'Modell'), 'sonnet');
    await choose(select(h, 'Denkstufe'), 'low');
    await send(h, 'mit Sonnet', true);
    const id = pick(h).value;
    await choose(select(h, 'Modell'), 'haiku');
    await choose(select(h, 'Denkstufe'), 'max');
    await act(async () => button(h, 'Neuer Chat').click());
    await openChat(h, id, 'mit Sonnet');
    expect(select(h, 'Modell').value).toBe('sonnet');
    expect(select(h, 'Denkstufe').value).toBe('low');
  });
});

describe('Standard und laufende Antworten', () => {
  const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

  it('übernimmt beim Laden auch den Standard eines gespeicherten Chats', async () => {
    const h = await mount();
    await send(h, 'mit Standard', true);
    const id = pick(h).value;
    await choose(select(h, 'Modell'), 'haiku');
    await act(async () => button(h, 'Neuer Chat').click());
    await openChat(h, id, 'mit Standard');
    expect(select(h, 'Modell').value).toBe('');
  });

  it('sperrt „Chat löschen“ und fragt vor Neuer Chat und Chat-Wechsel, solange eine Antwort läuft', async () => {
    const h = await mount();
    await send(h, 'fertig', true);
    const other = pick(h).value;
    await act(async () => button(h, 'Neuer Chat').click());
    await send(h, '!lang');
    await waitFor(() => !!button(h, 'Abbrechen') && pick(h).value !== '');
    expect((button(h, 'Chat löschen') as HTMLButtonElement).disabled).toBe(true);
    const confirm = vi.fn().mockReturnValueOnce(false).mockReturnValueOnce(false).mockReturnValueOnce(true);
    vi.stubGlobal('confirm', confirm);
    await act(async () => button(h, 'Neuer Chat').click());
    await choose(pick(h), other);
    expect(confirm).toHaveBeenNthCalledWith(1, 'Laufende Antwort abbrechen?');
    expect(messages(h)[0]).toBe('!lang'); // beide abgelehnt: die Antwort läuft weiter
    await openChat(h, other, 'fertig');
    await waitFor(() => !!button(h, 'Senden'));
    expect(messages(h)).toEqual(['fertig', 'Antwort auf: fertig']);
  });

  it('verwirft eine veraltete Antwort beim Laden eines Chats', async () => {
    const h = await mount();
    await send(h, 'Chat A', true);
    const a = pick(h).value;
    await act(async () => button(h, 'Neuer Chat').click());
    await send(h, 'Chat B', true);
    const b = pick(h).value;
    const original = backend.chatGet.bind(backend);
    vi.spyOn(backend, 'chatGet').mockImplementation(async (id) => {
      if (id === a) await sleep(60);
      return original(id);
    });
    await choose(pick(h), a);
    await choose(pick(h), b);
    await waitFor(() => messages(h)[0] === 'Chat B');
    await flush(100); // die verspätete Antwort für A trifft jetzt ein (ihr Timer ist älter) und darf nichts ändern
    expect(messages(h)).toEqual(['Chat B', 'Antwort auf: Chat B']);
    expect(pick(h).value).toBe(b);
  });

  it('lehnt das Löschen einer unbekannten Chat-Kennung ab (wie das Backend)', async () => {
    await expect(backend.chatDelete('gibt-es-nicht')).rejects.toThrow('kein Chat mit der id gibt-es-nicht');
  });
});
