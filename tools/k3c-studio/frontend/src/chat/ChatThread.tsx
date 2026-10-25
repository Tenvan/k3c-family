import { AssistantRuntimeProvider, ComposerPrimitive, ThreadPrimitive, useAuiState, useLocalRuntime, type ThreadMessage, type ThreadMessageLike } from '@assistant-ui/react';
import { Button } from '@webkit/atoms';
import { useEffect, useState } from 'react';
import { backend } from '../api';
import type { ChatChoiceValues } from '../api/chatTypes';
import { createChatAdapter } from './chatAdapter';
import { assignContextLine } from './assignContext';
import { chatContext, chatTask, loadDraft, useAssign, useCanLoadDraft } from './chatBridge';

const WHO = { user: 'Ich', assistant: 'Claude', system: 'System' } as const;

function messageText(m: ThreadMessage): string {
  return m.content.map((p) => (p.type === 'text' ? p.text : '')).join('');
}

/** Ausgang einer Antwort für die Anzeige: abgebrochen und Fehler stehen an der Antwort (WZ19, Fehlerfälle). */
function endNote(m: ThreadMessage): string {
  if (m.role !== 'assistant' || m.status.type !== 'incomplete') return '';
  if (m.status.reason === 'cancelled') return 'Abgebrochen';
  return `Fehler: ${String(m.status.error ?? 'Unbekannter Fehler')}`;
}

function Message({ message }: { message: ThreadMessage }) {
  const note = endNote(message);
  const body = messageText(message);
  const canLoad = useCanLoadDraft() && message.role === 'assistant' && message.status.type === 'complete';
  return (
    <div className="chat-msg" data-role={message.role} data-status={message.role === 'assistant' ? message.status.type : undefined}>
      <div className="chat-who">{WHO[message.role]}</div>
      {(body !== '' || !note) && <div className="chat-text">{body}</div>}
      {note && <div className="chat-note" role="status">{note}</div>}
      {canLoad && <Button size="1" variant="soft" onClick={() => void loadDraft(body)}>Als Entwurf laden</Button>}
    </div>
  );
}

/** Meldet, ob im Verlauf eine Antwort läuft (gehört in den Provider, liest den Zustand des Verlaufs). */
function RunningReporter({ onRunning }: { onRunning: (running: boolean) => void }) {
  const running = useAuiState((s) => s.thread.isRunning);
  useEffect(() => onRunning(running), [running, onRunning]);
  return null;
}

export interface ChatThreadProps {
  /** Chat des Servers, mit dem es weitergeht (leer = neuer Chat) */
  chat: string;
  /** Nachrichten eines geladenen Chats */
  initial: readonly ThreadMessageLike[];
  /** aktuelle Wahl von Anbieter, Modell und Denkstufe */
  current: () => ChatChoiceValues;
  onChat: (chat: string) => void;
  onEnd: () => void;
  /** eine Antwort läuft (true) oder nicht */
  onRunning: (running: boolean) => void;
}

/** Verlauf, Hinweiszeile und Eingabe eines Chats. Die Laufzeit entsteht einmal je Einhängen; ein anderer Chat hängt
 *  den Verlauf mit neuem `key` neu ein (laufende Antworten werden dabei abgebrochen). */
export function ChatThread({ chat, initial, current, onChat, onEnd, onRunning }: ChatThreadProps) {
  const [notice, setNotice] = useState('');
  const world = useCanLoadDraft(); // Welten Editor offen: Nachrichten sind Welt-Aufträge (B-516)
  const assign = useAssign(); // Sound- oder Grafik-Seite: Zuordnungs-Aufträge mit Seitenkontext (B-600)
  // einmal je Einhängen: Chat, Auswahl und Rückrufe ändern sich für diesen Verlauf nicht
  const [adapter] = useState(() => createChatAdapter(
    (c, text, options, signal) => backend.chatStream(c, text, options, signal),
    { chat, options: current, task: chatTask, context: chatContext, onNotice: setNotice, onChat, onEnd },
  ));
  const runtime = useLocalRuntime(adapter, { initialMessages: initial });
  useEffect(() => () => runtime.thread.cancelRun(), [runtime]);
  return (
    <AssistantRuntimeProvider runtime={runtime}>
      <RunningReporter onRunning={onRunning} />
      <ThreadPrimitive.Root className="chat-thread">
        <ThreadPrimitive.Viewport className="chat-viewport">
          <ThreadPrimitive.Empty><p className="chat-empty">Schreibe eine Nachricht an Claude.</p></ThreadPrimitive.Empty>
          <ThreadPrimitive.Messages>{({ message }) => <Message message={message} />}</ThreadPrimitive.Messages>
        </ThreadPrimitive.Viewport>
        {notice && <div className="chat-note" role="status">{notice}</div>}
        {world && <div className="chat-note chat-task">Welt-Auftrag: Das Grundgerüst für Welten geht mit, schreib nur deinen Wunsch.</div>}
        {!world && assign && <div className="chat-note chat-task" data-testid="chat-assign">Zuordnung: {assignContextLine(assign)}. Vorschläge erscheinen als Karten, geschrieben wird erst nach „Übernehmen“.</div>}
        <ComposerPrimitive.Root className="chat-composer">
          <ComposerPrimitive.Input className="chat-input" placeholder={world ? 'Wunsch, z. B. „Erstelle eine Welt Nordsee“' : 'Nachricht an Claude'} rows={3} />
          <ThreadPrimitive.If running={false}>
            <ComposerPrimitive.Send asChild><Button size="1">Senden</Button></ComposerPrimitive.Send>
          </ThreadPrimitive.If>
          <ThreadPrimitive.If running>
            <ComposerPrimitive.Cancel asChild><Button size="1" color="red">Abbrechen</Button></ComposerPrimitive.Cancel>
          </ThreadPrimitive.If>
        </ComposerPrimitive.Root>
      </ThreadPrimitive.Root>
    </AssistantRuntimeProvider>
  );
}
