import { Theme } from '@webkit/templates';
import { optionTexts, shownOptions } from '../../../../webkit/testing/options';
import { pickValue, selectedText, selectedValue } from '../../../../webkit/testing/select';
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
  await waitFor(async () => (await values(host!, 'Modell')).length > 1); // Optionen vom Mock sind da
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

/** Wählt in der Auswahl `label` den Eintrag mit dem Wert `value` (SelectField, Radix Select). */
export const choose = (h: HTMLElement, label: string, value: string) => pickValue(h, label, value);
/** Gewählter Wert der Auswahl `label`. */
export const value = (h: HTMLElement, label: string) => selectedValue(h, label);
/** Angezeigter Text der Auswahl `label` (leer, wenn kein Eintrag zum Wert passt). */
export const shown = (h: HTMLElement, label: string) => selectedText(h, label);
/** Werte bzw. Texte aller Einträge der Auswahl `label`, in Anzeige-Reihenfolge (öffnet und schließt die Liste). */
export const values = async (h: HTMLElement, label: string) => (await shownOptions(h, label)).map((o) => o.value);
export const labels = (h: HTMLElement, label: string) => optionTexts(h, label);

export const input = (h: HTMLElement) => h.querySelector<HTMLTextAreaElement>('textarea')!;
export const button = (h: HTMLElement, name: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === name)!;
export const panel = (h: HTMLElement) => h.querySelector<HTMLElement>('[data-testid="chat-panel"]')!;
export const messages = (h: HTMLElement) => [...h.querySelectorAll('.chat-msg')].map((m) => m.querySelector('.chat-text')?.textContent?.trim() ?? '');

/** Wartet, bis `cond` gilt: kurze Schritte unter act statt fester Zeiten, Fehler nach `ms` (Tests dürfen nicht von der Rechnerlast abhängen). */
export async function waitFor(cond: () => boolean | Promise<boolean>, ms = 3000) {
  const end = Date.now() + ms;
  while (!(await cond())) {
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
  await choose(h, 'Gespeicherte Chats', id);
  await waitFor(() => messages(h)[0] === first);
}
