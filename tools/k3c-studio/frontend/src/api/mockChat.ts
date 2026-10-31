import { ChatGoneError, type ChatAction, type ChatActionApi, type ChatApi, type ChatDetail, type ChatOptionsView, type ChatStatus, type ChatStream, type ChatSummary } from './chatTypes';

const wait = (ms: number, signal: AbortSignal) =>
  new Promise<void>((resolve) => {
    const timer = setTimeout(resolve, ms);
    signal.addEventListener('abort', () => { clearTimeout(timer); resolve(); }, { once: true });
  });

const OPTIONS: ChatOptionsView = {
  providers: [{ id: '', label: 'Standard' }, { id: 'claude-cli', label: 'Claude Code CLI' }],
  models: [{ id: '', label: 'Standard' }, { id: 'fable', label: 'Fable' }, { id: 'opus', label: 'Opus' }, { id: 'sonnet', label: 'Sonnet' }, { id: 'haiku', label: 'Haiku' }],
  efforts: [
    { id: '', label: 'Standard' }, { id: 'low', label: 'Niedrig' }, { id: 'medium', label: 'Mittel' },
    { id: 'high', label: 'Hoch' }, { id: 'xhigh', label: 'Sehr hoch' }, { id: 'max', label: 'Maximum' },
  ],
};

const titleOf = (text: string) => (text.length > 40 ? `${text.slice(0, 40)}…` : text);

/** Aktionskarten ohne Server (B-679): „!aktion“ legt eine ausführbare und eine gesperrte Karte an. */
function mockActions(): ChatActionApi & { propose: (chat: string) => string } {
  const actions = new Map<string, ChatAction[]>();
  let seq = 0;
  const take = (chat: string, id: string) => {
    const card = actions.get(chat)?.find((a) => a.id === id);
    if (!card) throw new Error(`keine Aktion ${id} in diesem Chat`);
    if (card.status !== 'offen') throw new Error(`aktion ${id} ist schon ${card.status}`);
    return card;
  };
  return {
    propose: (chat) => {
      const list = actions.get(chat) ?? [];
      list.push({ id: `a${++seq}`, art: 'welt_anlegen', titel: 'Welt anlegen: Nordsee', details: ['Vorlage: standard'], status: 'offen' },
        { id: `a${++seq}`, art: 'zaubern', titel: 'zaubern', details: [], gesperrt: 'unbekannte Art "zaubern"', status: 'offen' });
      actions.set(chat, list);
      return '\n\n(Studio: 2 Aktionen als Karten vorbereitet, 1 gesperrt; ausgeführt wird nur per Knopf.)';
    },
    chatActions: async (chat) => structuredClone(actions.get(chat) ?? []),
    chatActionRun: async (chat, id) => {
      const card = take(chat, id);
      if (card.gesperrt) throw new Error(`aktion gesperrt: ${card.gesperrt}`);
      Object.assign(card, { status: 'ausgefuehrt', ergebnis: 'gewählt: nordsee' });
      return structuredClone(card);
    },
    chatActionDrop: async (chat, id) => structuredClone(Object.assign(take(chat, id), { status: 'verworfen' as const })),
  };
}

/** Chat ohne Server (Test, ?mock=1): Chats nur im Speicher; antwortet in Stücken mit der Nachricht. „!fehler“ scheitert wie
 *  eine fehlende CLI, „!lang“ antwortet, bis jemand abbricht. */
export function mockChats(): ChatApi {
  const chats = new Map<string, ChatDetail>();
  const actions = mockActions();
  let counter = 0;

  const chatStream: ChatStream = async function* (chat, text, options, signal) {
    if (chat && !chats.has(chat)) throw new ChatGoneError('Der Chat wurde gelöscht'); // wie der Server (HTTP 410)
    const id = chat || `mock-chat-${++counter}`;
    const now = new Date().toISOString();
    const detail = chats.get(id) ?? { id, created: now, updated: now, messages: [] };
    chats.set(id, detail);
    detail.messages.push({ role: 'user', text, time: now });
    Object.assign(detail, { provider: options.provider, model: options.model, effort: options.effort });
    const finish = (answer: string, status: ChatStatus, error?: string) => {
      detail.updated = new Date().toISOString();
      detail.messages.push({ role: 'assistant', text: answer, status, error, time: detail.updated });
    };
    yield { kind: 'start', chat: id };
    if (text.startsWith('!fehler')) {
      const error = 'Claude Code CLI nicht gefunden: „claude“';
      finish('', 'fehler', error);
      yield { kind: 'done', status: 'fehler', error };
      return;
    }
    const words = text.startsWith('!lang') ? Array.from({ length: 200 }, () => 'wort') : `Antwort auf: ${text}`.split(' ');
    let acc = '';
    for (const word of words) {
      await wait(5, signal);
      if (signal.aborted) {
        finish(acc, 'abgebrochen', 'Abgebrochen');
        yield { kind: 'done', status: 'abgebrochen', error: 'Abgebrochen' };
        return;
      }
      acc += `${word} `;
      yield { kind: 'chunk', text: `${word} ` };
    }
    if (text.startsWith('!aktion')) {
      const note = actions.propose(id);
      acc += note;
      yield { kind: 'chunk', text: note };
    }
    finish(acc.trim(), 'fertig');
    yield { kind: 'done', status: 'fertig' };
  };

  return {
    chatActions: actions.chatActions,
    chatActionRun: actions.chatActionRun,
    chatActionDrop: actions.chatActionDrop,
    chatStream,
    ...mockChatStore(chats),
  };
}

function mockChatStore(chats: Map<string, ChatDetail>): Pick<ChatApi, 'chatList' | 'chatGet' | 'chatDelete' | 'chatOptions'> {
  return {
    chatList: async () => [...chats.values()].map((c): ChatSummary => ({
      id: c.id, title: titleOf(c.messages[0]?.text ?? ''), updated: c.updated, count: c.messages.length,
      provider: c.provider, model: c.model, effort: c.effort,
    })),
    chatGet: async (id) => {
      const c = chats.get(id);
      if (!c) throw new Error('Chat nicht gefunden');
      return structuredClone(c);
    },
    chatDelete: async (id) => {
      if (!chats.delete(id)) throw new Error(`kein Chat mit der id ${id}`);
    },
    chatOptions: async () => OPTIONS,
  };
}
