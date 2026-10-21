import { ChatGoneError, type ChatEvent, type ChatStatus, type ChatStream } from './chatTypes';

const URL = '/api/studio/chat';
const HEADERS = { 'Content-Type': 'application/json', 'X-K3C-Studio': '1' };

/** Ein Block des Stroms („event: …“ und „data: …“, durch eine Leerzeile getrennt) als Ereignis; Unbekanntes entfällt. */
export function parseChatBlock(block: string): ChatEvent | null {
  let name = '';
  let data = '';
  for (const line of block.split('\n')) {
    if (line.startsWith('event: ')) name = line.slice(7);
    else if (line.startsWith('data: ')) data += line.slice(6);
  }
  if (!name || !data) return null;
  const v = JSON.parse(data) as { chat?: string; text?: string; status?: ChatStatus; error?: string; notice?: string };
  if (name === 'start') return { kind: 'start', chat: v.chat ?? '' };
  if (name === 'chunk') return { kind: 'chunk', text: v.text ?? '' };
  if (name === 'done') return { kind: 'done', status: v.status ?? 'fehler', error: v.error, notice: v.notice };
  return null;
}

async function refusal(res: Response): Promise<string> {
  const body = (await res.json().catch(() => ({}))) as { error?: string };
  return body.error || `Fehler ${res.status} vom Server`;
}

/** POST mit Antwort als Strom; das Schließen der Verbindung (signal) bricht die CLI auf dem Server ab. */
export const chatStreamHttp: ChatStream = async function* (chat, text, options, signal) {
  const res = await fetch(URL, { method: 'POST', headers: HEADERS, body: JSON.stringify({ chat, text, ...options }), signal });
  if (res.status === 410) throw new ChatGoneError(await refusal(res));
  if (!res.ok || !res.body) throw new Error(await refusal(res));
  const reader = res.body.pipeThrough(new TextDecoderStream()).getReader();
  let rest = '';
  for (;;) {
    const { done, value } = await reader.read();
    if (done) return;
    rest += value;
    const blocks = rest.split('\n\n');
    rest = blocks.pop() ?? '';
    for (const block of blocks) {
      const ev = parseChatBlock(block);
      if (ev) yield ev;
    }
  }
};
