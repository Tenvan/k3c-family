import type { ThreadMessageLike } from '@assistant-ui/react';
import type { ChatStoredMessage, ChatSummary } from '../api/chatTypes';
import { answerText } from './chatAdapter';

const when = (time: string): Date | undefined => {
  const d = new Date(time);
  return Number.isNaN(d.getTime()) ? undefined : d;
};

/** Eine gespeicherte Nachricht als Nachricht der Oberfläche: abgebrochene und fehlgeschlagene Antworten bleiben unvollständig
 *  (mit ihrer Meldung), bei einem Fehler ohne doppelten Text der CLI (B-515). */
export function toThreadMessage(m: ChatStoredMessage, index: number): ThreadMessageLike {
  const createdAt = when(m.time);
  if (m.role === 'user') return { id: `u${index}`, role: 'user', content: [{ type: 'text', text: m.text }], createdAt };
  if (m.status === 'abgebrochen') {
    return { id: `a${index}`, role: 'assistant', content: [{ type: 'text', text: m.text }], createdAt, status: { type: 'incomplete', reason: 'cancelled' } };
  }
  if (m.status === 'fehler') {
    const error = m.error || 'Unbekannter Fehler';
    return {
      id: `a${index}`, role: 'assistant', createdAt, content: [{ type: 'text', text: answerText(m.text, m.error) }],
      status: { type: 'incomplete', reason: 'error', error },
    };
  }
  return { id: `a${index}`, role: 'assistant', content: [{ type: 'text', text: m.text }], createdAt, status: { type: 'complete', reason: 'stop' } };
}

export const toThreadMessages = (messages: readonly ChatStoredMessage[]): ThreadMessageLike[] => messages.map(toThreadMessage);

/** Liste der gespeicherten Chats, der zuletzt benutzte zuerst. */
export function newestFirst(chats: readonly ChatSummary[]): ChatSummary[] {
  return [...chats].sort((a, b) => (when(b.updated)?.getTime() ?? 0) - (when(a.updated)?.getTime() ?? 0));
}

/** Zeile der Chat-Auswahl: Titel und Zeitpunkt der letzten Nachricht. */
export function chatLabel(c: ChatSummary): string {
  const d = when(c.updated);
  const date = d ? d.toLocaleString('de-DE', { day: '2-digit', month: '2-digit', hour: '2-digit', minute: '2-digit' }) : '';
  return `${c.title || 'Ohne Titel'}${date ? ` · ${date}` : ''}`;
}
