import type { ThreadMessageLike } from '@assistant-ui/react';
import { useCallback, useEffect, useRef, useState } from 'react';
import { backend } from '../api';
import type { ChatSummary } from '../api/chatTypes';
import { newestFirst, toThreadMessages } from './chatConvert';
import type { ChatChoices } from './useChatChoices';

/** Der geöffnete Chat: `key` hängt den Verlauf neu ein, `chat` ist die Kennung auf dem Server (leer = noch nicht gespeichert). */
interface Session {
  key: number;
  chat: string;
  messages: ThreadMessageLike[];
}

const message = (e: unknown) => (e instanceof Error ? e.message : String(e));

/** Sitzung des Chats: gespeicherte Chats, der geöffnete Chat und die Handgriffe Neu, Wählen und Löschen. */
export function useChatSession(choices: ChatChoices) {
  const [chats, setChats] = useState<ChatSummary[]>([]);
  const [session, setSession] = useState<Session>({ key: 0, chat: '', messages: [] });
  const [error, setError] = useState('');
  const [running, setRunning] = useState(false);
  const request = useRef(0);

  const refresh = useCallback(() => {
    backend.chatList().then((list) => setChats(newestFirst(list))).catch(() => {}); // ohne Liste bleibt die alte stehen
  }, []);
  useEffect(refresh, [refresh]);

  const fresh = () => {
    setError('');
    setSession((s) => ({ key: s.key + 1, chat: '', messages: [] }));
  };
  /** Ein anderer Chat oder ein neuer hängt den Verlauf neu ein und bricht dabei eine laufende Antwort ab: vorher fragen. */
  const mayLeave = () => !running || window.confirm('Laufende Antwort abbrechen?');
  const select = async (id: string) => {
    if (!mayLeave()) return;
    const mine = ++request.current; // eine ältere Antwort von chatGet darf einen später gewählten Chat nicht überschreiben
    try {
      const detail = await backend.chatGet(id);
      if (mine !== request.current) return;
      choices.apply(detail);
      setError('');
      setSession((s) => ({ key: s.key + 1, chat: detail.id, messages: toThreadMessages(detail.messages) }));
    } catch (e) {
      if (mine === request.current) setError(`Chat nicht geladen: ${message(e)}`);
    }
  };
  const fromStart = () => {
    if (!mayLeave()) return;
    request.current++; // ein noch ladender Chat soll nicht mehr dazwischenkommen
    fresh();
  };
  const remove = async () => {
    const title = chats.find((c) => c.id === session.chat)?.title ?? '';
    if (!session.chat || running || !window.confirm(`Chat${title ? ` „${title}“` : ''} löschen?`)) return;
    try {
      await backend.chatDelete(session.chat);
      request.current++;
      fresh();
    } catch (e) {
      setError(`Chat nicht gelöscht: ${message(e)}`);
    }
    refresh();
  };

  return { chats, session, setSession, error, running, setRunning, refresh, onNew: fromStart, onSelect: (id: string) => void select(id), onDelete: () => void remove() };
}
