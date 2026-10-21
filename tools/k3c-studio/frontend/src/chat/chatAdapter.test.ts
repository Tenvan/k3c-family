import type { ChatModelRunResult } from '@assistant-ui/react';
import { describe, expect, it } from 'vitest';
import { ChatGoneError, type ChatChoiceValues, type ChatEvent, type ChatSendOptions, type ChatStream, type ChatTask } from '../api/chatTypes';
import { parseChatBlock } from '../api/chatHttp';
import { answerText, createChatAdapter } from './chatAdapter';

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
    ]), { onNotice: (n) => notices.push(n) }), 'x');
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

describe('Chat-Adapter: Auswahl, geladener Chat, Fehlertext (B-515)', () => {
  const choice = (provider: string, model: string, effort: string): ChatChoiceValues => ({ provider, model, effort });

  it('schickt die aktuelle Wahl bei jeder Nachricht mit', async () => {
    const seen: ChatChoiceValues[] = [];
    const stream: ChatStream = async function* (_chat, _text, options) {
      seen.push(options);
      yield { kind: 'done', status: 'fertig' };
    };
    let now = choice('claude-cli', 'opus', 'high');
    const adapter = createChatAdapter(stream, { options: () => now });
    await collect(adapter, 'eins');
    now = choice('', 'haiku', '');
    await collect(adapter, 'zwei');
    expect(seen).toEqual([choice('claude-cli', 'opus', 'high'), choice('', 'haiku', '')]);
  });

  it('schickt die Auftragsart nur mit, solange sie gesetzt ist (B-516)', async () => {
    const seen: ChatSendOptions[] = [];
    const stream: ChatStream = async function* (_chat, _text, options) {
      seen.push(options);
      yield { kind: 'done', status: 'fertig' };
    };
    let task: ChatTask | undefined = 'welt';
    const adapter = createChatAdapter(stream, { options: () => choice('', 'opus', ''), task: () => task });
    await collect(adapter, 'Erstelle eine Welt Nordsee');
    task = undefined;
    await collect(adapter, 'Hallo');
    expect(seen).toEqual([{ ...choice('', 'opus', ''), task: 'welt' }, choice('', 'opus', '')]);
  });

  it('setzt einen geladenen Chat fort und meldet Kennung und Ende', async () => {
    const seen: { chat: string; text: string }[] = [];
    const calls: string[] = [];
    const adapter = createChatAdapter(streamOf([{ kind: 'start', chat: 'c7' }, { kind: 'done', status: 'fertig' }], seen), {
      chat: 'c7', onChat: (c) => calls.push(`chat:${c}`), onEnd: () => calls.push('ende'),
    });
    await collect(adapter, 'weiter');
    expect(seen).toEqual([{ chat: 'c7', text: 'weiter' }]);
    expect(calls).toEqual(['chat:c7', 'ende']);
  });

  it('zeigt den Text der CLI bei einem Fehler nicht noch einmal als Antwort', async () => {
    const out = await collect(createChatAdapter(streamOf([
      { kind: 'chunk', text: 'Claude Code CLI nicht gefunden' }, { kind: 'done', status: 'fehler', error: 'Claude Code CLI nicht gefunden' },
    ])), 'x');
    expect(textOf(out.at(-1)!)).toBe('');
    expect(out.at(-1)?.status).toEqual({ type: 'incomplete', reason: 'error', error: 'Claude Code CLI nicht gefunden' });
  });

  it('entfernt nur die Meldung am Ende der Antwort, Teilstrings bleiben unverändert', () => {
    expect(answerText('Teil 1 Fehler: kaputt', 'Fehler: kaputt')).toBe('Teil 1');
    expect(answerText('  Fehler: kaputt  ','Fehler: kaputt')).toBe('');
    expect(answerText('Teil 1', 'kaputt')).toBe('Teil 1');
    expect(answerText('kaputt im Detail', 'kaputt')).toBe('kaputt im Detail'); // Meldung nur mittendrin
    expect(answerText('kaputt', 'Fehler: kaputt')).toBe('kaputt'); // Antwort ist nur ein Teil der Meldung
    expect(answerText('Antwort', undefined)).toBe('Antwort');
    expect(answerText('', 'kaputt')).toBe('');
  });
});

describe('Gelöschter Chat (HTTP 410)', () => {
  it('meldet den Fehler und beginnt mit der nächsten Nachricht einen neuen Chat ohne Kennung', async () => {
    const seen: string[] = [];
    const gone: ChatStream = async function* (chat) {
      seen.push(chat);
      if (chat) throw new ChatGoneError('Der Chat wurde gelöscht');
      yield { kind: 'done', status: 'fertig' };
    };
    const calls: string[] = [];
    const adapter = createChatAdapter(gone, { chat: 'c1', onChat: (c) => calls.push(`chat:${c}`) });
    const out = await collect(adapter, 'eins');
    expect(out.at(-1)?.status).toEqual({ type: 'incomplete', reason: 'error', error: 'Der Chat wurde gelöscht' });
    expect(calls).toEqual(['chat:']);
    await collect(adapter, 'zwei');
    expect(seen).toEqual(['c1', '']);
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
