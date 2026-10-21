import { ChatGoneError, type ChatApi, type ChatDetail, type ChatOptionsView, type ChatStatus, type ChatStream, type ChatSummary } from './chatTypes';

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

/** Chat ohne Server (Test, ?mock=1): Chats nur im Speicher; antwortet in Stücken mit der Nachricht. „!fehler“ scheitert wie
 *  eine fehlende CLI, „!lang“ antwortet, bis jemand abbricht. */
export function mockChats(): ChatApi {
  const chats = new Map<string, ChatDetail>();
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
    finish(acc.trim(), 'fertig');
    yield { kind: 'done', status: 'fertig' };
  };

  return {
    chatStream,
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
