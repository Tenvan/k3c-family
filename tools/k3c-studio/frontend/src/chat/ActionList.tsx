import { ActionButton, Flex, StatusBadge, Text } from '@webkit/atoms';
import { useCallback, useEffect, useState } from 'react';
import { backend } from '../api';
import type { ChatAction } from '../api/chatTypes';

const STATUS_TEXT: Record<ChatAction['status'], string> = {
  offen: 'offen', laeuft: 'läuft', ausgefuehrt: 'ausgeführt', fehler: 'Fehler', verworfen: 'verworfen',
};

function tone(a: ChatAction): 'ok' | 'warn' | 'error' | 'info' {
  if (a.gesperrt || a.status === 'fehler') return 'error';
  if (a.status === 'ausgefuehrt') return 'ok';
  return a.status === 'verworfen' ? 'warn' : 'info';
}

/** Karte einer Aktion (B-679): Titel, Details, Sperrgrund oder Ergebnis; Ausführen und Verwerfen nur, solange sie offen ist. */
export function ActionCard({ action: a, onRun, onDrop }: { action: ChatAction; onRun: (a: ChatAction) => Promise<void>; onDrop: (a: ChatAction) => Promise<void> }) {
  const open = a.status === 'offen';
  return (
    <li className="chat-sug" data-testid="action" data-art={a.art} data-status={a.status}>
      <Flex gap="2" align="center" justify="between" wrap="wrap">
        <Text size="2" weight="bold">{a.titel}</Text>
        <StatusBadge tone={tone(a)}>{a.gesperrt ? 'gesperrt' : STATUS_TEXT[a.status]}</StatusBadge>
      </Flex>
      {a.details.filter((d) => d !== '').map((d, i) => <Text key={i} size="1" color="gray">{d}</Text>)}
      {a.vorschau?.map((p) => (
        <Text key={p.feld} size="1" data-testid="action-preview">
          {p.feld}: Basis {p.basis}{p.alt && p.alt !== '–' ? ` (bisher ${p.alt})` : ''} → {p.neu}
        </Text>
      ))}
      {a.gesperrt && <div className="chat-note chat-sug-block" role="status">Gesperrt: {a.gesperrt}</div>}
      {a.ergebnis && <div className="chat-note" role="status">{a.ergebnis}</div>}
      {open && (
        <Flex gap="2">
          {!a.gesperrt && <ActionButton size="1" onClick={() => onRun(a)}>Ausführen</ActionButton>}
          <ActionButton size="1" variant="soft" color="gray" onClick={() => onDrop(a)}>Verwerfen</ActionButton>
        </Flex>
      )}
    </li>
  );
}

/** Aktionskarten des Chats (B-679) auf jeder Seite: vom Studio vorab geprüft, ausgeführt nur per Knopf. `round` lädt
 *  nach jeder Antwort neu; ohne Chat oder Karten bleibt der Bereich weg. */
export function ActionList({ chat, round = 0 }: { chat: string; round?: number }) {
  const [items, setItems] = useState<ChatAction[]>([]);
  const [note, setNote] = useState('');
  const load = useCallback(async () => {
    if (!chat) { setItems([]); return; }
    try {
      setItems(await backend.chatActions(chat));
      setNote('');
    } catch (e) {
      setNote(e instanceof Error ? e.message : String(e));
    }
  }, [chat]);
  useEffect(() => { void load(); }, [load, round]);

  const replace = (next: ChatAction) => setItems((list) => list.map((x) => (x.id === next.id ? next : x)));
  const act = (fn: typeof backend.chatActionRun) => async (a: ChatAction) => {
    replace({ ...a, status: 'laeuft' }); // kein zweiter Klick, solange der Aufruf läuft
    try {
      replace(await fn(chat, a.id));
      setNote('');
    } catch (e) {
      replace(a);
      setNote(e instanceof Error ? e.message : String(e));
    }
  };
  if (items.length === 0 && !note) return null;
  return (
    <section className="chat-sugs" aria-label="Aktionen" data-testid="actions">
      <div className="chat-head-row"><span className="chat-title">Aktionen</span></div>
      {note && <div className="chat-note" role="status">{note}</div>}
      <ul className="chat-sug-list">
        {items.map((a) => <ActionCard key={a.id} action={a} onRun={act(backend.chatActionRun)} onDrop={act(backend.chatActionDrop)} />)}
      </ul>
    </section>
  );
}
