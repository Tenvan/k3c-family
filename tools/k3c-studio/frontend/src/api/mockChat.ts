import type { ChatStream } from './chatTypes';

const wait = (ms: number, signal: AbortSignal) =>
  new Promise<void>((resolve) => {
    const timer = setTimeout(resolve, ms);
    signal.addEventListener('abort', () => { clearTimeout(timer); resolve(); }, { once: true });
  });

/** Chat ohne Server (Test, ?mock=1): antwortet in Stücken mit der Nachricht. „!fehler“ scheitert wie eine fehlende CLI,
 *  „!lang“ antwortet, bis jemand abbricht. */
export const mockChatStream: ChatStream = async function* (chat, text, signal) {
  yield { kind: 'start', chat: chat || 'mock-chat' };
  if (text.startsWith('!fehler')) {
    yield { kind: 'done', status: 'fehler', error: 'Claude Code CLI nicht gefunden: „claude“' };
    return;
  }
  const words = text.startsWith('!lang') ? Array.from({ length: 200 }, () => 'wort') : `Antwort auf: ${text}`.split(' ');
  for (const word of words) {
    await wait(5, signal);
    if (signal.aborted) {
      yield { kind: 'done', status: 'abgebrochen', error: 'Abgebrochen' };
      return;
    }
    yield { kind: 'chunk', text: `${word} ` };
  }
  yield { kind: 'done', status: 'fertig' };
};
