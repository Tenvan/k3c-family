import type { ChatModelRunResult } from '@assistant-ui/react';
import { describe, expect, it } from 'vitest';
import type { ChatEvent, ChatStream } from '../api/chatTypes';
import { parseChatBlock } from '../api/chatHttp';
import { createChatAdapter } from './chatAdapter';

// WZ19.2 / AC-03 (Chat): Der Adapter macht aus dem Strom des Servers eine fortlaufende Antwort mit Ausgang.

const user = (text: string) => ({ role: 'user', content: [{ type: 'text', text }] });

function streamOf(events: ChatEvent[], seen: { chat: string; text: string }[] = []): ChatStream {
  return async function* (chat, text) {
    seen.push({ chat, text });
    yield* events;
  };
}

async function collect(adapter: ReturnType<typeof createChatAdapter>, text: string, signal = new AbortController().signal) {
  const out: ChatModelRunResult[] = [];
  const run = adapter.run({ messages: [user(text)], abortSignal: signal } as never) as AsyncGenerator<ChatModelRunResult>;
  for await (const r of run) out.push(r);
  return out;
}

const textOf = (r: ChatModelRunResult) => (r.content?.[0] as { text: string }).text;

describe('Chat-Adapter (WZ19.2)', () => {
  it('zeigt die Antwort fortlaufend und schließt sie als fertig ab', async () => {
    const out = await collect(createChatAdapter(streamOf([
      { kind: 'start', chat: 'c1' }, { kind: 'chunk', text: 'Hal' }, { kind: 'chunk', text: 'lo' }, { kind: 'done', status: 'fertig' },
    ])), 'Hi');
    expect(out.map(textOf)).toEqual(['Hal', 'Hallo', 'Hallo']);
    expect(out.at(-1)?.status).toEqual({ type: 'complete', reason: 'stop' });
  });

  it('schickt die Nachricht von 🧑 und behält den Chat des Servers für die nächste Nachricht', async () => {
    const seen: { chat: string; text: string }[] = [];
    const adapter = createChatAdapter(streamOf([{ kind: 'start', chat: 'c1' }, { kind: 'done', status: 'fertig' }], seen));
    await collect(adapter, 'eins');
    await collect(adapter, 'zwei');
    expect(seen).toEqual([{ chat: '', text: 'eins' }, { chat: 'c1', text: 'zwei' }]);
  });

  it('markiert Zeitlimit und Abbruch als abgebrochen, der Text bleibt', async () => {
    const out = await collect(createChatAdapter(streamOf([
      { kind: 'chunk', text: 'Teil' }, { kind: 'done', status: 'abgebrochen', error: 'Abgebrochen' },
    ])), 'x');
    expect(textOf(out.at(-1)!)).toBe('Teil');
    expect(out.at(-1)?.status).toEqual({ type: 'incomplete', reason: 'cancelled' });
  });

  it('meldet CLI-Fehler mit ihrer Meldung und den Hinweis zum Verlauf', async () => {
    const notices: string[] = [];
    const out = await collect(createChatAdapter(streamOf([
      { kind: 'done', status: 'fehler', error: 'Claude Code CLI nicht gefunden', notice: 'Verlauf nicht gespeichert' },
    ]), (n) => notices.push(n)), 'x');
    expect(out.at(-1)?.status).toEqual({ type: 'incomplete', reason: 'error', error: 'Claude Code CLI nicht gefunden' });
    expect(notices).toEqual(['', 'Verlauf nicht gespeichert']);
  });

  it('fängt Fehler der Verbindung ab, ohne zu werfen', async () => {
    const broken: ChatStream = async function* () {
      yield { kind: 'chunk', text: 'a' };
      throw new Error('Fehler 409 vom Server');
    };
    const out = await collect(createChatAdapter(broken), 'x');
    expect(out.at(-1)?.status).toEqual({ type: 'incomplete', reason: 'error', error: 'Fehler 409 vom Server' });
  });

  it('meldet einen Strom ohne Abschluss als Fehler', async () => {
    const out = await collect(createChatAdapter(streamOf([{ kind: 'chunk', text: 'a' }])), 'x');
    expect(out.at(-1)?.status).toEqual({ type: 'incomplete', reason: 'error', error: 'Verbindung ohne Abschluss beendet' });
  });

  it('wertet einen Abbruch während der Verbindung als abgebrochen', async () => {
    const ctl = new AbortController();
    const hanging: ChatStream = async function* () {
      ctl.abort();
      throw new DOMException('Aborted', 'AbortError');
    };
    const out = await collect(createChatAdapter(hanging), 'x', ctl.signal);
    expect(out.at(-1)?.status).toEqual({ type: 'incomplete', reason: 'cancelled' });
  });
});

describe('Antwort-Strom lesen (WZ19.2)', () => {
  it('liest start, chunk und done', () => {
    expect(parseChatBlock('event: start\ndata: {"chat":"c1"}')).toEqual({ kind: 'start', chat: 'c1' });
    expect(parseChatBlock('event: chunk\ndata: {"text":"ä\\nb"}')).toEqual({ kind: 'chunk', text: 'ä\nb' });
    expect(parseChatBlock('event: done\ndata: {"status":"fehler","error":"x","notice":"n"}')).toEqual({ kind: 'done', status: 'fehler', error: 'x', notice: 'n' });
  });

  it('überspringt Unbekanntes', () => {
    expect(parseChatBlock('event: ping\ndata: {}')).toBeNull();
    expect(parseChatBlock(': Kommentar')).toBeNull();
  });
});
