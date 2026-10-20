import { Button } from '@radix-ui/themes';
import { setChatOpen, useChatOpen } from './chatBridge';
import { ChatControls } from './ChatControls';
import { ChatSplitter, useChatWidth } from './ChatSplitter';
import { ChatThread } from './ChatThread';
import { useChatChoices } from './useChatChoices';
import { useChatSession } from './useChatSession';

/** Der Chat rechts im Studio (B-488, WZ19.2): assistant-ui mit der Claude Code CLI als Modell, auf allen Seiten da,
 *  ein- und ausblendbar, durch eine Trennlinie in der Breite einstellbar. Kopf: Neuer Chat, gespeicherte Chats,
 *  Anbieter/Modell/Denkstufe. Bleibt auch ausgeblendet eingehängt, damit Verlauf und Entwurf stehen bleiben. */
export function ChatPanel() {
  const open = useChatOpen();
  const [width, setWidth, resetWidth] = useChatWidth();
  const choices = useChatChoices();
  const { chats, session, setSession, error, running, setRunning, refresh, onNew, onSelect, onDelete } = useChatSession(choices);

  const key = session.key;
  return (
    <>
      {open && <ChatSplitter width={width} onChange={setWidth} onReset={resetWidth} />}
      <aside className="chat" data-testid="chat-panel" hidden={!open} aria-label="Chat" style={{ flexBasis: width }}>
        <ChatControls chats={chats} chat={session.chat} choices={choices} running={running} error={error}
          onNew={onNew} onSelect={onSelect} onDelete={onDelete} onHide={() => setChatOpen(false)} />
        <ChatThread key={key} chat={session.chat} initial={session.messages} current={choices.current} onEnd={refresh} onRunning={setRunning}
          onChat={(id) => setSession((s) => (s.key === key ? { ...s, chat: id } : s))} />
      </aside>
    </>
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
