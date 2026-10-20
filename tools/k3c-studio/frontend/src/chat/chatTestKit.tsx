import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { ChatPanel, ChatToggle } from './ChatPanel';

// Hilfen der Chat-Tests im DOM (happy-dom, Mock-Backend): Einhängen, Eingabe von Hand, Knöpfe und Auswahlen.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

export const flush = (ms = 0) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });

export async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><ChatToggle /><ChatPanel /></Theme>));
  await waitFor(() => (select(host!, 'Modell')?.options.length ?? 0) > 1); // Optionen vom Mock sind da
  return host;
}

export function unmount() {
  act(() => root?.unmount());
  host?.remove();
  root = null;
  host = null;
}

/** Wie eine Eingabe von Hand: Wert über den nativen Setter, dann input-Ereignis (React hört darauf). */
export async function type(el: HTMLTextAreaElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => { el.dispatchEvent(new Event('input', { bubbles: true })); });
}

/** Wählt in einer Auswahl den Eintrag mit dem Wert `value`. */
export async function choose(el: HTMLSelectElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLSelectElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => { el.dispatchEvent(new Event('change', { bubbles: true })); });
}

export const input = (h: HTMLElement) => h.querySelector<HTMLTextAreaElement>('textarea')!;
export const button = (h: HTMLElement, name: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === name)!;
export const panel = (h: HTMLElement) => h.querySelector<HTMLElement>('[data-testid="chat-panel"]')!;
export const select = (h: HTMLElement, label: string) => h.querySelector<HTMLSelectElement>(`select[aria-label="${label}"]`)!;
export const messages = (h: HTMLElement) => [...h.querySelectorAll('.chat-msg')].map((m) => m.querySelector('.chat-text')?.textContent?.trim() ?? '');

/** Wartet, bis `cond` gilt: kurze Schritte unter act statt fester Zeiten, Fehler nach `ms` (Tests dürfen nicht von der Rechnerlast abhängen). */
export async function waitFor(cond: () => boolean, ms = 3000) {
  const end = Date.now() + ms;
  while (!cond()) {
    if (Date.now() > end) throw new Error('waitFor: Bedingung nicht erreicht');
    await flush(5);
  }
}

/** Die letzte Nachricht ist eine Antwort, die zu Ende ist (fertig, abgebrochen oder Fehler), und „Senden“ steht wieder da. */
export const answered = (h: HTMLElement) => {
  const last = [...h.querySelectorAll('.chat-msg')].at(-1);
  return last?.getAttribute('data-role') === 'assistant' && last.getAttribute('data-status') !== 'running' && !!button(h, 'Senden');
};

/** Schickt `text`; mit `settle` wartet es, bis die Antwort zu Ende ist. */
export async function send(h: HTMLElement, text: string, settle = false) {
  await type(input(h), text);
  await act(async () => button(h, 'Senden').click());
  if (settle) await waitFor(() => answered(h));
}

/** Wählt in der Chat-Auswahl `id` und wartet, bis der Verlauf mit `first` beginnt (der geladene Chat ist da). */
export async function openChat(h: HTMLElement, id: string, first: string) {
  await choose(select(h, 'Gespeicherte Chats'), id);
  await waitFor(() => messages(h)[0] === first);
}
