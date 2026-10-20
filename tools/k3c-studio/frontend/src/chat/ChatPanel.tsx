import { AssistantRuntimeProvider, ComposerPrimitive, ThreadPrimitive, useLocalRuntime, type ThreadMessage } from '@assistant-ui/react';
import { Button } from '@radix-ui/themes';
import { useEffect, useMemo, useState } from 'react';
import { backend } from '../api';
import { createChatAdapter } from './chatAdapter';
import { registerChatFill, setChatOpen, useChatOpen } from './chatBridge';

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
  return (
    <div className="chat-msg" data-role={message.role} data-status={message.role === 'assistant' ? message.status.type : undefined}>
      <div className="chat-who">{WHO[message.role]}</div>
      <div className="chat-text">{messageText(message)}</div>
      {note && <div className="chat-note" role="status">{note}</div>}
    </div>
  );
}

/** Der Chat rechts im Studio (B-488, WZ19.2): assistant-ui mit der Claude Code CLI als Modell, auf allen Seiten da,
 *  ein- und ausblendbar. Bleibt auch ausgeblendet eingehängt, damit Verlauf und Entwurf stehen bleiben. */
export function ChatPanel() {
  const open = useChatOpen();
  const [notice, setNotice] = useState('');
  const adapter = useMemo(() => createChatAdapter((chat, text, signal) => backend.chatStream(chat, text, signal), setNotice), []);
  const runtime = useLocalRuntime(adapter);
  useEffect(() => registerChatFill((text) => runtime.thread.composer.setText(text)), [runtime]);
  return (
    <aside className="chat" data-testid="chat-panel" hidden={!open} aria-label="Chat">
      <AssistantRuntimeProvider runtime={runtime}>
        <ThreadPrimitive.Root className="chat-thread">
          <div className="chat-head">
            <span>Chat</span>
            <Button size="1" variant="ghost" onClick={() => setChatOpen(false)}>Ausblenden</Button>
          </div>
          <ThreadPrimitive.Viewport className="chat-viewport">
            <ThreadPrimitive.Empty><p className="chat-empty">Schreibe eine Nachricht an Claude.</p></ThreadPrimitive.Empty>
            <ThreadPrimitive.Messages>{({ message }) => <Message message={message} />}</ThreadPrimitive.Messages>
          </ThreadPrimitive.Viewport>
          {notice && <div className="chat-note" role="status">{notice}</div>}
          <ComposerPrimitive.Root className="chat-composer">
            <ComposerPrimitive.Input className="chat-input" placeholder="Nachricht an Claude" rows={3} />
            <ThreadPrimitive.If running={false}>
              <ComposerPrimitive.Send asChild><Button size="1">Senden</Button></ComposerPrimitive.Send>
            </ThreadPrimitive.If>
            <ThreadPrimitive.If running>
              <ComposerPrimitive.Cancel asChild><Button size="1" color="red">Abbrechen</Button></ComposerPrimitive.Cancel>
            </ThreadPrimitive.If>
          </ComposerPrimitive.Root>
        </ThreadPrimitive.Root>
      </AssistantRuntimeProvider>
    </aside>
  );
}

/** Knopf der Kopfzeile: blendet den Chat ein oder aus. */
export function ChatToggle() {
  const open = useChatOpen();
  return (
    <Button size="1" variant={open ? 'solid' : 'soft'} onClick={() => setChatOpen(!open)} aria-pressed={open} data-testid="chat-toggle">
      Chat
    </Button>
  );
}
