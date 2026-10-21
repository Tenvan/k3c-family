import { afterEach, describe, expect, it, vi } from 'vitest';
import { chatStreamHttp } from '../api/chatHttp';
import { ChatGoneError, type ChatStoredMessage, type ChatSummary } from '../api/chatTypes';
import { chatLabel, newestFirst, toThreadMessage } from './chatConvert';

// Gespeicherte Chats als Verlauf der Oberfläche (Studio-Chat) und der Aufruf des Servers mit der Wahl.

const stored = (m: Partial<ChatStoredMessage>): ChatStoredMessage => ({ role: 'assistant', text: 'x', time: '2026-10-20T10:00:00Z', ...m });
const summary = (id: string, updated: string): ChatSummary => ({ id, title: id, updated, count: 2 });

describe('Gespeicherte Nachrichten', () => {
  it('übernimmt Nachrichten von 🧑 und fertige Antworten', () => {
    expect(toThreadMessage(stored({ role: 'user', text: 'Hi' }), 0)).toMatchObject({ role: 'user', content: [{ type: 'text', text: 'Hi' }] });
    expect(toThreadMessage(stored({ status: 'fertig', text: 'Hallo' }), 1)).toMatchObject({ role: 'assistant', status: { type: 'complete' }, content: [{ text: 'Hallo' }] });
  });

  it('übernimmt eine Antwort ohne Ausgang als fertig', () => {
    expect(toThreadMessage(stored({}), 1).status).toMatchObject({ type: 'complete' });
  });

  it('markiert abgebrochene Antworten als abgebrochen, der Text bleibt', () => {
    const m = toThreadMessage(stored({ status: 'abgebrochen', text: 'Teil' }), 1);
    expect(m.status).toEqual({ type: 'incomplete', reason: 'cancelled' });
    expect(m.content).toEqual([{ type: 'text', text: 'Teil' }]);
  });

  it('markiert Fehler mit ihrer Meldung und lässt den doppelten Text der CLI weg (B-515)', () => {
    const m = toThreadMessage(stored({ status: 'fehler', text: 'CLI kaputt', error: 'CLI kaputt' }), 1);
    expect(m.status).toEqual({ type: 'incomplete', reason: 'error', error: 'CLI kaputt' });
    expect(m.content).toEqual([{ type: 'text', text: '' }]);
    expect(toThreadMessage(stored({ status: 'fehler', text: 'Teil', error: 'CLI kaputt' }), 1).content).toEqual([{ type: 'text', text: 'Teil' }]);
    expect(toThreadMessage(stored({ status: 'fehler' }), 1).status).toMatchObject({ error: 'Unbekannter Fehler' });
  });

  it('ordnet die Liste neueste zuerst und beschriftet mit Titel und Zeit', () => {
    const list = newestFirst([summary('alt', '2026-10-19T09:00:00Z'), summary('neu', '2026-10-20T09:00:00Z'), summary('kaputt', 'gestern')]);
    expect(list.map((c) => c.id)).toEqual(['neu', 'alt', 'kaputt']);
    expect(chatLabel({ ...summary('a', '2026-10-20T09:00:00Z'), title: 'Insel bauen' })).toMatch(/^Insel bauen · \d{2}\.\d{2}\./);
    expect(chatLabel({ ...summary('a', 'gestern'), title: '' })).toBe('Ohne Titel');
  });
});

describe('Aufruf des Servers', () => {
  afterEach(() => vi.unstubAllGlobals());

  it('schickt Chat, Text und die Wahl im Körper', async () => {
    const fetchMock = vi.fn().mockResolvedValue({ ok: true, body: new Response('event: done\ndata: {"status":"fertig"}\n\n').body });
    vi.stubGlobal('fetch', fetchMock);
    const events = [];
    for await (const ev of chatStreamHttp('c1', 'Hallo', { provider: 'claude-cli', model: 'opus', effort: '' }, new AbortController().signal)) events.push(ev);
    expect(JSON.parse(fetchMock.mock.calls[0][1].body)).toEqual({ chat: 'c1', text: 'Hallo', provider: 'claude-cli', model: 'opus', effort: '' });
    expect(events).toEqual([{ kind: 'done', status: 'fertig', error: undefined, notice: undefined }]);
  });

  it('meldet einen gelöschten Chat (HTTP 410) als ChatGoneError mit der Meldung des Servers', async () => {
    const body = JSON.stringify({ error: 'Der Chat wurde gelöscht' });
    vi.stubGlobal('fetch', vi.fn().mockImplementation(async () => new Response(body, { status: 410 })));
    const run = async () => { for await (const _ of chatStreamHttp('c1', 'x', { provider: '', model: '', effort: '' }, new AbortController().signal)) { /* nichts */ } };
    await expect(run()).rejects.toThrow(ChatGoneError);
    await expect(run()).rejects.toThrow('Der Chat wurde gelöscht');
  });
});
