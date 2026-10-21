import { Button, SelectField, type SelectOption } from '@webkit/atoms';
import type { ChatChoice, ChatChoiceValues, ChatSummary } from '../api/chatTypes';
import { chatLabel } from './chatConvert';
import { choiceList, type ChatChoices } from './useChatChoices';

const CHOICES: { field: keyof ChatChoiceValues; label: string }[] = [
  { field: 'provider', label: 'Anbieter' },
  { field: 'model', label: 'Modell' },
  { field: 'effort', label: 'Denkstufe' },
];

function Choice({ label, value, list, onChange }: { label: string; value: string; list: ChatChoice[]; onChange: (v: string) => void }) {
  return (
    <label className="chat-choice">
      <span>{label}</span>
      <SelectField className="chat-select" size="1" aria-label={label} value={value} options={list.map((c) => ({ value: c.id, label: c.label }))} onChange={onChange} />
    </label>
  );
}

/** Einträge der Chat-Auswahl: „Neuer Chat“ (leer), der noch nicht gelistete aktuelle Chat, dann die gespeicherten. */
function chatOptions(chats: readonly ChatSummary[], chat: string): SelectOption<string>[] {
  const listed = chats.some((c) => c.id === chat);
  return [
    { value: '', label: 'Neuer Chat' },
    ...(chat && !listed ? [{ value: chat, label: 'Aktueller Chat' }] : []),
    ...chats.map((c) => ({ value: c.id, label: chatLabel(c) })),
  ];
}

export interface ChatControlsProps {
  /** gespeicherte Chats, der neueste zuerst */
  chats: readonly ChatSummary[];
  /** Kennung des aktuellen Chats, leer = noch nicht gespeichert */
  chat: string;
  choices: ChatChoices;
  /** eine Antwort läuft: Löschen ist gesperrt */
  running: boolean;
  /** Fehler beim Laden oder Löschen */
  error: string;
  onNew: () => void;
  onSelect: (id: string) => void;
  onDelete: () => void;
  onHide: () => void;
}

/** Kopf des Chats: Neuer Chat, Auswahl gespeicherter Chats, Löschen und die Wahl von Anbieter, Modell und Denkstufe. */
export function ChatControls({ chats, chat, choices, running, error, onNew, onSelect, onDelete, onHide }: ChatControlsProps) {
  return (
    <div className="chat-head">
      <div className="chat-head-row">
        <span className="chat-title">Chat</span>
        <Button size="1" variant="soft" onClick={onNew}>Neuer Chat</Button>
        <Button size="1" variant="soft" color="red" disabled={!chat || running} onClick={onDelete}>Chat löschen</Button>
        <Button size="1" variant="ghost" onClick={onHide}>Ausblenden</Button>
      </div>
      <SelectField className="chat-select" size="1" aria-label="Gespeicherte Chats" value={chat} options={chatOptions(chats, chat)}
        onChange={(v) => (v ? onSelect(v) : onNew())} />
      <div className="chat-head-row chat-choices">
        {CHOICES.map(({ field, label }) => (
          <Choice key={field} label={label} value={choices.values[field]} list={choiceList(choices.view, field)} onChange={(v) => choices.set(field, v)} />
        ))}
      </div>
      {error && <div className="chat-note chat-head-note" role="status">{error}</div>}
    </div>
  );
}
