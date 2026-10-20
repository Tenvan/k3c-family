import type { ChatModelAdapter, ChatModelRunResult } from '@assistant-ui/react';
import type { ChatStream } from '../api/chatTypes';

/** Text der letzten Nachricht von 🧑 (die CLI kennt den Verlauf nur über den Chat des Servers). */
function lastUserText(messages: readonly { role: string; content: readonly { type: string; text?: string }[] }[]): string {
  const last = [...messages].reverse().find((m) => m.role === 'user');
  return (last?.content ?? []).map((p) => (p.type === 'text' ? p.text ?? '' : '')).join('');
}

const text = (acc: string): ChatModelRunResult => ({ content: [{ type: 'text', text: acc }] });

/** Bindet den Antwort-Strom des Studios an assistant-ui: Stücke wachsen zu einer Antwort, `done` setzt den Ausgang
 *  (fertig, abgebrochen, Fehler), `onNotice` meldet z. B. „Verlauf nicht gespeichert“. Der Chat des Servers
 *  (`start`) bleibt über alle Nachrichten derselben Oberfläche erhalten. */
export function createChatAdapter(stream: ChatStream, onNotice: (notice: string) => void = () => {}): ChatModelAdapter {
  let chat = '';
  return {
    async *run({ messages, abortSignal }) {
      let acc = '';
      let ended = false;
      onNotice('');
      try {
        for await (const ev of stream(chat, lastUserText(messages), abortSignal)) {
          if (ev.kind === 'start') chat = ev.chat;
          if (ev.kind === 'chunk') yield text((acc += ev.text));
          if (ev.kind !== 'done') continue;
          ended = true;
          if (ev.notice) onNotice(ev.notice);
          if (ev.status === 'fertig') yield { ...text(acc), status: { type: 'complete', reason: 'stop' } };
          else if (ev.status === 'abgebrochen') yield { ...text(acc), status: { type: 'incomplete', reason: 'cancelled' } };
          else yield { ...text(acc), status: { type: 'incomplete', reason: 'error', error: ev.error ?? 'Unbekannter Fehler' } };
        }
        if (!ended) yield { ...text(acc), status: { type: 'incomplete', reason: 'error', error: 'Verbindung ohne Abschluss beendet' } };
      } catch (e) {
        if (abortSignal.aborted) {
          yield { ...text(acc), status: { type: 'incomplete', reason: 'cancelled' } };
          return;
        }
        yield { ...text(acc), status: { type: 'incomplete', reason: 'error', error: e instanceof Error ? e.message : String(e) } };
      }
    },
  };
}
